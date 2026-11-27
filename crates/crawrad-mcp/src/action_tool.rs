//! MCP host tools: Action catalogue and extension capability dispatch.
//!
//! Validates preconditions, view affinity, and enabled predicates before running
//! actions, and shapes standard JSON-RPC results and audit records upon completion.

use crawrad_actions::capability::CapabilityError;
use crawrad_actions::{Action, Context, ViewAffinity};
use serde_json::{Value, json};

use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;

/// Snapshotted IDE state used to determine if an action mutated document or view state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSnapshot {
    pub revision: Option<u64>,
    pub selection: Option<Vec<crawrad_ir::NodeId>>,
    pub center_view: String,
    pub is_dirty: bool,
}

impl StateSnapshot {
    /// Captures a snapshot of the current document revision, selection, view, and dirty flag.
    pub fn capture(doc: Option<&crawrad_doc::Doc>, center_view: &str, is_dirty: bool) -> Self {
        Self {
            revision: doc.map(|d| d.revision()),
            selection: doc.map(|d| d.selection().to_vec()),
            center_view: center_view.to_string(),
            is_dirty,
        }
    }
}

/// Evaluates modal dialog state, argument schemas, and view affinity guards for a catalogue action.
pub fn check_action_preconditions(
    action: &Action,
    args: &Value,
    modal_open: bool,
    is_design_center: bool,
) -> Result<(), Answer> {
    if let Some(why) = crate::dialogs::modal_refusal(modal_open, action.label) {
        let text = format!("Action '{}' was not run: {why}.", action.id);
        return Err((
            ToolCallResult::error_text("action-disabled", text),
            "rejected",
        ));
    }

    if let Err(err_msg) = crate::tools::validate_arguments(action, args) {
        return Err((
            ToolCallResult::error_text("invalid-params", &err_msg),
            "error",
        ));
    }

    match action.view {
        needs @ ViewAffinity::Design if !is_design_center => {
            let (code, said) = crate::edit::wrong_view(needs);
            return Err((ToolCallResult::error_text(code, said), "rejected"));
        }
        needs @ ViewAffinity::Code if is_design_center => {
            let (code, said) = crate::edit::wrong_view(needs);
            return Err((ToolCallResult::error_text(code, said), "rejected"));
        }
        _ => {}
    }

    Ok(())
}

/// Evaluates whether an action is enabled in the current IDE execution context.
pub fn check_action_enabled(action: &Action, ctx: &Context<'_>) -> Result<(), Answer> {
    if !action.enabled.allows(ctx) {
        let instruction = match Some(action.enabled.need).filter(|n| !n.met(ctx)) {
            Some(n) => format!("It needs {}. {}", n.reason(), n.remedy()),
            None => "Check active selection and IDE state and resend.".to_string(),
        };
        let err = ToolCallResult::error_text(
            "action-disabled",
            format!(
                "Action '{}' is currently disabled in the IDE context: {instruction}",
                action.id
            ),
        );
        return Err((err, "rejected"));
    }
    Ok(())
}

/// Formats the success response for an executed catalogue action based on before/after snapshots.
pub fn format_action_result(
    name: &str,
    before: &StateSnapshot,
    after: &StateSnapshot,
) -> ToolCallResult {
    let mudou = before.revision != after.revision
        || before.selection != after.selection
        || before.center_view != after.center_view
        || before.is_dirty != after.is_dirty;

    if mudou {
        ToolCallResult::success_json(
            json!({
                "action": name,
                "mutated": true,
                "revision_before": before.revision,
                "revision_after": after.revision,
                "selection": after.selection,
                "center_view": after.center_view,
                "is_dirty": after.is_dirty,
            }),
            format!(
                "Action '{name}' applied successfully (rev: {}).",
                after.revision.unwrap_or(0)
            ),
        )
    } else {
        ToolCallResult::success_json(
            json!({
                "action": name,
                "mutated": false,
                "revision": after.revision,
                "selection": after.selection,
                "center_view": after.center_view,
                "is_dirty": after.is_dirty,
            }),
            format!("Action '{name}' executed (no document or state change)."),
        )
    }
}

/// Builds the success envelope for an extension capability execution.
pub fn format_capability_result(
    cap_id: &str,
    status_msg: Option<String>,
    ext_result: Option<Value>,
    messages: &[Value],
) -> ToolCallResult {
    let mut data_map = serde_json::Map::new();
    data_map.insert("capability".to_string(), json!(cap_id));
    data_map.insert(
        "status".to_string(),
        json!(status_msg.unwrap_or_else(|| "executed".to_string())),
    );
    if let Some(res) = ext_result {
        data_map.insert("result".to_string(), res);
    }
    if !messages.is_empty() {
        data_map.insert("messages".to_string(), json!(messages));
    }

    ToolCallResult::success_json(
        Value::Object(data_map),
        format!("Capability '{cap_id}' executed successfully."),
    )
}

/// Converts a [`CapabilityError`] to the corresponding standard MCP error result.
pub fn format_capability_error(cap_err: CapabilityError) -> ToolCallResult {
    match cap_err {
        CapabilityError::ViewMismatch { required, .. } => {
            let (code, msg) = crate::edit::wrong_view(required);
            ToolCallResult::error_text(code, msg)
        }
        CapabilityError::MissingParameter {
            param,
            capability_id,
        } => ToolCallResult::error_text(
            "invalid-params",
            format!("Missing required parameter '{param}' for capability '{capability_id}'"),
        ),
        CapabilityError::Disabled(id) => ToolCallResult::error_text(
            "action-disabled",
            format!("Capability '{id}' is currently disabled in the IDE context."),
        ),
        other => ToolCallResult::error_text("execution-failed", other.to_string()),
    }
}

/// Validates arguments and existence of an action for the designer runner.
pub fn check_designer_action(name: &str, args: &Value) -> Result<Action, String> {
    let action = crawrad_actions::catalogue()
        .into_iter()
        .find(|a| a.id == name)
        .ok_or_else(|| format!("Unknown action: '{name}'"))?;
    if let Err(err_msg) = crate::tools::validate_arguments(&action, args) {
        return Err(format!("Invalid arguments for '{name}': {err_msg}"));
    }
    Ok(action)
}
