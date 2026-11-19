//! External Extension Process Host (ADR 0029).
//!
//! Executes extensions in decoupled subprocesses communicating over JSON-RPC on stdio.
//! Provides failure isolation (process crash doesn't affect IDE), on-demand activation,
//! heartbeat/liveness checks, and graceful shutdown.

use std::collections::{BTreeMap, HashMap};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crawrad_extension_api::error::ExtensionError;
use crawrad_extension_api::manifest::ExtensionManifest;
use crawrad_extension_api::marks::{LineMarkClicked, SetLineMarksParams};
use crawrad_extension_api::protocol::{
    ExecuteCommandParams, ExecuteCommandResult, IncomingMessage, InitializeParams, JsonRpcRequest,
    METHOD_EDITOR_LINE_MARK_CLICKED, METHOD_EDITOR_SET_LINE_MARKS, METHOD_EXECUTE_COMMAND,
    METHOD_INITIALIZE, METHOD_PING, METHOD_PONG, METHOD_SHOW_MESSAGE, METHOD_SHUTDOWN,
    METHOD_UI_CLOSE_CARD, METHOD_UI_EVENT, METHOD_UI_RENDER_CARD, METHOD_UI_UPDATE_ELEMENT,
    METHOD_VIEW_ITEM_CLICKED, METHOD_VIEW_SET_TREE, ShowMessageNotification, parse_message,
};
use crawrad_extension_api::traits::MessageLevel;
use crawrad_extension_api::ui::{
    CloseCardParams, RenderCardParams, UiEventKind, UiEventNotification, UpdateElementParams,
};
use crawrad_extension_api::views::{ItemClicked, SetTreeParams};
use serde_json::Value;

use crate::cards::ExtensionCardManager;
use crate::line_marks::LineMarkStore;
use crate::views::ViewStore;
use crawrad_note::note;

#[note("process-processstatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    /// Not yet launched (waiting for activation event).
    Unstarted,
    /// Subprocess is running and responsive.
    Running,
    /// Subprocess has exited unexpectedly with status code.
    Crashed(Option<i32>),
    /// Subprocess terminated gracefully or was stopped.
    Stopped,
}

type ResponseSender = std::sync::mpsc::Sender<Result<Value, ExtensionError>>;
type PendingResponseMap = Arc<Mutex<HashMap<u64, ResponseSender>>>;
type StdinWriter = Arc<Mutex<Box<dyn Write + Send>>>;

#[note("process-processextension")]
pub struct ProcessExtension {
    pub manifest: ExtensionManifest,
    pub binary_path: PathBuf,
    pub workspace_root: Option<PathBuf>,
    pub status: ProcessStatus,
    pub card_manager: Option<Arc<Mutex<ExtensionCardManager>>>,
    /// Where this extension's line marks go.
    pub line_marks: Option<Arc<Mutex<LineMarkStore>>>,
    /// Where this extension's view trees go.
    pub views: Option<Arc<Mutex<ViewStore>>>,
    child: Option<Child>,
    stdin_tx: Option<StdinWriter>,
    request_id_counter: Arc<AtomicU64>,
    pending_responses: PendingResponseMap,
    messages_buffer: Arc<Mutex<Vec<(MessageLevel, String)>>>,
    reader_thread: Option<std::thread::JoinHandle<()>>,
}

impl ProcessExtension {
    #[note("process-processextension-new")]
    pub fn new(
        manifest: ExtensionManifest,
        binary_path: PathBuf,
        workspace_root: Option<PathBuf>,
    ) -> Self {
        Self {
            manifest,
            binary_path,
            workspace_root,
            status: ProcessStatus::Unstarted,
            card_manager: None,
            line_marks: None,
            views: None,
            child: None,
            stdin_tx: None,
            request_id_counter: Arc::new(AtomicU64::new(1)),
            pending_responses: Arc::new(Mutex::new(HashMap::new())),
            messages_buffer: Arc::new(Mutex::new(Vec::new())),
            reader_thread: None,
        }
    }

    #[note("process-processextension-set-card-manager")]
    pub fn set_card_manager(&mut self, mgr: Arc<Mutex<ExtensionCardManager>>) {
        self.card_manager = Some(mgr);
    }

    #[note("process-processextension-is-alive")]
    pub fn is_alive(&mut self) -> bool {
        if matches!(
            self.status,
            ProcessStatus::Crashed(_) | ProcessStatus::Stopped
        ) {
            return false;
        }

        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(None) => true,
                Ok(Some(status)) => {
                    let code = status.code();
                    self.status = ProcessStatus::Crashed(code);
                    false
                }
                Err(_) => {
                    self.status = ProcessStatus::Crashed(None);
                    false
                }
            }
        } else {
            false
        }
    }

    #[note("process-processextension-start")]
    pub fn start(&mut self) -> Result<(), ExtensionError> {
        if self.is_alive() {
            return Ok(());
        }

        if !self.binary_path.exists() {
            return Err(ExtensionError::NotFound(format!(
                "Extension binary not found at: {}",
                self.binary_path.display()
            )));
        }

        let mut cmd = crawrad_os::process::no_console(&self.binary_path);
        if let Some(ref root) = self.workspace_root {
            cmd.current_dir(root);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            ExtensionError::ProcessError(format!(
                "Failed to spawn extension process '{}': {e}",
                self.manifest.id
            ))
        })?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| ExtensionError::IpcError("Failed to open child stdin pipe".into()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| ExtensionError::IpcError("Failed to open child stdout pipe".into()))?;

        let stdin_writer: Box<dyn Write + Send> = Box::new(stdin);
        let stdin_tx = Arc::new(Mutex::new(stdin_writer));

        let pending = self.pending_responses.clone();
        let messages = self.messages_buffer.clone();
        let card_mgr = self.card_manager.clone();
        let marks = self.line_marks.clone();
        let views = self.views.clone();
        let ext_id = self.manifest.id.clone();

        let reader_thread = std::thread::Builder::new()
            .name(format!("ext-reader-{}", self.manifest.id))
            .spawn(move || {
                let reader = BufReader::new(stdout);
                for line_res in reader.lines() {
                    match line_res {
                        Ok(line) => {
                            if line.trim().is_empty() {
                                continue;
                            }
                            match parse_message(&line) {
                                Ok(IncomingMessage::Response(resp)) => {
                                    if let Some(req_id) = resp.id
                                        && let Ok(mut map) = pending.lock()
                                        && let Some(sender) = map.remove(&req_id)
                                    {
                                        if let Some(res_val) = resp.result {
                                            let _ = sender.send(Ok(res_val));
                                        } else if let Some(err) = resp.error {
                                            let _ =
                                                sender.send(Err(ExtensionError::ExecutionFailed(
                                                    format!("[{}] {}", err.code, err.message),
                                                )));
                                        } else {
                                            let _ = sender.send(Ok(Value::Null));
                                        }
                                    }
                                }
                                Ok(IncomingMessage::Request(req)) => {
                                    if req.method == METHOD_SHOW_MESSAGE {
                                        if let Some(params) = req.params
                                            && let Ok(notif) =
                                                serde_json::from_value::<ShowMessageNotification>(
                                                    params,
                                                )
                                        {
                                            let level = match notif.level.to_lowercase().as_str() {
                                                "warn" | "warning" => MessageLevel::Warning,
                                                "err" | "error" => MessageLevel::Error,
                                                _ => MessageLevel::Info,
                                            };
                                            if let Ok(mut msg_guard) = messages.lock() {
                                                msg_guard.push((level, notif.message));
                                            }
                                            // the window reads it on its next wake
                                            if let Some(Ok(m)) = marks.as_ref().map(|m| m.lock()) {
                                                m.wake();
                                            }
                                        }
                                    } else if req.method == METHOD_EDITOR_SET_LINE_MARKS {
                                        let params = req.params.and_then(|p| {
                                            serde_json::from_value::<SetLineMarksParams>(p).ok()
                                        });
                                        if let (Some(store), Some(params)) = (&marks, params)
                                            && let Ok(mut g) = store.lock()
                                        {
                                            g.set(&ext_id, params);
                                        }
                                    } else if req.method == METHOD_VIEW_SET_TREE {
                                        let params = req.params.and_then(|p| {
                                            serde_json::from_value::<SetTreeParams>(p).ok()
                                        });
                                        if let (Some(store), Some(params)) = (&views, params)
                                            && let Ok(mut g) = store.lock()
                                        {
                                            g.set(&ext_id, params);
                                        }
                                    } else if req.method == METHOD_UI_RENDER_CARD {
                                        if let (Some(mgr), Some(params)) = (&card_mgr, req.params)
                                            && let Ok(render_params) =
                                                serde_json::from_value::<RenderCardParams>(params)
                                            && let Ok(mut g) = mgr.lock()
                                        {
                                            let _ = g.render_card(
                                                &render_params.card_id,
                                                render_params.root,
                                            );
                                        }
                                    } else if req.method == METHOD_UI_UPDATE_ELEMENT {
                                        if let (Some(mgr), Some(params)) = (&card_mgr, req.params)
                                            && let Ok(update_params) =
                                                serde_json::from_value::<UpdateElementParams>(
                                                    params,
                                                )
                                            && let Ok(mut g) = mgr.lock()
                                        {
                                            let _ = g.update_element(
                                                &update_params.card_id,
                                                &update_params.element_id,
                                                update_params.patch,
                                            );
                                        }
                                    } else if req.method == METHOD_UI_CLOSE_CARD
                                        && let (Some(mgr), Some(params)) = (&card_mgr, req.params)
                                        && let Ok(close_params) =
                                            serde_json::from_value::<CloseCardParams>(params)
                                        && let Ok(mut g) = mgr.lock()
                                    {
                                        let _ = g.close_card(&close_params.card_id);
                                    }
                                }
                                Err(err) => {
                                    eprintln!(
                                        "[ext:{ext_id}] protocol warning parsing line: {err}"
                                    );
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }

                // Subprocess stdout closed (process exited or EOF). Drain and notify all pending requests.
                if let Ok(mut map) = pending.lock() {
                    for (_req_id, sender) in map.drain() {
                        let _ = sender.send(Err(ExtensionError::ProcessCrashed {
                            extension_id: ext_id.clone(),
                            exit_code: None,
                        }));
                    }
                }
            })
            .map_err(|e| ExtensionError::Internal(format!("Failed to spawn reader thread: {e}")))?;

        self.child = Some(child);
        self.stdin_tx = Some(stdin_tx);
        self.reader_thread = Some(reader_thread);
        self.status = ProcessStatus::Running;

        // Perform initialize handshake
        let ws_str = self
            .workspace_root
            .as_ref()
            .map(|p| p.to_string_lossy().to_string());
        let init_params = InitializeParams {
            extension_id: self.manifest.id.clone(),
            workspace_path: ws_str,
        };

        let init_val = serde_json::to_value(init_params).map_err(|e| {
            ExtensionError::ProtocolError(format!("Failed to serialize initialize params: {e}"))
        })?;

        let _ = self.send_request(METHOD_INITIALIZE, Some(init_val), Duration::from_secs(5))?;

        Ok(())
    }

    #[note("process-processextension-send-request")]
    pub fn send_request(
        &mut self,
        method: &str,
        params: Option<Value>,
        timeout: Duration,
    ) -> Result<Value, ExtensionError> {
        if !self.is_alive() {
            return Err(ExtensionError::ProcessCrashed {
                extension_id: self.manifest.id.clone(),
                exit_code: match self.status {
                    ProcessStatus::Crashed(code) => code,
                    _ => None,
                },
            });
        }

        let id = self.request_id_counter.fetch_add(1, Ordering::SeqCst);
        let req = JsonRpcRequest::new_request(id, method, params);
        let json_line = serde_json::to_string(&req).map_err(|e| {
            ExtensionError::ProtocolError(format!("Failed to serialize request: {e}"))
        })? + "\n";

        let (reply_tx, reply_rx) = std::sync::mpsc::channel();
        {
            let mut pending = self
                .pending_responses
                .lock()
                .map_err(|_| ExtensionError::Internal("Pending responses lock poisoned".into()))?;
            pending.insert(id, reply_tx);
        }

        if let Some(ref stdin_arc) = self.stdin_tx {
            let mut writer = stdin_arc
                .lock()
                .map_err(|_| ExtensionError::Internal("Stdin lock poisoned".into()))?;
            writer.write_all(json_line.as_bytes()).map_err(|e| {
                ExtensionError::IpcError(format!("Failed to write to extension stdin: {e}"))
            })?;
            writer.flush().map_err(|e| {
                ExtensionError::IpcError(format!("Failed to flush extension stdin: {e}"))
            })?;
        } else {
            return Err(ExtensionError::IpcError("Stdin not available".into()));
        }

        let result = match reply_rx.recv_timeout(timeout) {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(ExtensionError::ProcessCrashed { .. })) => {
                let mut code = None;
                if let Some(ref mut child) = self.child {
                    for _ in 0..50 {
                        if let Ok(Some(st)) = child.try_wait() {
                            code = st.code();
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
                self.status = ProcessStatus::Crashed(code);
                Err(ExtensionError::ProcessCrashed {
                    extension_id: self.manifest.id.clone(),
                    exit_code: code,
                })
            }
            Ok(Err(err)) => Err(err),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if !self.is_alive() {
                    Err(ExtensionError::ProcessCrashed {
                        extension_id: self.manifest.id.clone(),
                        exit_code: match self.status {
                            ProcessStatus::Crashed(c) => c,
                            _ => None,
                        },
                    })
                } else {
                    Err(ExtensionError::ExecutionFailed(format!(
                        "Request '{method}' timed out after {:?}",
                        timeout
                    )))
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                let mut code = None;
                if let Some(ref mut child) = self.child {
                    for _ in 0..50 {
                        if let Ok(Some(st)) = child.try_wait() {
                            code = st.code();
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
                self.status = ProcessStatus::Crashed(code);
                Err(ExtensionError::ProcessCrashed {
                    extension_id: self.manifest.id.clone(),
                    exit_code: code,
                })
            }
        };

        if let Ok(mut pending) = self.pending_responses.lock() {
            pending.remove(&id);
        }

        result
    }

    #[note("process-processextension-execute-command")]
    pub fn execute_command(
        &mut self,
        command: &str,
        arguments: Option<Value>,
    ) -> Result<Option<Value>, ExtensionError> {
        if !self.is_alive() {
            self.start()?;
        }

        let params = ExecuteCommandParams {
            command: command.to_string(),
            arguments,
        };
        let params_val = serde_json::to_value(params).map_err(|e| {
            ExtensionError::ProtocolError(format!("Failed to serialize execute params: {e}"))
        })?;

        let res = self.send_request(
            METHOD_EXECUTE_COMMAND,
            Some(params_val),
            Duration::from_secs(10),
        )?;

        if let Ok(parsed) = serde_json::from_value::<ExecuteCommandResult>(res.clone()) {
            if parsed.success {
                Ok(parsed.result)
            } else {
                Err(ExtensionError::ExecutionFailed(
                    parsed
                        .error
                        .unwrap_or_else(|| "Command execution returned failure".into()),
                ))
            }
        } else {
            Ok(Some(res))
        }
    }

    #[note("process-processextension-send-ui-event")]
    pub fn send_ui_event(
        &mut self,
        card_id: &str,
        element_id: &str,
        event: UiEventKind,
    ) -> Result<(), ExtensionError> {
        let notif = UiEventNotification {
            card_id: card_id.to_string(),
            element_id: element_id.to_string(),
            event,
        };
        let val = serde_json::to_value(&notif).map_err(|e| {
            ExtensionError::ProtocolError(format!("Failed to serialize UI event: {e}"))
        })?;
        let req = JsonRpcRequest::new_notification(METHOD_UI_EVENT, Some(val));
        let json_line = serde_json::to_string(&req).map_err(|e| {
            ExtensionError::ProtocolError(format!("Failed to serialize request: {e}"))
        })? + "\n";

        if let Some(ref stdin_arc) = self.stdin_tx {
            let mut writer = stdin_arc
                .lock()
                .map_err(|_| ExtensionError::Internal("Stdin lock poisoned".into()))?;
            writer.write_all(json_line.as_bytes()).map_err(|e| {
                ExtensionError::IpcError(format!("Failed to write to extension stdin: {e}"))
            })?;
            writer.flush().map_err(|e| {
                ExtensionError::IpcError(format!("Failed to flush extension stdin: {e}"))
            })?;
        } else {
            return Err(ExtensionError::IpcError("Stdin not available".into()));
        }

        if let Some(ref mgr) = self.card_manager
            && let Ok(mut g) = mgr.lock()
        {
            g.record_event(notif);
        }

        Ok(())
    }

    #[note("process-processextension-notify")]
    pub fn notify(&mut self, method: &str, params: Value) -> Result<(), ExtensionError> {
        let req = JsonRpcRequest::new_notification(method, Some(params));
        let line = serde_json::to_string(&req)
            .map_err(|e| ExtensionError::ProtocolError(format!("Failed to serialize: {e}")))?
            + "\n";
        let Some(ref stdin_arc) = self.stdin_tx else {
            return Err(ExtensionError::IpcError("Stdin not available".into()));
        };
        let mut writer = stdin_arc
            .lock()
            .map_err(|_| ExtensionError::Internal("Stdin lock poisoned".into()))?;
        writer
            .write_all(line.as_bytes())
            .and_then(|()| writer.flush())
            .map_err(|e| {
                ExtensionError::IpcError(format!("Failed to write to extension stdin: {e}"))
            })
    }

    #[note("process-processextension-ping")]
    pub fn ping(&mut self) -> Result<bool, ExtensionError> {
        let res = self.send_request(METHOD_PING, None, Duration::from_secs(2))?;
        if let Some(s) = res.as_str() {
            Ok(s == METHOD_PONG || s == "pong")
        } else {
            Ok(true)
        }
    }

    #[note("process-processextension-take-messages")]
    pub fn take_messages(&self) -> Vec<(MessageLevel, String)> {
        if let Ok(mut guard) = self.messages_buffer.lock() {
            std::mem::take(&mut *guard)
        } else {
            Vec::new()
        }
    }

    #[note("process-processextension-shutdown")]
    pub fn shutdown(&mut self) -> Result<(), ExtensionError> {
        if self.child.is_some() && self.is_alive() {
            let _ = self.send_request(METHOD_SHUTDOWN, None, Duration::from_millis(500));
            std::thread::sleep(Duration::from_millis(50));
            if self.is_alive()
                && let Some(ref mut child) = self.child
            {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
        self.status = ProcessStatus::Stopped;
        self.stdin_tx = None;
        self.child = None;
        Ok(())
    }
}

impl Drop for ProcessExtension {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[note("process-processextensionhost")]
pub struct ProcessExtensionHost {
    pub workspace_root: Option<PathBuf>,
    pub user_dir: Option<PathBuf>,
    pub extensions: BTreeMap<String, ProcessExtension>,
    pub action_to_extension: BTreeMap<String, String>,
    pub card_manager: Arc<Mutex<ExtensionCardManager>>,
    /// Every extension's line marks.
    pub line_marks: Arc<Mutex<LineMarkStore>>,
    /// Every extension's views.
    pub views: Arc<Mutex<ViewStore>>,
}

impl ProcessExtensionHost {
    pub fn new(
        workspace_root: Option<PathBuf>,
        card_manager: Arc<Mutex<ExtensionCardManager>>,
    ) -> Self {
        Self {
            workspace_root,
            user_dir: None,
            extensions: BTreeMap::new(),
            action_to_extension: BTreeMap::new(),
            card_manager,
            line_marks: Arc::default(),
            views: Arc::default(),
        }
    }

    #[note("process-processextensionhost-register-manifest")]
    pub fn register_manifest(
        &mut self,
        manifest: ExtensionManifest,
    ) -> Result<bool, ExtensionError> {
        let Some(ref entrypoint) = manifest.entrypoint else {
            return Ok(false);
        };

        // Under development: next to the project. Installed: its own folder, the user store first,
        // then a project's legacy `.crawrad/extensions/<id>/`.
        let folders: Vec<PathBuf> = [
            self.user_dir
                .as_ref()
                .map(|d| d.join(format!("{}@{}", manifest.id, manifest.version))),
            self.workspace_root
                .as_ref()
                .map(|r| r.join(".crawrad").join("extensions").join(&manifest.id)),
        ]
        .into_iter()
        .flatten()
        .collect();
        let binary_path = match &self.workspace_root {
            Some(root) if root.join(entrypoint).exists() => root.join(entrypoint),
            _ => folders
                .iter()
                .find(|f| f.is_dir())
                .or(folders.first())
                .map_or_else(|| PathBuf::from(entrypoint), |f| f.join(entrypoint)),
        };

        let ext_id = manifest.id.clone();
        for action in &manifest.contributions.actions {
            self.action_to_extension
                .insert(action.id.clone(), ext_id.clone());
            if !action.id.contains('.') {
                self.action_to_extension
                    .insert(format!("{}.{}", ext_id, action.id), ext_id.clone());
            }
        }
        for tool in &manifest.contributions.tools {
            self.action_to_extension
                .insert(tool.name.clone(), ext_id.clone());
            self.action_to_extension
                .insert(format!("{}.tool.{}", ext_id, tool.name), ext_id.clone());
        }

        let mut ext =
            ProcessExtension::new(manifest.clone(), binary_path, self.workspace_root.clone());
        ext.set_card_manager(self.card_manager.clone());
        ext.line_marks = Some(self.line_marks.clone());
        ext.views = Some(self.views.clone());
        if let Ok(mut v) = self.views.lock() {
            v.declare(&ext_id, &manifest.contributions.views);
        }
        if manifest.should_activate_on("*") {
            let _ = ext.start();
        }

        self.extensions.insert(ext_id, ext);
        Ok(true)
    }

    #[note("process-processextensionhost-line-mark-clicked")]
    pub fn line_mark_clicked(
        &mut self,
        extension: &str,
        id: &str,
        file: &str,
    ) -> Result<(), ExtensionError> {
        let ext = self
            .extensions
            .get_mut(extension)
            .ok_or_else(|| ExtensionError::NotFound(extension.to_string()))?;
        let clicked = LineMarkClicked {
            id: id.to_string(),
            file: file.to_string(),
        };
        let params = serde_json::to_value(clicked)
            .map_err(|e| ExtensionError::ProtocolError(e.to_string()))?;
        ext.notify(METHOD_EDITOR_LINE_MARK_CLICKED, params)
    }

    #[note("process-processextensionhost-view-item-clicked")]
    pub fn view_item_clicked(
        &mut self,
        extension: &str,
        view_id: &str,
        item_id: &str,
    ) -> Result<(), ExtensionError> {
        let ext = self
            .extensions
            .get_mut(extension)
            .ok_or_else(|| ExtensionError::NotFound(extension.to_string()))?;
        let clicked = ItemClicked {
            view_id: view_id.to_string(),
            item_id: item_id.to_string(),
        };
        let params = serde_json::to_value(clicked)
            .map_err(|e| ExtensionError::ProtocolError(e.to_string()))?;
        ext.notify(METHOD_VIEW_ITEM_CLICKED, params)
    }

    #[note("process-processextensionhost-send-ui-event")]
    pub fn send_ui_event(
        &mut self,
        card_id: &str,
        element_id: &str,
        event: UiEventKind,
    ) -> Result<(), ExtensionError> {
        let ext_id = {
            let mgr = self.card_manager.lock().unwrap();
            mgr.get_card(card_id).map(|c| c.extension_id.clone())
        };
        if let Some(ext_id) = ext_id
            && let Some(ext) = self.extensions.get_mut(&ext_id)
        {
            return ext.send_ui_event(card_id, element_id, event);
        }
        if let Ok(mut mgr) = self.card_manager.lock() {
            mgr.record_event(UiEventNotification {
                card_id: card_id.to_string(),
                element_id: element_id.to_string(),
                event,
            });
        }
        Ok(())
    }

    #[note("process-processextensionhost-can-handle")]
    pub fn can_handle(&self, action_id: &str) -> bool {
        self.action_to_extension.contains_key(action_id)
    }

    #[note("process-processextensionhost-execute-command")]
    pub fn execute_command(
        &mut self,
        action_id: &str,
        arguments: Option<Value>,
    ) -> Result<Option<Value>, ExtensionError> {
        let ext_id = self
            .action_to_extension
            .get(action_id)
            .cloned()
            .ok_or_else(|| ExtensionError::NotFound(action_id.to_string()))?;

        let ext = self
            .extensions
            .get_mut(&ext_id)
            .ok_or_else(|| ExtensionError::NotFound(ext_id.clone()))?;

        ext.execute_command(action_id, arguments)
    }

    #[note("process-processextensionhost-take-messages")]
    pub fn take_messages(&self) -> Vec<(MessageLevel, String)> {
        let mut all_msgs = Vec::new();
        for ext in self.extensions.values() {
            all_msgs.extend(ext.take_messages());
        }
        all_msgs
    }

    #[note("process-processextensionhost-unregister")]
    pub fn unregister(&mut self, id: &str) -> bool {
        self.action_to_extension.retain(|_, ext| ext != id);
        if let Ok(mut marks) = self.line_marks.lock() {
            marks.forget(id);
        }
        if let Ok(mut views) = self.views.lock() {
            views.forget(id);
        }
        match self.extensions.remove(id) {
            Some(mut ext) => {
                let _ = ext.shutdown();
                true
            }
            None => false,
        }
    }

    pub fn shutdown_all(&mut self) {
        for ext in self.extensions.values_mut() {
            let _ = ext.shutdown();
        }
    }
}

impl Drop for ProcessExtensionHost {
    fn drop(&mut self) {
        self.shutdown_all();
    }
}
