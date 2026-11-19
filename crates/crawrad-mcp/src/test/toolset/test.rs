use super::*;

#[test]
fn every_listed_tool_belongs_to_some_group() {
    let orphans: Vec<String> = crate::tools::build_tools_list()
        .into_iter()
        .map(|t| t.name)
        .filter(|name| {
            // `extensions` takes any prefix no IDE group knows: it does not count
            GROUPS
                .iter()
                .filter(|(g, _)| *g != EXTENSIONS)
                .all(|(g, _)| !Toolset::parse(g).unwrap().allows(name))
        })
        .collect();
    assert!(orphans.is_empty(), "tools no group carries: {orphans:?}");
}

#[test]
fn extension_tools_ride_in_their_own_group_and_only_there() {
    let ext = "acme.linter.tool.analyze";
    assert!(is_extension_tool(ext));
    assert!(Toolset::parse("extensions").unwrap().allows(ext));
    assert!(
        Toolset::parse("code,extensions")
            .unwrap()
            .allows("editor.go_to")
    );
    for (g, _) in GROUPS.iter().filter(|(g, _)| *g != EXTENSIONS) {
        assert!(!Toolset::parse(g).unwrap().allows(ext), "{g} carries {ext}");
    }
    // the extension's id may look like an IDE area: still the extension's
    assert!(!Toolset::parse("code").unwrap().allows("editor.tool.lint"));
    assert!(!is_extension_tool(".tool.x") && !is_extension_tool("a.tool."));
    // a prefix no IDE group knows is an extension's, even before tools/list
    assert!(Toolset::parse("extensions").unwrap().allows("todos.list"));
    assert!(!Toolset::parse("code,project").unwrap().allows("todos.list"));
}

#[test]
fn the_list_teaches_which_tools_came_from_extensions() {
    let list = || {
        serde_json::json!({"jsonrpc": "2.0", "id": 1, "result": {
            "tools": [{"name": "editor.go_to"}, {"name": "editor.lint"}, {"name": "todos.list"}],
            "_meta": {"version": "1.0.0", "extensionTools": ["editor.lint", "todos.list"]}}})
    };
    let names = |resp: &serde_json::Value| -> Vec<String> {
        resp["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap().to_string())
            .collect()
    };
    // `editor.lint` looks like the editor's, but an extension sent it
    let mut code = Toolset::parse("code").unwrap();
    let mut resp = list();
    code.filter_list_response(&mut resp);
    assert_eq!(names(&resp), ["editor.go_to"]);
    assert!(!code.allows("editor.lint"));
    let mut ext = Toolset::parse("extensions").unwrap();
    let mut resp = list();
    ext.filter_list_response(&mut resp);
    assert_eq!(names(&resp), ["editor.lint", "todos.list"]);
    let call = serde_json::json!({"jsonrpc": "2.0", "id": 2, "method": "tools/call",
        "params": {"name": "editor.lint"}});
    assert!(ext.refuse_call(&call).is_none() && code.refuse_call(&call).is_some());
}

#[test]
fn the_ide_lists_what_its_extensions_contributed() {
    let result = crate::tools::tools_list_result(None);
    assert_eq!(result["_meta"]["extensionTools"], serde_json::json!([]));
    assert!(result["tools"].as_array().is_some_and(|t| t.len() > 100));
}

#[test]
fn no_ide_tool_is_named_like_an_extension_tool() {
    let clash: Vec<String> = crate::tools::build_tools_list()
        .into_iter()
        .map(|t| t.name)
        .filter(|n| is_extension_tool(n))
        .collect();
    assert!(
        clash.is_empty(),
        "IDE tools read as extension tools: {clash:?}"
    );
}

#[test]
fn the_groups_narrow_and_all_lets_everything_through() {
    let design = Toolset::parse("design").unwrap();
    assert!(design.allows("designer.add_component"));
    assert!(design.allows("view.toggle_code"));
    assert!(!design.allows("debug.step_over"));
    let both = Toolset::parse(" design , debug ").unwrap();
    assert!(both.allows("debug.step_over"));
    assert_eq!(both.describe(), "design, debug");
    assert_eq!(Toolset::parse("all").unwrap(), Toolset::all());
    assert_eq!(Toolset::parse("").unwrap(), Toolset::all());
    assert!(Toolset::all().allows("anything.new"));
}

#[test]
fn an_unknown_group_is_refused_with_the_valid_ones() {
    let err = Toolset::parse("design,ui").unwrap_err();
    assert!(
        err.contains("'ui'") && err.contains("debug") && err.contains("all"),
        "{err}"
    );
}

#[test]
fn the_list_response_keeps_only_the_groups_tools() {
    let mut resp = serde_json::json!({"jsonrpc": "2.0", "id": 1, "result": {"tools": [
        {"name": "debug.step_over"}, {"name": "designer.add_component"}, {"name": "lsp.hover"}
    ]}});
    Toolset::parse("debug")
        .unwrap()
        .filter_list_response(&mut resp);
    let names: Vec<&str> = resp["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["debug.step_over"]);
}

#[test]
fn a_call_outside_the_groups_is_refused_and_inside_goes_through() {
    let set = Toolset::parse("code").unwrap();
    let out = serde_json::json!({"jsonrpc": "2.0", "id": 7, "method": "tools/call",
        "params": {"name": "debug.step_over", "arguments": {}}});
    let err = set.refuse_call(&out).expect("refused");
    assert_eq!(err["id"], 7);
    assert_eq!(err["error"]["code"], -32602);
    let inside = serde_json::json!({"jsonrpc": "2.0", "id": 8, "method": "tools/call",
        "params": {"name": "editor.go_to"}});
    assert!(set.refuse_call(&inside).is_none());
    let list = serde_json::json!({"jsonrpc": "2.0", "id": 9, "method": "tools/list"});
    assert!(set.refuse_call(&list).is_none());
}
