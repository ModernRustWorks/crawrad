//! MCP host tools: Project & Workspace (project.*).

use super::*;
use crawrad_agent::diff_review::DiffReviewMode;
use crawrad_mcp::protocol::ToolCallResult;
use makepad_widgets::Cx;
use serde_json::{Value, json};
impl McpServer {
    // --- Category 3: Project & Workspace (project.*) ---
    pub(super) fn tool_project_create_file(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        args: &Value,
    ) -> ToolCallResult {
        let tool = "project.create_file";
        let (path_str, content, open_in_editor) =
            match crawrad_mcp::project_tool::parse_create_file_args(args) {
                Ok(parsed) => parsed,
                Err(refused) => return self.audited(tool, args, refused),
            };
        let target_path =
            crawrad_mcp::project_tree::resolve(path_str, app.raiz_projeto().as_deref());
        let pending = app.has_pending_diff_review(&target_path);
        if let Some(busy) = self.review_busy(pending, tool, args, path_str) {
            return busy;
        }
        let current_text = app
            .get_editor_buffer(&target_path)
            .unwrap_or_else(|| std::fs::read_to_string(&target_path).unwrap_or_default());
        let (path, gate) = (
            target_path.display().to_string(),
            !self.disable_diff_review_gate,
        );
        let mode = app.acp_config.diff_review_mode;
        if gate && mode == DiffReviewMode::WaitForApproval {
            if open_in_editor {
                app.abrir_no_editor(cx, target_path.clone());
            }
            // Written on Accept; the next edit bases on the predicted hash.
            let edit = (current_text, content.to_string());
            let landed = app.land_edit(tool, target_path, edit, gate);
            if let Ok(Some(review)) = &landed {
                let hash = review.predicted_hash.clone();
                self.session_read_hashes.insert(canonical_key(&path), hash);
            }
            return self.answer_edit(
                tool,
                args,
                (&path, content),
                landed,
                json!({ "path": &path }),
            );
        }
        if gate && mode == DiffReviewMode::ShowAndApply {
            let edit = (current_text.clone(), content.to_string());
            let _ = app.propose_diff_review(target_path.clone(), edit.0, edit.1, tool);
        }
        app.editor_undo_stacks
            .entry(target_path.clone())
            .or_default()
            .push(current_text);
        if let Some(parent) = target_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = std::fs::write(&target_path, content) {
            self.record_audit(tool, args, "error");
            return ToolCallResult::error_text(
                "io-error",
                format!("Failed to create file '{path_str}': {e}"),
            );
        }
        if open_in_editor {
            app.abrir_no_editor(cx, target_path);
        }
        self.record_audit(tool, args, "applied");
        crawrad_mcp::project_tool::file_created_result(&path, content.len())
    }

    pub(super) fn tool_project_create_form(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        args: &Value,
    ) -> ToolCallResult {
        let (a, form) = crawrad_mcp::project_tool::create_form(app.raiz_projeto(), args);
        if let Some(form) = form {
            let _ = app.abrir_formulario(cx, &form);
        }
        self.audited("project.create_form", args, a)
    }

    pub(super) fn tool_project_delete_file(
        &mut self,
        app: &mut IdeApp,
        _cx: &mut Cx,
        args: &Value,
    ) -> ToolCallResult {
        let root = app.raiz_projeto();
        let (a, gone) = crawrad_mcp::project_tool::delete_file(root.as_deref(), args);
        if let Some(gone) = gone {
            app.close_editor_buffer(&gone);
        }
        self.audited("project.delete_file", args, a)
    }
}
