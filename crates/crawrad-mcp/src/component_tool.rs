//! The `project.install_component` tool: a package enters the palette now.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;
use crawrad_note::note;

/// What one install gives back: the package, its classes and where it landed.
pub type Installed = (String, Vec<String>, PathBuf);

fn refuse(code: &str, message: &str) -> Answer {
    (ToolCallResult::error_text(code, message), "error")
}

#[note("component-tool-answer")]
pub fn answer(args: &Value, install: impl FnOnce(&Path) -> Result<Installed, String>) -> Answer {
    let Some(path) = args.get("path").and_then(Value::as_str).map(Path::new) else {
        return refuse(
            "invalid-arguments",
            "'path' is required: the package folder.",
        );
    };
    if !path.is_absolute() {
        return refuse(
            "invalid-arguments",
            "'path' must be an absolute folder path.",
        );
    }
    match install(path) {
        Ok((package, classes, destination)) => {
            let summary = format!(
                "Installed '{package}': {} component(s) in the palette.",
                classes.len()
            );
            let data = json!({
                "package": package,
                "components": classes,
                "folder": destination.display().to_string(),
            });
            (ToolCallResult::success_json(data, summary), "applied")
        }
        Err(reason) => refuse("install-failed", &reason),
    }
}

#[cfg(test)]
#[path = "test/component_tool/test.rs"]
mod tests;
