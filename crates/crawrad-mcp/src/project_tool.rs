//! The `project.*` tools that only touch files: the tree, a new form, a
//! deleted file and the dependencies. Pure of the window, so they live here;
//! the shell opens or closes what the IDE shows and writes the audit line.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

/// Why `project.open` / `project.create` refuse while the open project has unsaved
/// work: switching would close it, and only the person at the IDE may discard it.
pub const UNSAVED_SWITCH: &str = "The open project has unsaved changes. Save them     (file.save_all) or ask the user, then resend — opening another project closes this one.";

use crate::cargo_tool;
use crate::protocol::ToolCallResult;
use crate::run_tool::{Answer, no_project};
use crawrad_note::note;

fn missing(key: &str) -> Answer {
    let said = format!("Missing '{key}' parameter");
    (ToolCallResult::error_text("invalid-params", said), "error")
}

fn io_error(said: String) -> Answer {
    (ToolCallResult::error_text("io-error", said), "error")
}

#[note("project-tool-get-tree")]
pub fn get_tree(root: Option<PathBuf>, args: &Value) -> Answer {
    let Some(root) = root else {
        return no_project();
    };
    let max_depth = args.get("max_depth").and_then(Value::as_u64).unwrap_or(4) as usize;
    let tree = crate::project_tree::walk(&root, 1, max_depth);
    let said = format!("Directory tree for '{}' retrieved.", root.display());
    let data = json!({ "root": root.display().to_string(), "tree": tree });
    (ToolCallResult::success_json(data, said), "applied")
}

#[note("project-tool-create-form")]
pub fn create_form(root: Option<PathBuf>, args: &Value) -> (Answer, Option<PathBuf>) {
    let Some(name) = cargo_tool::nonempty(args, "name") else {
        return (missing("name"), None);
    };
    let Some(root) = root else {
        return (no_project(), None);
    };
    let folder = args.get("folder").and_then(Value::as_str).unwrap_or("src");
    let target_dir = root.join(folder);
    let _ = std::fs::create_dir_all(&target_dir);
    let form_path = target_dir.join(format!("{name}.crawform"));
    let code_path = target_dir.join(format!("{name}.rs"));
    let form_json = crawrad_ir::Document::new(name, "Form")
        .to_json()
        .unwrap_or_else(|_| "{}".to_string());
    if let Err(e) = std::fs::write(&form_path, form_json) {
        return (io_error(format!("Failed to write form file: {e}")), None);
    }
    let _ = std::fs::write(&code_path, cargo_tool::form_unit(name));
    let said = format!(
        "Form '{name}' created at '{}' and opened in designer.",
        form_path.display()
    );
    let data = json!({
        "form_file": form_path.display().to_string(),
        "code_file": code_path.display().to_string(),
        "status": "created"
    });
    (
        (ToolCallResult::success_json(data, said), "applied"),
        Some(form_path),
    )
}

#[note("project-tool-delete-file")]
pub fn delete_file(root: Option<&Path>, args: &Value) -> (Answer, Option<PathBuf>) {
    let Some(path_str) = cargo_tool::nonempty(args, "path") else {
        return (missing("path"), None);
    };
    let target = crate::project_tree::resolve(path_str, root);
    if !target.exists() {
        let said = format!("File '{path_str}' does not exist.");
        return (
            (ToolCallResult::error_text("file-not-found", said), "error"),
            None,
        );
    }
    if let Err(e) = std::fs::remove_file(&target) {
        return (
            io_error(format!("Failed to delete file '{path_str}': {e}")),
            None,
        );
    }
    let said = format!("File '{}' deleted.", target.display());
    let data = json!({ "deleted_path": target.display().to_string(), "status": "deleted" });
    (
        (ToolCallResult::success_json(data, said), "applied"),
        Some(target),
    )
}

#[note("project-tool-get-dependencies")]
pub fn get_dependencies(root: Option<PathBuf>) -> Answer {
    let Some(root) = root else {
        return no_project();
    };
    let Ok(toml) = std::fs::read_to_string(root.join("Cargo.toml")) else {
        return io_error("Failed to read Cargo.toml in project root.".into());
    };
    let deps = cargo_tool::dependencies(&toml);
    let said = format!("Found {} dependencies in Cargo.toml.", deps.len());
    let data = json!({ "dependencies": deps, "count": deps.len() });
    (ToolCallResult::success_json(data, said), "applied")
}

#[note("project-tool-create-args")]
pub fn create_args(args: &Value) -> Result<(&str, &str, &str, bool, Option<&str>), Answer> {
    let empty = |key: &str| {
        let said = format!("Missing or empty '{key}' parameter");
        (ToolCallResult::error_text("invalid-params", said), "error")
    };
    let name = cargo_tool::nonempty(args, "name").ok_or_else(|| empty("name"))?;
    let key = if args.get("folder").is_some() {
        "folder"
    } else {
        "path"
    };
    let folder = cargo_tool::nonempty(args, key).ok_or_else(|| empty("folder"))?;
    let template = args
        .get("template")
        .and_then(Value::as_str)
        .unwrap_or("gui_app");
    let git_init = args
        .get("git_init")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let framework = args.get("framework").and_then(Value::as_str);
    Ok((name, folder, template, git_init, framework))
}

#[note("project-tool-created")]
pub fn created(made: &Result<PathBuf, String>, name: &str, template: &str) -> Answer {
    match made {
        Ok(dir) => {
            let at = dir.display().to_string();
            let said = format!("Project '{name}' created successfully at '{at}'");
            let data = json!({ "created_path": at, "template": template, "status": "ok" });
            (ToolCallResult::success_json(data, said), "applied")
        }
        Err(e) => (
            ToolCallResult::error_text("creation-error", e.clone()),
            "error",
        ),
    }
}

#[cfg(test)]
#[path = "test/project_tool/test.rs"]
mod tests;
