//! Continuous Agent Client Protocol (ACP) Client (ADR 0029).
//!
//! Provides a stateful, streaming ACP client communicating with agent subprocesses
//! over JSON-RPC 2.0 stdio pipes. Supports version negotiation, session continuity
//! across prompt turns, streaming updates, cancellation with timeout, and serving
//! live in-memory IDE editor buffers via `fs/read_text_file`.
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::session::PlanStatus;
use crawrad_mcp::AuditLogger;
use crawrad_note::note;

#[note("client-acpstreamevent")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcpStreamEvent {
    /// Incremental chunk of response text.
    TextDelta(String),
    /// Incremental chunk of internal reasoning/thought.
    ThoughtDelta(String),
    /// Tool invocation request by the agent.
    ToolCall {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    /// Tool invocation status update.
    ToolCallUpdate {
        id: String,
        name: String,
        status: super::markdown::ToolCallStatus,
        summary: String,
    },
    /// Request for user permission to execute a tool or action.
    PermissionRequest(Arc<super::permission::PermissionRequest>),
    /// Request to write a file from the agent to the IDE.
    WriteFile(Arc<super::file_write::FileWriteRequest>),
    /// Implementation plan update.
    Plan(PlanStatus),
    /// Prompt completed successfully with stop reason.
    Done { stop_reason: String },
    /// Agent execution or communication error.
    Error(String),
}

type StdinWriter = Arc<Mutex<Box<dyn Write + Send>>>;
type PendingResponses = Arc<Mutex<HashMap<u64, Sender<Result<serde_json::Value, String>>>>>;
/// The end of what the agent wrote on stderr, and whether that stream closed.
type StderrTail = Arc<Mutex<(String, bool)>>;

/// How much of the agent's stderr is kept to explain an early exit.
const STDERR_TAIL_BYTES: usize = 2_000;

/// Drains the agent's stderr (a full pipe would stall the agent) and keeps its tail.
fn keep_stderr_tail<R: std::io::Read + Send + 'static>(stderr: R) -> StderrTail {
    let tail: StderrTail = Arc::new(Mutex::new((String::new(), false)));
    let sink = Arc::clone(&tail);
    let spawned = std::thread::Builder::new()
        .name("acp-client-stderr".to_string())
        .spawn(move || {
            for line in
                std::io::BufRead::lines(std::io::BufReader::new(stderr)).map_while(Result::ok)
            {
                if let Ok(mut kept) = sink.lock() {
                    kept.0.push_str(&line);
                    kept.0.push('\n');
                    let cut = kept.0.len().saturating_sub(STDERR_TAIL_BYTES);
                    let cut = (cut..kept.0.len())
                        .find(|&i| kept.0.is_char_boundary(i))
                        .unwrap_or(0);
                    kept.0.drain(..cut);
                }
            }
            if let Ok(mut kept) = sink.lock() {
                kept.1 = true;
            }
        });
    if spawned.is_err()
        && let Ok(mut kept) = tail.lock()
    {
        kept.1 = true;
    }
    tail
}

/// Why the agent stopped: its exit code and the last thing it wrote on stderr.
fn agent_exit_message(exit_code: Option<i32>, stderr_tail: &str) -> String {
    let head = match exit_code {
        Some(c) => format!("Agent process terminated unexpectedly with exit code {c}"),
        None => "Agent process closed communication stream".to_string(),
    };
    match stderr_tail.trim() {
        "" => head,
        said => format!("{head}: {said}"),
    }
}

#[note("client-acpclient")]
pub struct AcpClient {
    child: Arc<Mutex<Option<Child>>>,
    /// The agent and every process it starts, ended together. An `npx` agent is
    /// `npx` → `cmd` → `node`: killing `child` alone left the `node` running with
    /// a dead parent, one pair per agent switch, New, or test run (the warm-up
    /// made it visible). Also kill-on-close, so a crashed IDE leaves none.
    tree: Option<crawrad_os::tree::ProcessTree>,
    stdin_tx: StdinWriter,
    request_id: Arc<AtomicU64>,
    pending_responses: PendingResponses,
    active_stream_tx: Arc<Mutex<Option<Sender<AcpStreamEvent>>>>,
    /// The prompt turn that owns [`Self::active_stream_tx`]. Written and read
    /// only while that mutex is held — see [`release_stream_if_current`].
    active_turn: Arc<AtomicU64>,
    live_buffers: Arc<Mutex<HashMap<String, String>>>,
    current_session_id: Arc<Mutex<Option<String>>>,
    session_permissions: Arc<Mutex<super::permission::SessionPermissions>>,
    auto_approve_reads: bool,
    allow_native_agent_tools: bool,
    last_activity_ms: Arc<AtomicU64>,
    cancelled_at_ms: Arc<AtomicU64>,
    session_cwd: Arc<Mutex<Option<std::path::PathBuf>>>,
    /// Every file edit must stop at the IDE's review: see [`Self::set_review_edits`].
    review_edits: Arc<AtomicBool>,
    /// The whole session/new result, keeping configOptions and capabilities.
    last_session: Arc<Mutex<Option<serde_json::Value>>>,
    /// The `authMethods` ids the agent offered in `initialize`, in its order.
    auth_methods: Arc<Mutex<Vec<String>>>,
    /// Whether `initialize` declared `agentCapabilities.loadSession`.
    loads_sessions: Arc<std::sync::atomic::AtomicBool>,
    /// Whether `initialize` declared `agentCapabilities.promptCapabilities.image`.
    accepts_images: Arc<std::sync::atomic::AtomicBool>,
    reader_thread: Option<std::thread::JoinHandle<()>>,
}

#[note("client-auth-method-ids")]
pub fn auth_method_ids(initialize_result: &serde_json::Value) -> Vec<String> {
    let methods = initialize_result
        .get("authMethods")
        .and_then(|m| m.as_array());
    let id = |m: &serde_json::Value| m.get("id").and_then(|i| i.as_str()).map(str::to_string);
    methods
        .map(|m| m.iter().filter_map(id).collect())
        .unwrap_or_default()
}

#[note("client-is-auth-required")]
pub fn is_auth_required(error: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(error)
        .ok()
        .and_then(|e| e.get("code").and_then(|c| c.as_i64()))
        == Some(-32000)
}

#[note("client-bridge-next-to-this-exe")]
pub fn bridge_next_to_this_exe() -> std::path::PathBuf {
    let name = if cfg!(windows) {
        "crawrad-mcp-bridge.exe"
    } else {
        "crawrad-mcp-bridge"
    };
    let beside = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(name)));
    beside.unwrap_or_else(|| std::path::PathBuf::from(name))
}

#[note("client-ide-mcp-server")]
pub fn ide_mcp_server(bridge: &Path, instance: &str) -> serde_json::Value {
    let command = if bridge.is_absolute() {
        bridge.to_path_buf()
    } else if let Ok(exe) = std::env::current_exe() {
        exe.parent()
            .map(|dir| dir.join(bridge))
            .unwrap_or_else(|| bridge.to_path_buf())
    } else if let Ok(curr) = std::env::current_dir() {
        curr.join(bridge)
    } else {
        bridge.to_path_buf()
    };
    serde_json::json!({
        "name": "crawrad-ide",
        "command": command.to_string_lossy().to_string(),
        "args": ["--instance", instance],
        "env": []
    })
}

fn current_epoch_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn slice_file_lines(content: &str, line: Option<u64>, limit: Option<u64>) -> String {
    if line.is_none() && limit.is_none() {
        return content.to_string();
    }
    let start_idx = line
        .map(|l| if l > 0 { (l - 1) as usize } else { 0 })
        .unwrap_or(0);
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    if start_idx >= total {
        return String::new();
    }
    let end_idx = match limit {
        Some(lim) => (start_idx + lim as usize).min(total),
        None => total,
    };
    let mut sliced = lines[start_idx..end_idx].join("\n");
    if content.ends_with('\n') {
        sliced.push('\n');
    }
    sliced
}

fn is_path_within_dir(path: &Path, base: &Path) -> bool {
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    };
    let mut normalized = std::path::PathBuf::new();
    for comp in resolved.components() {
        match comp {
            std::path::Component::Prefix(p) => normalized.push(p.as_os_str()),
            std::path::Component::RootDir => {
                normalized.push(std::path::Component::RootDir.as_os_str())
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::Normal(c) => normalized.push(c),
        }
    }
    let mut norm_base = std::path::PathBuf::new();
    for comp in base.components() {
        match comp {
            std::path::Component::Prefix(p) => norm_base.push(p.as_os_str()),
            std::path::Component::RootDir => {
                norm_base.push(std::path::Component::RootDir.as_os_str())
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                norm_base.pop();
            }
            std::path::Component::Normal(c) => norm_base.push(c),
        }
    }
    normalized.starts_with(&norm_base)
}

fn sanitize_mcp_servers(servers: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    servers
        .into_iter()
        .map(|mut s| {
            if let Some(obj) = s.as_object_mut() {
                if !obj.contains_key("env") {
                    obj.insert("env".to_string(), serde_json::json!([]));
                }
                if let Some(cmd) = obj.get("command").and_then(|c| c.as_str()) {
                    let cmd_path = Path::new(cmd);
                    if !cmd_path.is_absolute() {
                        let abs = if let Ok(exe) = std::env::current_exe() {
                            exe.parent()
                                .map(|dir| dir.join(cmd_path))
                                .unwrap_or_else(|| cmd_path.to_path_buf())
                        } else if let Ok(curr) = std::env::current_dir() {
                            curr.join(cmd_path)
                        } else {
                            cmd_path.to_path_buf()
                        };
                        obj.insert(
                            "command".to_string(),
                            serde_json::Value::String(abs.to_string_lossy().to_string()),
                        );
                    }
                }
            }
            s
        })
        .collect()
}

fn update_last_session_config_options(
    last_session: &Arc<Mutex<Option<serde_json::Value>>>,
    opts: Vec<serde_json::Value>,
) {
    if let Ok(mut guard) = last_session.lock() {
        if let Some(ref mut obj) = *guard
            && let Some(map) = obj.as_object_mut()
        {
            map.insert("configOptions".to_string(), serde_json::Value::Array(opts));
            return;
        }
        *guard = Some(serde_json::json!({ "configOptions": opts }));
    }
}

#[note("client-readershared")]
struct ReaderShared {
    pending: PendingResponses,
    stderr_tail: StderrTail,
    active_stream: Arc<Mutex<Option<Sender<AcpStreamEvent>>>>,
    buffers: Arc<Mutex<HashMap<String, String>>>,
    stdin_ref: StdinWriter,
    auto_approve_reads: bool,
    allow_native_agent_tools: bool,
    session_permissions: Arc<Mutex<super::permission::SessionPermissions>>,
    child_ref: Arc<Mutex<Option<Child>>>,
    last_activity: Arc<AtomicU64>,
    session_cwd: Arc<Mutex<Option<std::path::PathBuf>>>,
    last_session: Arc<Mutex<Option<serde_json::Value>>>,
}

impl AcpClient {
    #[note("client-acpclient-spawn")]
    pub fn spawn(
        binary_path: &Path,
        args: &[String],
        cwd: Option<&Path>,
        auto_approve_reads: bool,
    ) -> Result<Self, String> {
        Self::spawn_with_native_tools(binary_path, args, cwd, auto_approve_reads, false)
    }

    #[note("client-acpclient-spawn-with-native-tools")]
    pub fn spawn_with_native_tools(
        binary_path: &Path,
        args: &[String],
        cwd: Option<&Path>,
        auto_approve_reads: bool,
        allow_native_agent_tools: bool,
    ) -> Result<Self, String> {
        Self::spawn_full(
            binary_path,
            args,
            cwd,
            auto_approve_reads,
            allow_native_agent_tools,
            None,
        )
    }

    #[note("client-acpclient-spawn-full")]
    pub fn spawn_full(
        binary_path: &Path,
        args: &[String],
        cwd: Option<&Path>,
        auto_approve_reads: bool,
        allow_native_agent_tools: bool,
        external_session_permissions: Option<Arc<Mutex<super::permission::SessionPermissions>>>,
    ) -> Result<Self, String> {
        let mut cmd = crawrad_os::process::no_console(binary_path);
        cmd.args(args);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn ACP agent `{}`: {e}", binary_path.display()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Failed to capture ACP child stdout".to_string())?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| "Failed to capture ACP child stdin".to_string())?;
        let stderr_tail = match child.stderr.take() {
            Some(stderr) => keep_stderr_tail(stderr),
            None => Arc::new(Mutex::new((String::new(), true))),
        };

        let tree = crawrad_os::tree::ProcessTree::adopt(&child).ok();
        let child_arc = Arc::new(Mutex::new(Some(child)));
        let stdin_tx: StdinWriter = Arc::new(Mutex::new(Box::new(stdin)));
        let request_id = Arc::new(AtomicU64::new(1));
        let pending_responses: PendingResponses = Arc::new(Mutex::new(HashMap::new()));
        let active_stream_tx = Arc::new(Mutex::new(None));
        let live_buffers = Arc::new(Mutex::new(HashMap::new()));
        let current_session_id = Arc::new(Mutex::new(None));
        let session_permissions = external_session_permissions
            .unwrap_or_else(|| Arc::new(Mutex::new(super::permission::SessionPermissions::new())));

        let last_activity_ms = Arc::new(AtomicU64::new(current_epoch_millis()));
        let cancelled_at_ms = Arc::new(AtomicU64::new(0));
        let session_cwd = Arc::new(Mutex::new(cwd.map(|p| p.to_path_buf())));
        let last_session = Arc::new(Mutex::new(None));

        let reader_thread = {
            let pending = Arc::clone(&pending_responses);
            let active_stream = Arc::clone(&active_stream_tx);
            let buffers = Arc::clone(&live_buffers);
            let stdin_ref = Arc::clone(&stdin_tx);
            let child_ref = Arc::clone(&child_arc);
            let perms = Arc::clone(&session_permissions);
            let last_activity = Arc::clone(&last_activity_ms);
            let session_cwd_ref = Arc::clone(&session_cwd);
            let last_session_ref = Arc::clone(&last_session);

            std::thread::Builder::new()
                .name("acp-client-reader".to_string())
                .spawn(move || {
                    Self::reader_loop(
                        stdout,
                        ReaderShared {
                            pending,
                            active_stream,
                            buffers,
                            stdin_ref,
                            auto_approve_reads,
                            allow_native_agent_tools,
                            session_permissions: perms,
                            child_ref,
                            last_activity,
                            session_cwd: session_cwd_ref,
                            last_session: last_session_ref,
                            stderr_tail,
                        },
                    );
                })
                .map_err(|e| format!("Failed to spawn ACP reader thread: {e}"))?
        };

        let client = Self {
            child: child_arc,
            tree,
            stdin_tx,
            request_id,
            pending_responses,
            active_stream_tx,
            active_turn: Arc::new(AtomicU64::new(0)),
            live_buffers,
            current_session_id,
            session_permissions,
            auto_approve_reads,
            allow_native_agent_tools,
            last_activity_ms,
            cancelled_at_ms,
            session_cwd,
            last_session,
            review_edits: Arc::default(),
            auth_methods: Arc::default(),
            loads_sessions: Arc::default(),
            accepts_images: Arc::default(),
            reader_thread: Some(reader_thread),
        };

        Ok(client)
    }

    #[note("client-acpclient-reader-loop")]
    fn reader_loop<R: std::io::Read>(reader: R, shared: ReaderShared) {
        let ReaderShared {
            pending,
            stderr_tail,
            active_stream,
            buffers,
            stdin_ref,
            auto_approve_reads,
            allow_native_agent_tools,
            session_permissions,
            child_ref,
            last_activity,
            session_cwd,
            last_session,
        } = shared;
        let buf_reader = BufReader::new(reader);
        let mut titles: HashMap<String, String> = HashMap::new();

        for line in buf_reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let msg: serde_json::Value = match serde_json::from_str(trimmed) {
                Ok(v) => v,
                Err(_) => continue,
            };

            last_activity.store(current_epoch_millis(), Ordering::SeqCst);

            // 1. Response to client request (has `id`, no `method`)
            if msg.get("method").is_none() {
                if let Some(id) = msg.get("id").and_then(|i| i.as_u64()) {
                    let mut guard = pending.lock().unwrap();
                    if let Some(sender) = guard.remove(&id) {
                        if let Some(err) = msg.get("error") {
                            let _ = sender.send(Err(err.to_string()));
                        } else {
                            let res = msg
                                .get("result")
                                .cloned()
                                .unwrap_or(serde_json::Value::Null);
                            let _ = sender.send(Ok(res));
                        }
                    }
                }
                continue;
            }

            let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");
            let req_id = msg.get("id").and_then(|i| i.as_u64());

            // 2. Notification from agent: session/update
            if method == "session/update" {
                if let Some(params) = msg.get("params") {
                    let update = params.get("update").unwrap_or(params);
                    if update.get("sessionUpdate").and_then(|s| s.as_str())
                        == Some("config_option_update")
                        && let Some(opts) = update.get("configOptions").and_then(|o| o.as_array())
                    {
                        update_last_session_config_options(&last_session, opts.clone());
                    }
                    if let Some(ev) = super::acp_update::parse_update(update, &mut titles)
                        && let Ok(guard) = active_stream.lock()
                        && let Some(ref tx) = *guard
                    {
                        let _ = tx.send(ev);
                    }
                }
                continue;
            }

            // 3. Agent request to IDE: fs/read_text_file
            if method == "fs/read_text_file" {
                if let Some(id) = req_id {
                    let params = msg.get("params");
                    let path = params
                        .and_then(|p| p.get("path"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("");
                    let line_param = params.and_then(|p| p.get("line")).and_then(|v| v.as_u64());
                    let limit_param = params.and_then(|p| p.get("limit")).and_then(|v| v.as_u64());

                    let content = {
                        let buf_guard = buffers.lock().unwrap();
                        if let Some(c) = buf_guard.get(path) {
                            Some(c.clone())
                        } else {
                            // Try matching by filename or normalized path
                            let match_entry = buf_guard
                                .iter()
                                .find(|(k, _)| k.ends_with(path) || path.ends_with(k.as_str()));
                            match_entry.map(|(_, v)| v.clone())
                        }
                    };

                    let file_content = content.or_else(|| std::fs::read_to_string(path).ok());
                    let resp = match file_content {
                        Some(c) => {
                            let sliced = slice_file_lines(&c, line_param, limit_param);
                            serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": { "content": sliced }
                            })
                        }
                        None => serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": { "code": -32602, "message": format!("File '{path}' not found") }
                        }),
                    };

                    if let Ok(mut writer) = stdin_ref.lock() {
                        let _ = writeln!(writer, "{resp}");
                        let _ = writer.flush();
                    }
                }
                continue;
            }

            // 4. Agent request to IDE: fs/write_text_file
            if method == "fs/write_text_file" {
                if let Some(id) = req_id {
                    let path = msg
                        .get("params")
                        .and_then(|p| p.get("path"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("");
                    let content = msg
                        .get("params")
                        .and_then(|p| p.get("content"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("");

                    let is_allowed = {
                        if let Ok(cwd_guard) = session_cwd.lock() {
                            if let Some(ref cwd) = *cwd_guard {
                                is_path_within_dir(Path::new(path), cwd)
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    };

                    if !is_allowed {
                        let resp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32602,
                                "message": format!("Access denied: path '{path}' is outside session cwd")
                            }
                        });

                        if let Ok(mut writer) = stdin_ref.lock() {
                            let _ = writeln!(writer, "{resp}");
                            let _ = writer.flush();
                        }
                        continue;
                    }

                    if let Ok(guard) = active_stream.lock()
                        && let Some(ref stream_tx) = *guard
                    {
                        let id_val = msg.get("id").cloned().unwrap_or(serde_json::json!(id));
                        let buffers_clone = Arc::clone(&buffers);
                        let stdin_clone = Arc::clone(&stdin_ref);
                        let path_str = path.to_string();
                        let content_str = content.to_string();

                        let responder = move |result: Result<(), String>| {
                            let resp = match result {
                                Ok(()) => {
                                    if let Ok(mut buf_guard) = buffers_clone.lock() {
                                        buf_guard.insert(path_str.clone(), content_str.clone());
                                        let match_keys: Vec<String> = buf_guard
                                            .keys()
                                            .filter(|k| {
                                                k.ends_with(&path_str)
                                                    || path_str.ends_with(k.as_str())
                                            })
                                            .cloned()
                                            .collect();
                                        for k in match_keys {
                                            buf_guard.insert(k, content_str.clone());
                                        }
                                    }
                                    serde_json::json!({
                                        "jsonrpc": "2.0",
                                        "id": id_val,
                                        "result": serde_json::Value::Null
                                    })
                                }
                                Err(msg) => {
                                    serde_json::json!({
                                        "jsonrpc": "2.0",
                                        "id": id_val,
                                        "error": {
                                            "code": -32603,
                                            "message": msg
                                        }
                                    })
                                }
                            };
                            if let Ok(mut writer) = stdin_clone.lock() {
                                let _ = writeln!(writer, "{resp}");
                                let _ = writer.flush();
                            }
                        };

                        let req = super::file_write::FileWriteRequest::new(
                            std::path::PathBuf::from(path),
                            content.to_string(),
                            responder,
                        );
                        let _ = stream_tx.send(AcpStreamEvent::WriteFile(Arc::new(req)));
                        continue;
                    }

                    if let Ok(mut buf_guard) = buffers.lock() {
                        buf_guard.insert(path.to_string(), content.to_string());
                        let match_keys: Vec<String> = buf_guard
                            .keys()
                            .filter(|k| k.ends_with(path) || path.ends_with(k.as_str()))
                            .cloned()
                            .collect();
                        for k in match_keys {
                            buf_guard.insert(k, content.to_string());
                        }
                    }

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": serde_json::Value::Null
                    });

                    if let Ok(mut writer) = stdin_ref.lock() {
                        let _ = writeln!(writer, "{resp}");
                        let _ = writer.flush();
                    }
                }
                continue;
            }

            // 5. Agent request to IDE: session/request_permission
            if method == "session/request_permission" {
                if let Some(id) = req_id {
                    let params = msg.get("params").cloned().unwrap_or(serde_json::json!({}));
                    let session_id = params
                        .get("sessionId")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    let tool_call = params
                        .get("toolCall")
                        .cloned()
                        .unwrap_or(serde_json::json!({}));
                    let tool_name = tool_call
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown_tool")
                        .to_string();

                    let target = tool_call
                        .get("target")
                        .and_then(|t| t.as_str())
                        .map(str::to_string)
                        .or_else(|| {
                            // Fallback: busca path / target / file nos arguments
                            tool_call
                                .get("arguments")
                                .and_then(|a| {
                                    a.get("path")
                                        .or_else(|| a.get("target"))
                                        .or_else(|| a.get("file"))
                                })
                                .and_then(|v| v.as_str())
                                .map(str::to_string)
                        });

                    let args_val = tool_call
                        .get("arguments")
                        .cloned()
                        .unwrap_or(serde_json::json!({}));
                    let args_summary =
                        if args_val.is_object() && !args_val.as_object().unwrap().is_empty() {
                            serde_json::to_string(&args_val).unwrap_or_default()
                        } else {
                            String::new()
                        };

                    // Take the options sent by the agent, or fall back to the 4 canonical defaults
                    let mut options = Vec::new();
                    if let Some(opts_arr) = params.get("options").and_then(|o| o.as_array()) {
                        for opt_val in opts_arr {
                            let opt_id = opt_val
                                .get("optionId")
                                .or_else(|| opt_val.get("id"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("allow_once")
                                .to_string();
                            let opt_name = opt_val
                                .get("name")
                                .and_then(|s| s.as_str())
                                .unwrap_or(&opt_id)
                                .to_string();
                            let kind_str = opt_val
                                .get("kind")
                                .and_then(|s| s.as_str())
                                .unwrap_or(&opt_id);
                            let kind =
                                super::permission::PermissionOptionKind::parse_kind(kind_str);
                            options.push(super::permission::PermissionOption::new(
                                opt_id, opt_name, kind,
                            ));
                        }
                    }
                    if options.is_empty() {
                        options = super::permission::PermissionOption::default_options();
                    }

                    // Policy for the agent's native tools: blocked by default unless explicitly allowed
                    let is_native_cli_tool = matches!(
                        tool_name.to_lowercase().as_str(),
                        "bash"
                            | "sh"
                            | "cmd"
                            | "powershell"
                            | "terminal"
                            | "exec"
                            | "execute_command"
                    );
                    if !allow_native_agent_tools && is_native_cli_tool {
                        let reject_opt_id = options
                            .iter()
                            .find(|o| {
                                matches!(
                                    o.kind,
                                    super::permission::PermissionOptionKind::RejectOnce
                                        | super::permission::PermissionOptionKind::RejectAlways
                                )
                            })
                            .map(|o| o.option_id.clone())
                            .unwrap_or_else(|| "reject_once".to_string());

                        AuditLogger::record(
                            &tool_name,
                            &args_val,
                            "rejected_native_policy",
                            &session_id,
                        );

                        let resp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "outcome": {
                                    "outcome": "selected",
                                    "optionId": reject_opt_id
                                }
                            }
                        });
                        if let Ok(mut writer) = stdin_ref.lock() {
                            let _ = writeln!(writer, "{resp}");
                            let _ = writer.flush();
                        }
                        continue;
                    }

                    // 1. Auto-approve reads if enabled in the options
                    let acp_kind = tool_call.get("kind").and_then(|k| k.as_str());
                    if auto_approve_reads
                        && super::permission::is_read_only_tool(&tool_name)
                        && super::edit_review::kind_is_read(acp_kind)
                    {
                        let allow_opt_id = options
                            .iter()
                            .find(|o| {
                                matches!(
                                    o.kind,
                                    super::permission::PermissionOptionKind::AllowOnce
                                        | super::permission::PermissionOptionKind::AllowAlways
                                )
                            })
                            .map(|o| o.option_id.clone())
                            .unwrap_or_else(|| "allow_once".to_string());

                        AuditLogger::record(
                            &tool_name,
                            &args_val,
                            "permitted_auto_read",
                            &session_id,
                        );

                        let resp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "outcome": {
                                    "outcome": "selected",
                                    "optionId": allow_opt_id
                                }
                            }
                        });
                        if let Ok(mut writer) = stdin_ref.lock() {
                            let _ = writeln!(writer, "{resp}");
                            let _ = writer.flush();
                        }
                        continue;
                    }

                    // 2. Session rule: "Allow always" (keyed by tool AND target)
                    let session_allows = {
                        let perms_guard = session_permissions.lock().unwrap();
                        perms_guard.is_allowed(&tool_name, target.as_deref())
                    };
                    if session_allows {
                        let allow_opt_id = options
                            .iter()
                            .find(|o| {
                                matches!(
                                    o.kind,
                                    super::permission::PermissionOptionKind::AllowOnce
                                        | super::permission::PermissionOptionKind::AllowAlways
                                )
                            })
                            .map(|o| o.option_id.clone())
                            .unwrap_or_else(|| "allow_once".to_string());

                        AuditLogger::record(
                            &tool_name,
                            &args_val,
                            "permitted_session_rule",
                            &session_id,
                        );

                        let resp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "outcome": {
                                    "outcome": "selected",
                                    "optionId": allow_opt_id
                                }
                            }
                        });
                        if let Ok(mut writer) = stdin_ref.lock() {
                            let _ = writeln!(writer, "{resp}");
                            let _ = writer.flush();
                        }
                        continue;
                    }

                    // 3. Session rule: "Reject always"
                    let session_rejects = {
                        let perms_guard = session_permissions.lock().unwrap();
                        perms_guard.is_rejected(&tool_name, target.as_deref())
                    };
                    if session_rejects {
                        let reject_opt_id = options
                            .iter()
                            .find(|o| {
                                matches!(
                                    o.kind,
                                    super::permission::PermissionOptionKind::RejectOnce
                                        | super::permission::PermissionOptionKind::RejectAlways
                                )
                            })
                            .map(|o| o.option_id.clone())
                            .unwrap_or_else(|| "reject_once".to_string());

                        AuditLogger::record(
                            &tool_name,
                            &args_val,
                            "rejected_session_rule",
                            &session_id,
                        );

                        let resp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "outcome": {
                                    "outcome": "selected",
                                    "optionId": reject_opt_id
                                }
                            }
                        });
                        if let Ok(mut writer) = stdin_ref.lock() {
                            let _ = writeln!(writer, "{resp}");
                            let _ = writer.flush();
                        }
                        continue;
                    }

                    // 4. Interactive user decision: dispatch to the UI and wait with a timeout
                    let (tx, rx) = std::sync::mpsc::channel();
                    let edit = super::edit_ask::ProposedEdit::from_tool_call(&tool_call);
                    // A diff takes longer to read than a one-line card.
                    let timeout = Duration::from_secs(if edit.is_some() { 900 } else { 60 });
                    let perm_req = Arc::new(
                        super::permission::PermissionRequest::new(
                            id,
                            session_id.clone(),
                            tool_name.clone(),
                            target.clone(),
                            args_summary,
                            options.clone(),
                            tx,
                            timeout,
                        )
                        .with_proposed_edit(edit),
                    );

                    // Register in the stream queue for the IDE
                    if let Ok(guard) = active_stream.lock()
                        && let Some(ref stream_tx) = *guard
                    {
                        let _ = stream_tx
                            .send(AcpStreamEvent::PermissionRequest(Arc::clone(&perm_req)));
                    }

                    // Wait for the answer (or cancel on the 60 s timeout)
                    let decision = match rx.recv_timeout(timeout) {
                        Ok(dec) => dec,
                        Err(_) => super::permission::PermissionDecision::Cancelled,
                    };

                    let resp = match decision {
                        super::permission::PermissionDecision::Selected(opt_id) => {
                            if let Some(opt) = options.iter().find(|o| o.option_id == opt_id) {
                                match opt.kind {
                                    super::permission::PermissionOptionKind::AllowAlways => {
                                        let mut perms_guard = session_permissions.lock().unwrap();
                                        perms_guard.allow_always(tool_name.clone(), target.clone());
                                    }
                                    super::permission::PermissionOptionKind::RejectAlways => {
                                        let mut perms_guard = session_permissions.lock().unwrap();
                                        perms_guard
                                            .reject_always(tool_name.clone(), target.clone());
                                    }
                                    _ => {}
                                }
                            }

                            let is_allow = options
                                .iter()
                                .find(|o| o.option_id == opt_id)
                                .map(|o| {
                                    matches!(
                                        o.kind,
                                        super::permission::PermissionOptionKind::AllowOnce
                                            | super::permission::PermissionOptionKind::AllowAlways
                                    )
                                })
                                .unwrap_or(true);

                            AuditLogger::record(
                                &tool_name,
                                &args_val,
                                if is_allow { "permitted" } else { "rejected" },
                                &session_id,
                            );

                            serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "outcome": {
                                        "outcome": "selected",
                                        "optionId": opt_id
                                    }
                                }
                            })
                        }
                        super::permission::PermissionDecision::Cancelled => {
                            AuditLogger::record(&tool_name, &args_val, "cancelled", &session_id);

                            serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "outcome": {
                                        "outcome": "cancelled"
                                    }
                                }
                            })
                        }
                    };

                    if let Ok(mut writer) = stdin_ref.lock() {
                        let _ = writeln!(writer, "{resp}");
                        let _ = writer.flush();
                    }
                }
                continue;
            }
        }

        // EOF: Process terminated or pipe closed
        let exit_code = if let Ok(mut guard) = child_ref.lock() {
            if let Some(ref mut child) = *guard {
                let mut code = child.try_wait().ok().flatten().and_then(|s| s.code());
                if code.is_none() {
                    for _ in 0..10 {
                        std::thread::sleep(Duration::from_millis(20));
                        code = child.try_wait().ok().flatten().and_then(|s| s.code());
                        if code.is_some() {
                            break;
                        }
                    }
                }
                code
            } else {
                None
            }
        } else {
            None
        };

        // stderr closes with the process; give its reader a moment to finish.
        for _ in 0..25 {
            if stderr_tail.lock().map(|kept| kept.1).unwrap_or(true) {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        let said = stderr_tail
            .lock()
            .map(|kept| kept.0.clone())
            .unwrap_or_default();
        let err_msg = agent_exit_message(exit_code, &said);
        // A request still waiting would otherwise sit out its whole timeout.
        if let Ok(mut waiting) = pending.lock() {
            for (_, tx) in waiting.drain() {
                let _ = tx.send(Err(err_msg.clone()));
            }
        }
        if let Ok(guard) = active_stream.lock()
            && let Some(ref tx) = *guard
        {
            let _ = tx.send(AcpStreamEvent::Error(err_msg));
        };
    }

    #[note("client-acpclient-send-request")]
    pub fn send_request(
        &self,
        method: &str,
        params: serde_json::Value,
        timeout: Duration,
    ) -> Result<serde_json::Value, String> {
        let req_id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = channel();

        {
            let mut guard = self.pending_responses.lock().unwrap();
            guard.insert(req_id, tx);
        }

        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "method": method,
            "params": params,
        });

        {
            let mut writer = self
                .stdin_tx
                .lock()
                .map_err(|_| "Failed to lock stdin writer")?;
            writeln!(writer, "{envelope}")
                .map_err(|e| format!("Failed to write to agent stdin: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("Failed to flush agent stdin: {e}"))?;
        }

        rx.recv_timeout(timeout).map_err(|e| {
            // Clean up on timeout
            let mut guard = self.pending_responses.lock().unwrap();
            guard.remove(&req_id);
            format!("ACP request `{method}` timed out after {timeout:?}: {e}")
        })?
    }

    #[note("client-acpclient-initialize")]
    pub fn initialize(&self) -> Result<serde_json::Value, String> {
        let params = serde_json::json!({
            "protocolVersion": 1,
            "clientCapabilities": {
                "fs": {
                    "readTextFile": true,
                    "writeTextFile": true
                }
            },
            "clientInfo": {
                "name": "CrawRAD",
                "version": "0.1.0"
            }
        });

        let res = self.send_request("initialize", params, Duration::from_secs(60))?;
        if let Ok(mut methods) = self.auth_methods.lock() {
            *methods = auth_method_ids(&res);
        }
        let loads = res.pointer("/agentCapabilities/loadSession");
        self.loads_sessions.store(
            loads.and_then(|l| l.as_bool()).unwrap_or(false),
            Ordering::SeqCst,
        );
        let accepts = res.pointer("/agentCapabilities/promptCapabilities/image");
        self.accepts_images.store(
            accepts.and_then(|a| a.as_bool()).unwrap_or(false),
            Ordering::SeqCst,
        );
        Ok(res)
    }

    #[note("client-acpclient-can-load-session")]
    pub fn can_load_session(&self) -> bool {
        self.loads_sessions.load(Ordering::SeqCst)
    }

    #[note("client-acpclient-accepts-images")]
    pub fn accepts_images(&self) -> bool {
        self.accepts_images.load(Ordering::SeqCst)
    }

    #[note("client-acpclient-new-session")]
    pub fn new_session(
        &self,
        cwd: Option<&Path>,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<String, String> {
        let abs_cwd = match cwd {
            Some(dir) => {
                if dir.is_absolute() {
                    dir.to_path_buf()
                } else {
                    let current = std::env::current_dir().map_err(|e| {
                        format!("Failed to determine working directory for ACP session: {e}")
                    })?;
                    current.join(dir)
                }
            }
            None => std::env::current_dir().map_err(|e| {
                format!("Failed to determine working directory for ACP session: {e}")
            })?,
        };

        if let Ok(mut cwd_guard) = self.session_cwd.lock() {
            *cwd_guard = Some(abs_cwd.clone());
        }

        let sanitized_mcp_servers = sanitize_mcp_servers(mcp_servers);

        let mut params = serde_json::Map::new();
        params.insert(
            "cwd".to_string(),
            serde_json::Value::String(abs_cwd.to_string_lossy().to_string()),
        );
        params.insert(
            "mcpServers".to_string(),
            serde_json::Value::Array(sanitized_mcp_servers),
        );
        if self.review_edits.load(Ordering::Relaxed) {
            params.insert(
                "_meta".to_string(),
                super::edit_review::ask_before_edits_meta(),
            );
        }

        let params = serde_json::Value::Object(params);
        let res = match self.send_request("session/new", params.clone(), Duration::from_secs(60)) {
            // The agent wants a login first: the protocol's `authenticate`, with
            // the first method it offered, then the session once more.
            Err(e) if is_auth_required(&e) => {
                let first = self
                    .auth_methods
                    .lock()
                    .ok()
                    .and_then(|m| m.first().cloned());
                let method = first.ok_or_else(|| {
                    format!("The agent requires a login and offers no method: {e}")
                })?;
                let auth = serde_json::json!({ "methodId": method });
                self.send_request("authenticate", auth, Duration::from_secs(300))?;
                self.send_request("session/new", params, Duration::from_secs(60))?
            }
            other => other?,
        };
        let session_id = res
            .get("sessionId")
            .and_then(|s| s.as_str())
            .ok_or_else(|| "Missing `sessionId` in `session/new` response".to_string())?
            .to_string();

        if let Ok(mut guard) = self.current_session_id.lock() {
            *guard = Some(session_id.clone());
        }
        if let Ok(mut guard) = self.last_session.lock() {
            *guard = Some(res);
        }

        Ok(session_id)
    }

    #[note("client-acpclient-load-session")]
    pub fn load_session(
        &self,
        session_id: &str,
        cwd: Option<&Path>,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<String, String> {
        let abs_cwd = match cwd {
            Some(dir) => {
                if dir.is_absolute() {
                    dir.to_path_buf()
                } else {
                    let current = std::env::current_dir().map_err(|e| {
                        format!("Failed to determine working directory for ACP session: {e}")
                    })?;
                    current.join(dir)
                }
            }
            None => std::env::current_dir().map_err(|e| {
                format!("Failed to determine working directory for ACP session: {e}")
            })?,
        };

        if let Ok(mut cwd_guard) = self.session_cwd.lock() {
            *cwd_guard = Some(abs_cwd.clone());
        }

        let sanitized_mcp_servers = sanitize_mcp_servers(mcp_servers);

        let mut params = serde_json::Map::new();
        params.insert(
            "sessionId".to_string(),
            serde_json::Value::String(session_id.to_string()),
        );
        params.insert(
            "cwd".to_string(),
            serde_json::Value::String(abs_cwd.to_string_lossy().to_string()),
        );
        params.insert(
            "mcpServers".to_string(),
            serde_json::Value::Array(sanitized_mcp_servers),
        );
        if self.review_edits.load(Ordering::Relaxed) {
            params.insert(
                "_meta".to_string(),
                super::edit_review::ask_before_edits_meta(),
            );
        }

        // the agent replays the conversation first (no turn listens, so it is dropped)
        let res = self.send_request(
            "session/load",
            serde_json::Value::Object(params),
            Duration::from_secs(60),
        )?;
        let loaded_id = res
            .get("sessionId")
            .and_then(|s| s.as_str())
            .unwrap_or(session_id)
            .to_string();

        if let Ok(mut guard) = self.current_session_id.lock() {
            *guard = Some(loaded_id.clone());
        }
        // its configOptions, like session/new's, feed the model and effort menus
        if let Ok(mut guard) = self.last_session.lock() {
            *guard = Some(res);
        }

        Ok(loaded_id)
    }

    #[note("client-acpclient-ensure-session")]
    pub fn ensure_session(
        &self,
        cwd: Option<&Path>,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<String, String> {
        if let Ok(guard) = self.current_session_id.lock()
            && let Some(ref id) = *guard
        {
            return Ok(id.clone());
        }
        self.new_session(cwd, mcp_servers)
    }

    #[note("client-acpclient-active-session-id")]
    pub fn active_session_id(&self) -> Option<String> {
        self.current_session_id.lock().ok().and_then(|g| g.clone())
    }

    #[note("client-acpclient-config-options")]
    pub fn config_options(&self) -> Vec<serde_json::Value> {
        self.last_session
            .lock()
            .ok()
            .and_then(|g| {
                g.as_ref()
                    .and_then(|v| v.get("configOptions"))
                    .and_then(|o| o.as_array().cloned())
            })
            .unwrap_or_default()
    }

    #[note("client-acpclient-set-config-option")]
    pub fn set_config_option(
        &self,
        config_id: &str,
        value: &str,
    ) -> Result<Vec<serde_json::Value>, String> {
        let session_id = self
            .active_session_id()
            .ok_or_else(|| "No active ACP session to set config option".to_string())?;
        let params = serde_json::json!({
            "sessionId": session_id,
            "configId": config_id,
            "value": value,
        });
        let res =
            self.send_request("session/set_config_option", params, Duration::from_secs(30))?;
        let opts = res
            .get("configOptions")
            .and_then(|o| o.as_array().cloned())
            .unwrap_or_default();
        update_last_session_config_options(&self.last_session, opts.clone());
        Ok(opts)
    }

    #[note("client-acpclient-prompt")]
    pub fn prompt(
        &self,
        prompt_text: &str,
        images: &[crate::chat_attachment::ImageAttachment],
        cwd: Option<&Path>,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<Receiver<AcpStreamEvent>, String> {
        let session_id = self.ensure_session(cwd, mcp_servers)?;

        let (stream_tx, stream_rx) = channel();
        let turn = {
            let mut guard = self.active_stream_tx.lock().unwrap();
            *guard = Some(stream_tx.clone());
            self.active_turn.fetch_add(1, Ordering::SeqCst) + 1
        };

        let params = serde_json::json!({
            "sessionId": session_id,
            "prompt": crate::chat_attachment::prompt_blocks(prompt_text, images, self.accepts_images()),
        });

        // Spawn waiter thread to handle `session/prompt` response asynchronously
        let req_id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let (resp_tx, resp_rx) = channel();
        {
            let mut guard = self.pending_responses.lock().unwrap();
            guard.insert(req_id, resp_tx);
        }

        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "method": "session/prompt",
            "params": params,
        });

        self.cancelled_at_ms.store(0, Ordering::SeqCst);
        self.last_activity_ms
            .store(current_epoch_millis(), Ordering::SeqCst);

        {
            let mut writer = self
                .stdin_tx
                .lock()
                .map_err(|_| "Failed to lock stdin writer")?;
            writeln!(writer, "{envelope}").map_err(|e| format!("Failed to send prompt: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("Failed to flush prompt: {e}"))?;
        }

        let active_stream_ref = Arc::clone(&self.active_stream_tx);
        let active_turn_ref = Arc::clone(&self.active_turn);
        let last_activity_ref = Arc::clone(&self.last_activity_ms);
        let cancelled_at_ref = Arc::clone(&self.cancelled_at_ms);

        std::thread::spawn(move || {
            loop {
                match resp_rx.recv_timeout(Duration::from_millis(200)) {
                    Ok(Ok(res)) => {
                        let stop_reason = res
                            .get("stopReason")
                            .and_then(|s| s.as_str())
                            .unwrap_or("end_turn")
                            .to_string();
                        let _ = stream_tx.send(AcpStreamEvent::Done { stop_reason });
                        break;
                    }
                    Ok(Err(e)) => {
                        let _ = stream_tx.send(AcpStreamEvent::Error(e));
                        break;
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        let now = current_epoch_millis();

                        // Safety net: if cancel was requested, finish after 10 s if agent never answers
                        let cancel_time = cancelled_at_ref.load(Ordering::SeqCst);
                        if cancel_time > 0 && now.saturating_sub(cancel_time) >= 10_000 {
                            let _ = stream_tx.send(AcpStreamEvent::Done {
                                stop_reason: "cancelled".to_string(),
                            });
                            break;
                        }

                        // Inactivity check: 600 s (10 minutes) with no messages from agent
                        let last_act = last_activity_ref.load(Ordering::SeqCst);
                        if last_act > 0 && now.saturating_sub(last_act) >= 600_000 {
                            let _ = stream_tx.send(AcpStreamEvent::Error(
                                "The agent sent nothing for 10 minutes".to_string(),
                            ));
                            break;
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
            }
            release_stream_if_current(&active_stream_ref, &active_turn_ref, turn);
        });

        Ok(stream_rx)
    }

    #[note("client-acpclient-prompt-sync")]
    pub fn prompt_sync(
        &self,
        prompt_text: &str,
        cwd: Option<&Path>,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<String, String> {
        let rx = self.prompt(prompt_text, &[], cwd, mcp_servers)?;
        let mut full_text = String::new();

        while let Ok(event) = rx.recv() {
            match event {
                AcpStreamEvent::TextDelta(t) => full_text.push_str(&t),
                AcpStreamEvent::Done { .. } => break,
                AcpStreamEvent::Error(err) => return Err(err),
                // No one reviews a synchronous prompt: its writes land as asked.
                AcpStreamEvent::WriteFile(req) => req.respond(Ok(())),
                _ => {}
            }
        }

        Ok(full_text)
    }

    #[note("client-acpclient-cancel")]
    pub fn cancel(&self) -> Result<(), String> {
        let session_id = match self.active_session_id() {
            Some(id) => id,
            None => return Ok(()),
        };

        self.cancelled_at_ms
            .store(current_epoch_millis(), Ordering::SeqCst);

        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "session/cancel",
            "params": {
                "sessionId": session_id,
            }
        });

        let mut writer = self
            .stdin_tx
            .lock()
            .map_err(|_| "Failed to lock stdin writer")?;
        writeln!(writer, "{envelope}")
            .map_err(|e| format!("Failed to send cancel notification: {e}"))?;
        writer
            .flush()
            .map_err(|e| format!("Failed to flush cancel notification: {e}"))?;

        Ok(())
    }

    #[note("client-acpclient-set-live-buffer")]
    pub fn set_live_buffer(&self, path: impl Into<String>, content: impl Into<String>) {
        if let Ok(mut guard) = self.live_buffers.lock() {
            guard.insert(path.into(), content.into());
        }
    }

    #[note("client-acpclient-auto-approve-reads")]
    pub fn auto_approve_reads(&self) -> bool {
        self.auto_approve_reads
    }

    #[note("client-acpclient-clear-live-buffers")]
    pub fn clear_live_buffers(&self) {
        if let Ok(mut guard) = self.live_buffers.lock() {
            guard.clear();
        }
    }

    #[note("client-acpclient-is-alive")]
    pub fn is_alive(&self) -> bool {
        if let Ok(mut guard) = self.child.lock()
            && let Some(ref mut child) = *guard
        {
            return matches!(child.try_wait(), Ok(None));
        }
        false
    }

    #[note("client-acpclient-session-permissions")]
    pub fn session_permissions(&self) -> Arc<Mutex<super::permission::SessionPermissions>> {
        Arc::clone(&self.session_permissions)
    }

    #[note("client-acpclient-is-permission-allowed")]
    pub fn is_permission_allowed(&self, tool: &str, target: Option<&str>) -> bool {
        if let Ok(guard) = self.session_permissions.lock() {
            guard.is_allowed(tool, target)
        } else {
            false
        }
    }

    #[note("client-acpclient-is-permission-rejected")]
    pub fn is_permission_rejected(&self, tool: &str, target: Option<&str>) -> bool {
        if let Ok(guard) = self.session_permissions.lock() {
            guard.is_rejected(tool, target)
        } else {
            false
        }
    }

    #[note("client-acpclient-allow-always")]
    pub fn allow_always(&self, tool: impl Into<String>, target: Option<String>) {
        if let Ok(mut guard) = self.session_permissions.lock() {
            guard.allow_always(tool, target);
        }
    }

    #[note("client-acpclient-clear-session-permissions")]
    pub fn clear_session_permissions(&self) {
        if let Ok(mut guard) = self.session_permissions.lock() {
            guard.clear();
        }
    }

    #[note("client-acpclient-set-review-edits")]
    pub fn set_review_edits(&self, on: bool) {
        self.review_edits.store(on, Ordering::Relaxed);
    }

    #[note("client-acpclient-allow-native-agent-tools")]
    pub fn allow_native_agent_tools(&self) -> bool {
        self.allow_native_agent_tools
    }

    #[note("client-acpclient-shutdown")]
    pub fn shutdown(&mut self) {
        let _ = self.send_request(
            "session/shutdown",
            serde_json::json!({}),
            Duration::from_millis(500),
        );
        if let Ok(mut guard) = self.child.lock() {
            if let Some(ref mut child) = *guard {
                let _ = child.kill();
                let _ = child.wait();
            }
            *guard = None;
        }
        if let Some(tree) = &self.tree {
            tree.end();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for AcpClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[note("client-release-stream-if-current")]
pub(crate) fn release_stream_if_current(
    sink: &Mutex<Option<Sender<AcpStreamEvent>>>,
    active_turn: &AtomicU64,
    turn: u64,
) {
    if let Ok(mut guard) = sink.lock()
        && active_turn.load(Ordering::SeqCst) == turn
    {
        *guard = None;
    }
}

#[cfg(test)]
#[path = "test/client/test.rs"]
mod tests;
