//! Operations on open dialogs: `dialog.press` and `dialog.set_field`.
//!
//! Pure of the window, so it lives in `crawrad-mcp`; the IDE applies Makepad widget
//! actions to real controls and records the audit log.

use crate::cargo_tool::nonempty;
use crate::dialogs::{ControlKind, DialogSpec};
use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;
use crawrad_note::note;
use serde_json::{Value, json};

#[note("dialog-tool-dialogop")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogOp {
    /// Presses a button.
    Press {
        /// Name of the button to press.
        control: String,
    },
    /// Flips a check box, as a click on it does.
    Toggle {
        /// Name of the check box to flip.
        control: String,
    },
    /// Sets the text content of an input field.
    SetText {
        /// Name of the text control.
        control: String,
        /// New text value.
        value: String,
    },
    /// Sets the boolean state of a check box.
    SetChecked {
        /// Name of the check box.
        control: String,
        /// New checked value.
        value: bool,
    },
    /// Clicks a card (a Row).
    Click {
        /// Name of the card to click.
        control: String,
    },
    /// Selects the item whose text is `label` in a drop-down.
    Pick {
        /// Name of the drop-down control.
        control: String,
        /// Item text to select.
        label: String,
    },
}

impl DialogOp {
    #[note("dialog-tool-dialogop-control")]
    pub fn control(&self) -> &str {
        match self {
            Self::Press { control }
            | Self::Toggle { control }
            | Self::SetText { control, .. }
            | Self::SetChecked { control, .. }
            | Self::Click { control }
            | Self::Pick { control, .. } => control,
        }
    }

    #[note("dialog-tool-dialogop-kind-str")]
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Press { .. } => "press",
            Self::Toggle { .. } => "toggle",
            Self::SetText { .. } => "set_text",
            Self::SetChecked { .. } => "set_checked",
            Self::Click { .. } => "click",
            Self::Pick { .. } => "pick",
        }
    }
}

#[note("dialog-tool-refuse")]
fn refuse(code: &str, said: impl Into<String>) -> Option<Result<DialogOp, Answer>> {
    Some(Err((ToolCallResult::error_text(code, said), "error")))
}

#[note("dialog-tool-plan")]
pub fn plan(
    name: &str,
    args: &Value,
    open: Option<&DialogSpec>,
) -> Option<Result<DialogOp, Answer>> {
    if name != "dialog.press" && name != "dialog.set_field" {
        return None;
    }

    let Some(spec) = open else {
        let said = "No dialog is currently open.";
        return refuse("no-dialog", said);
    };

    let Some(control_name) = nonempty(args, "control") else {
        let said = "Missing or empty 'control' parameter.";
        return refuse("invalid-arguments", said);
    };

    let controls = spec.expanded();
    let Some(found) = controls.iter().find(|c| c.name == control_name) else {
        let valid: Vec<&str> = controls.iter().map(|c| c.name.as_str()).collect();
        let said = format!(
            "Unknown control '{control_name}' in dialog '{}'. Valid controls: {}",
            spec.name,
            valid.join(", ")
        );
        return refuse("unknown-control", said);
    };

    match found.kind {
        ControlKind::Label => {
            let said = format!("Control '{control_name}' is a label: labels are read-only.");
            refuse("unsupported-control", said)
        }
        ControlKind::Row if name == "dialog.press" => Some(Ok(DialogOp::Click {
            control: control_name.to_string(),
        })),
        ControlKind::Row => {
            let said = format!("Control '{control_name}' is a card: dialog.press clicks it.");
            refuse("unsupported-control", said)
        }
        ControlKind::Drop if name == "dialog.press" => {
            let said = format!(
                "Control '{control_name}' is a drop-down: dialog.set_field picks an item by its text."
            );
            refuse("unsupported-control", said)
        }
        ControlKind::Drop => {
            let Some(val) = args.get("value").and_then(Value::as_str) else {
                let said = format!(
                    "Control '{control_name}' is a drop-down: dialog.set_field requires the item's text as a string 'value'."
                );
                return refuse("invalid-arguments", said);
            };
            Some(Ok(DialogOp::Pick {
                control: control_name.to_string(),
                label: val.to_string(),
            }))
        }
        ControlKind::Button if name == "dialog.press" => Some(Ok(DialogOp::Press {
            control: control_name.to_string(),
        })),
        ControlKind::Button => {
            let said = format!(
                "Control '{control_name}' is a button: dialog.set_field takes a Text or a Check."
            );
            refuse("unsupported-control", said)
        }
        ControlKind::Check if name == "dialog.press" => Some(Ok(DialogOp::Toggle {
            control: control_name.to_string(),
        })),
        ControlKind::Check => {
            let Some(val) = args.get("value").and_then(Value::as_bool) else {
                let said = format!(
                    "Control '{control_name}' is a check box: dialog.set_field requires a boolean 'value'."
                );
                return refuse("invalid-arguments", said);
            };
            Some(Ok(DialogOp::SetChecked {
                control: control_name.to_string(),
                value: val,
            }))
        }
        ControlKind::Text if name == "dialog.press" => {
            let said = format!(
                "Control '{control_name}' is a text field: dialog.press takes a Button or a Check."
            );
            refuse("unsupported-control", said)
        }
        ControlKind::Text => {
            let Some(val) = args.get("value").and_then(Value::as_str) else {
                let said = format!(
                    "Control '{control_name}' is a text field: dialog.set_field requires a string 'value'."
                );
                return refuse("invalid-arguments", said);
            };
            Some(Ok(DialogOp::SetText {
                control: control_name.to_string(),
                value: val.to_string(),
            }))
        }
    }
}

#[note("dialog-tool-done")]
pub fn done(op: &DialogOp, dialog: &str, driven: Result<(), &str>) -> Answer {
    if let Err(code) = driven {
        let control = op.control();
        let said = match op {
            DialogOp::Pick { label, .. } if code == "unknown-item" => {
                format!("'{label}' is not an item of drop-down '{control}' in dialog '{dialog}'.")
            }
            _ => format!(
                "Control '{control}' is not on screen in dialog '{dialog}': show its page first."
            ),
        };
        return (ToolCallResult::error_text(code, said), "error");
    }

    let data = json!({
        "dialog": dialog,
        "control": op.control(),
        "op": op.kind_str(),
        "status": "ok",
    });
    let said = match op {
        DialogOp::Press { control } => {
            format!("Pressed '{control}' in dialog '{dialog}'.")
        }
        DialogOp::Toggle { control } => {
            format!("Toggled '{control}' in dialog '{dialog}'.")
        }
        DialogOp::SetText { control, value } => {
            format!("Set text of '{control}' to \"{value}\" in dialog '{dialog}'.")
        }
        DialogOp::SetChecked { control, value } => {
            format!("Set check of '{control}' to {value} in dialog '{dialog}'.")
        }
        DialogOp::Click { control } => {
            format!("Clicked '{control}' in dialog '{dialog}'.")
        }
        DialogOp::Pick { control, label } => {
            format!("Picked \"{label}\" in '{control}' in dialog '{dialog}'.")
        }
    };
    (ToolCallResult::success_json(data, said), "applied")
}

#[cfg(test)]
#[path = "test/dialog_tool/test.rs"]
mod tests;
