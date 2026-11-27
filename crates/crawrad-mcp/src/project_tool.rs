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

/// Builds the workspace and buffer status summary for `project.info`.
pub fn workspace_info(
    root: Option<String>,
    active_file: Option<String>,
    center_view: &str,
    is_dirty: bool,
    open_buffers: Vec<String>,
) -> ToolCallResult {
    let result = json!({
        "workspace_root": root,
        "active_file": active_file,
        "center_view": center_view,
        "is_dirty": is_dirty,
        "open_buffers": open_buffers,
    });
    ToolCallResult::success_json(result, "Project workspace info retrieved.")
}

/// Builds the audited answer pair for `project.info` from raw host paths.
pub fn workspace_info_answer(
    root: Option<&Path>,
    active_file: Option<&Path>,
    center_view: &str,
    is_dirty: bool,
    open_files: &[PathBuf],
) -> Answer {
    let r = root.map(|p| p.display().to_string());
    let a = active_file.map(|p| p.display().to_string());
    let o = open_files.iter().map(|p| p.display().to_string()).collect();
    (workspace_info(r, a, center_view, is_dirty, o), "applied")
}

/// Parses arguments for `project.create_file`.
pub fn parse_create_file_args(args: &Value) -> Result<(&str, &str, bool), Answer> {
    let Some(path_str) = cargo_tool::nonempty(args, "path") else {
        return Err((
            ToolCallResult::error_text("invalid-params", "Missing 'path' parameter"),
            "error",
        ));
    };
    let content = args.get("content").and_then(Value::as_str).unwrap_or("");
    let open_in_editor = args
        .get("open_in_editor")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    Ok((path_str, content, open_in_editor))
}

/// Builds the success envelope for a created project file.
pub fn file_created_result(path: &str, bytes_written: usize) -> ToolCallResult {
    let created = json!({ "path": path, "bytes_written": bytes_written, "status": "created" });
    ToolCallResult::success_json(created, format!("File '{path}' created successfully."))
}

/// Builds the success envelope for an opened file in editor tab.
pub fn open_file_result(path_str: &str) -> ToolCallResult {
    ToolCallResult::success_json(
        json!({
            "path": path_str,
            "status": "opened"
        }),
        format!("Opened '{path_str}' in editor tab."),
    )
}

/// Parses and validates the crate name argument for `project.add_dependency`.
pub fn parse_add_dependency_arg(args: &Value) -> Result<&str, Answer> {
    let Some(crate_name) = cargo_tool::nonempty(args, "crate_name") else {
        return Err((
            ToolCallResult::error_text("invalid-params", "Missing 'crate_name' parameter"),
            "error",
        ));
    };
    Ok(crate_name)
}

/// Builds the error response when unsaved changes exist in project.
pub fn unsaved_changes_refusal() -> Answer {
    (
        ToolCallResult::error_text("unsaved-changes", UNSAVED_SWITCH),
        "error",
    )
}

/// Builds the refusal when the open visual form has unsaved work.
pub fn unsaved_work_refusal() -> Answer {
    let said = "The open form has unsaved changes: run file.save_all or close it";
    (ToolCallResult::error_text("unsaved-work", said), "rejected")
}

/// Validates the path argument for `project.open`.
pub fn check_open_path(args: &Value) -> Result<PathBuf, Answer> {
    let Some(path_str) = cargo_tool::nonempty(args, "path") else {
        let said = "Missing or empty 'path' parameter";
        return Err((ToolCallResult::error_text("invalid-params", said), "error"));
    };
    let target = PathBuf::from(path_str);
    if !target.exists() {
        let said = format!("Path '{path_str}' does not exist.");
        return Err((ToolCallResult::error_text("not-found", said), "error"));
    }
    Ok(target)
}

/// Builds the answer for extension installation or uninstallation.
pub fn extension_tool_result(done: Result<String, String>) -> Answer {
    match done {
        Ok(said) => (
            ToolCallResult::success_json(json!({ "status": said }), said),
            "applied",
        ),
        Err(said) => (
            ToolCallResult::error_text("extension-refused", said),
            "error",
        ),
    }
}

/// Adds a crate dependency in the project root directory.
pub fn add_dependency_in_project(root: Option<PathBuf>, args: &Value) -> Answer {
    let crate_name = match parse_add_dependency_arg(args) {
        Ok(name) => name,
        Err(refused) => return refused,
    };
    crate::run_tool::in_project(root, |d| cargo_tool::add_dependency(d, crate_name, args))
}

/// Dispatches workspace inspection and dependency queries.
pub fn dispatch_workspace(name: &str, root: Option<PathBuf>, args: &Value) -> Option<Answer> {
    match name {
        "project.get_tree" => Some(get_tree(root, args)),
        "project.get_dependencies" => Some(get_dependencies(root)),
        "project.add_dependency" => Some(add_dependency_in_project(root, args)),
        "project.build" => Some(crate::run_tool::in_project(root, |d| {
            cargo_tool::run(d, args)
        })),
        _ => None,
    }
}

/// The target of an open command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenDestination {
    /// Opens an entire project folder.
    Project(PathBuf),
    /// Opens a visual form file (`.crawform`).
    Form(PathBuf),
    /// Opens a generic file in the text editor.
    File(PathBuf),
}

/// Evaluates unsaved changes guard and resolves the target destination for `project.open`.
pub fn plan_open(args: &Value, has_unsaved_changes: bool) -> Result<OpenDestination, Answer> {
    let target_path = check_open_path(args)?;
    if target_path.is_dir() && has_unsaved_changes {
        return Err(unsaved_changes_refusal());
    }
    if target_path.is_dir() {
        Ok(OpenDestination::Project(target_path))
    } else if target_path.extension().is_some_and(|ext| ext == "crawform") {
        Ok(OpenDestination::Form(target_path))
    } else {
        Ok(OpenDestination::File(target_path))
    }
}

/// Prepares parameters and verifies guards for `project.create_file`.
pub fn prepare_create_file<'a>(
    args: &'a Value,
    root: Option<&Path>,
    pending_review: bool,
    disable_gate: bool,
) -> Result<(&'a str, &'a str, bool, PathBuf, String), Answer> {
    let (path_str, content, open_in_editor) = parse_create_file_args(args)?;
    let target_path = crate::project_tree::resolve(path_str, root);
    if let Some(busy) = crate::edit::check_review_busy(pending_review, disable_gate, path_str) {
        return Err(busy);
    }
    let path = target_path.display().to_string();
    Ok((path_str, content, open_in_editor, target_path, path))
}

/// Writes a newly created file to disk, creating parent folders if needed.
pub fn write_created_file(
    target_path: &Path,
    content: &str,
    path_str: &str,
) -> Result<usize, Answer> {
    if let Some(parent) = target_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(target_path, content)
        .map(|()| content.len())
        .map_err(|e| {
            (
                ToolCallResult::error_text(
                    "io-error",
                    format!("Failed to create file '{path_str}': {e}"),
                ),
                "error",
            )
        })
}

/// Validates arguments and state guards for creating a new project.
pub fn check_create_guards(
    args: &Value,
    has_unsaved_changes: bool,
    has_unsaved_doc: bool,
) -> Result<(&str, &str, &str, bool, Option<&str>), Answer> {
    let parsed = create_args(args)?;
    if has_unsaved_changes {
        return Err(unsaved_changes_refusal());
    }
    if has_unsaved_doc {
        return Err(unsaved_work_refusal());
    }
    Ok(parsed)
}

/// Shapes the success message after opening a project destination.
pub fn open_destination_result(path: &Path) -> ToolCallResult {
    ToolCallResult::success_text(format!("Opened '{}' in the IDE.", path.display()))
}

#[cfg(test)]
#[path = "test/project_tool/test.rs"]
mod tests;
