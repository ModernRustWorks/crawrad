//! The in-IDE Model Context Protocol (MCP) server.
//!
//! Executes inside the IDE process, translating incoming JSON-RPC 2.0 requests
//! (`tools/list`, `tools/call`) to IDE actions on the UI thread with
//! `Author::Agent` transaction signing, context guards, and audit logging.

use crate::ide_app::IdeApp;
use crate::ide_app::source::Center;
use crawrad_actions::ViewAffinity;
use crawrad_agent::chat_history;
/// The hash the AI diff review keys on: one function, the agent's.
pub use crawrad_agent::diff_review::hex_sha256;
use crawrad_host::{HostModel, ViewCenter};
use crawrad_mcp::audit::AuditLogger;
pub use crawrad_mcp::listener::McpRequest;
use crawrad_mcp::project_tree::canonical_key;
use crawrad_mcp::protocol::{JsonRpcResponse, ToolCallResult};
use crawrad_mcp::review_tool;
use crawrad_mcp::tools::tools_list_result;
use crawrad_mcp::view_tool::Drive;
use makepad_widgets::Cx;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;

#[note("ide-app-mcp-host-start-mcp-listener")]
pub fn start_mcp_listener(
    instance_id: &str,
) -> std::io::Result<std::sync::mpsc::Receiver<McpRequest>> {
    use makepad_widgets::makepad_platform::thread::SignalToUI;
    crawrad_mcp::listener::start(instance_id, SignalToUI::set_ui_signal)
}
#[note("ide-app-mcp-host-mcpserver")]
pub struct McpServer {
    /// Identifier for this running IDE instance.
    pub instance_id: String,
    /// Identifier for this agent session.
    pub session_id: String,
    /// Map of canonical file path -> SHA-256 hash of content last read in this session.
    pub session_read_hashes: HashMap<String, String>,
    /// Optional custom audit log path for testing.
    pub audit_log_path: Option<PathBuf>,
    /// Optional child process spawned by run.start.
    pub running_process: crawrad_mcp::run_tool::RunSlot,
    /// Mutation proof flag: disables the diff review gate when set to true.
    pub disable_diff_review_gate: bool,
}
impl Default for McpServer {
    fn default() -> Self {
        Self::new("default")
    }
}
impl McpServer {
    #[note("ide-app-mcp-host-mcpserver-new")]
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance = instance_id.into();
        let session = format!("sess-{}", std::process::id());
        Self {
            instance_id: instance,
            session_id: session,
            session_read_hashes: HashMap::new(),
            audit_log_path: None,
            running_process: Default::default(),
            disable_diff_review_gate: false,
        }
    }
    #[note("ide-app-mcp-host-mcpserver-with-audit-log")]
    pub fn with_audit_log(mut self, path: PathBuf) -> Self {
        self.audit_log_path = Some(path);
        self
    }
    fn record_audit(&self, tool: &str, args: &Value, outcome: &str) {
        if let Some(ref path) = self.audit_log_path {
            AuditLogger::record_to_path(path, tool, args, outcome, &self.session_id);
        } else {
            AuditLogger::record(tool, args, outcome, &self.session_id);
        }
    }
    #[note("ide-app-mcp-host-mcpserver-handle-raw-request")]
    pub fn handle_raw_request(&mut self, app: &mut IdeApp, cx: &mut Cx, raw_line: &str) -> String {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        let req = match crawrad_mcp::protocol::parse_raw_request(trimmed) {
            Ok(r) => r,
            Err(err_json) => {
                self.record_audit("invalid_json", &json!(trimmed), "error");
                return err_json;
            }
        };
        // JSON-RPC 2.0: If id is None, it is a notification and MUST NOT receive a response.
        let id = match req.id {
            Some(id) => id,
            None => {
                self.dispatch_notification(app, cx, req.method, req.params.unwrap_or(Value::Null));
                return String::new();
            }
        };
        let resp = self.dispatch(app, cx, req.method, req.params.unwrap_or(Value::Null), id);
        serde_json::to_string(&resp).unwrap_or_default()
    }
    fn dispatch_notification(
        &mut self,
        _app: &mut IdeApp,
        _cx: &mut Cx,
        method: String,
        params: Value,
    ) {
        // Every notification is only recorded (`notifications/initialized` too).
        self.record_audit(&method, &params, "applied");
    }
    fn dispatch(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        method: String,
        params: Value,
        id: Value,
    ) -> JsonRpcResponse {
        match method.as_str() {
            "initialize" => {
                self.record_audit("initialize", &params, "applied");
                JsonRpcResponse::success(id, crawrad_mcp::protocol::initialize_result())
            }
            "notifications/initialized" => {
                self.record_audit("notifications/initialized", &params, "applied");
                JsonRpcResponse::success(id, json!({}))
            }
            "tools/list" => {
                self.record_audit("tools/list", &params, "applied");
                let result = tools_list_result(Some(app.capabilities()));
                JsonRpcResponse::success(id, result)
            }
            "tools/call" => {
                let call_result = self.handle_tools_call(app, cx, &params);
                JsonRpcResponse::success(id, json!(call_result))
            }
            _ => {
                self.record_audit(&method, &params, "error");
                JsonRpcResponse::error(id, -32601, format!("Method not found: '{method}'"), None)
            }
        }
    }
    fn handle_tools_call(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        params: &Value,
    ) -> ToolCallResult {
        // Every command checks itself BEFORE it runs: the name is read, an
        // `action.execute` wrapper unwrapped, relative paths made absolute, and
        // the guardrails and the pending-review save guard consulted — all in
        // `crawrad_mcp`, so the shell holds no copy of that logic.
        let (name, args_val) = match crawrad_mcp::guardrails::prepare_tools_call(
            params,
            &[app.raiz(), app.membro()],
            crawrad_mcp::guardrails::State {
                modal_open: app.modal_open(),
                build_running: app.build_tasks.is_running("build"),
                has_selection: !app.host.selection().is_empty(),
            },
            self.disable_diff_review_gate,
            app.has_any_pending_diff_review(),
        ) {
            Ok(found) => found,
            Err(refusal) => return self.audited(&refusal.0, &refusal.1, (refusal.2, refusal.3)),
        };
        let args = &args_val;
        // The tools that only read a part of the view: what each answers and
        // the line it says, audited and answered alike.
        let read = match name {
            "editor.get_completion" => Some(app.completion_json(cx)),
            "editor.get_choices" => Some(app.nav.choices_json()),
            "view.get_inspector" => Some(app.inspector_json(cx)),
            "editor.get_rename" => Some(app.nav.rename.to_json()),
            "editor.get_references" => Some(app.nav.references.to_json()),
            "editor.get_hover" => Some(app.hover_json()),
            "view.get_dialog" => Some(app.dialog_json(cx)),
            "view.get_status_bar" => Some(app.status_bar_json()),
            "view.get_history" => Some(app.history_tab_json(cx)),
            "view.get_designer" => Some(app.designer_json(cx)),
            "view.get_welcome" => Some(app.welcome_json(cx)),
            "view.get_explorer" => Some(app.explorer_drawn.clone()),
            "view.get_notes" => Some(app.notes_json(cx)),
            "view.get_tests" => Some(app.tests_json(cx)),
            "form.check" => Some(app.form_check_json()),
            "view.sizes" => Some(app.sizes_json()),
            "strings.list" => Some(app.strings_list_json(args)),
            "view.get_rail" => Some(app.rail_json()),
            "view.get_toolbox" => Some(app.toolbox_json(cx)),
            "view.set_toolbox_filter" => Some(app.set_toolbox_filter(cx, &args["text"])),
            "view.get_chat" => Some(app.chat_messages_json(cx)),
            "view.get_extension_views" => Some(app.extension_views_json(cx)),
            "editor.get_line_marks" => Some(app.line_marks_json()),
            "editor.get_semantic_tokens" => Some(app.semantic_json()),
            "editor.get_code_lenses" => Some(app.code_lenses_json()),
            "editor.get_code_actions" => Some(app.code_actions_json(cx)),
            "editor.get_signature_help" => Some(app.signature_json()),
            "editor.get_decorations" => Some(app.squiggles_json()),
            _ => None,
        };
        if let Some(answer) = read {
            self.record_audit(name, args, "applied");
            if let Some(res) = crawrad_mcp::view_tool::view_read_result(name, answer) {
                return res;
            }
        }
        let open = app.open_dialog_spec();
        if let Some(asked) = crawrad_mcp::dialog_tool::plan(name, args, open.as_ref()) {
            let answer = match (asked, open) {
                (Ok(op), Some(spec)) => app.operate_dialog(cx, &spec, &op),
                (Err(refused), _) => refused,
                (Ok(_), None) => unreachable!("plan refuses when no dialog is open"),
            };
            return self.audited(name, args, answer);
        }
        let known = |id: &str| {
            crawrad_actions::is_known_action(id) || app.capabilities().resolve_id(id).is_some()
        };
        if let Some(asked) = crawrad_mcp::view_tool::drive(name, args, app.host.doc(), known) {
            let drive = match asked {
                Ok(drive) => drive,
                Err(refused) => return self.audited(name, args, refused),
            };
            match &drive {
                Drive::RunAction(id) => {
                    let given = crawrad_actions::args_from_json(&args["args"]);
                    if let Some(why) = app.refusal(id, &given) {
                        let refused = crawrad_mcp::view_tool::refused(id, &why);
                        return self.audited(name, args, refused);
                    }
                    app.execute_with(cx, id, given);
                    app.reabrir_no_canvas(cx);
                }
                Drive::ShowPanel(p) => {
                    app.aplicar_arranjo(cx, crawrad_actions::LayoutCommand::Show(*p))
                }
                Drive::Select(id) => app.select_node(cx, id),
            }
            app.sincronizar(cx);
            let a = crawrad_mcp::view_tool::done(&drive, app.host.doc());
            return self.audited(name, args, a);
        }
        match name {
            "strings.set" => {
                let params = crawrad_mcp::strings_tool::params(args);
                let result = params.map(|(key, lang, text)| app.strings_set(cx, key, lang, text));
                return self.audited(name, args, crawrad_mcp::strings_tool::answer(result));
            }
            "code.read" | "editor.read_file" => {
                return self.tool_code_read(&app.host, &app.diff_reviews, args);
            }
            "code.edit" | "editor.edit_file" => return self.tool_code_edit(app, args),
            "editor.get_buffer_content" => return self.tool_editor_get_buffer_content(app, args),
            "editor.apply_edit" => return self.tool_editor_apply_edit(app, args),
            "form.read" | "designer.get_active_form" => return self.tool_form_read(&app.host),
            "designer.list_components" => {
                let a = crawrad_mcp::designer_tool::list_components(app.host.doc());
                return self.audited(name, &json!({}), a);
            }
            "designer.get_selection" => {
                let a = crawrad_mcp::designer_tool::get_selection(app.host.doc());
                return self.audited(name, &json!({}), a);
            }
            "designer.get_component_properties" => {
                let a = crawrad_mcp::designer_tool::get_component_properties(app.host.doc(), args);
                return self.audited(name, args, a);
            }
            "designer.add_component"
            | "designer.set_component_properties"
            | "designer.delete_component"
            | "designer.wire_event" => return self.tool_designer_on_form(app, cx, name, args),
            "editor.get_active_file" => {
                let front = (app.here(cx), app.editavel.is_none(), app.status.clone());
                let items = app.editor_breadcrumb_items();
                let active = app.abas.active_file();
                let disk_changed = active.is_some_and(|a| app.disk.conflict() == Some(a));
                let deleted_on_disk = active.is_some_and(|a| !a.exists());
                return self.tool_editor_get_active_file(
                    &app.host,
                    front,
                    items,
                    (disk_changed, deleted_on_disk),
                );
            }
            "editor.get_selection" => {
                let range = app.editor_selection_bounds();
                let range =
                    range.map(|r| crawrad_mcp::edit::selection_range(&app.texto_na_vista, r));
                let selected = (app.editor_selection_text(), range);
                return self.tool_editor_get_selection(&app.host, selected);
            }
            "editor.open_file" => return self.tool_editor_open_file(app, cx, args),
            "editor.list_open_files" => return self.tool_editor_list_open_files(&app.host),
            "view.get_open_menu" => {
                self.record_audit(name, args, "applied");
                let (rows, sub_rows) = (app.menu_rows(), app.submenu_rows());
                let sub = app
                    .open_submenu
                    .as_ref()
                    .map(|(n, ..)| (n.as_ref(), sub_rows.as_slice()));
                let open = app.open_menu.map(|m| format!("{m:?}"));
                let hint = app.menu_hint.as_ref().map(|(_, why)| why.clone());
                return crawrad_mcp::view_tool::open_menu_result(
                    open,
                    app.menu_highlight(),
                    &rows,
                    sub,
                    app.menu_rects_json(cx),
                    hint,
                );
            }
            "view.capture" => {
                let dpi = cx.windows[app.window_id(cx)].window_geom.dpi_factor;
                return self.audited(name, args, crawrad_mcp::capture_tool::answer(args, dpi));
            }
            "project.install_component" => {
                let answer = crawrad_mcp::component_tool::answer(args, |path| {
                    crawrad_project::installation::install_now(path).into_parts()
                });
                app.reload_registry(cx);
                return self.audited(name, args, answer);
            }
            "designer.set_grid_size" => {
                let size = match crawrad_mcp::designer_tool::set_grid_size_arg(args) {
                    Ok(s) => s,
                    Err(e) => return e,
                };
                app.set_grid_size(cx, size);
                self.record_audit(name, args, "applied");
                return ToolCallResult::success_json(app.designer_json(cx), "Grid size set.");
            }
            "designer.set_language" => {
                let code = match crawrad_mcp::designer_tool::set_language_arg(args) {
                    Ok(c) => c,
                    Err(e) => return e,
                };
                if let Err(valids) = app.set_design_language(Some(code)) {
                    return crawrad_mcp::designer_tool::unknown_language_error(code, valids);
                }
                app.reabrir_no_canvas(cx);
                app.check_form(cx);
                app.check_sizes(cx);
                app.sincronizar(cx);
                cx.redraw_all();
                self.record_audit(name, args, "applied");
                return ToolCallResult::success_json(app.designer_json(cx), "Language set.");
            }
            "acp.list_agents" | "acp.install_agent" | "acp.connect_agent" => {
                let call = crawrad_agent::acp_registry::mcp_call(name, args, &mut app.acp_config);
                if let Some((agent, rx)) = call.install {
                    app.watch_agent_install(&agent, rx);
                }
                if name == "acp.connect_agent" {
                    app.warm_live_agent(cx);
                }
                self.record_audit(name, args, call.audit);
                return call.result;
            }
            "chat.send" | "chat.stop" | "chat.new_session" | "chat.set_context"
            | "chat.context_menu" | "chat.permission" | "chat.queue" => {
                self.record_audit(name, args, "applied");
                app.chat_mcp(cx, name, args);
                return ToolCallResult::success_json(app.chat_messages_json(cx), "Chat updated.");
            }
            "chat.history" => {
                self.record_audit(name, args, "applied");
                app.chat_mcp(cx, name, args);
                let found = chat_history::mcp_user_json(app.chat.conversation.as_deref(), args);
                return ToolCallResult::success_json(found, "Chat history read.");
            }
            "chat.get_config_options" | "chat.set_config_option" | "chat.copy_message" => {
                self.record_audit(name, args, "applied");
                let agent = app.get_active_agent_name();
                let chat = (app.chat.session.messages.as_slice(), agent.as_str());
                let done = crawrad_agent::chat_config::mcp_call(
                    name,
                    args,
                    &mut app.chat.live,
                    &mut app.acp_config,
                    chat,
                );
                if let Some(text) = done.as_ref().ok().and_then(|v| v["copied"].as_str()) {
                    crawrad_os::clipboard::write_text(text);
                }
                app.sync_model_button(cx);
                return match done {
                    Ok(found) => ToolCallResult::success_json(found, "Chat config options."),
                    Err(err) => ToolCallResult::error_text("config_error", err),
                };
            }
            _ => {}
        }
        // 6. Project tools
        if name == "project.info" {
            return self.tool_project_info(&app.host);
        }
        if name == "project.get_tree" {
            let a = crawrad_mcp::project_tool::get_tree(app.host.workspace_root(), args);
            return self.audited(name, args, a);
        }
        if name == "project.create_file" {
            return self.tool_project_create_file(app, cx, args);
        }
        if name == "project.create_form" {
            return self.tool_project_create_form(app, cx, args);
        }
        if name == "project.delete_file" {
            return self.tool_project_delete_file(app, cx, args);
        }
        if name == "project.get_dependencies" {
            let a = crawrad_mcp::project_tool::get_dependencies(app.host.workspace_root());
            return self.audited(name, &json!({}), a);
        }
        if name == "project.install_extension" || name == "project.uninstall_extension" {
            let done = app.extension_tool(cx, name, args);
            let a = crawrad_mcp::project_tool::extension_tool_result(done);
            return self.audited(name, args, a);
        }
        if name == "project.add_dependency" {
            let a = crawrad_mcp::project_tool::add_dependency_in_project(
                app.host.workspace_root(),
                args,
            );
            return self.audited(name, args, a);
        }
        if name == "project.create" {
            return self.tool_project_create(app, cx, args);
        }
        if name == "project.open" {
            return self.tool_project_open(app, cx, args);
        }
        if name == "project.build" {
            let a = crawrad_mcp::run_tool::in_project(app.host.workspace_root(), |d| {
                crawrad_mcp::cargo_tool::run(d, args)
            });
            return self.audited(name, args, a);
        }
        // 7. Build & Run tools
        if name == "build.check" {
            let a = crawrad_mcp::run_tool::check_in_project(app.host.workspace_root());
            return self.audited(name, &json!({}), a);
        }
        if name == "build.build" {
            let a = crawrad_mcp::run_tool::build_in_project(app.host.workspace_root(), args);
            return self.audited(name, args, a);
        }
        if name == "build.get_diagnostics" {
            let a = crawrad_mcp::run_tool::diagnostics_in_project(app.host.workspace_root());
            return self.audited(name, &json!({}), a);
        }
        // The diff review, reachable from outside: a write tool answers
        // "pending_approval" and the edit waits in the editor. Without these the
        // caller is told to wait for a human and has no way to finish.
        if name == "review.status" {
            return self.tool_review_status(&app.diff_reviews);
        }
        if name == "review.accept" || name == "review.reject" {
            return self.tool_review_decide(app, cx, args, name == "review.accept");
        }
        if name == "run.start" {
            // The form IS the program: generate first, as the IDE's Run does.
            if app.host.has_doc() {
                app.gerar_projeto(cx);
                let said = app.status.clone().unwrap_or_default();
                if crawrad_project::generator::refused(&said) {
                    return ToolCallResult::error_text("generate-refused", said);
                }
            }
            return self.tool_run_start(&app.host, args);
        }
        if name == "run.stop" {
            let a = self.running_process.stop();
            return self.audited(name, &json!({}), a);
        }
        if name == "debug.state" {
            let state = app.host.debug_state_json();
            return ToolCallResult::success_json(state.clone(), state.to_string());
        }
        if name == "debug.set_breakpoints" {
            let root = app.raiz();
            return match app.host.set_breakpoints_json(args, root.as_deref()) {
                Ok(set) => ToolCallResult::success_json(set, "breakpoints set"),
                Err(why) => ToolCallResult::error_text("invalid-params", why),
            };
        }
        // 8. Action from catalogue or dynamic capability from CapabilityRegistry
        let action_opt = crawrad_actions::catalogue()
            .into_iter()
            .find(|a| a.id == name);
        if let Some(action) = action_opt {
            let is_design = app.center() == Center::Design;
            if let Err((refusal, outcome)) = crawrad_mcp::action_tool::check_action_preconditions(
                &action,
                args,
                app.modal_open(),
                is_design,
            ) {
                self.record_audit(name, args, outcome);
                return refusal;
            }

            let crawrad_args = convert_json_args_to_crawrad(args);
            let registry = &crawrad_project::installed::ide_catalog().catalog;
            let ctx = crawrad_actions::Context {
                doc: app.doc(),
                registry,
                clipboard: app.area_transferencia(),
                args: crawrad_args,
                project: app.projeto_contexto(),
                author: crawrad_actions::Author::Agent(format!("mcp-{}", self.session_id)),
                editor_dirty: app.is_editor_dirty(),
                text: app.editor_text(),
            };

            if let Err((refusal, outcome)) =
                crawrad_mcp::action_tool::check_action_enabled(&action, &ctx)
            {
                self.record_audit(name, args, outcome);
                return refusal;
            }

            let snap_antes = crawrad_mcp::action_tool::StateSnapshot::capture(
                app.doc(),
                app.center().as_str(),
                app.is_editor_dirty(),
            );

            let efeito = (action.execute)(&ctx);

            match app.apply_effect(cx, efeito) {
                Ok(()) => {
                    let snap_depois = crawrad_mcp::action_tool::StateSnapshot::capture(
                        app.doc(),
                        app.center().as_str(),
                        app.is_editor_dirty(),
                    );
                    self.record_audit(name, args, "applied");
                    crawrad_mcp::action_tool::format_action_result(name, &snap_antes, &snap_depois)
                }
                Err(e) => {
                    self.record_audit(name, args, "error");
                    ToolCallResult::error_text("effect-error", e)
                }
            }
        } else if let Some(cap_id) = app.capabilities().resolve_id(name) {
            let crawrad_args = convert_json_args_to_crawrad(args);
            let active_view = match app.center() {
                Center::Design => Some(ViewAffinity::Design),
                Center::Code => Some(ViewAffinity::Code),
                _ => None,
            };
            let author = crawrad_actions::Author::Agent(format!("mcp-{}", self.session_id));

            match app.dispatch_capability(&cap_id, crawrad_args, author, active_view) {
                Ok(efeito) => {
                    let mut ext_result = app
                        .execute_extension_command_with_args(&cap_id, Some(args.clone()))
                        .ok()
                        .flatten();
                    if ext_result.is_none() && name != cap_id {
                        ext_result = app
                            .execute_extension_command_with_args(name, Some(args.clone()))
                            .ok()
                            .flatten();
                    }

                    let msgs: Vec<Value> = app
                        .take_extension_messages()
                        .iter()
                        .map(|(lvl, msg)| json!({ "level": format!("{lvl:?}"), "message": msg }))
                        .collect();
                    let status_msg = efeito.status.clone();
                    let _ = app.apply_effect(cx, efeito);
                    self.record_audit(name, args, "applied");
                    crawrad_mcp::action_tool::format_capability_result(
                        &cap_id, status_msg, ext_result, &msgs,
                    )
                }
                Err(cap_err) => {
                    self.record_audit(name, args, "rejected");
                    crawrad_mcp::action_tool::format_capability_error(cap_err)
                }
            }
        } else {
            self.record_audit(name, args, "error");
            ToolCallResult::error_text("unknown-tool", format!("Unknown tool: '{name}'"))
        }
    }
    /// What is waiting in the editor, so the caller can decide knowing what it decides.
    /// Takes the manager, not the shell: the state already lives in `crawrad-agent`.
    fn tool_review_status(&mut self, reviews: &crate::ide_app::AiDiffReviews) -> ToolCallResult {
        let Some(review) = reviews.any_pending() else {
            return review_tool::none_pending();
        };
        review_tool::pending(
            &review.file_path.display().to_string(),
            &review.author_tool,
            review.total_additions(),
            review.total_deletions(),
            review.hunks.len(),
        )
    }

    /// Accepts or rejects the whole review, the same as the bar's two buttons.
    fn tool_review_decide(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        args: &Value,
        accept: bool,
    ) -> ToolCallResult {
        let tool = review_tool::tool_name(accept);
        let path = review_tool::path_asked(args).or_else(|| app.any_reviewed_path());
        let Some(path) = path else {
            self.record_audit(tool, args, "rejected");
            return review_tool::no_review(None);
        };
        if !app.has_pending_diff_review(&path) {
            self.record_audit(tool, args, "rejected");
            return review_tool::no_review(Some(&path.display().to_string()));
        }
        let outcome = match accept {
            true => app.accept_diff_review(cx, &path),
            false => app.reject_diff_review(cx, &path),
        };
        match outcome {
            Ok(_) => {
                self.record_audit(tool, args, "applied");
                review_tool::decided(&path.display().to_string(), accept)
            }
            Err(e) => {
                self.record_audit(tool, args, "error");
                review_tool::failed(e)
            }
        }
    }

    #[note("ide-app-mcp-host-mcpserver-wrong-view")]
    fn wrong_view(&mut self, tool: &str, args: &Value, needs: ViewAffinity) -> ToolCallResult {
        self.record_audit(tool, args, "rejected");
        let (code, said) = crawrad_mcp::edit::wrong_view(needs);
        ToolCallResult::error_text(code, said)
    }

    /// Takes what it uses — window and manager — not the whole shell.
    fn tool_code_read(
        &mut self,
        host: &HostModel,
        reviews: &crate::ide_app::AiDiffReviews,
        args: &Value,
    ) -> ToolCallResult {
        // View guard: code actions require Code view (or FormSource)
        if host.center() == ViewCenter::Design {
            return self.wrong_view("code.read", args, ViewAffinity::Code);
        }
        let path_str = match crawrad_mcp::edit::parse_code_read_path(args) {
            Ok(p) => p,
            Err(refused) => return self.audited("code.read", args, refused),
        };
        let path_buf = PathBuf::from(path_str);
        let predicted = reviews
            .get_pending_review(&path_buf)
            .map(|r| r.predicted_hash.clone());
        let at = (path_str, path_buf.as_path());
        self.paged_read("code.read", host, args, at, predicted)
    }

    fn tool_code_edit(&mut self, app: &mut IdeApp, args: &Value) -> ToolCallResult {
        // View guard
        if app.center() == Center::Design {
            return self.wrong_view("code.edit", args, ViewAffinity::Code);
        }
        let (path_str, base_hash, content) = match crawrad_mcp::edit::parse_code_edit_args(args) {
            Ok(found) => found,
            Err(refused) => return self.audited("code.edit", args, refused),
        };
        let pending = app.has_pending_diff_review(std::path::Path::new(path_str));
        if let Some(busy) = self.review_busy(pending, "code.edit", args, path_str) {
            return busy;
        }
        let ckey = canonical_key(path_str);
        // Section C: Verify read before edit
        if let Err((refusal, outcome)) = crawrad_mcp::edit::verify_read_before_edit(
            &self.session_read_hashes,
            &ckey,
            path_str,
            base_hash,
        ) {
            self.record_audit("code.edit", args, outcome);
            return refusal;
        }
        let current = app.host.read_text(std::path::Path::new(path_str), &ckey);
        let edit = (current.unwrap_or_default(), content.to_string());
        let gate = !self.disable_diff_review_gate;
        let landed = app.land_edit("code.edit", PathBuf::from(path_str), edit, gate);
        self.answer_edit("code.edit", args, (path_str, content), landed, json!({}))
    }

    fn tool_form_read(&mut self, host: &HostModel) -> ToolCallResult {
        let center = host.center().as_str();
        match crawrad_mcp::form_json::form_read_answer(host.doc(), center, host.is_dirty()) {
            Ok(res) => {
                self.record_audit("form.read", &json!({}), "applied");
                res
            }
            Err(refused) => self.audited("form.read", &json!({}), refused),
        }
    }

    fn tool_project_info(&mut self, host: &HostModel) -> ToolCallResult {
        let root = host.workspace_root().map(|p| p.display().to_string());
        let active = host.active_file().map(|p| p.display().to_string());
        let open: Vec<String> = host
            .open_file_paths()
            .into_iter()
            .map(|p| p.display().to_string())
            .collect();
        self.record_audit("project.info", &json!({}), "applied");
        crawrad_mcp::project_tool::workspace_info(
            root,
            active,
            host.center().as_str(),
            host.is_dirty(),
            open,
        )
    }

    fn tool_project_create(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        args: &Value,
    ) -> ToolCallResult {
        let (name, folder, template, git_init, framework) =
            match crawrad_mcp::project_tool::create_args(args) {
                Ok(found) => found,
                Err(refused) => return self.audited("project.create", args, refused),
            };
        if app.has_anything_to_save() {
            return self.audited(
                "project.create",
                args,
                crawrad_mcp::project_tool::unsaved_changes_refusal(),
            );
        }
        let framework = match crawrad_project::frameworks::resolve_framework(framework) {
            Ok(f) => f,
            Err(e) => {
                let refused = (ToolCallResult::error_text("invalid-params", e), "error");
                return self.audited("project.create", args, refused);
            }
        };
        if app
            .host
            .doc()
            .is_some_and(|d| d.revision() != app.host.saved_revision())
        {
            return self.audited(
                "project.create",
                args,
                crawrad_mcp::project_tool::unsaved_work_refusal(),
            );
        }
        let made = crate::ide_app::dialogs::new_project::create_project_template(
            &PathBuf::from(folder),
            name,
            template,
            git_init,
            framework,
        );
        if let Ok(dir) = &made {
            sync_workspace_mcp_config(dir, &self.instance_id);
            app.open_created_project(cx, dir);
        }
        let a = crawrad_mcp::project_tool::created(&made, name, template);
        self.audited("project.create", args, a)
    }

    fn tool_project_open(&mut self, app: &mut IdeApp, cx: &mut Cx, args: &Value) -> ToolCallResult {
        let target_path = match crawrad_mcp::project_tool::check_open_path(args) {
            Ok(path) => path,
            Err(refused) => return self.audited("project.open", args, refused),
        };

        if target_path.is_dir() && app.has_anything_to_save() {
            return self.audited(
                "project.open",
                args,
                crawrad_mcp::project_tool::unsaved_changes_refusal(),
            );
        }
        if target_path.is_dir() {
            sync_workspace_mcp_config(&target_path, &self.instance_id);
            app.open_created_project(cx, &target_path);
        } else if target_path.extension().is_some_and(|ext| ext == "crawform") {
            let _ = app.abrir_formulario(cx, &target_path);
        } else {
            app.abrir_no_editor(cx, target_path.clone());
        }

        self.record_audit("project.open", args, "applied");
        ToolCallResult::success_text(format!("Opened '{}' in the IDE.", target_path.display()))
    }

    fn tool_run_start(&mut self, host: &HostModel, args: &Value) -> ToolCallResult {
        let a = self.running_process.start(host.workspace_root(), args);
        self.audited("run.start", args, a)
    }
}

#[path = "mcp_host/build_run.rs"]
mod build_run;
#[path = "mcp_host/designer.rs"]
mod designer;
#[path = "mcp_host/editor.rs"]
mod editor;
#[path = "mcp_host/project.rs"]
mod project;

/// Moved to `crawrad-mcp` (pure: no window). Kept here for its callers.
pub use crawrad_mcp::workspace_config::sync as sync_workspace_mcp_config;

/// Moved out (pure): the JSON arguments to `crawrad-actions`. Kept here under
/// its old name for the callers.
pub(super) use crawrad_actions::args_from_json as convert_json_args_to_crawrad;
use crawrad_note::note;
