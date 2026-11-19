//! MCP host tools: Code Editor & Buffer (editor.*).

use super::*;
use crawrad_host::HostModel;
use crawrad_mcp::protocol::ToolCallResult;
use crawrad_note::note;
use makepad_widgets::Cx;
use serde_json::{Value, json};
use std::path::PathBuf;
impl McpServer {
    // --- Category 2: Code Editor & Buffer (editor.*) ---
    #[note("ide-app-mcp-host-editor-mcpserver-tool-editor-get-active-file")]
    pub(super) fn tool_editor_get_active_file(
        &mut self,
        host: &HostModel,
        (here, read_only, status): (
            Option<crawrad_host::navigation::Place>,
            bool,
            Option<String>,
        ),
        breadcrumb: Vec<String>,
        (disk_changed, deleted_on_disk): (bool, bool),
    ) -> ToolCallResult {
        let path = host.active_file().map(|p| p.display().to_string());
        let tab = here.as_ref().map(|p| p.file.display().to_string());
        let (json, said) =
            crawrad_mcp::edit::active_file_json(crawrad_mcp::edit::ActiveFileParams {
                path: path.as_deref(),
                is_dirty: host.is_dirty(),
                is_editable: host.center().is_text() && host.active_file().is_some(),
                center_view: host.center().as_str(),
                tab_file: tab.as_deref(),
                read_only,
                caret: here.map(|p| (p.line + 1, p.byte)),
                status: status.as_deref(),
                breadcrumb,
                disk_changed,
                deleted_on_disk,
            });
        self.record_audit("editor.get_active_file", &json!({}), "applied");
        ToolCallResult::success_json(json, said)
    }
    pub(super) fn tool_editor_get_buffer_content(
        &mut self,
        app: &IdeApp,
        args: &Value,
    ) -> ToolCallResult {
        let path_str = match crawrad_mcp::edit::resolve_path(args, app.host.active_file()) {
            Ok(p) => p,
            Err((code, said)) => {
                self.record_audit("editor.get_buffer_content", args, "error");
                return ToolCallResult::error_text(code, said);
            }
        };
        let path_buf = PathBuf::from(&path_str);
        let predicted = app
            .get_pending_diff_review(&path_buf)
            .map(|r| r.predicted_hash.clone());
        let at = (path_str.as_str(), path_buf.as_path());
        self.paged_read("editor.get_buffer_content", &app.host, args, at, predicted)
    }

    #[note("ide-app-mcp-host-editor-mcpserver-paged-read")]
    pub(super) fn paged_read(
        &mut self,
        tool: &str,
        host: &HostModel,
        args: &Value,
        (path_str, path): (&str, &std::path::Path),
        predicted: Option<String>,
    ) -> ToolCallResult {
        let ckey = canonical_key(path_str);
        let text = match host.read_text(path, &ckey) {
            Ok(text) => text,
            Err((code, said)) => {
                self.record_audit(tool, args, "error");
                return ToolCallResult::error_text(code, said);
            }
        };
        let hash = hex_sha256(text.as_bytes());
        self.session_read_hashes.insert(ckey, hash.clone());
        let (answer, said) =
            crawrad_mcp::paging::read_answer(path_str, &text, &hash, args, predicted);
        self.record_audit(tool, args, "applied");
        ToolCallResult::success_json(answer, said)
    }

    pub(super) fn tool_editor_apply_edit(
        &mut self,
        app: &mut IdeApp,
        args: &Value,
    ) -> ToolCallResult {
        let tool = "editor.apply_edit";
        let path_str =
            match crawrad_mcp::edit::resolve_path(args, app.active_file_path().as_deref()) {
                Ok(p) => p,
                Err((code, said)) => {
                    self.record_audit(tool, args, "error");
                    return ToolCallResult::error_text(code, said);
                }
            };
        let pending = app.has_pending_diff_review(std::path::Path::new(&path_str));
        if let Some(busy) = self.review_busy(pending, tool, args, &path_str) {
            return busy;
        }
        let ckey = canonical_key(&path_str);
        if let Some(base_hash) = args.get("base_hash").and_then(Value::as_str)
            && let Some(expected_hash) = self.session_read_hashes.get(&ckey)
            && expected_hash != base_hash
        {
            self.record_audit(tool, args, "rejected");
            return ToolCallResult::error_text(
                "hash-mismatch",
                format!(
                    "base_hash mismatch for '{path_str}'. Expected '{expected_hash}', got '{base_hash}'. Refresh with editor.get_buffer_content first."
                ),
            );
        }
        let current = app.host.read_text(std::path::Path::new(&path_str), &ckey);
        let current = current.unwrap_or_default();
        let after = match crawrad_mcp::edit::splice(&current, args) {
            Ok(after) => after,
            Err((code, said)) => {
                self.record_audit(tool, args, "error");
                return ToolCallResult::error_text(code, said);
            }
        };
        let lines = json!({ "path": path_str, "lines_count": after.lines().count(), "total_bytes": after.len() });
        let gate = !self.disable_diff_review_gate;
        let landed = app.land_edit(
            tool,
            PathBuf::from(&path_str),
            (current, after.clone()),
            gate,
        );
        self.answer_edit(tool, args, (&path_str, &after), landed, lines)
    }

    #[note("ide-app-mcp-host-editor-mcpserver-review-busy")]
    pub(super) fn review_busy(
        &mut self,
        pending: bool,
        tool: &str,
        args: &Value,
        path: &str,
    ) -> Option<ToolCallResult> {
        if self.disable_diff_review_gate || !pending {
            return None;
        }
        self.record_audit(tool, args, "rejected");
        let said = crawrad_mcp::edit::busy(path);
        Some(ToolCallResult::error_text("pending-review-busy", said))
    }

    #[note("ide-app-mcp-host-editor-mcpserver-answer-edit")]
    pub(super) fn answer_edit(
        &mut self,
        tool: &str,
        args: &Value,
        (path_str, after): (&str, &str),
        landed: Result<Option<crawrad_agent::diff_review::DiffReview>, String>,
        mut answer: Value,
    ) -> ToolCallResult {
        let review = match landed {
            Ok(review) => review,
            Err(e) => {
                self.record_audit(tool, args, "error");
                return ToolCallResult::error_text("diff-review-error", e);
            }
        };
        self.record_audit(tool, args, "applied");
        if let Some(review) = review {
            answer["content_hash"] = json!(review.predicted_hash);
            answer["status"] = json!("pending_approval");
            answer["pending_diff_review"] = json!(true);
            answer["additions"] = json!(review.total_additions());
            answer["deletions"] = json!(review.total_deletions());
            let hash = &review.predicted_hash;
            let said = format!(
                "Edit proposed for '{path_str}' (predicted hash: {hash}). Awaiting human review in editor."
            );
            return ToolCallResult::success_json(answer, said);
        }
        let hash = hex_sha256(after.as_bytes());
        self.session_read_hashes
            .insert(canonical_key(path_str), hash.clone());
        answer["content_hash"] = json!(hash);
        answer["status"] = json!("ok");
        answer["pending_diff_review"] = json!(false);
        ToolCallResult::success_json(answer, format!("Edited '{path_str}' (hash: {hash})."))
    }

    #[note("ide-app-mcp-host-editor-mcpserver-tool-editor-get-selection")]
    pub(super) fn tool_editor_get_selection(
        &mut self,
        host: &HostModel,
        (selected, range): (Option<String>, Option<Value>),
    ) -> ToolCallResult {
        let text = host.active_file().and_then(|p| host.buffer(p));
        self.record_audit("editor.get_selection", &json!({}), "applied");
        ToolCallResult::success_json(
            json!({
                "active_file": host.active_file().map(|p| p.display().to_string()),
                "has_active_editor": text.is_some(),
                "total_text_length": text.map(|t| t.len()).unwrap_or(0),
                "text": selected,
                "range": range,
            }),
            "Editor selection inspected.",
        )
    }

    pub(super) fn tool_editor_open_file(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        args: &Value,
    ) -> ToolCallResult {
        let Some(path_str) = crawrad_mcp::cargo_tool::nonempty(args, "path") else {
            self.record_audit("editor.open_file", args, "error");
            return ToolCallResult::error_text("invalid-params", "Missing 'path' parameter");
        };
        let path_buf = PathBuf::from(path_str);
        if !path_buf.exists() && app.get_editor_buffer(&path_buf).is_none() {
            self.record_audit("editor.open_file", args, "error");
            return ToolCallResult::error_text(
                "file-not-found",
                format!("File '{path_str}' does not exist on disk."),
            );
        }
        app.abrir_no_editor(cx, path_buf.clone());
        self.record_audit("editor.open_file", args, "applied");
        ToolCallResult::success_json(
            json!({
                "path": path_str,
                "status": "opened"
            }),
            format!("Opened '{path_str}' in editor tab."),
        )
    }

    pub(super) fn tool_editor_list_open_files(&mut self, host: &HostModel) -> ToolCallResult {
        let files = host.open_files_json();
        self.record_audit("editor.list_open_files", &json!({}), "applied");
        let said = format!("{} files open in editor.", files["count"]);
        ToolCallResult::success_json(files, said)
    }
}
