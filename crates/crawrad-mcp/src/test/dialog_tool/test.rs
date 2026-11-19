//! Unit tests for dialog_tool.

use super::{DialogOp, done, plan};
use crate::dialogs::{NEW_PROJECT, OPTIONS};
use crate::protocol::{ToolCallResult, ToolContent};
use serde_json::{Value, json};

fn refusal_code_and_message(res: super::Answer) -> (Value, String) {
    let data = res.0.data.unwrap_or(Value::Null);
    let msg = data["message"].as_str().unwrap_or_default().to_string();
    (data["code"].clone(), msg)
}

fn said(res: &ToolCallResult) -> &str {
    match &res.content[0] {
        ToolContent::Text { text } => text,
        ToolContent::Image { .. } => panic!("expected Text content"),
    }
}

#[test]
fn plan_returns_none_for_unrelated_tool_names() {
    assert!(plan("view.get_dialog", &json!({}), Some(&OPTIONS)).is_none());
    assert!(plan("editor.open_file", &json!({}), Some(&OPTIONS)).is_none());
    assert!(plan("dialog.unknown", &json!({}), Some(&OPTIONS)).is_none());
}

#[test]
fn plan_refuses_when_no_dialog_is_open() {
    let args = json!({ "control": "opt_close" });
    let res = plan("dialog.press", &args, None);
    let (code, outcome) = res.unwrap().unwrap_err();
    assert_eq!(outcome, "error");
    assert_eq!(code.data.unwrap()["code"], json!("no-dialog"));

    let res = plan("dialog.set_field", &args, None);
    let (code, outcome) = res.unwrap().unwrap_err();
    assert_eq!(outcome, "error");
    assert_eq!(code.data.unwrap()["code"], json!("no-dialog"));
}

#[test]
fn plan_refuses_when_control_is_missing_or_empty() {
    let empty_cases = [
        json!({}),
        json!({ "control": "" }),
        json!({ "control": "   " }),
    ];

    for args in &empty_cases {
        let res = plan("dialog.press", args, Some(&OPTIONS));
        let (code, _) = refusal_code_and_message(res.unwrap().unwrap_err());
        assert_eq!(code, json!("invalid-arguments"));

        let res = plan("dialog.set_field", args, Some(&OPTIONS));
        let (code, _) = refusal_code_and_message(res.unwrap().unwrap_err());
        assert_eq!(code, json!("invalid-arguments"));
    }
}

#[test]
fn plan_refuses_unknown_control_and_lists_valid_names() {
    let args = json!({ "control": "nonexistent_btn" });
    let res = plan("dialog.press", &args, Some(&OPTIONS));
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unknown-control"));
    assert!(msg.contains("Unknown control 'nonexistent_btn' in dialog 'options'"));
    assert!(msg.contains("opt_close"));
    assert!(msg.contains("opt_search_input"));

    let res = plan("dialog.set_field", &args, Some(&NEW_PROJECT));
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unknown-control"));
    assert!(msg.contains("Unknown control 'nonexistent_btn' in dialog 'new_project'"));
    assert!(msg.contains("np_close"));
    assert!(msg.contains("np_nome"));
}

#[test]
fn plan_refuses_unsupported_control_kinds_with_explanatory_message() {
    // Label press
    let res = plan(
        "dialog.press",
        &json!({ "control": "opt_title" }),
        Some(&OPTIONS),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("is a label: labels are read-only."));

    // Label set_field
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "opt_title", "value": "text" }),
        Some(&OPTIONS),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("is a label: labels are read-only."));

    // Card (Row) set_field
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "np_card_0", "value": "x" }),
        Some(&NEW_PROJECT),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("is a card: dialog.press clicks it."));

    // Drop-down press
    let res = plan(
        "dialog.press",
        &json!({ "control": "opt_editor_font_size" }),
        Some(&OPTIONS),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("is a drop-down: dialog.set_field picks an item by its text."));
}

#[test]
fn plan_accepts_card_press_and_refuses_card_set_field() {
    let res = plan(
        "dialog.press",
        &json!({ "control": "np_card_0" }),
        Some(&NEW_PROJECT),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::Click {
            control: "np_card_0".into(),
        }
    );

    let res = plan(
        "dialog.set_field",
        &json!({ "control": "np_card_0", "value": "template" }),
        Some(&NEW_PROJECT),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("is a card: dialog.press clicks it."));
}

#[test]
fn plan_accepts_drop_down_set_field_and_refuses_invalid_calls() {
    // Valid string value -> Pick
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "opt_editor_font_size", "value": "14 pt" }),
        Some(&OPTIONS),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::Pick {
            control: "opt_editor_font_size".into(),
            label: "14 pt".into(),
        }
    );

    // Missing or non-string value -> invalid-arguments
    let bad_values = [
        json!({ "control": "opt_editor_font_size" }),
        json!({ "control": "opt_editor_font_size", "value": 14 }),
        json!({ "control": "opt_editor_font_size", "value": true }),
    ];
    for args in &bad_values {
        let res = plan("dialog.set_field", args, Some(&OPTIONS));
        let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
        assert_eq!(code, json!("invalid-arguments"));
        assert!(msg.contains(
            "is a drop-down: dialog.set_field requires the item's text as a string 'value'."
        ));
    }

    // dialog.press on drop-down -> unsupported-control
    let res = plan(
        "dialog.press",
        &json!({ "control": "opt_editor_font_size" }),
        Some(&OPTIONS),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("is a drop-down: dialog.set_field picks an item by its text."));
}

#[test]
fn plan_refuses_mismatched_kind_operations() {
    // dialog.press on a Text field
    let res = plan(
        "dialog.press",
        &json!({ "control": "opt_search_input" }),
        Some(&OPTIONS),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("dialog.press takes a Button or a Check"));

    // dialog.set_field on a Button
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "opt_close", "value": "x" }),
        Some(&OPTIONS),
    );
    let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
    assert_eq!(code, json!("unsupported-control"));
    assert!(msg.contains("dialog.set_field takes a Text or a Check"));
}

#[test]
fn plan_refuses_invalid_field_values() {
    // Text field requires string value
    let bad_text_values = [
        json!({ "control": "opt_search_input" }),
        json!({ "control": "opt_search_input", "value": true }),
        json!({ "control": "opt_search_input", "value": 42 }),
    ];
    for args in &bad_text_values {
        let res = plan("dialog.set_field", args, Some(&OPTIONS));
        let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
        assert_eq!(code, json!("invalid-arguments"));
        assert!(msg.contains("requires a string 'value'"));
    }

    // Check box requires boolean value
    let bad_check_values = [
        json!({ "control": "opt_cu_line_numbers" }),
        json!({ "control": "opt_cu_line_numbers", "value": "true" }),
        json!({ "control": "opt_cu_line_numbers", "value": 1 }),
    ];
    for args in &bad_check_values {
        let res = plan("dialog.set_field", args, Some(&OPTIONS));
        let (code, msg) = refusal_code_and_message(res.unwrap().unwrap_err());
        assert_eq!(code, json!("invalid-arguments"));
        assert!(msg.contains("requires a boolean 'value'"));
    }
}

#[test]
fn plan_accepts_valid_press_operations() {
    // Button in OPTIONS
    let res = plan(
        "dialog.press",
        &json!({ "control": "opt_close" }),
        Some(&OPTIONS),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::Press {
            control: "opt_close".into()
        }
    );

    // Check in OPTIONS (toggles)
    let res = plan(
        "dialog.press",
        &json!({ "control": "opt_cu_line_numbers" }),
        Some(&OPTIONS),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::Toggle {
            control: "opt_cu_line_numbers".into()
        }
    );

    // Button in NEW_PROJECT
    let res = plan(
        "dialog.press",
        &json!({ "control": "np_close" }),
        Some(&NEW_PROJECT),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::Press {
            control: "np_close".into()
        }
    );

    // Check in NEW_PROJECT
    let res = plan(
        "dialog.press",
        &json!({ "control": "np_opt_example" }),
        Some(&NEW_PROJECT),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::Toggle {
            control: "np_opt_example".into()
        }
    );
}

#[test]
fn plan_accepts_valid_set_field_operations() {
    // Text in OPTIONS
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "opt_search_input", "value": "dark" }),
        Some(&OPTIONS),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::SetText {
            control: "opt_search_input".into(),
            value: "dark".into(),
        }
    );

    // Check in OPTIONS
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "opt_cu_line_numbers", "value": false }),
        Some(&OPTIONS),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::SetChecked {
            control: "opt_cu_line_numbers".into(),
            value: false,
        }
    );

    // Text in NEW_PROJECT
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "np_nome", "value": "test_proj" }),
        Some(&NEW_PROJECT),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::SetText {
            control: "np_nome".into(),
            value: "test_proj".into(),
        }
    );

    // Check in NEW_PROJECT
    let res = plan(
        "dialog.set_field",
        &json!({ "control": "np_opt_tracing", "value": true }),
        Some(&NEW_PROJECT),
    );
    assert_eq!(
        res.unwrap().unwrap(),
        DialogOp::SetChecked {
            control: "np_opt_tracing".into(),
            value: true,
        }
    );
}

#[test]
fn done_produces_success_answer_with_proper_structure() {
    let press_op = DialogOp::Press {
        control: "opt_close".into(),
    };
    let (res, outcome) = done(&press_op, "options", Ok(()));
    assert_eq!(outcome, "applied");
    assert!(!res.is_error);
    let data = res.data.clone().unwrap();
    assert_eq!(data["dialog"], json!("options"));
    assert_eq!(data["control"], json!("opt_close"));
    assert_eq!(data["op"], json!("press"));
    assert_eq!(data["status"], json!("ok"));
    assert_eq!(said(&res), "Pressed 'opt_close' in dialog 'options'.");

    let text_op = DialogOp::SetText {
        control: "opt_search_input".into(),
        value: "foo".into(),
    };
    let (res, outcome) = done(&text_op, "options", Ok(()));
    assert_eq!(outcome, "applied");
    assert!(!res.is_error);
    let data = res.data.clone().unwrap();
    assert_eq!(data["dialog"], json!("options"));
    assert_eq!(data["control"], json!("opt_search_input"));
    assert_eq!(data["op"], json!("set_text"));
    assert_eq!(data["status"], json!("ok"));
    assert_eq!(
        said(&res),
        "Set text of 'opt_search_input' to \"foo\" in dialog 'options'."
    );

    let check_op = DialogOp::SetChecked {
        control: "opt_cu_line_numbers".into(),
        value: true,
    };
    let (res, outcome) = done(&check_op, "options", Ok(()));
    assert_eq!(outcome, "applied");
    assert!(!res.is_error);
    let data = res.data.clone().unwrap();
    assert_eq!(data["dialog"], json!("options"));
    assert_eq!(data["control"], json!("opt_cu_line_numbers"));
    assert_eq!(data["op"], json!("set_checked"));
    assert_eq!(data["status"], json!("ok"));
    assert_eq!(
        said(&res),
        "Set check of 'opt_cu_line_numbers' to true in dialog 'options'."
    );

    let click_op = DialogOp::Click {
        control: "np_card_0".into(),
    };
    let (res, outcome) = done(&click_op, "new_project", Ok(()));
    assert_eq!(outcome, "applied");
    assert!(!res.is_error);
    let data = res.data.clone().unwrap();
    assert_eq!(data["dialog"], json!("new_project"));
    assert_eq!(data["control"], json!("np_card_0"));
    assert_eq!(data["op"], json!("click"));
    assert_eq!(data["status"], json!("ok"));
    assert_eq!(said(&res), "Clicked 'np_card_0' in dialog 'new_project'.");

    let pick_op = DialogOp::Pick {
        control: "opt_editor_font_size".into(),
        label: "14 pt".into(),
    };
    let (res, outcome) = done(&pick_op, "options", Ok(()));
    assert_eq!(outcome, "applied");
    assert!(!res.is_error);
    let data = res.data.clone().unwrap();
    assert_eq!(data["dialog"], json!("options"));
    assert_eq!(data["control"], json!("opt_editor_font_size"));
    assert_eq!(data["op"], json!("pick"));
    assert_eq!(data["status"], json!("ok"));
    assert_eq!(
        said(&res),
        "Picked \"14 pt\" in 'opt_editor_font_size' in dialog 'options'."
    );
}

#[test]
fn done_refuses_unknown_item_when_pick_not_applied() {
    let pick_op = DialogOp::Pick {
        control: "opt_editor_font_size".into(),
        label: "99 pt".into(),
    };
    let (res, outcome) = done(&pick_op, "options", Err("unknown-item"));
    assert_eq!(outcome, "error");
    assert!(res.is_error);
    let (code, msg) = refusal_code_and_message((res.clone(), outcome));
    assert_eq!(code, json!("unknown-item"));
    assert_eq!(
        msg,
        "'99 pt' is not an item of drop-down 'opt_editor_font_size' in dialog 'options'."
    );
    assert_eq!(
        said(&res),
        "[unknown-item] '99 pt' is not an item of drop-down 'opt_editor_font_size' in dialog 'options'."
    );
}

#[test]
fn done_refuses_a_control_that_is_not_on_screen() {
    let click_op = DialogOp::Click {
        control: "np_card_2".into(),
    };
    let (res, outcome) = done(&click_op, "new_project", Err("hidden-control"));
    assert_eq!(outcome, "error");
    let (code, msg) = refusal_code_and_message((res, outcome));
    assert_eq!(code, json!("hidden-control"));
    assert_eq!(
        msg,
        "Control 'np_card_2' is not on screen in dialog 'new_project': show its page first."
    );
}
