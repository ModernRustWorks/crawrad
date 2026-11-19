//! Unit tests for view_tool.

use super::{VALID_PANEL_NAMES, panel_shown, parse_panel_name};
use crawrad_actions::Panel;
use serde_json::json;

#[test]
fn parse_panel_name_resolves_all_standard_names_and_aliases() {
    assert_eq!(parse_panel_name("Explorer"), Some(Panel::Explorer));
    assert_eq!(parse_panel_name("explorer"), Some(Panel::Explorer));
    assert_eq!(parse_panel_name("Inspector"), Some(Panel::Inspector));
    assert_eq!(parse_panel_name("object inspector"), Some(Panel::Inspector));
    assert_eq!(parse_panel_name("Palette"), Some(Panel::Components));
    assert_eq!(parse_panel_name("Structure"), Some(Panel::Components));
    assert_eq!(parse_panel_name("Components"), Some(Panel::Components));
    assert_eq!(parse_panel_name("packages"), Some(Panel::Packages));
    assert_eq!(parse_panel_name("Debug"), Some(Panel::Debug));
    assert_eq!(parse_panel_name("run and debug"), Some(Panel::Debug));
    assert_eq!(parse_panel_name("Extensions"), Some(Panel::Extensions));
    assert_eq!(parse_panel_name("chat"), Some(Panel::Chat));
    assert_eq!(parse_panel_name("terminal"), Some(Panel::Terminal));
    assert_eq!(parse_panel_name("references"), Some(Panel::References));
    assert_eq!(parse_panel_name("problems"), Some(Panel::Problems));
    assert_eq!(parse_panel_name("Problems"), Some(Panel::Problems));
    assert_eq!(parse_panel_name("design"), Some(Panel::Design));
    assert_eq!(parse_panel_name("Notes"), Some(Panel::Notes));
    assert_eq!(parse_panel_name("notes"), Some(Panel::Notes));

    assert_eq!(parse_panel_name(""), None);
    assert_eq!(parse_panel_name("   "), None);
    assert_eq!(parse_panel_name("nonexistent_panel"), None);
}

#[test]
fn panel_shown_produces_success_answer() {
    let (res, outcome) = panel_shown(Panel::Inspector);
    assert_eq!(outcome, "applied");
    assert!(!res.is_error);
    let data = res.data.unwrap();
    assert_eq!(data["panel"], json!("inspector"));
    assert_eq!(data["label"], json!("Object Inspector"));
    assert_eq!(data["status"], json!("ok"));
}

#[test]
fn valid_panel_names_contains_primary_rail_and_dock_names() {
    assert!(VALID_PANEL_NAMES.contains(&"Explorer"));
    assert!(VALID_PANEL_NAMES.contains(&"Inspector"));
    assert!(VALID_PANEL_NAMES.contains(&"Palette"));
    assert!(VALID_PANEL_NAMES.contains(&"Packages"));
    assert!(VALID_PANEL_NAMES.contains(&"Debug"));
}

#[test]
fn drive_parses_each_tool_and_refuses_the_unknown() {
    use super::{Drive, done, drive};
    let known = |id: &str| id == "tools.options";
    let code = |a: crate::run_tool::Answer| a.0.data.unwrap()["code"].clone();

    assert!(drive("designer.list_components", &json!({}), None, known).is_none());
    assert_eq!(
        drive(
            "ide.run_action",
            &json!({ "id": "tools.options" }),
            None,
            known
        )
        .and_then(Result::ok),
        Some(Drive::RunAction("tools.options".into()))
    );
    let unknown = drive("ide.run_action", &json!({ "id": "nope" }), None, known);
    assert_eq!(code(unknown.unwrap().unwrap_err()), json!("unknown-action"));
    let missing = drive("ide.run_action", &json!({}), None, known);
    assert_eq!(code(missing.unwrap().unwrap_err()), json!("invalid-params"));
    assert_eq!(
        drive(
            "view.show_panel",
            &json!({ "name": "inspector" }),
            None,
            known
        )
        .and_then(Result::ok),
        Some(Drive::ShowPanel(Panel::Inspector))
    );
    let bad = drive("view.show_panel", &json!({ "name": "nope" }), None, known);
    assert_eq!(code(bad.unwrap().unwrap_err()), json!("unknown-panel"));
    let no_doc = drive("designer.select", &json!({ "id": "Button1" }), None, known);
    assert_eq!(code(no_doc.unwrap().unwrap_err()), json!("no-form"));

    let (ran, outcome) = done(&Drive::RunAction("tools.options".into()), None);
    assert_eq!(
        (outcome, ran.data.unwrap()["action"].clone()),
        ("applied", json!("tools.options"))
    );
}
