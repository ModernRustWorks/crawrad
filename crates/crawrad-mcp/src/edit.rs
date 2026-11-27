//! The text half of the edit tools (`editor.apply_edit`, `code.edit`): what
//! the new buffer is, from the arguments and the text the editor holds. The
//! window decides where it lands (review, undo, buffer).

use crawrad_actions::ViewAffinity;
use crawrad_note::note;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;

/// Computes lowercase hex-encoded SHA-256 hash of bytes.
pub fn hex_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Refuses an edit when a diff review is already pending for this file.
pub fn check_review_busy(pending: bool, disable_gate: bool, path: &str) -> Option<Answer> {
    if disable_gate || !pending {
        None
    } else {
        Some((review_busy_error(path), "rejected"))
    }
}

#[note("edit-refusal")]
pub type Refusal = (&'static str, String);

#[note("edit-busy")]
pub fn busy(path: &str) -> String {
    format!(
        "File '{path}' has a pending diff review awaiting user decision. Please wait for the user to accept or reject before submitting further edits."
    )
}

#[note("edit-splice")]
pub fn splice(current: &str, args: &Value) -> Result<String, Refusal> {
    let line = |k: &str| args.get(k).and_then(Value::as_u64).map(|n| n as usize);
    let text = |k: &str| args.get(k).and_then(Value::as_str);
    match (
        line("start_line"),
        line("end_line"),
        text("new_text"),
        text("content"),
    ) {
        (Some(start), Some(end), Some(replacement), _) => {
            let lines: Vec<&str> = current.lines().collect();
            let total = lines.len();
            if start == 0 || start > end || start > total + 1 {
                let said =
                    format!("Invalid line range {start}..={end} for file with {total} lines.");
                return Err(("invalid-line-range", said));
            }
            let after = &lines[end.min(total)..];
            let mut out = String::new();
            for l in &lines[..start - 1] {
                out.push_str(l);
                out.push('\n');
            }
            out.push_str(replacement);
            if !replacement.ends_with('\n') && !after.is_empty() {
                out.push('\n');
            }
            for (i, l) in after.iter().enumerate() {
                out.push_str(l);
                if i + 1 < after.len() || current.ends_with('\n') {
                    out.push('\n');
                }
            }
            Ok(out)
        }
        (_, _, _, Some(full)) => Ok(full.to_string()),
        _ => Err((
            "invalid-params",
            "Must provide either (start_line, end_line, new_text) or 'content'".into(),
        )),
    }
}

#[note("edit-selection-range")]
pub fn selection_range(text: &str, ends: [(usize, usize); 2]) -> Value {
    let at = |(line, byte): (usize, usize)| {
        let row = text.lines().nth(line).unwrap_or("");
        let column = row.get(..byte).map_or(byte, |head| head.chars().count());
        serde_json::json!({ "line": line + 1, "column": column + 1 })
    };
    serde_json::json!({ "start": at(ends[0]), "end": at(ends[1]) })
}

#[note("edit-resolve-path")]
pub fn resolve_path(
    args: &Value,
    active: Option<&std::path::Path>,
) -> Result<String, (&'static str, &'static str)> {
    args.get("path")
        .and_then(Value::as_str)
        .map(String::from)
        .or_else(|| active.map(|p| p.display().to_string()))
        .ok_or((
            "no-file",
            "No file specified and no active file currently in editor.",
        ))
}

#[note("edit-absolute-path-arg")]
pub fn absolute_path_arg(args: &Value, bases: &[Option<PathBuf>]) -> Option<Value> {
    let bases: Vec<&Path> = bases.iter().flatten().map(PathBuf::as_path).collect();
    let path_str = args.get("path").and_then(Value::as_str)?;
    if path_str.is_empty() || bases.is_empty() {
        return None;
    }
    let path = Path::new(path_str);
    if path.is_absolute() || path.has_root() {
        return None;
    }
    let resolved = bases
        .iter()
        .map(|base| base.join(path))
        .find(|candidate| candidate.exists())
        .unwrap_or_else(|| bases[0].join(path));
    let mut new_args = args.clone();
    if let Some(obj) = new_args.as_object_mut() {
        obj.insert(
            "path".to_string(),
            Value::String(resolved.display().to_string()),
        );
    }
    Some(new_args)
}

#[note("edit-activefileparams")]
pub struct ActiveFileParams<'a> {
    pub path: Option<&'a str>,
    pub is_dirty: bool,
    pub is_editable: bool,
    pub center_view: &'a str,
    pub tab_file: Option<&'a str>,
    pub read_only: bool,
    pub caret: Option<(usize, usize)>,
    pub status: Option<&'a str>,
    pub breadcrumb: Vec<String>,
    pub disk_changed: bool,
    pub deleted_on_disk: bool,
}

#[note("edit-active-file-json")]
pub fn active_file_json(params: ActiveFileParams<'_>) -> (Value, String) {
    let caret = params
        .caret
        .map(|(line, byte)| serde_json::json!({ "line": line, "byte": byte }));
    let said = match params.path {
        Some(p) => format!("Active editor file: '{p}' (dirty: {})", params.is_dirty),
        None => "No file is currently active in the editor.".to_string(),
    };
    let json = serde_json::json!({
        "path": params.path,
        "is_dirty": params.is_dirty,
        "is_editable": params.is_editable,
        "center_view": params.center_view,
        "tab_file": params.tab_file,
        "read_only": params.read_only,
        "caret": caret,
        "status": params.status,
        "breadcrumb": params.breadcrumb,
        "disk_changed": params.disk_changed,
        "deleted_on_disk": params.deleted_on_disk,
    });
    (json, said)
}

#[note("edit-wrong-view")]
pub fn wrong_view(needs: crawrad_actions::ViewAffinity) -> (&'static str, &'static str) {
    use crawrad_actions::ViewAffinity::{Code, Design};
    match needs {
        Design => (
            "wrong-view-code",
            "The form designer is not active. Call 'view.toggle_code' and resend.",
        ),
        Code => (
            "wrong-view-design",
            "The code editor is not active. Call 'view.toggle_code' and resend.",
        ),
        _ => (
            "wrong-view",
            "Active view is not compatible with this capability.",
        ),
    }
}

/// Builds the standard error result when a file has a pending diff review.
pub fn review_busy_error(path: &str) -> ToolCallResult {
    ToolCallResult::error_text("pending-review-busy", busy(path))
}

/// Formats the result envelope when an edit produces a pending diff review.
pub fn pending_review_result(
    mut answer: Value,
    path_str: &str,
    predicted_hash: &str,
    additions: usize,
    deletions: usize,
) -> ToolCallResult {
    answer["content_hash"] = json!(predicted_hash);
    answer["status"] = json!("pending_approval");
    answer["pending_diff_review"] = json!(true);
    answer["additions"] = json!(additions);
    answer["deletions"] = json!(deletions);
    let said = format!(
        "Edit proposed for '{path_str}' (predicted hash: {predicted_hash}). Awaiting human review in editor."
    );
    ToolCallResult::success_json(answer, said)
}

/// Formats the result envelope when an edit is directly applied without diff review gate.
pub fn applied_edit_result(mut answer: Value, path_str: &str, hash: &str) -> ToolCallResult {
    answer["content_hash"] = json!(hash);
    answer["status"] = json!("ok");
    answer["pending_diff_review"] = json!(false);
    ToolCallResult::success_json(answer, format!("Edited '{path_str}' (hash: {hash})."))
}

/// Builds the inspection envelope for editor selection and active document info.
pub fn selection_info(
    active_file: Option<String>,
    has_active_editor: bool,
    total_length: usize,
    text: Option<String>,
    range: Option<Value>,
) -> ToolCallResult {
    let data = json!({
        "active_file": active_file,
        "has_active_editor": has_active_editor,
        "total_text_length": total_length,
        "text": text,
        "range": range,
    });
    ToolCallResult::success_json(data, "Editor selection inspected.")
}

/// Builds the result envelope for listing open files in editor.
pub fn open_files_result(files: Value) -> ToolCallResult {
    let said = format!("{} files open in editor.", files["count"]);
    ToolCallResult::success_json(files, said)
}

/// Verifies that a file was previously read in the current session and base_hash matches.
pub fn verify_read_before_edit(
    session_read_hashes: &std::collections::HashMap<String, String>,
    ckey: &str,
    path_str: &str,
    base_hash: &str,
) -> Result<(), Answer> {
    let Some(expected_hash) = session_read_hashes.get(ckey) else {
        let said =
            format!("File '{path_str}' has not been read in this session. Call 'code.read' first.");
        return Err((
            ToolCallResult::error_text("no-prior-read", said),
            "rejected",
        ));
    };
    if expected_hash != base_hash {
        let said = format!(
            "base_hash mismatch for '{path_str}'. Expected '{expected_hash}', got '{base_hash}'. Call 'code.read' to refresh."
        );
        return Err((
            ToolCallResult::error_text("hash-mismatch", said),
            "rejected",
        ));
    }
    Ok(())
}

/// Directly builds a ToolCallResult from [`ActiveFileParams`].
pub fn active_file_result(params: ActiveFileParams) -> ToolCallResult {
    let (json, said) = active_file_json(params);
    ToolCallResult::success_json(json, said)
}

/// Builds the metadata summary envelope for a spliced editor edit.
pub fn edit_lines_summary(path_str: &str, after: &str) -> Value {
    json!({
        "path": path_str,
        "lines_count": after.lines().count(),
        "total_bytes": after.len(),
    })
}

/// Validates optional base_hash against session hashes if present.
pub fn check_optional_base_hash(
    session_read_hashes: &std::collections::HashMap<String, String>,
    ckey: &str,
    path_str: &str,
    args: &Value,
) -> Result<(), Answer> {
    if let Some(base_hash) = args.get("base_hash").and_then(Value::as_str)
        && let Some(expected_hash) = session_read_hashes.get(ckey)
        && expected_hash != base_hash
    {
        let said = format!(
            "base_hash mismatch for '{path_str}'. Expected '{expected_hash}', got '{base_hash}'. Refresh with editor.get_buffer_content first."
        );
        return Err((
            ToolCallResult::error_text("hash-mismatch", said),
            "rejected",
        ));
    }
    Ok(())
}

/// Parses the required path argument for `code.read`.
pub fn parse_code_read_path(args: &Value) -> Result<&str, Answer> {
    crate::cargo_tool::required(args, "code.read", &["path"])
        .map(|f| f[0])
        .map_err(|e| (ToolCallResult::error_text("invalid-params", e), "error"))
}

/// Parses required path, base_hash, and content arguments for `code.edit`.
pub fn parse_code_edit_args(args: &Value) -> Result<(&str, &str, &str), Answer> {
    crate::cargo_tool::required(args, "code.edit", &["path", "base_hash", "content"])
        .map(|f| (f[0], f[1], f[2]))
        .map_err(|e| (ToolCallResult::error_text("invalid-params", e), "error"))
}

/// Validates the path argument and existence for `editor.open_file`.
pub fn parse_editor_open_file_path(args: &Value, exists: bool) -> Result<&str, Answer> {
    let Some(path_str) = crate::cargo_tool::nonempty(args, "path") else {
        return Err((
            ToolCallResult::error_text("invalid-params", "Missing 'path' parameter"),
            "error",
        ));
    };
    if !exists {
        return Err((
            ToolCallResult::error_text(
                "file-not-found",
                format!("File '{path_str}' does not exist on disk."),
            ),
            "error",
        ));
    }
    Ok(path_str)
}

/// Prepares a `code.edit` operation by validating view affinity, arguments, pending review state,
/// and verifying that the file was previously read in this session.
pub fn prepare_code_edit<'a>(
    args: &'a Value,
    is_design_view: bool,
    pending_review: bool,
    disable_gate: bool,
    session_read_hashes: &HashMap<String, String>,
    current_content: Option<String>,
) -> Result<(&'a str, PathBuf, String, &'a str), Answer> {
    if is_design_view {
        let (code, said) = wrong_view(ViewAffinity::Code);
        return Err((ToolCallResult::error_text(code, said), "rejected"));
    }
    let (path_str, base_hash, content) = parse_code_edit_args(args)?;
    if let Some(busy) = check_review_busy(pending_review, disable_gate, path_str) {
        return Err(busy);
    }
    let ckey = crate::project_tree::canonical_key(path_str);
    if let Err((refusal, outcome)) =
        verify_read_before_edit(session_read_hashes, &ckey, path_str, base_hash)
    {
        return Err((refusal, outcome));
    }
    let current = current_content.unwrap_or_default();
    Ok((path_str, PathBuf::from(path_str), current, content))
}

/// Prepares an `editor.apply_edit` operation by resolving path, verifying review and hash guards,
/// and splicing text.
pub fn prepare_apply_edit(
    args: &Value,
    active_path: Option<&Path>,
    pending_review: bool,
    disable_gate: bool,
    session_read_hashes: &HashMap<String, String>,
    current_content: Option<String>,
) -> Result<(String, PathBuf, String, String, Value), Answer> {
    let path_str = resolve_path(args, active_path)
        .map_err(|(code, said)| (ToolCallResult::error_text(code, said), "error"))?;
    if let Some(busy) = check_review_busy(pending_review, disable_gate, &path_str) {
        return Err(busy);
    }
    let ckey = crate::project_tree::canonical_key(&path_str);
    check_optional_base_hash(session_read_hashes, &ckey, &path_str, args)?;
    let current = current_content.unwrap_or_default();
    let after = splice(&current, args)
        .map_err(|(code, said)| (ToolCallResult::error_text(code, said), "error"))?;
    let lines = edit_lines_summary(&path_str, &after);
    Ok((
        path_str.clone(),
        PathBuf::from(&path_str),
        current,
        after,
        lines,
    ))
}

/// Prepares a unified read target for `code.read`, `editor.read_file`, or `editor.get_buffer_content`.
pub fn prepare_read_target(
    tool: &str,
    args: &Value,
    is_design_view: bool,
    active_path: Option<&Path>,
) -> Result<String, Answer> {
    if tool.starts_with("code.") && is_design_view {
        let (code, said) = wrong_view(ViewAffinity::Code);
        return Err((ToolCallResult::error_text(code, said), "rejected"));
    }
    if tool.starts_with("code.") {
        parse_code_read_path(args).map(|s| s.to_string())
    } else {
        resolve_path(args, active_path)
            .map_err(|(code, said)| (ToolCallResult::error_text(code, said), "error"))
    }
}

/// Prepares a unified edit target for `code.edit`, `editor.edit_file`, or `editor.apply_edit`.
pub fn prepare_unified_edit(
    tool: &str,
    args: &Value,
    is_design_view: bool,
    active_path: Option<&Path>,
    pending_review: bool,
    disable_gate: bool,
    session_read_hashes: &HashMap<String, String>,
    current_content: Option<String>,
) -> Result<(String, PathBuf, Option<String>, String, Value), Answer> {
    if tool.starts_with("code.") {
        let (path_str, path_buf, current, content) = prepare_code_edit(
            args,
            is_design_view,
            pending_review,
            disable_gate,
            session_read_hashes,
            current_content,
        )?;
        Ok((
            path_str.to_string(),
            path_buf,
            Some(current),
            content.to_string(),
            serde_json::json!({}),
        ))
    } else {
        let (path_str, path_buf, current, after, lines) = prepare_apply_edit(
            args,
            active_path,
            pending_review,
            disable_gate,
            session_read_hashes,
            current_content,
        )?;
        Ok((path_str, path_buf, Some(current), after, lines))
    }
}

/// Answers `editor.get_selection` by computing character ranges and formatting results.
pub fn editor_selection_answer(
    active_file: Option<&Path>,
    buffer: Option<&str>,
    selected_text: Option<String>,
    bounds: Option<[(usize, usize); 2]>,
    text_in_view: &str,
) -> Answer {
    let range = bounds.map(|r| selection_range(text_in_view, r));
    let len = buffer.map(|t| t.len()).unwrap_or(0);
    let res = selection_info(
        active_file.map(|p| p.display().to_string()),
        buffer.is_some(),
        len,
        selected_text,
        range,
    );
    (res, "applied")
}

/// Builds the active file tool call result from raw host and view parameters.
pub fn make_active_file_result(
    active_path: Option<&Path>,
    tab_path: Option<&Path>,
    is_dirty: bool,
    center_view: &str,
    is_text: bool,
    read_only: bool,
    caret: Option<(usize, usize)>,
    status: Option<&str>,
    breadcrumb: Vec<String>,
    disk_flags: (bool, bool),
) -> ToolCallResult {
    let path = active_path.map(|p| p.display().to_string());
    let tab = tab_path.map(|p| p.display().to_string());
    active_file_result(ActiveFileParams {
        path: path.as_deref(),
        is_dirty,
        is_editable: is_text && active_path.is_some(),
        center_view,
        tab_file: tab.as_deref(),
        read_only,
        caret,
        status,
        breadcrumb,
        disk_changed: disk_flags.0,
        deleted_on_disk: disk_flags.1,
    })
}

/// Information from a landed diff review.
#[derive(Debug, Clone)]
pub struct LandedReview<'a> {
    pub predicted_hash: &'a str,
    pub total_additions: usize,
    pub total_deletions: usize,
}

/// Shapes the edit result for both applied edits and pending diff reviews.
pub fn finish_edit_result(
    path_str: &str,
    after: &str,
    landed: Result<Option<LandedReview<'_>>, &str>,
    answer: Value,
) -> Result<(ToolCallResult, String, Option<String>), Answer> {
    match landed {
        Err(e) => Err((ToolCallResult::error_text("diff-review-error", e), "error")),
        Ok(Some(review)) => {
            let res = pending_review_result(
                answer,
                path_str,
                review.predicted_hash,
                review.total_additions,
                review.total_deletions,
            );
            Ok((res, crate::project_tree::canonical_key(path_str), None))
        }
        Ok(None) => {
            let hash = hex_sha256(after.as_bytes());
            let res = applied_edit_result(answer, path_str, &hash);
            Ok((
                res,
                crate::project_tree::canonical_key(path_str),
                Some(hash),
            ))
        }
    }
}

/// Finishes a paged read, calculating hash and formatting the success result.
pub fn finish_paged_read(
    path_str: &str,
    text: Result<String, (&str, String)>,
    args: &Value,
    predicted: Option<String>,
) -> Result<(ToolCallResult, String, String), Answer> {
    let text = match text {
        Ok(t) => t,
        Err((code, said)) => return Err((ToolCallResult::error_text(code, said), "error")),
    };
    let hash = hex_sha256(text.as_bytes());
    let ckey = crate::project_tree::canonical_key(path_str);
    let res = crate::paging::paged_read_success(path_str, &text, &hash, args, predicted);
    Ok((res, ckey, hash))
}

/// Builds the audited answer pair for `editor.list_open_files`.
pub fn list_open_files_answer(files: Value) -> Answer {
    (open_files_result(files), "applied")
}

#[cfg(test)]
#[path = "test/edit/test.rs"]
mod tests;
