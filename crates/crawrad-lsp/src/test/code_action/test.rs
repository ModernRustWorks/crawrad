//! Code actions: what can be applied, the diagnostics of a line, the offer.

use super::{CodeActionAnswer, Offer, on_line, parse};
use serde_json::json;
use std::path::Path;

/// What rust-analyzer sent (captured 2026-10-04) for `HashMap::new()`
/// without its `use`, without `resolveSupport`: the edit inline.
#[test]
fn the_import_comes_with_its_edit() {
    let answer = json!([
        { "title": "Import `std::collections::HashMap`", "kind": "quickfix",
          "edit": { "documentChanges": [{
              "textDocument": { "version": 1, "uri": "file:///c:/p/src/main.rs" },
              "edits": [{ "range": { "start": { "line": 0, "character": 0 },
                                     "end": { "line": 0, "character": 0 } },
                          "newText": "use std::collections::HashMap;\n\n" }] }] } },
        { "title": "Qualify as `std::collections::HashMap`", "kind": "quickfix",
          "edit": { "changes": { "file:///c:/p/src/main.rs": [{
              "range": { "start": { "line": 1, "character": 33 },
                         "end": { "line": 1, "character": 40 } },
              "newText": "std::collections::HashMap" }] } } }
    ]);
    let actions = parse(&answer);
    assert_eq!(actions.len(), 2);
    assert_eq!(actions[0].title, "Import `std::collections::HashMap`");
    assert_eq!(actions[0].kind.as_deref(), Some("quickfix"));
    let edits = &actions[0].edit["file:///c:/p/src/main.rs"];
    assert_eq!(edits[0].new_text, "use std::collections::HashMap;\n\n");
}

/// **Only what the IDE can apply:** a bare `Command`, an action with only a
/// command, a `disabled` one, an empty edit and a file operation are left out;
/// `isPreferred` is read.
#[test]
fn commands_disabled_and_file_operations_are_left_out() {
    let edit = json!({ "changes": { "file:///a.rs": [{
        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 0 } },
        "newText": "x" }] } });
    let answer = json!([
        { "title": "Run", "command": "rust-analyzer.runSingle" },
        { "title": "Only a command", "command": { "title": "t", "command": "c" } },
        { "title": "Disabled", "disabled": { "reason": "no" }, "edit": edit },
        { "title": "Empty", "edit": { "changes": {} } },
        { "title": "Create", "edit": { "documentChanges": [{ "kind": "create", "uri": "file:///b.rs" }] } },
        { "title": "Fix", "isPreferred": true, "edit": edit }
    ]);
    let actions = parse(&answer);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].title, "Fix");
    assert!(actions[0].preferred);
    assert!(parse(&json!(null)).is_empty());
}

/// The diagnostics whose range touches the line, multi-line ones included.
#[test]
fn the_diagnostics_of_a_line() {
    let d = |s: u32, e: u32| {
        json!({ "range": { "start": { "line": s, "character": 0 },
                                                "end": { "line": e, "character": 1 } },
                                     "message": format!("{s}-{e}") })
    };
    let published = vec![d(1, 1), d(2, 4), d(5, 5)];
    let on3: Vec<_> = on_line(&published, 3)
        .iter()
        .map(|v| v["message"].clone())
        .collect();
    assert_eq!(on3, vec![json!("2-4")]);
    assert_eq!(on_line(&published, 1).len(), 1);
    assert!(on_line(&published, 0).is_empty());
}

/// **The offer:** a place asked is not asked again; an old ticket's answer
/// is dropped; the answer is kept for its place only — and the place is the
/// caret's byte, not only its line.
#[test]
fn a_place_is_asked_once_and_kept() {
    let f = || Path::new("a.rs").to_path_buf();
    let mut offer = Offer::default();
    let old = offer.ask((f(), 1, (4, 0)), false);
    let new = offer.ask((f(), 1, (4, 9)), true);
    assert!(offer.knows(&(f(), 1, (4, 9))));
    assert!(
        !offer.knows(&(f(), 1, (4, 3))),
        "the same line, another byte"
    );
    let answer = |ticket| CodeActionAnswer {
        ticket,
        file: f(),
        version: 1,
        caret: (4, 9),
        actions: Ok(Vec::new()),
    };
    assert_eq!(
        offer.answered(answer(old)),
        None,
        "the old ticket is dropped"
    );
    assert_eq!(offer.answered(answer(new)), Some(true), "asked by key");
    assert_eq!(offer.at(&(f(), 1, (4, 9))).map(<[_]>::len), Some(0));
    assert!(
        offer.at(&(f(), 2, (4, 9))).is_none(),
        "another version is another place"
    );
    assert!(!offer.knows(&(f(), 2, (4, 9))));
}
