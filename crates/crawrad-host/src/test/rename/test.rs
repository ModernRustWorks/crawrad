//! The RAD guard of Rename Symbol and the dialog's stages.

use super::{
    ComponentRename, FileChange, Owner, Refs, Rename, Stage, find_default_handlers,
    find_field_caret, forbidden, form_owner,
};
use crawrad_ir::{Document, Node};
use std::path::{Path, PathBuf};

fn form() -> Document {
    let mut doc = Document::new("form_main", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    doc.add_child("form_main", btn).unwrap();
    doc
}

/// **A component's name and a bound handler belong to the form; an
/// ordinary name does not.**
#[test]
fn the_form_owns_its_components_and_its_handlers() {
    let doc = form();
    let forms = [("form_main".to_string(), &doc)];
    assert_eq!(
        form_owner("btn_ok", &forms),
        Some(Owner::Component {
            form: "form_main".into(),
            name: "btn_ok".into()
        })
    );
    assert_eq!(
        form_owner("btn_ok_click", &forms),
        Some(Owner::Handler {
            form: "form_main".into(),
            component: "btn_ok".into(),
            event: "on_click".into()
        })
    );
    assert_eq!(form_owner("area", &forms), None);
    // the form's own root is not a component to refuse by name
    assert_eq!(form_owner("form_main", &forms), None);
}

/// **The refusals give the right road: the Inspector's Name for a
/// component, its Events (and the issue) for a handler.**
#[test]
fn the_refusal_names_the_road() {
    let doc = form();
    let forms = [("form_main".to_string(), &doc)];
    let c = form_owner("btn_ok", &forms).unwrap().refusal();
    assert!(
        c.contains("Object Inspector (Name") && c.contains("component.rename"),
        "{c}"
    );
    let h = form_owner("btn_ok_click", &forms).unwrap().refusal();
    assert!(
        h.contains("btn_ok.on_click") && h.contains("event.rename"),
        "{h}"
    );
}

/// **A generated source, a file outside the project and a toolchain file
/// are forbidden; the project's own files are not.**
#[test]
fn forbidden_files_are_the_generated_the_outside_and_the_foreign() {
    let project = Path::new("C:/p");
    let files = [
        PathBuf::from("C:/p/src/main.rs"),
        PathBuf::from("C:/p/src/form_main.designer.rs"),
        PathBuf::from("C:/other/lib.rs"),
        PathBuf::from("C:/Users/u/.cargo/registry/x/lib.rs"),
    ];
    let foreign = [PathBuf::from("C:/Users/u/.cargo/registry")];
    let got = forbidden(&files, project, &foreign, ".designer.rs");
    assert_eq!(got, files[1..].to_vec());
    assert!(forbidden(&files[..1], project, &foreign, ".designer.rs").is_empty());
}

/// **The stages: asked, named, previewed; typing again drops the preview;
/// the same name or an empty one cannot be asked.**
#[test]
fn the_dialog_walks_its_stages() {
    let mut r = Rename::default();
    assert!(!r.is_open());
    let t = r.start(PathBuf::from("C:/p/src/area.rs"), (3, 8), 4, "area");
    assert_eq!(
        (r.stage.clone(), r.new.as_str(), r.pending.ticket()),
        (Stage::Preparing, "area", Some(t))
    );
    r.stage = Stage::Naming;
    assert_eq!(
        r.cannot_ask(),
        Some("The new name is the same as the old one")
    );
    r.typed("  ");
    assert_eq!(r.cannot_ask(), Some("Type the new name"));
    r.typed("size");
    assert_eq!(r.cannot_ask(), None);
    r.stage = Stage::Preview(vec![
        FileChange {
            file: "C:/p/src/area.rs".into(),
            count: 1,
            before: "a".into(),
            after: "b".into(),
        },
        FileChange {
            file: "C:/p/src/enc.rs".into(),
            count: 2,
            before: "a".into(),
            after: "b".into(),
        },
    ]);
    let p = r.preview();
    assert!(p.starts_with("3 occurrence(s) in 2 file(s)"), "{p}");
    assert!(p.contains("enc.rs  \u{2014}  2"), "{p}");
    assert_eq!(r.to_json()["files"][1]["count"], 2);
    r.typed("size2");
    assert_eq!(r.stage, Stage::Naming);
    r.close();
    assert!(!r.is_open() && r.old.is_empty());
}

#[test]
fn apply_preview_and_refusal_helpers() {
    let mut r = Rename::default();
    r.apply_preview(Ok(vec![]));
    assert_eq!(r.stage, Stage::Naming);
    assert_eq!(r.message, "Nothing to rename");

    r.apply_preview(Err(PathBuf::from("missing.rs")));
    assert_eq!(r.stage, Stage::Naming);
    assert!(r.message.contains("Cannot read"));

    let doc = form();
    let forms = [("form_main".to_string(), &doc)];
    assert!(super::check_form_owner_refusal("btn_ok", &forms).is_some());
    assert!(super::check_form_owner_refusal("regular_func", &forms).is_none());
}

#[test]
fn component_rename_needs_card_false_when_no_handlers_and_refs_found_empty() {
    let cr = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![],
        Refs::Found(vec![]),
    );
    assert!(!cr.needs_card());

    let cr_with_handler = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![("btn1_click".into(), "btn_ok_click".into())],
        Refs::Found(vec![]),
    );
    assert!(cr_with_handler.needs_card());

    let cr_with_refs = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![],
        Refs::Found(vec![FileChange {
            file: PathBuf::from("a.rs"),
            count: 2,
            before: "btn1".into(),
            after: "btn_ok".into(),
        }]),
    );
    assert!(cr_with_refs.needs_card());
}

#[test]
fn component_rename_handler_line_format() {
    let cr = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![("btn1_click".into(), "btn_ok_click".into())],
        Refs::Loading,
    );
    assert_eq!(cr.handler_line(), "btn1_click \u{2192} btn_ok_click");

    let cr_multiple = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![
            ("btn1_click".into(), "btn_ok_click".into()),
            ("btn1_enter".into(), "btn_ok_enter".into()),
        ],
        Refs::Loading,
    );
    assert_eq!(
        cr_multiple.handler_line(),
        "btn1_click \u{2192} btn_ok_click, btn1_enter \u{2192} btn_ok_enter"
    );
}

#[test]
fn component_rename_refs_line_loading_and_found() {
    let cr_loading = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![],
        Refs::Loading,
    );
    assert_eq!(cr_loading.refs_line(), "rust-analyzer is still loading");

    let cr_unavailable = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![],
        Refs::Unavailable,
    );
    assert_eq!(cr_unavailable.refs_line(), "rust-analyzer is still loading");

    let cr_found = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![],
        Refs::Found(vec![
            FileChange {
                file: PathBuf::from("C:/p/src/a.rs"),
                count: 3,
                before: "btn1".into(),
                after: "btn_ok".into(),
            },
            FileChange {
                file: PathBuf::from("C:/p/src/b.rs"),
                count: 1,
                before: "btn1".into(),
                after: "btn_ok".into(),
            },
        ]),
    );
    assert_eq!(
        cr_found.refs_line(),
        "4 places in a.rs, b.rs (rust-analyzer)"
    );
}

#[test]
fn component_rename_edits_to_apply_and_everywhere() {
    let changes = vec![FileChange {
        file: PathBuf::from("a.rs"),
        count: 2,
        before: "btn1".into(),
        after: "btn_ok".into(),
    }];
    let mut cr = ComponentRename::new(
        "btn1".into(),
        "btn1".into(),
        "btn_ok".into(),
        "form_main.designer.rs".into(),
        vec![("btn1_click".into(), "btn_ok_click".into())],
        Refs::Found(changes.clone()),
    );
    assert!(cr.everywhere());
    assert_eq!(cr.edits_to_apply(), changes.as_slice());

    cr.rename_refs = false;
    assert_eq!(cr.edits_to_apply(), &[]);

    cr.rename_handlers = false;
    assert!(!cr.everywhere());
}

#[test]
fn find_default_handlers_matches_convention() {
    let mut events = std::collections::BTreeMap::new();
    events.insert("on_click".into(), "btn1_click".into());
    events.insert("on_enter".into(), "btn1_enter".into());
    events.insert("custom_event".into(), "my_custom_code".into());

    let found = find_default_handlers(&events, "btn1", "btn_ok");
    assert_eq!(
        found,
        vec![
            ("btn1_click".into(), "btn_ok_click".into()),
            ("btn1_enter".into(), "btn_ok_enter".into()),
        ]
    );
}

#[test]
fn find_field_caret_points_at_the_field_name() {
    let code = "
pub struct FormMain {
    pub btn1: Button,
    pub label1: Label,
}
";
    assert_eq!(find_field_caret(code, "btn1"), Some((2, 8)));
    assert_eq!(find_field_caret(code, "btn9"), None);
}
