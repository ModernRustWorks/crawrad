//! Configuration options menus and serialization for ACP agent settings.
#![forbid(unsafe_code)]

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[note("chat-config-configmenu")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigMenu {
    pub config_id: String,
    pub title: String,
    pub items: Vec<MenuItem>,
}

#[note("chat-config-menuitem")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MenuItem {
    pub value: String,
    pub label: String,
    pub selected: bool,
}

#[note("chat-config-flatten-options")]
pub fn flatten_options(options: &[Value]) -> Vec<(String, String)> {
    let mut result = Vec::new();
    flatten_options_recursive(options, &mut result);
    result
}

fn flatten_options_recursive(options: &[Value], out: &mut Vec<(String, String)>) {
    for opt in options {
        if let Some(sub_opts) = opt.get("options").and_then(|o| o.as_array()) {
            flatten_options_recursive(sub_opts, out);
        } else if let Some(val) = opt.get("value").and_then(|v| v.as_str()) {
            let name = opt.get("name").and_then(|n| n.as_str()).unwrap_or(val);
            out.push((val.to_string(), name.to_string()));
        }
    }
}

#[note("chat-config-menu-for")]
pub fn menu_for(options: &[Value], category: &str) -> Option<ConfigMenu> {
    for entry in options {
        let is_select = entry.get("type").and_then(|t| t.as_str()) == Some("select");
        let matches_category = entry.get("category").and_then(|c| c.as_str()) == Some(category);
        if is_select && matches_category {
            let config_id = entry
                .get("id")
                .and_then(|i| i.as_str())
                .unwrap_or_default()
                .to_string();
            let title = entry
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or_default()
                .to_string();
            let current_value = entry
                .get("currentValue")
                .and_then(|c| c.as_str())
                .unwrap_or_default();
            let opts_array = entry
                .get("options")
                .and_then(|o| o.as_array())
                .map(|a| a.as_slice())
                .unwrap_or(&[]);
            let flat = flatten_options(opts_array);
            let items = flat
                .into_iter()
                .map(|(val, name)| {
                    let selected = val == current_value;
                    // the owner's word: the agent deciding is "Auto" (Claude says "Default (recommended)")
                    let label = if val == "default" {
                        "Auto".to_string()
                    } else {
                        name
                    };
                    MenuItem {
                        value: val,
                        label,
                        selected,
                    }
                })
                .collect();
            return Some(ConfigMenu {
                config_id,
                title,
                items,
            });
        }
    }
    None
}

#[note("chat-config-button-label")]
pub fn button_label(menu: &ConfigMenu) -> String {
    let item = menu
        .items
        .iter()
        .find(|i| i.selected)
        .or_else(|| menu.items.first());
    match item {
        Some(it) => format!("{} ▾", it.label),
        None => format!("{} ▾", menu.title),
    }
}

#[note("chat-config-as-json")]
pub fn as_json(options: &[Value]) -> Value {
    let mut list = Vec::new();
    for entry in options {
        let id = entry.get("id").and_then(|s| s.as_str()).unwrap_or_default();
        let name = entry
            .get("name")
            .and_then(|s| s.as_str())
            .unwrap_or_default();
        let category = entry
            .get("category")
            .and_then(|s| s.as_str())
            .unwrap_or_default();
        let current = entry
            .get("currentValue")
            .and_then(|s| s.as_str())
            .unwrap_or_default();
        let opts_array = entry
            .get("options")
            .and_then(|o| o.as_array())
            .map(|a| a.as_slice())
            .unwrap_or(&[]);
        let flat = flatten_options(opts_array);
        let opts_json: Vec<Value> = flat
            .into_iter()
            .map(|(val, nm)| {
                serde_json::json!({
                    "value": val,
                    "name": nm,
                })
            })
            .collect();
        list.push(serde_json::json!({
            "id": id,
            "name": name,
            "category": category,
            "current": current,
            "options": opts_json,
        }));
    }
    Value::Array(list)
}

#[note("chat-config-config-options-payload")]
pub fn config_options_payload(live: &crate::acp_live::LiveAgent) -> Value {
    let live_str = match live.status() {
        crate::acp_live::LiveStatus::Ready { .. } => "ready",
        crate::acp_live::LiveStatus::Connecting => "connecting",
        crate::acp_live::LiveStatus::Failed(_) => "failed",
        crate::acp_live::LiveStatus::Idle => "idle",
    };
    let opts = live.config_options();
    serde_json::json!({
        "live": live_str,
        "options": as_json(&opts),
    })
}

#[note("chat-config-apply-choice")]
pub fn apply_choice(
    live: &mut crate::acp_live::LiveAgent,
    config: &mut crate::acp::AcpConfig,
    config_id: &str,
    value: &str,
) -> Result<Value, String> {
    if !matches!(live.status(), crate::acp_live::LiveStatus::Ready { .. }) {
        return Err("Live agent is not ready".to_string());
    }
    let opts = live.config_options();
    let target = opts
        .iter()
        .find(|o| o.get("id").and_then(|i| i.as_str()) == Some(config_id))
        .ok_or_else(|| format!("Unknown config option: {config_id}"))?;
    let sub = target
        .get("options")
        .and_then(|o| o.as_array())
        .ok_or_else(|| format!("Config option '{config_id}' has no options array"))?;
    if !flatten_options(sub).iter().any(|(v, _)| v == value) {
        return Err(format!("Unknown value '{value}' for config '{config_id}'"));
    }
    live.choose(config_id, value);
    config.record_session_choice(config_id, value);
    let _ = config.save();
    Ok(as_json(&live.config_options()))
}

#[note("chat-config-mcp-call")]
pub fn mcp_call(
    name: &str,
    args: &Value,
    live: &mut crate::acp_live::LiveAgent,
    config: &mut crate::acp::AcpConfig,
    chat: (&[crate::session::ChatMessage], &str),
) -> Result<Value, String> {
    if name == "chat.copy_message" {
        let text = crate::chat_layout::copy_message(chat.0, chat.1, args)?;
        return Ok(serde_json::json!({ "copied": text }));
    }
    if name == "chat.get_config_options" {
        return Ok(config_options_payload(live));
    }
    let arg = |k: &str| {
        args.get(k)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    apply_choice(live, config, &arg("config_id"), &arg("value"))
}

#[note("chat-config-menu-width")]
pub const MENU_WIDTH: f64 = 180.0;

#[note("chat-config-menu-origin")]
pub fn menu_origin(button_x: f64, panel: [f64; 4], bar_y: f64, menu_h: f64) -> (f64, f64) {
    let [px, py, pw, ph] = panel;
    let x = if button_x > 0.0 {
        button_x - px - 2.0
    } else {
        44.0
    };
    let x = x.min(pw - MENU_WIDTH - 8.0).max(8.0);
    let y = if bar_y > 0.0 && ph > 0.0 {
        bar_y - py - menu_h - 4.0
    } else {
        ph - menu_h - 85.0
    };
    (x, y.max(4.0))
}

#[note("chat-config-catalogue")]
pub const CATALOGUE: &str = "";

#[note("chat-config-menu-or-catalogue")]
pub fn menu_or_catalogue(
    live: &crate::acp_live::LiveAgent,
    config: &crate::acp::AcpConfig,
    category: &str,
) -> Option<ConfigMenu> {
    if let Some(menu) = menu_for(&live.config_options(), category) {
        return Some(menu);
    }
    if category != "model" {
        return None;
    }
    let current = config.model.to_ascii_lowercase();
    let is_current = |id: &str| {
        current == id.to_ascii_lowercase()
            || (id == "auto" && (current == "auto" || current.is_empty()))
            || (id == "claude-3-5-haiku" && current == "haiku")
    };
    let items = config.models_for_active_provider().into_iter();
    let items = items.map(|m| MenuItem {
        selected: is_current(&m.id),
        value: m.id,
        label: m.display_name,
    });
    Some(ConfigMenu {
        config_id: CATALOGUE.to_string(),
        title: "Model".to_string(),
        items: items.collect(),
    })
}

#[note("chat-config-pick")]
pub fn pick(
    live: &mut crate::acp_live::LiveAgent,
    config: &mut crate::acp::AcpConfig,
    menu: &ConfigMenu,
    index: usize,
) -> Result<Value, String> {
    let item = menu.items.get(index).ok_or("No such menu item")?;
    if menu.config_id != CATALOGUE {
        return apply_choice(live, config, &menu.config_id, &item.value);
    }
    config.model = item.value.clone();
    let provider = config.provider.clone();
    config
        .models_per_provider
        .insert(provider, item.value.clone());
    config.save().map_err(|e| e.to_string())?;
    Ok(Value::Null)
}

#[note("chat-config-button-labels")]
pub fn button_labels(
    live: &crate::acp_live::LiveAgent,
    config: &crate::acp::AcpConfig,
) -> (String, Option<String>) {
    let options = live.config_options();
    let model = match menu_for(&options, "model") {
        Some(menu) => button_label(&menu),
        None => crate::acp::format_model_label_pure(&config.model),
    };
    let effort = menu_for(&options, "thought_level").map(|m| button_label(&m));
    (model, effort)
}

#[cfg(test)]
#[path = "test/chat_config/test.rs"]
mod tests;
