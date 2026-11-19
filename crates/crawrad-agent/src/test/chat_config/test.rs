use serde_json::json;

use super::*;

fn claude_fixture() -> Vec<serde_json::Value> {
    vec![
        json!({
            "id": "mode",
            "name": "Mode",
            "category": "mode",
            "type": "select",
            "currentValue": "code",
            "options": [
                { "value": "code", "name": "Code" },
                { "value": "architect", "name": "Architect" }
            ]
        }),
        json!({
            "id": "model",
            "name": "Model",
            "category": "model",
            "type": "select",
            "currentValue": "default",
            "options": [
                { "value": "default", "name": "Default" },
                { "value": "claude-3-7-sonnet", "name": "Claude 3.7 Sonnet" },
                { "value": "claude-3-5-sonnet", "name": "Claude 3.5 Sonnet" },
                { "value": "claude-3-5-haiku", "name": "Claude 3.5 Haiku" },
                { "value": "claude-opus-4", "name": "Claude Opus 4" },
                { "value": "claude-opus-4-7", "name": "Claude Opus 4.7" },
                { "value": "claude-opus-5", "name": "Claude Opus 5" },
                { "value": "claude-opus-5-5", "name": "Opus 5.5" },
                { "value": "claude-sonnet-4-6", "name": "Claude Sonnet 4.6" },
                { "value": "claude-sonnet-5", "name": "Claude Sonnet 5" },
                { "value": "claude-fable-5-1", "name": "Claude Fable 5.1" },
                { "value": "claude-custom", "name": "Claude Custom" }
            ]
        }),
        json!({
            "id": "effort",
            "name": "Reasoning Effort",
            "category": "thought_level",
            "type": "select",
            "currentValue": "high",
            "options": [
                { "value": "default", "name": "Default" },
                { "value": "low", "name": "Low" },
                { "value": "medium", "name": "Medium" },
                { "value": "high", "name": "High" },
                { "value": "xhigh", "name": "Extra High" },
                { "value": "max", "name": "Maximum" }
            ]
        }),
        json!({
            "id": "fast",
            "name": "Fast Mode",
            "category": "model_config",
            "type": "select",
            "currentValue": "auto",
            "options": [
                { "value": "auto", "name": "Auto" },
                { "value": "on", "name": "On" },
                { "value": "off", "name": "Off" }
            ]
        }),
    ]
}

fn codex_fixture() -> Vec<serde_json::Value> {
    vec![
        json!({
            "id": "mode",
            "name": "Mode",
            "category": "mode",
            "type": "select",
            "currentValue": "agent",
            "options": [{ "value": "agent", "name": "Agent" }]
        }),
        json!({
            "id": "collaboration_mode",
            "name": "Collaboration Mode",
            "category": "mode",
            "type": "select",
            "currentValue": "pair",
            "options": [{ "value": "pair", "name": "Pair" }]
        }),
        json!({
            "id": "codex_model",
            "name": "Codex Model",
            "category": "model",
            "type": "select",
            "currentValue": "gpt-5",
            "options": [
                { "value": "gpt-5", "name": "GPT-5" },
                { "value": "gpt-5-mini", "name": "GPT-5 Mini" },
                { "value": "o3", "name": "o3" },
                { "value": "o3-mini", "name": "o3 Mini" },
                { "value": "o4-mini", "name": "o4 Mini" },
                { "value": "o4-preview", "name": "o4 Preview" },
                { "value": "codex-advanced", "name": "Codex Advanced" }
            ]
        }),
        json!({
            "id": "reasoning_effort",
            "name": "Reasoning Effort",
            "category": "thought_level",
            "type": "select",
            "currentValue": "medium",
            "options": [
                { "value": "low", "name": "Low" },
                { "value": "medium", "name": "Medium" },
                { "value": "high", "name": "High" },
                { "value": "ultra", "name": "Ultra" }
            ]
        }),
        json!({
            "id": "fast-mode",
            "name": "Fast",
            "category": "mode_flag",
            "type": "select",
            "currentValue": "disabled",
            "options": [{ "value": "disabled", "name": "Disabled" }]
        }),
    ]
}

fn antigravity_fixture() -> Vec<serde_json::Value> {
    vec![
        json!({
            "id": "model_selection",
            "name": "Model Selection",
            "category": "model",
            "type": "select",
            "currentValue": "gemini-3.8-flash-high",
            "options": [
                { "value": "gemini-3.8-flash-low", "name": "Gemini 3.8 Flash (Low)" },
                { "value": "gemini-3.8-flash-medium", "name": "Gemini 3.8 Flash (Medium)" },
                { "value": "gemini-3.8-flash-high", "name": "Gemini 3.8 Flash (High)" },
                { "value": "gemini-3.8-pro-low", "name": "Gemini 3.8 Pro (Low)" },
                { "value": "gemini-3.8-pro-medium", "name": "Gemini 3.8 Pro (Medium)" },
                { "value": "gemini-3.8-pro-high", "name": "Gemini 3.8 Pro (High)" },
                { "value": "gemini-3.8-ultra", "name": "Gemini 3.8 Ultra" },
                { "value": "gemini-2.5-flash", "name": "Gemini 2.5 Flash" },
                { "value": "gemini-2.5-pro", "name": "Gemini 2.5 Pro" },
                { "value": "gemini-experimental", "name": "Gemini Experimental" },
                { "value": "gemini-custom", "name": "Gemini Custom" }
            ]
        }),
        json!({
            "id": "mode",
            "name": "Operation Mode",
            "category": "mode",
            "type": "select",
            "currentValue": "chat",
            "options": [{ "value": "chat", "name": "Chat" }]
        }),
    ]
}

#[test]
fn claude_shaped_options_resolve_model_and_thought_level() {
    let opts = claude_fixture();

    let model_menu = menu_for(&opts, "model").expect("model menu must exist");
    assert_eq!(model_menu.config_id, "model");
    assert_eq!(model_menu.title, "Model");
    assert_eq!(model_menu.items.len(), 12);
    assert_eq!(model_menu.items[0].value, "default");
    assert_eq!(model_menu.items[0].label, "Auto");
    assert!(model_menu.items[0].selected);
    assert!(!model_menu.items[1].selected);
    assert_eq!(button_label(&model_menu), "Auto ▾");

    let effort_menu = menu_for(&opts, "thought_level").expect("thought_level menu must exist");
    assert_eq!(effort_menu.config_id, "effort");
    assert_eq!(effort_menu.items.len(), 6);
    let selected = effort_menu.items.iter().find(|i| i.selected).unwrap();
    assert_eq!(selected.value, "high");
    assert_eq!(selected.label, "High");
    assert_eq!(button_label(&effort_menu), "High ▾");
}

#[test]
fn codex_shaped_options_resolve_model_and_reasoning_effort_by_category() {
    let opts = codex_fixture();

    let model_menu = menu_for(&opts, "model").expect("model menu found by category");
    assert_eq!(model_menu.config_id, "codex_model");
    assert_eq!(model_menu.items.len(), 7);
    assert_eq!(button_label(&model_menu), "GPT-5 ▾");

    let effort_menu = menu_for(&opts, "thought_level").expect("thought_level found by category");
    assert_eq!(effort_menu.config_id, "reasoning_effort");
    assert_eq!(effort_menu.items.len(), 4);
    assert_eq!(button_label(&effort_menu), "Medium ▾");
}

#[test]
fn antigravity_shaped_options_have_no_thought_level() {
    let opts = antigravity_fixture();

    let model_menu = menu_for(&opts, "model").expect("model menu must exist");
    assert_eq!(model_menu.config_id, "model_selection");
    assert_eq!(model_menu.items.len(), 11);
    assert_eq!(button_label(&model_menu), "Gemini 3.8 Flash (High) ▾");

    let effort_menu = menu_for(&opts, "thought_level");
    assert!(
        effort_menu.is_none(),
        "Antigravity has no thought_level category, must return None"
    );
}

#[test]
fn grouped_options_are_flattened() {
    let grouped = vec![json!({
        "id": "model",
        "name": "Model",
        "category": "model",
        "type": "select",
        "currentValue": "smart",
        "options": [
            {
                "group": "fast_group",
                "name": "Fast Models",
                "options": [
                    { "value": "fast-1", "name": "Fast One" },
                    { "value": "fast-2", "name": "Fast Two" }
                ]
            },
            {
                "group": "smart_group",
                "name": "Smart Models",
                "options": [
                    { "value": "smart", "name": "Smart Model" }
                ]
            }
        ]
    })];

    let menu = menu_for(&grouped, "model").expect("menu should resolve");
    assert_eq!(menu.items.len(), 3);
    assert_eq!(menu.items[0].value, "fast-1");
    assert_eq!(menu.items[0].label, "Fast One");
    assert!(!menu.items[0].selected);
    assert_eq!(menu.items[2].value, "smart");
    assert_eq!(menu.items[2].label, "Smart Model");
    assert!(menu.items[2].selected);
    assert_eq!(button_label(&menu), "Smart Model ▾");
}

#[test]
fn button_label_falls_back_to_first_item_when_none_selected() {
    let menu = ConfigMenu {
        config_id: "m".to_string(),
        title: "Model".to_string(),
        items: vec![
            MenuItem {
                value: "a".to_string(),
                label: "Option A".to_string(),
                selected: false,
            },
            MenuItem {
                value: "b".to_string(),
                label: "Option B".to_string(),
                selected: false,
            },
        ],
    };
    assert_eq!(button_label(&menu), "Option A ▾");
}

#[test]
fn as_json_serializes_flattened_mcp_format() {
    let opts = vec![json!({
        "id": "model",
        "name": "Model",
        "category": "model",
        "type": "select",
        "currentValue": "m2",
        "options": [
            { "value": "m1", "name": "Model 1" },
            {
                "group": "g",
                "name": "Group",
                "options": [{ "value": "m2", "name": "Model 2" }]
            }
        ]
    })];

    let val = as_json(&opts);
    let arr = val.as_array().expect("array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["id"], "model");
    assert_eq!(arr[0]["name"], "Model");
    assert_eq!(arr[0]["category"], "model");
    assert_eq!(arr[0]["current"], "m2");

    let items = arr[0]["options"].as_array().expect("options array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["value"], "m1");
    assert_eq!(items[0]["name"], "Model 1");
    assert_eq!(items[1]["value"], "m2");
    assert_eq!(items[1]["name"], "Model 2");
}

#[test]
fn the_menu_stays_inside_the_panel_and_above_the_bar() {
    // panel at x=1536, 380 wide; the effort button sits 20 px from its right edge
    let panel = [1536.0, 40.0, 380.0, 960.0];
    let (x, y) = menu_origin(1536.0 + 360.0, panel, 900.0, 200.0);
    assert!(
        x + MENU_WIDTH <= 380.0,
        "the menu's right edge {} is past the panel",
        x + MENU_WIDTH
    );
    assert_eq!(y, 900.0 - 40.0 - 200.0 - 4.0, "right above the prompt bar");
    // a button on the left: anchored at it
    let (x, _) = menu_origin(1536.0 + 20.0, panel, 900.0, 200.0);
    assert_eq!(x, 18.0);
}
