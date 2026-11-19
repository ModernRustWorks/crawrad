use super::*;

#[test]
fn every_builtin_has_a_stop_and_a_final_caret() {
    for t in builtin() {
        assert!(t.body.contains("${1:"), "{} has no first stop", t.trigger);
        assert!(t.body.contains("$0"), "{} has no final caret", t.trigger);
        assert!(!crate::snippet::expand(&t.body, "").stops.is_empty());
    }
}

#[test]
fn builtin_triggers_are_unique() {
    let list = builtin();
    for (i, t) in list.iter().enumerate() {
        assert!(
            !list[i + 1..].iter().any(|o| o.trigger == t.trigger),
            "{}",
            t.trigger
        );
    }
}

#[test]
fn the_users_templates_come_after_the_shipped_ones() {
    let user = vec![(
        "evlog".to_string(),
        "Event with log".to_string(),
        "x$0".to_string(),
    )];
    let list = all(&user);
    assert_eq!(list.len(), builtin().len() + 1);
    let last = list.last().expect("one");
    assert_eq!(
        (last.trigger.as_str(), last.origin),
        ("evlog", TemplateOrigin::Mine)
    );
    assert_eq!(list[0].origin, TemplateOrigin::BuiltIn);
}

#[test]
fn a_row_is_a_snippet_with_its_preview_as_doc() {
    let rows = items(&builtin());
    let evt = rows.iter().find(|r| r.rotulo == "evt").expect("evt");
    assert!(evt.snippet);
    assert_eq!(evt.kind, ItemKind::Snippet);
    assert_eq!(evt.origin.as_deref(), Some("Built-in"));
    assert_eq!(evt.detalhe.as_deref(), Some("Event handler"));
    assert_eq!(
        evt.doc.as_deref(),
        Some("pub fn btn_ok_click(&mut self) {\n    \n}")
    );
}

#[test]
fn the_preview_fills_the_stops_with_their_placeholders() {
    assert_eq!(preview("pub fn ${1:a}() {}"), "pub fn a() {}");
}

#[test]
fn manager_starts_with_first_builtin_selected() {
    let mgr = TemplateManager::new(vec![]);
    assert_eq!(mgr.selected, Selected::BuiltIn(0));
    assert!(mgr.problem.is_none());
    let (tr, desc, body, read_only) = mgr.fields();
    assert_eq!(tr, "evt");
    assert_eq!(desc, "Event handler");
    assert!(body.contains("${1:btn_ok}"));
    assert!(read_only);
}

#[test]
fn manager_rows_lists_builtins_then_mine_with_selection_flag() {
    let custom = vec![(
        "my_tpl".to_string(),
        "My custom snippet".to_string(),
        "body $0".to_string(),
    )];
    let mut mgr = TemplateManager::new(custom.clone());
    let rows = mgr.rows();
    assert_eq!(rows.len(), builtin().len() + 1);
    assert_eq!(rows[0].origin, TemplateOrigin::BuiltIn);
    assert!(rows[0].selected);
    assert_eq!(rows.last().unwrap().origin, TemplateOrigin::Mine);
    assert!(!rows.last().unwrap().selected);

    mgr.select(Selected::Mine(0));
    let rows_after = mgr.rows();
    assert!(!rows_after[0].selected);
    assert!(rows_after.last().unwrap().selected);
}

#[test]
fn manager_edits_apply_only_to_mine() {
    let custom = vec![("orig".to_string(), "desc".to_string(), "body".to_string())];
    let mut mgr = TemplateManager::new(custom);

    // Editing while built-in is selected is a no-op
    mgr.set_trigger("hacked");
    mgr.set_description("hacked");
    mgr.set_body("hacked");
    let (tr, _, _, read_only) = mgr.fields();
    assert_eq!(tr, "evt");
    assert!(read_only);

    // Select Mine entry and edit
    mgr.select(Selected::Mine(0));
    let (tr, desc, body, read_only) = mgr.fields();
    assert_eq!(tr, "orig");
    assert_eq!(desc, "desc");
    assert_eq!(body, "body");
    assert!(!read_only);

    mgr.set_trigger("updated");
    mgr.set_description("new desc");
    mgr.set_body("new body $0");
    let (tr, desc, body, read_only) = mgr.fields();
    assert_eq!(tr, "updated");
    assert_eq!(desc, "new desc");
    assert_eq!(body, "new body $0");
    assert!(!read_only);
}

#[test]
fn manager_add_new_allocates_first_free_trigger() {
    let mut mgr = TemplateManager::new(vec![]);
    mgr.add_new();
    assert_eq!(mgr.selected, Selected::Mine(0));
    assert_eq!(mgr.fields(), ("new", "", "$0", false));

    mgr.add_new();
    assert_eq!(mgr.selected, Selected::Mine(1));
    assert_eq!(mgr.fields(), ("new2", "", "$0", false));

    mgr.add_new();
    assert_eq!(mgr.selected, Selected::Mine(2));
    assert_eq!(mgr.fields(), ("new3", "", "$0", false));
}

#[test]
fn manager_duplicate_allocates_first_free_trigger() {
    let mut mgr = TemplateManager::new(vec![]);
    // Select built-in "evt"
    mgr.select(Selected::BuiltIn(0));
    mgr.duplicate();
    assert_eq!(mgr.selected, Selected::Mine(0));
    let (tr, desc, body, read_only) = mgr.fields();
    assert_eq!(tr, "evt2");
    assert_eq!(desc, "Event handler");
    assert!(body.contains("${1:btn_ok}"));
    assert!(!read_only);

    // Duplicate again from the new Mine entry
    mgr.duplicate();
    assert_eq!(mgr.selected, Selected::Mine(1));
    let (tr, _, _, _) = mgr.fields();
    assert_eq!(tr, "evt22");
}

#[test]
fn manager_delete_removes_entry_and_adjusts_selection() {
    let mut mgr = TemplateManager::new(vec![]);
    // Delete on built-in is a no-op
    mgr.delete();
    assert_eq!(mgr.selected, Selected::BuiltIn(0));

    mgr.add_new(); // Mine(0): "new"
    mgr.add_new(); // Mine(1): "new2"
    mgr.add_new(); // Mine(2): "new3"

    // Delete middle item
    mgr.select(Selected::Mine(1));
    mgr.delete();
    assert_eq!(mgr.mine.len(), 2);
    assert_eq!(mgr.selected, Selected::Mine(1));
    assert_eq!(mgr.fields().0, "new3");

    // Delete last item
    mgr.delete();
    assert_eq!(mgr.mine.len(), 1);
    assert_eq!(mgr.selected, Selected::Mine(0));
    assert_eq!(mgr.fields().0, "new");

    // Delete remaining item
    mgr.delete();
    assert!(mgr.mine.is_empty());
    assert_eq!(mgr.selected, Selected::BuiltIn(0));
}

#[test]
fn manager_import_adds_and_replaces_by_trigger() {
    let initial = vec![
        (
            "alpha".to_string(),
            "original alpha".to_string(),
            "a".to_string(),
        ),
        (
            "beta".to_string(),
            "original beta".to_string(),
            "b".to_string(),
        ),
    ];
    let mut mgr = TemplateManager::new(initial);

    let imported = vec![
        (
            "beta".to_string(),
            "imported beta".to_string(),
            "b2".to_string(),
        ),
        (
            "gamma".to_string(),
            "new gamma".to_string(),
            "g".to_string(),
        ),
    ];
    mgr.import(imported);

    assert_eq!(mgr.mine.len(), 3);
    assert_eq!(
        mgr.mine[1],
        (
            "beta".to_string(),
            "imported beta".to_string(),
            "b2".to_string()
        )
    );
    assert_eq!(
        mgr.mine[2],
        (
            "gamma".to_string(),
            "new gamma".to_string(),
            "g".to_string()
        )
    );
}

#[test]
fn manager_validate_detects_empty_and_duplicate_triggers() {
    let mut mgr = TemplateManager::new(vec![]);
    assert!(mgr.validate().is_ok());

    mgr.add_new(); // "new"
    assert!(mgr.validate().is_ok());

    mgr.set_trigger("   ");
    assert!(mgr.validate().is_err());

    mgr.set_trigger("valid");
    assert!(mgr.validate().is_ok());

    mgr.add_new(); // "new"
    mgr.set_trigger("valid");
    assert!(mgr.validate().is_err());
}
