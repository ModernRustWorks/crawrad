//! The in-IDE Model Context Protocol (MCP) server.
//!
//! Executes inside the IDE process, translating incoming JSON-RPC 2.0 requests
//! (`tools/list`, `tools/call`) to IDE actions on the UI thread with
//! `Author::Agent` transaction signing, context guards, and audit logging.

use crate::ide_app::IdeApp;
use crate::ide_app::source::Center;
use crawrad_actions::ViewAffinity;
use crawrad_actions::capability::CapabilityError;
use crawrad_agent::chat_history;
/// The hash the AI diff review keys on: one function, the agent's.
pub use crawrad_agent::diff_review::hex_sha256;
use crawrad_host::{HostModel, ViewCenter};
use crawrad_mcp::audit::AuditLogger;
pub use crawrad_mcp::listener::McpRequest;
use crawrad_mcp::project_tool;
use crawrad_mcp::project_tree::canonical_key;
use crawrad_mcp::protocol::{JsonRpcRequest, JsonRpcResponse, ToolCallResult};
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
        let req: JsonRpcRequest = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                self.record_audit("invalid_json", &json!(trimmed), "error");
                let resp =
                    JsonRpcResponse::error(Value::Null, -32700, format!("Parse error: {e}"), None);
                return serde_json::to_string(&resp).unwrap_or_default();
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
        let Some(name) = params.get("name").and_then(Value::as_str) else {
            self.record_audit("unknown", params, "error");
            return ToolCallResult::error_text(
                "invalid-request",
                "Missing 'name' field in tools/call parameters",
            );
        };
        let default_args = json!({});
        let args = params.get("arguments").unwrap_or(&default_args);
        let (name, args) = crawrad_mcp::tools::unwrap_action_execute(name, args);
        let rewritten = crawrad_mcp::edit::absolute_path_arg(args, &[app.raiz(), app.membro()]);
        let args = rewritten.as_ref().unwrap_or(args);
        // Guard: refuses to save while a review is pending
        if !self.disable_diff_review_gate
            && crawrad_mcp::tools::is_save_tool(name)
            && app.has_any_pending_diff_review()
        {
            self.record_audit("action.execute", args, "rejected");
            return ToolCallResult::error_text(
                "pending-diff-review",
                "Cannot save while a diff review is pending. Accept or reject the diff first.",
            );
        }
        // The tools that only read a part of the view: what each answers and
        // the line it says, audited and answered alike.
        let read = match name {
            "editor.get_completion" => Some((app.completion_json(cx), "Completion list read.")),
            "editor.get_choices" => Some((app.nav.choices_json(), "Definition choices read.")),
            "view.get_inspector" => Some((
                crawrad_actions::inspector_model::rows_json(&app.linhas_do_inspector()),
                "Object Inspector read.",
            )),
            "editor.get_rename" => Some((app.nav.rename.to_json(), "Rename dialog read.")),
            "editor.get_references" => Some((app.nav.references.to_json(), "References read.")),
            "editor.get_hover" => Some((app.hover_json(), "Hover card read.")),
            "view.get_dialog" => Some((app.dialog_json(cx), "Dialog read.")),
            "view.get_status_bar" => Some((app.status_bar_json(), "Status bar read.")),
            "view.get_history" => Some((app.history_tab_json(cx), "History view read.")),
            "view.get_designer" => Some((app.designer_json(cx), "Designer bar read.")),
            "view.get_welcome" => Some((app.welcome_json(cx), "Welcome page read.")),
            "view.get_explorer" => Some((app.explorer_drawn.clone(), "Explorer read.")),
            "view.get_notes" => Some((app.notes_json(), "Notes panel read.")),
            "view.get_tests" => Some((app.tests_json(), "Tests panel read.")),
            "form.check" => Some((app.form_check_json(), "Form checked.")),
            "view.sizes" => Some((app.sizes_json(), "Sizes preview read.")),
            "strings.list" => Some((app.strings_list_json(args), "Strings read.")),
            "view.get_rail" => Some((app.rail_json(), "Activity rail read.")),
            "view.get_toolbox" => Some((app.toolbox_json(cx), "Toolbox read.")),
            "view.set_toolbox_filter" => {
                Some((app.set_toolbox_filter(cx, &args["text"]), "Filtered."))
            }
            "view.get_chat" => Some((app.chat_messages_json(cx), "Chat messages read.")),
            "view.get_extension_views" => {
                Some((app.extension_views_json(cx), "Extension views read."))
            }
            "editor.get_line_marks" => Some((app.line_marks_json(), "Line marks read.")),
            "editor.get_semantic_tokens" => Some((app.semantic_json(), "Semantic tokens read.")),
            "editor.get_code_lenses" => Some((app.code_lenses_json(), "Code lenses read.")),
            "editor.get_code_actions" => Some((app.code_actions_json(cx), "Code actions read.")),
            "editor.get_signature_help" => Some((app.signature_json(), "Parameter card read.")),
            "editor.get_decorations" => Some((app.squiggles_json(), "Decorations read.")),
            _ => None,
        };
        if let Some((answer, said)) = read {
            self.record_audit(name, args, "applied");
            return ToolCallResult::success_json(answer, said);
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
            "code.read" | "editor.read_file" => return self.tool_code_read(app, args),
            "code.edit" | "editor.edit_file" => return self.tool_code_edit(app, args),
            "editor.get_buffer_content" => return self.tool_editor_get_buffer_content(app, args),
            "editor.apply_edit" => return self.tool_editor_apply_edit(app, args),
            "form.read" | "designer.get_active_form" => return self.tool_form_read(&app.host),
            "designer.list_components" => return self.tool_designer_list_components(&app.host),
            "designer.get_selection" => return self.tool_designer_get_selection(&app.host),
            "designer.get_component_properties" => {
                return self.tool_designer_get_component_properties(&app.host, args);
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
                let mut state = crawrad_actions::menu::open_menu_json(
                    open,
                    app.menu_highlight(),
                    &rows,
                    sub,
                    app.menu_rects_json(cx),
                );
                state["hint"] = serde_json::json!(app.menu_hint.as_ref().map(|(_, why)| why));
                return ToolCallResult::success_json(state, "Open menu read.");
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
                let Some(size) = args.get("size").and_then(|v| v.as_f64()) else {
                    return ToolCallResult::error_text(
                        "bad-args",
                        "designer.set_grid_size needs `size`",
                    );
                };
                app.set_grid_size(cx, size as f32);
                self.record_audit(name, args, "applied");
                return ToolCallResult::success_json(app.designer_json(cx), "Grid size set.");
            }
            "designer.set_language" => {
                let Some(code) = args.get("code").and_then(|v| v.as_str()) else {
                    return ToolCallResult::error_text(
                        "bad-args",
                        "designer.set_language needs `code`",
                    );
                };
                if let Err(valids) = app.set_design_language(Some(code)) {
                    let mut list = vec!["en".to_string()];
                    list.extend(valids);
                    let valid_list = list.join(", ");
                    return ToolCallResult::error_text(
                        "unknown-language",
                        format!("Unknown language '{code}'. Valid languages: {valid_list}"),
                    );
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
            return self.tool_project_get_tree(&app.host, args);
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
            return self.tool_project_get_dependencies(&app.host);
        }
        if name == "project.install_extension" || name == "project.uninstall_extension" {
            let done = app.extension_tool(cx, name, args);
            self.record_audit(name, args, if done.is_ok() { "applied" } else { "error" });
            return match done {
                Ok(said) => ToolCallResult::success_json(json!({ "status": said }), said),
                Err(said) => ToolCallResult::error_text("extension-refused", said),
            };
        }
        if name == "project.add_dependency" {
            return self.tool_project_add_dependency(&app.host, args);
        }
        if name == "project.create" {
            return self.tool_project_create(app, cx, args);
        }
        if name == "project.open" {
            return self.tool_project_open(app, cx, args);
        }
        if name == "project.build" {
            return self.tool_project_build(&app.host, args);
        }
        // 7. Build & Run tools
        if name == "build.check" {
            return self.tool_build_check(&app.host);
        }
        if name == "build.build" {
            return self.tool_build_build(&app.host, args);
        }
        if name == "build.get_diagnostics" {
            return self.tool_build_get_diagnostics(&app.host);
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
            return self.tool_run_stop();
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
            if let Some(why) = crawrad_mcp::dialogs::modal_refusal(app.modal_open(), action.label) {
                self.record_audit(name, args, "rejected");
                let text = format!("Action '{name}' was not run: {why}.");
                return ToolCallResult::error_text("action-disabled", text);
            }

            // Validate arguments against action.input_schema()
            if let Err(err_msg) = crawrad_mcp::tools::validate_arguments(&action, args) {
                self.record_audit(name, args, "error");
                return ToolCallResult::error_text("invalid-params", &err_msg);
            }

            // Context Guard: View Guard declared on action descriptor
            match action.view {
                needs @ ViewAffinity::Design if app.center() != Center::Design => {
                    return self.wrong_view(name, args, needs);
                }
                needs @ ViewAffinity::Code if app.center() == Center::Design => {
                    return self.wrong_view(name, args, needs);
                }
                _ => {}
            }

            // Context Guard: Action enabled predicate with realistic project context
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

            if !action.enabled.allows(&ctx) {
                self.record_audit(name, args, "rejected");
                let instruction = match Some(action.enabled.need).filter(|n| !n.met(&ctx)) {
                    Some(n) => format!("It needs {}. {}", n.reason(), n.remedy()),
                    None => "Check active selection and IDE state and resend.".to_string(),
                };
                return ToolCallResult::error_text(
                    "action-disabled",
                    format!(
                        "Action '{}' is currently disabled in the IDE context: {instruction}",
                        action.id
                    ),
                );
            }

            // Execute action producing Efeito
            let efeito = (action.execute)(&ctx);

            let rev_antes = app.doc().map(|d| d.revision());
            let sel_antes = app.doc().map(|d| d.selection().to_vec());
            let centro_antes = app.center();
            let sujo_antes = app.is_editor_dirty();

            // Apply effect via IDE app on UI thread
            match app.apply_effect(cx, efeito) {
                Ok(()) => {
                    let rev_depois = app.doc().map(|d| d.revision());
                    let sel_depois = app.doc().map(|d| d.selection().to_vec());
                    let centro_depois = app.center();
                    let sujo_depois = app.is_editor_dirty();

                    let mudou = rev_antes != rev_depois
                        || sel_antes != sel_depois
                        || centro_antes != centro_depois
                        || sujo_antes != sujo_depois;

                    self.record_audit(name, args, "applied");
                    if mudou {
                        ToolCallResult::success_json(
                            json!({
                                "action": name,
                                "mutated": true,
                                "revision_before": rev_antes,
                                "revision_after": rev_depois,
                                "selection": sel_depois,
                                "center_view": centro_depois.as_str(),
                                "is_dirty": sujo_depois,
                            }),
                            format!(
                                "Action '{}' applied successfully (rev: {}).",
                                name,
                                rev_depois.unwrap_or(0)
                            ),
                        )
                    } else {
                        ToolCallResult::success_json(
                            json!({
                                "action": name,
                                "mutated": false,
                                "revision": rev_depois,
                                "selection": sel_depois,
                                "center_view": centro_depois.as_str(),
                                "is_dirty": sujo_depois,
                            }),
                            format!("Action '{}' executed (no document or state change).", name),
                        )
                    }
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

                    let ext_messages = app.take_extension_messages();
                    let status_msg = efeito.status.clone();
                    let _ = app.apply_effect(cx, efeito);
                    self.record_audit(name, args, "applied");

                    let mut data_map = serde_json::Map::new();
                    data_map.insert("capability".to_string(), json!(cap_id));
                    data_map.insert(
                        "status".to_string(),
                        json!(status_msg.unwrap_or_else(|| "executed".to_string())),
                    );
                    if let Some(res) = ext_result {
                        data_map.insert("result".to_string(), res);
                    }
                    if !ext_messages.is_empty() {
                        let msgs: Vec<Value> = ext_messages
                            .iter()
                            .map(|(lvl, msg)| {
                                json!({
                                    "level": format!("{:?}", lvl),
                                    "message": msg
                                })
                            })
                            .collect();
                        data_map.insert("messages".to_string(), json!(msgs));
                    }

                    ToolCallResult::success_json(
                        Value::Object(data_map),
                        format!("Capability '{}' executed successfully.", cap_id),
                    )
                }
                Err(cap_err) => {
                    self.record_audit(name, args, "rejected");
                    match cap_err {
                        CapabilityError::ViewMismatch { required, .. } => {
                            let (code, msg) = crawrad_mcp::edit::wrong_view(required);
                            ToolCallResult::error_text(code, msg)
                        }
                        CapabilityError::MissingParameter {
                            param,
                            capability_id,
                        } => ToolCallResult::error_text(
                            "invalid-params",
                            format!(
                                "Missing required parameter '{param}' for capability '{capability_id}'"
                            ),
                        ),
                        CapabilityError::Disabled(id) => ToolCallResult::error_text(
                            "action-disabled",
                            format!("Capability '{id}' is currently disabled in the IDE context."),
                        ),
                        other => ToolCallResult::error_text("execution-failed", other.to_string()),
                    }
                }
            }
        } else {
            self.record_audit(name, args, "error");
            ToolCallResult::error_text("unknown-tool", format!("Unknown tool: '{name}'"))
        }
    }

    #[note("ide-app-mcp-host-mcpserver-wrong-view")]
    fn wrong_view(&mut self, tool: &str, args: &Value, needs: ViewAffinity) -> ToolCallResult {
        self.record_audit(tool, args, "rejected");
        let (code, said) = crawrad_mcp::edit::wrong_view(needs);
        ToolCallResult::error_text(code, said)
    }

    fn tool_code_read(&mut self, app: &IdeApp, args: &Value) -> ToolCallResult {
        // View guard: code actions require Code view (or FormSource)
        if app.host.center() == ViewCenter::Design {
            return self.wrong_view("code.read", args, ViewAffinity::Code);
        }

        let path_str = match crawrad_mcp::cargo_tool::required(args, "code.read", &["path"]) {
            Ok(found) => found[0],
            Err(e) => {
                self.record_audit("code.read", args, "error");
                return ToolCallResult::error_text("invalid-params", e);
            }
        };

        let path_buf = PathBuf::from(path_str);
        let predicted = app
            .get_pending_diff_review(&path_buf)
            .map(|r| r.predicted_hash.clone());
        let at = (path_str, path_buf.as_path());
        self.paged_read("code.read", &app.host, args, at, predicted)
    }

    fn tool_code_edit(&mut self, app: &mut IdeApp, args: &Value) -> ToolCallResult {
        // View guard
        if app.center() == Center::Design {
            return self.wrong_view("code.edit", args, ViewAffinity::Code);
        }

        let keys = ["path", "base_hash", "content"];
        let (path_str, base_hash, content) =
            match crawrad_mcp::cargo_tool::required(args, "code.edit", &keys) {
                Ok(found) => (found[0], found[1], found[2]),
                Err(e) => {
                    self.record_audit("code.edit", args, "error");
                    return ToolCallResult::error_text("invalid-params", e);
                }
            };

        let pending = app.has_pending_diff_review(std::path::Path::new(path_str));
        if let Some(busy) = self.review_busy(pending, "code.edit", args, path_str) {
            return busy;
        }
        let ckey = canonical_key(path_str);
        // Section C: Verify read before edit
        let Some(expected_hash) = self.session_read_hashes.get(&ckey) else {
            self.record_audit("code.edit", args, "rejected");
            return ToolCallResult::error_text(
                "no-prior-read",
                format!(
                    "File '{}' has not been read in this session. Call 'code.read' first.",
                    path_str
                ),
            );
        };
        if expected_hash != base_hash {
            self.record_audit("code.edit", args, "rejected");
            return ToolCallResult::error_text(
                "hash-mismatch",
                format!(
                    "base_hash mismatch for '{}'. Expected '{}', got '{}'. Call 'code.read' to refresh.",
                    path_str, expected_hash, base_hash
                ),
            );
        }
        let current = app.host.read_text(std::path::Path::new(path_str), &ckey);
        let edit = (current.unwrap_or_default(), content.to_string());
        let gate = !self.disable_diff_review_gate;
        let landed = app.land_edit("code.edit", PathBuf::from(path_str), edit, gate);
        self.answer_edit("code.edit", args, (path_str, content), landed, json!({}))
    }

    fn tool_form_read(&mut self, host: &HostModel) -> ToolCallResult {
        let Some(doc) = host.doc() else {
            return self.no_form("form.read", &json!({}));
        };
        let (mut form, count) = crawrad_mcp::form_json::form(doc);
        form["center_view"] = json!(host.center().as_str());
        form["is_dirty"] = json!(host.is_dirty());
        self.record_audit("form.read", &json!({}), "applied");
        let said = format!(
            "Active form inspected (rev: {}, {count} child nodes)",
            doc.revision()
        );
        ToolCallResult::success_json(form, said)
    }

    fn tool_project_info(&mut self, host: &HostModel) -> ToolCallResult {
        let root = host.workspace_root().map(|p| p.display().to_string());
        let active_file = host.active_file().map(|p| p.display().to_string());

        let open_buffers: Vec<String> = host
            .open_file_paths()
            .into_iter()
            .map(|p| p.display().to_string())
            .collect();

        let result = json!({
            "workspace_root": root,
            "active_file": active_file,
            "center_view": host.center().as_str(),
            "is_dirty": host.is_dirty(),
            "open_buffers": open_buffers,
        });

        self.record_audit("project.info", &json!({}), "applied");
        ToolCallResult::success_json(result, "Project workspace info retrieved.")
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
            self.record_audit("project.create", args, "error");
            return ToolCallResult::error_text("unsaved-changes", project_tool::UNSAVED_SWITCH);
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
            .is_some_and(|d| d.revision() != app.rev_salva)
        {
            self.record_audit("project.create", args, "rejected");
            let said = "The open form has unsaved changes: run file.save_all or close it";
            return ToolCallResult::error_text("unsaved-work", said);
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
        let Some(path_str) = crawrad_mcp::cargo_tool::nonempty(args, "path") else {
            self.record_audit("project.open", args, "error");
            let said = "Missing or empty 'path' parameter";
            return ToolCallResult::error_text("invalid-params", said);
        };

        let target_path = PathBuf::from(path_str);
        if !target_path.exists() {
            self.record_audit("project.open", args, "error");
            return ToolCallResult::error_text(
                "not-found",
                format!("Path '{}' does not exist.", path_str),
            );
        }

        if target_path.is_dir() && app.has_anything_to_save() {
            self.record_audit("project.open", args, "error");
            return ToolCallResult::error_text("unsaved-changes", project_tool::UNSAVED_SWITCH);
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

    fn tool_project_build(&mut self, host: &HostModel, args: &Value) -> ToolCallResult {
        let root = host.workspace_root();
        let a = crawrad_mcp::run_tool::in_project(root, |d| crawrad_mcp::cargo_tool::run(d, args));
        self.audited("project.build", args, a)
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
