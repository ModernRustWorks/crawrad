//! The `view.*` tools: panels, navigation and UI switches. Pure of the window,
//! so they live here; the shell applies the layout command and writes the audit line.

use crawrad_actions::Panel;
use serde_json::{Value, json};

use crate::cargo_tool::nonempty;
use crate::designer_tool::{no_form, select, selected};
use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;
use crawrad_note::note;

#[note("view-tool-drive")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drive {
    /// `ide.run_action`: an action of the catalogue, by id.
    RunAction(String),
    /// `view.show_panel`: a side panel of the rail.
    ShowPanel(Panel),
    /// `designer.select`: a component of the active form, by its resolved id.
    Select(String),
}

#[note("view-tool-drive-2")]
pub fn drive(
    name: &str,
    args: &Value,
    doc: Option<&crawrad_doc::Doc>,
    known: impl Fn(&str) -> bool,
) -> Option<Result<Drive, Answer>> {
    let key = match name {
        "ide.run_action" | "designer.select" => "id",
        "view.show_panel" => "name",
        _ => return None,
    };
    let Some(value) = nonempty(args, key) else {
        let said = if name == "designer.select" && args.get("ids").is_some() {
            "designer.select takes one component in 'id' (a string); 'ids' is not supported: call it once per component".to_string()
        } else {
            format!("Missing or empty '{key}' parameter")
        };
        return Some(Err((
            ToolCallResult::error_text("invalid-params", said),
            "error",
        )));
    };
    Some(match name {
        "ide.run_action" if known(value) => Ok(Drive::RunAction(value.to_string())),
        "ide.run_action" => {
            let said = format!("Unknown action '{value}'.");
            Err((ToolCallResult::error_text("unknown-action", said), "error"))
        }
        "designer.select" => select(doc, value).map(Drive::Select),
        _ => parse_panel_name(value)
            .map(Drive::ShowPanel)
            .ok_or_else(|| {
                let said = format!(
                    "Unknown panel '{value}'. Valid panels: {}",
                    VALID_PANEL_NAMES.join(", ")
                );
                (ToolCallResult::error_text("unknown-panel", said), "error")
            }),
    })
}

#[note("view-tool-done")]
pub fn done(drive: &Drive, doc: Option<&crawrad_doc::Doc>) -> Answer {
    match drive {
        Drive::RunAction(id) => {
            let data = json!({ "action": id, "executed": true, "status": "ok" });
            let said = format!("Action '{id}' executed.");
            (ToolCallResult::success_json(data, said), "applied")
        }
        Drive::ShowPanel(panel) => panel_shown(*panel),
        Drive::Select(id) => doc.map_or_else(no_form, |doc| selected(doc, id)),
    }
}

#[note("view-tool-refused")]
pub fn refused(id: &str, why: &str) -> Answer {
    let said = format!("Action '{id}' was not run: {why}.");
    (ToolCallResult::error_text("action-disabled", said), "error")
}

#[note("view-tool-valid-panel-names")]
pub const VALID_PANEL_NAMES: &[&str] = &[
    "Explorer",
    "Inspector",
    "Palette",
    "Structure",
    "Components",
    "Packages",
    "Debug",
    "Extensions",
    "Chat",
    "Terminal",
    "References",
    "Problems",
    "Design",
    "Notes",
];

#[note("view-tool-parse-panel-name")]
pub fn parse_panel_name(name: &str) -> Option<Panel> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }
    match trimmed.to_ascii_lowercase().as_str() {
        "explorer" | "files" => Some(Panel::Explorer),
        "inspector" | "object inspector" => Some(Panel::Inspector),
        "palette" | "structure" | "components" | "toolbox" => Some(Panel::Components),
        "packages" | "package manager" => Some(Panel::Packages),
        "debug" | "run and debug" | "run & debug" => Some(Panel::Debug),
        "extensions" => Some(Panel::Extensions),
        "chat" | "ai" => Some(Panel::Chat),
        "terminal" => Some(Panel::Terminal),
        "references" => Some(Panel::References),
        "problems" => Some(Panel::Problems),
        "design" => Some(Panel::Design),
        "notes" => Some(Panel::Notes),
        _ => Panel::ALL.iter().copied().find(|p| {
            p.id().eq_ignore_ascii_case(trimmed) || p.label().eq_ignore_ascii_case(trimmed)
        }),
    }
}

#[note("view-tool-panel-shown")]
pub fn panel_shown(panel: Panel) -> Answer {
    let data = json!({
        "panel": panel.id(),
        "label": panel.label(),
        "status": "ok"
    });
    let said = format!("Switched side panel to '{}'.", panel.label());
    (ToolCallResult::success_json(data, said), "applied")
}

/// Static success description for read-only view inspection tools.
pub fn view_read_message(name: &str) -> Option<&'static str> {
    match name {
        "editor.get_completion" => Some("Completion list read."),
        "editor.get_choices" => Some("Definition choices read."),
        "view.get_inspector" => Some("Object Inspector read."),
        "editor.get_rename" => Some("Rename dialog read."),
        "editor.get_references" => Some("References read."),
        "editor.get_hover" => Some("Hover card read."),
        "view.get_dialog" => Some("Dialog read."),
        "view.get_status_bar" => Some("Status bar read."),
        "view.get_history" => Some("History view read."),
        "view.get_designer" => Some("Designer bar read."),
        "view.get_welcome" => Some("Welcome page read."),
        "view.get_explorer" => Some("Explorer read."),
        "view.get_notes" => Some("Notes panel read."),
        "view.get_tests" => Some("Tests panel read."),
        "form.check" => Some("Form checked."),
        "view.sizes" => Some("Sizes preview read."),
        "strings.list" => Some("Strings read."),
        "view.get_rail" => Some("Activity rail read."),
        "view.get_toolbox" => Some("Toolbox read."),
        "view.set_toolbox_filter" => Some("Filtered."),
        "view.get_chat" => Some("Chat messages read."),
        "view.get_extension_views" => Some("Extension views read."),
        "editor.get_line_marks" => Some("Line marks read."),
        "editor.get_semantic_tokens" => Some("Semantic tokens read."),
        "editor.get_code_lenses" => Some("Code lenses read."),
        "editor.get_code_actions" => Some("Code actions read."),
        "editor.get_signature_help" => Some("Parameter card read."),
        "editor.get_decorations" => Some("Decorations read."),
        _ => None,
    }
}

/// Formats the success envelope for a read-only view inspection tool call.
pub fn view_read_result(name: &str, answer: Value) -> Option<ToolCallResult> {
    let said = view_read_message(name)?;
    Some(ToolCallResult::success_json(answer, said))
}

/// Formats the menu hierarchy state and status for `view.get_open_menu`.
pub fn open_menu_result(
    open: Option<String>,
    highlight: Option<usize>,
    rows: &[crawrad_actions::menu::Row],
    sub: Option<(&str, &[crawrad_actions::menu::Row])>,
    rects: Value,
    hint: Option<String>,
) -> ToolCallResult {
    let mut state = crawrad_actions::menu::open_menu_json(open, highlight, rows, sub, rects);
    state["hint"] = json!(hint);
    ToolCallResult::success_json(state, "Open menu read.")
}

#[cfg(test)]
#[path = "test/view_tool/test.rs"]
mod tests;
