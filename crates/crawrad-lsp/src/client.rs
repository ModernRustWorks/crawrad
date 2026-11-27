//! Language Server Protocol (LSP) client process manager and request dispatcher.

use std::collections::HashMap;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::code_lens::CodeLens;
use crate::inlay::InlayHint;
use crate::protocol::{
    CompletionItem, CompletionResponse, Hover, Location, Position, PublishDiagnosticsParams, Range,
};
use crate::transport::{TransportError, read_message, write_message};
use crate::uri::path_to_uri;
use crawrad_note::note;

#[derive(Debug, Error)]
pub enum LspError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Transport error: {0}")]
    Transport(#[from] TransportError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("LSP server executable not found: {0}")]
    ServerNotFound(String),
    #[error("Request timed out")]
    Timeout,
    #[error("Server responded with error ({code}): {message}")]
    RpcError { code: i64, message: String },
    #[error("Channel closed")]
    ChannelClosed,
    #[error("Server not initialized")]
    NotInitialized,
    /// A newer request replaced this one; `$/cancelRequest` was sent.
    #[error("Request cancelled")]
    Cancelled,
    #[error("Not supported: {0}")]
    Unsupported(String),
}

pub(crate) type ResponseCallback =
    Box<dyn FnOnce(Result<serde_json::Value, LspError>) + Send + 'static>;

#[note("client-lspclient")]
pub struct LspClient {
    child: Arc<Mutex<Option<Child>>>,
    /// The server and everything it starts (flycheck's `cargo check`,
    /// `rustc`), ended together on shutdown.
    tree: Mutex<Option<crawrad_os::tree::ProcessTree>>,
    writer_tx: Sender<String>,
    next_id: Arc<AtomicU64>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseCallback>>>,
    diagnostics_rx: Mutex<Receiver<PublishDiagnosticsParams>>,
    is_alive: Arc<AtomicBool>,
    /// What the server last said about ITSELF — above all whether it managed to
    /// load the project. An empty answer on a project that never loaded is not
    /// "none found", and this is what lets the IDE tell the two apart.
    says: crate::server_says::Watch,
    /// Set when the server sends `workspace/inlayHint/refresh` (it finished
    /// indexing, say): the IDE asks for the hints again.
    inlay_refresh: Arc<AtomicBool>,
    /// Set when the server sends `workspace/codeLens/refresh`: the IDE asks
    /// for the lenses again.
    code_lens_refresh: Arc<AtomicBool>,
    /// The same for the semantic tokens: measured on rust-analyzer
    /// 0.3.3065, the tokens asked before it finished loading lack the
    /// `deprecated` modifier, and the ones asked after carry it.
    semantic_refresh: Arc<AtomicBool>,
    /// The server's work in progress (`$/progress`), by token: what the
    /// status bar shows while rust-analyzer loads and indexes.
    progress: Arc<Mutex<std::collections::BTreeMap<String, String>>>,
    /// Wakes the window when the server says something it shows.
    waker: Waker,
    /// `serverInfo` from the `initialize` answer: `rust-analyzer 0.3.2593-standalone`.
    server_version: Mutex<Option<String>>,
    /// Where the server came from: the bar warns about the machine's copy.
    source: crate::locate::ServerSource,
    /// What the server accepts in `didChange` (`capabilities.textDocumentSync`).
    sync: Mutex<crate::sync::SyncKind>,
    /// The column unit both sides use (LSP `positionEncoding`).
    encoding: Mutex<crate::encoding::PositionEncoding>,
    /// `completionProvider.triggerCharacters`: what opens the list by itself.
    completion_triggers: Mutex<Vec<String>>,
    /// `signatureHelpProvider`'s characters: what opens the parameter card.
    signature_triggers: Mutex<Vec<String>>,
    /// `semanticTokensProvider.legend`.
    semantic_legend: Mutex<crate::semantic::Legend>,
    /// The last `publishDiagnostics` of each file, raw, by [`file_key`]: what
    /// a code action request sends as its context.
    published: Arc<Mutex<HashMap<String, Vec<serde_json::Value>>>>,
    /// Asks for the server's own diagnostics, merged into `published`.
    pull: Arc<crate::pull::Pull>,
    /// The text last sent of each open document, by URI: the base of the
    /// next incremental change.
    documents: Mutex<HashMap<String, String>>,
    /// Bytes of the last `didChange` notification — what a key costs on the
    /// wire (measured by the real-server test).
    last_change_bytes: AtomicU64,
}

pub(crate) type Waker = Arc<Mutex<Option<Box<dyn Fn() + Send>>>>;

#[note("client-completion-triggers-from")]
pub fn completion_triggers_from(capabilities: Option<&serde_json::Value>) -> Vec<String> {
    capabilities
        .and_then(|c| c.pointer("/completionProvider/triggerCharacters"))
        .and_then(serde_json::Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|t| t.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[note("client-cancel-poll")]
pub const CANCEL_POLL: Duration = Duration::from_millis(10);

#[note("client-shutdown-wait")]
const SHUTDOWN_WAIT: Duration = Duration::from_secs(2);
const EXIT_WAIT: Duration = Duration::from_secs(1);

impl LspClient {
    #[note("client-lspclient-find-rust-analyzer")]
    #[must_use]
    pub fn find_rust_analyzer() -> Option<PathBuf> {
        crate::locate::locate().ok().map(|found| found.path)
    }

    #[note("client-lspclient-spawn")]
    pub fn spawn(server_path: Option<PathBuf>, workspace_root: &Path) -> Result<Self, LspError> {
        let found = match server_path {
            Some(path) => crate::locate::LocatedServer {
                path,
                source: crate::locate::ServerSource::Override,
            },
            None => crate::locate::locate()
                .map_err(|missing| LspError::ServerNotFound(missing.to_string()))?,
        };
        let executable = found.path;

        let mut child = crawrad_os::process::no_console(&executable)
            .current_dir(workspace_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(LspError::Io)?;
        // Before the server can start anything: its children join the tree.
        let tree = crawrad_os::tree::ProcessTree::adopt(&child).ok();

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| std::io::Error::other("Failed to capture stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::other("Failed to capture stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| std::io::Error::other("Failed to capture stderr"))?;

        let client = Self::over_streams(Some(child), stdin, stdout, Some(stderr), found.source)?;
        if let Ok(mut slot) = client.tree.lock() {
            *slot = tree;
        }
        client.initialize(workspace_root)?;
        Ok(client)
    }

    #[note("client-lspclient-over-streams")]
    pub(crate) fn over_streams(
        child: Option<Child>,
        stdin: impl std::io::Write + Send + 'static,
        stdout: impl std::io::Read + Send + 'static,
        stderr: Option<impl std::io::Read + Send + 'static>,
        source: crate::locate::ServerSource,
    ) -> Result<Self, LspError> {
        let is_alive = Arc::new(AtomicBool::new(true));
        let says = crate::server_says::Watch::new();
        let inlay_refresh = Arc::new(AtomicBool::new(false));
        let code_lens_refresh = Arc::new(AtomicBool::new(false));
        let semantic_refresh = Arc::new(AtomicBool::new(false));
        let progress: Arc<Mutex<std::collections::BTreeMap<String, String>>> = Arc::default();
        let waker: Waker = Arc::default();
        let (writer_tx, writer_rx) = channel::<String>();
        let (diag_tx, diag_rx) = channel::<PublishDiagnosticsParams>();
        let published: Arc<Mutex<HashMap<String, Vec<serde_json::Value>>>> = Arc::default();
        let pending_requests: Arc<Mutex<HashMap<u64, ResponseCallback>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let next_id = Arc::new(AtomicU64::new(1));
        let pull = Arc::new(crate::pull::Pull {
            enabled: AtomicBool::new(false),
            next_id: Arc::clone(&next_id),
            writer: writer_tx.clone(),
            pending: Arc::clone(&pending_requests),
            published: Arc::clone(&published),
            diagnostics: diag_tx.clone(),
            waker: Arc::clone(&waker),
            state: Mutex::default(),
        });

        // 1. Writer thread
        {
            let is_alive_clone = Arc::clone(&is_alive);
            thread::Builder::new()
                .name("lsp-writer".into())
                .spawn(move || {
                    let mut writer = BufWriter::new(stdin);
                    while let Ok(msg) = writer_rx.recv() {
                        if write_message(&mut writer, &msg).is_err() {
                            break;
                        }
                    }
                    is_alive_clone.store(false, Ordering::SeqCst);
                })?;
        }

        // 2. Reader thread
        {
            let is_alive_clone = Arc::clone(&is_alive);
            let pending_clone = Arc::clone(&pending_requests);
            let reply_tx = writer_tx.clone();
            let says_clone = says.clone();
            let refresh = Arc::clone(&inlay_refresh);
            let lens_refresh = Arc::clone(&code_lens_refresh);
            let work = Arc::clone(&progress);
            let wake = Arc::clone(&waker);
            let pull = Arc::clone(&pull);
            let semantic = Arc::clone(&semantic_refresh);
            thread::Builder::new()
                .name("lsp-reader".into())
                .spawn(move || {
                    let mut reader = BufReader::new(stdout);
                    while let Ok(payload) = read_message(&mut reader) {
                        let refresh_diagnostics =
                            payload.contains("\"workspace/diagnostic/refresh\"");
                        if payload.contains("\"workspace/semanticTokens/refresh\"") {
                            semantic.store(true, Ordering::SeqCst);
                        }
                        if answer_server_request(&payload, &reply_tx, &refresh, &lens_refresh) {
                            if refresh_diagnostics {
                                pull.ask_all();
                            }
                            continue;
                        }
                        if track_progress(&payload, &work, &refresh) {
                            if let Some(w) = wake.lock().ok().as_deref().and_then(Option::as_ref) {
                                w();
                            }
                            // The server is done loading or checking: what
                            // it answered meanwhile may be empty.
                            if work.lock().is_ok_and(|w| w.is_empty()) {
                                pull.ask_all();
                                semantic.store(true, Ordering::SeqCst);
                            }
                            continue;
                        }
                        if !pull.on_push(&payload) {
                            handle_incoming_message(
                                &payload,
                                &pending_clone,
                                &diag_tx,
                                &says_clone,
                            );
                        }
                    }
                    is_alive_clone.store(false, Ordering::SeqCst);
                })?;
        }

        // 3. Stderr drain thread to prevent child pipe buffering deadlocks
        if let Some(stderr) = stderr {
            thread::Builder::new()
                .name("lsp-stderr".into())
                .spawn(move || {
                    use std::io::BufRead;
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        if line.is_err() {
                            break;
                        }
                    }
                })?;
        }

        let client = Self {
            child: Arc::new(Mutex::new(child)),
            tree: Mutex::new(None),
            writer_tx,
            next_id,
            pending_requests,
            diagnostics_rx: Mutex::new(diag_rx),
            is_alive,
            says,
            inlay_refresh,
            code_lens_refresh,
            semantic_refresh,
            progress,
            waker,
            server_version: Mutex::new(None),
            sync: Mutex::new(crate::sync::SyncKind::Full),
            completion_triggers: Mutex::new(Vec::new()),
            signature_triggers: Mutex::new(Vec::new()),
            semantic_legend: Mutex::new(crate::semantic::Legend::default()),
            published,
            pull,
            encoding: Mutex::new(crate::encoding::PositionEncoding::Utf16),
            documents: Mutex::new(HashMap::new()),
            last_change_bytes: AtomicU64::new(0),
            source,
        };

        Ok(client)
    }

    #[note("client-lspclient-is-alive")]
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::SeqCst)
    }

    #[note("client-lspclient-poll-diagnostics")]
    pub fn poll_diagnostics(&self) -> Vec<PublishDiagnosticsParams> {
        let mut results = Vec::new();
        if let Ok(rx) = self.diagnostics_rx.lock() {
            while let Ok(diag) = rx.try_recv() {
                results.push(diag);
            }
        }
        results
    }

    #[note("client-lspclient-send-request")]
    pub fn send_request(
        &self,
        method: &str,
        params: serde_json::Value,
        timeout: Duration,
    ) -> Result<serde_json::Value, LspError> {
        self.send_request_until(method, params, timeout, &mut || false)
    }

    #[note("client-lspclient-send-request-until")]
    pub fn send_request_until(
        &self,
        method: &str,
        params: serde_json::Value,
        timeout: Duration,
        superseded: &mut dyn FnMut() -> bool,
    ) -> Result<serde_json::Value, LspError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let (resp_tx, resp_rx) = channel::<Result<serde_json::Value, LspError>>();
        {
            let mut pending = self.pending_requests.lock().unwrap();
            pending.insert(
                id,
                Box::new(move |res| {
                    let _ = resp_tx.send(res);
                }),
            );
        }

        let body = serde_json::to_string(&request_payload)?;
        self.writer_tx
            .send(body)
            .map_err(|_| LspError::ChannelClosed)?;

        let deadline = std::time::Instant::now() + timeout;
        loop {
            // Asked first: a request already replaced when it left is
            // cancelled at once, not after the server's answer.
            if superseded() {
                self.cancel(id);
                return Err(LspError::Cancelled);
            }
            match resp_rx.recv_timeout(CANCEL_POLL) {
                Ok(res) => return res,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(LspError::ChannelClosed);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            }
            if std::time::Instant::now() >= deadline {
                if let Ok(mut pending) = self.pending_requests.lock() {
                    pending.remove(&id);
                }
                return Err(LspError::Timeout);
            }
        }
    }

    #[note("client-lspclient-cancel")]
    fn cancel(&self, id: u64) {
        if let Ok(mut pending) = self.pending_requests.lock() {
            pending.remove(&id);
        }
        let _ = self.send_notification("$/cancelRequest", serde_json::json!({ "id": id }));
    }

    #[note("client-lspclient-send-notification")]
    pub fn send_notification(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<(), LspError> {
        let notification_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        let body = serde_json::to_string(&notification_payload)?;
        self.writer_tx
            .send(body)
            .map_err(|_| LspError::ChannelClosed)
    }

    #[note("client-lspclient-initialize")]
    fn initialize(&self, workspace_root: &Path) -> Result<(), LspError> {
        let root_uri = path_to_uri(workspace_root);
        let init_params = serde_json::json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "capabilities": {
                // LSP 3.17: the column unit; the editor's is UTF-8 bytes.
                "general": { "positionEncodings": crate::encoding::PositionEncoding::OFFERED },
                "textDocument": {
                    "synchronization": {
                        "openClose": true,
                        // The kind is the SERVER's (`textDocumentSync` in its
                        // answer); `did_change` follows it.
                        "change": 2
                    },
                    "completion": {
                        "completionItem": {
                            // The editor fills snippets (Tab through
                            // `$1`, `$2`… to `$0`); before it, `$0` landed
                            // in the code as text.
                            "snippetSupport": true,
                            // Without it the server glues `(as Clone)` onto
                            // the label, and the label is what gets inserted.
                            "labelDetailsSupport": true,
                            // The documentation comes in
                            // `completionItem/resolve`, for the item selected.
                            "resolveSupport": { "properties": ["documentation"] },
                            "documentationFormat": ["markdown", "plaintext"],
                            // A `#[deprecated]` item struck through.
                            "tagSupport": { "valueSet": [1] },
                            "deprecatedSupport": true
                        }
                    },
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    // Offsets for the parameter (no substring guessing),
                    // its own active parameter, and the request's context.
                    // Literal actions (not bare commands), their
                    // `isPreferred`; no `resolveSupport`, so each edit comes
                    // in the answer.
                    "codeAction": {
                        "codeActionLiteralSupport": {
                            "codeActionKind": {
                                "valueSet": ["", "quickfix", "refactor", "refactor.extract",
                                             "refactor.inline", "refactor.rewrite", "source"]
                            }
                        },
                        "isPreferredSupport": true,
                        "disabledSupport": true
                    },
                    // The whole file, then deltas, relative format.
                    "semanticTokens": {
                        "requests": { "full": { "delta": true }, "range": false },
                        "tokenTypes": ["macro", "interface", "variable", "parameter",
                                       "property", "enumMember"],
                        "tokenModifiers": ["mutable", "declaration", "deprecated", "unsafe"],
                        "formats": ["relative"],
                        "overlappingTokenSupport": false,
                        "multilineTokenSupport": false
                    },
                    "signatureHelp": {
                        "signatureInformation": {
                            "documentationFormat": ["plaintext"],
                            "parameterInformation": { "labelOffsetSupport": true },
                            "activeParameterSupport": true
                        },
                        "contextSupport": true
                    },
                    "definition": {
                        "linkSupport": true
                    },
                    "declaration": { "linkSupport": true },
                    "typeDefinition": { "linkSupport": true },
                    "implementation": { "linkSupport": true },
                    "references": {},
                    "rename": { "prepareSupport": true },
                    "formatting": {},
                    "publishDiagnostics": {
                        // Unused code faded, deprecated struck through.
                        "tagSupport": { "valueSet": [1, 2] },
                        "relatedInformation": true
                    },
                    "inlayHint": {},
                    "codeLens": {},
                    "documentHighlight": {},
                    // The server's own diagnostics (`crate::pull`).
                    "diagnostic": { "dynamicRegistration": false }
                },
                "workspace": {
                    "inlayHint": { "refreshSupport": true },
                    "codeLens": { "refreshSupport": true },
                    "semanticTokens": { "refreshSupport": true },
                    "diagnostics": { "refreshSupport": true }
                },
                // Without this the server sends no `$/progress` at all, and
                // the status bar cannot say it is indexing.
                "window": { "workDoneProgress": true },
                // rust-analyzer only sends the code lenses whose
                // commands the client says it runs (none without this).
                "experimental": {
                    "commands": {
                        "commands": [
                            "rust-analyzer.showReferences",
                            "rust-analyzer.runSingle",
                            "rust-analyzer.debugSingle"
                        ]
                    }
                }
            }
        });

        // Initialize can take a few seconds as rust-analyzer scans Cargo.toml
        let answer = self.send_request("initialize", init_params, Duration::from_secs(10))?;
        if let (Some(caps), Ok(mut sync)) = (answer.get("capabilities"), self.sync.lock()) {
            *sync = crate::sync::sync_kind_from(caps);
        }
        if let Ok(mut enc) = self.encoding.lock() {
            *enc = crate::encoding::PositionEncoding::from_capabilities(answer.get("capabilities"));
        }
        if let Ok(mut legend) = self.semantic_legend.lock() {
            *legend = crate::semantic::Legend::from_capabilities(answer.get("capabilities"));
        }
        if let Ok(mut triggers) = self.signature_triggers.lock() {
            *triggers = crate::signature::triggers_from(answer.get("capabilities"));
        }
        if let Ok(mut triggers) = self.completion_triggers.lock() {
            *triggers = completion_triggers_from(answer.get("capabilities"));
        }
        let pulls = answer.pointer("/capabilities/diagnosticProvider").is_some();
        self.pull.enabled.store(pulls, Ordering::SeqCst);
        if let Some(info) = answer.get("serverInfo") {
            let name = info
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("server");
            let version = info.get("version").and_then(|v| v.as_str());
            if let Ok(mut slot) = self.server_version.lock() {
                *slot = Some(match version {
                    Some(v) => format!("{name} {v}"),
                    None => name.to_string(),
                });
            }
        }

        // Send `initialized` notification
        self.send_notification("initialized", serde_json::json!({}))?;

        Ok(())
    }

    #[note("client-lspclient-request-inlay-hints")]
    pub fn request_inlay_hints(
        &self,
        file_path: &Path,
        end: Position,
        timeout: Duration,
        superseded: &mut dyn FnMut() -> bool,
    ) -> Result<Vec<InlayHint>, LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": { "line": end.line, "character": end.character }
            }
        });
        let value =
            self.send_request_until("textDocument/inlayHint", params, timeout, superseded)?;
        Ok(serde_json::from_value(value).unwrap_or_default())
    }

    #[note("client-lspclient-request-code-lenses")]
    pub fn request_code_lenses(
        &self,
        file_path: &Path,
        timeout: Duration,
        superseded: &mut dyn FnMut() -> bool,
    ) -> Result<Vec<CodeLens>, LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) }
        });
        let value =
            self.send_request_until("textDocument/codeLens", params, timeout, superseded)?;
        Ok(crate::code_lens::code_lenses_from(&value))
    }

    #[note("client-lspclient-resolve-code-lens")]
    pub fn resolve_code_lens(
        &self,
        lens: &CodeLens,
        timeout: Duration,
        superseded: &mut dyn FnMut() -> bool,
    ) -> Result<CodeLens, LspError> {
        let params = lens.to_json();
        let value = self.send_request_until("codeLens/resolve", params, timeout, superseded)?;
        crate::code_lens::parse_code_lens(&value).ok_or_else(|| LspError::RpcError {
            code: -1,
            message: "invalid code lens resolve response".to_string(),
        })
    }

    #[note("client-lspclient-request-document-highlight")]
    pub fn request_document_highlight(
        &self,
        file_path: &Path,
        at: Position,
        timeout: Duration,
        superseded: &mut dyn FnMut() -> bool,
    ) -> Result<Vec<crate::highlight::DocumentHighlight>, LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "position": { "line": at.line, "character": at.character }
        });
        let value = self.send_request_until(
            "textDocument/documentHighlight",
            params,
            timeout,
            superseded,
        )?;
        Ok(serde_json::from_value(value).unwrap_or_default())
    }

    #[note("client-lspclient-progress")]
    pub fn progress(&self) -> Option<String> {
        let work = self.progress.lock().ok()?;
        let line = work.values().next()?;
        Some(
            line.split_once('\u{1}')
                .map_or(line.as_str(), |(_, shown)| shown)
                .to_string(),
        )
    }

    #[note("client-lspclient-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        if let Ok(mut w) = self.waker.lock() {
            *w = Some(Box::new(wake));
        }
    }

    #[note("client-lspclient-take-inlay-refresh")]
    pub fn take_inlay_refresh(&self) -> bool {
        self.inlay_refresh.swap(false, Ordering::SeqCst)
    }

    /// What the server last said about its own health.
    ///
    /// The project failing to load, above all. Read it when an answer comes back
    /// EMPTY: that is the one moment where "there are none" and "the server knows
    /// nothing" look exactly the same.
    #[note("client-server-health")]
    pub fn server_health(&self) -> crate::server_says::Health {
        self.says.health()
    }

    #[note("client-lspclient-take-code-lens-refresh")]
    pub fn take_code_lens_refresh(&self) -> bool {
        self.code_lens_refresh.swap(false, Ordering::SeqCst)
    }

    #[note("client-lspclient-take-semantic-refresh")]
    pub fn take_semantic_refresh(&self) -> bool {
        self.semantic_refresh.swap(false, Ordering::SeqCst)
    }

    #[note("client-lspclient-did-open")]
    pub fn did_open(
        &self,
        file_path: &Path,
        language_id: &str,
        version: i32,
        text: &str,
    ) -> Result<(), LspError> {
        let uri = path_to_uri(file_path);
        let params = serde_json::json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": version,
                "text": text,
            }
        });
        if let Ok(mut docs) = self.documents.lock() {
            docs.insert(uri, text.to_string());
        }
        self.send_notification("textDocument/didOpen", params)?;
        self.pull.ask(file_path, version);
        Ok(())
    }

    #[note("client-lspclient-position-encoding")]
    pub fn position_encoding(&self) -> crate::encoding::PositionEncoding {
        self.encoding
            .lock()
            .map_or(crate::encoding::PositionEncoding::Utf16, |e| *e)
    }

    #[note("client-lspclient-completion-triggers")]
    pub fn completion_triggers(&self) -> Vec<String> {
        self.completion_triggers
            .lock()
            .map(|t| t.clone())
            .unwrap_or_default()
    }

    #[note("client-lspclient-semantic-legend")]
    pub fn semantic_legend(&self) -> crate::semantic::Legend {
        self.semantic_legend
            .lock()
            .map(|l| l.clone())
            .unwrap_or_default()
    }

    #[note("client-lspclient-signature-triggers")]
    pub fn signature_triggers(&self) -> Vec<String> {
        self.signature_triggers
            .lock()
            .map(|t| t.clone())
            .unwrap_or_default()
    }

    #[note("client-lspclient-diagnostics-on-line")]
    pub fn diagnostics_on_line(&self, file: &Path, line: u32) -> Vec<serde_json::Value> {
        self.published
            .lock()
            .ok()
            .and_then(|p| {
                let all = p.get(&file_key(file))?;
                Some(crate::code_action::on_line(all, line))
            })
            .unwrap_or_default()
    }

    #[note("client-lspclient-diagnostics-of")]
    pub fn diagnostics_of(&self, file: &Path) -> Option<Vec<crate::protocol::Diagnostic>> {
        let kept = self.published.lock().ok()?;
        let raw = kept.get(&file_key(file))?;
        Some(
            raw.iter()
                .filter_map(|d| serde_json::from_value(d.clone()).ok())
                .collect(),
        )
    }

    #[note("client-lspclient-problem-counts")]
    pub fn problem_counts(&self) -> Vec<(String, usize, usize)> {
        self.published
            .lock()
            .map(|k| problem_counts_of(&k))
            .unwrap_or_default()
    }

    #[note("client-lspclient-problem-totals")]
    pub fn problem_totals(&self) -> Option<(usize, usize)> {
        let kept = self.published.lock().ok()?;
        if kept.is_empty() {
            return None;
        }
        let sum = |(e, w), (_, a, b): (String, usize, usize)| (e + a, w + b);
        Some(problem_counts_of(&kept).into_iter().fold((0, 0), sum))
    }

    #[note("client-lspclient-sync-kind")]
    pub fn sync_kind(&self) -> crate::sync::SyncKind {
        self.sync.lock().map_or(crate::sync::SyncKind::Full, |s| *s)
    }

    #[note("client-lspclient-last-change-bytes")]
    pub fn last_change_bytes(&self) -> u64 {
        self.last_change_bytes.load(Ordering::Relaxed)
    }

    #[note("client-lspclient-did-change")]
    pub fn did_change(&self, file_path: &Path, version: i32, text: &str) -> Result<(), LspError> {
        use crate::sync::SyncKind;
        let uri = path_to_uri(file_path);
        let kind = self.sync_kind();
        if kind == SyncKind::None {
            return Ok(());
        }
        let previous = self
            .documents
            .lock()
            .ok()
            .and_then(|mut docs| docs.insert(uri.clone(), text.to_string()));
        let change = match (kind, previous) {
            (SyncKind::Incremental, Some(old)) => {
                match crate::sync::incremental_change(&old, text, self.position_encoding()) {
                    Some(c) => serde_json::json!({
                        "range": {
                            "start": { "line": c.range.start.line, "character": c.range.start.character },
                            "end": { "line": c.range.end.line, "character": c.range.end.character }
                        },
                        "text": c.text
                    }),
                    None => return Ok(()),
                }
            }
            _ => serde_json::json!({ "text": text }),
        };
        let params = serde_json::json!({
            "textDocument": { "uri": uri, "version": version },
            "contentChanges": [change]
        });
        self.last_change_bytes
            .store(params.to_string().len() as u64, Ordering::Relaxed);
        self.send_notification("textDocument/didChange", params)?;
        self.pull.ask(file_path, version);
        Ok(())
    }

    #[note("client-lspclient-did-save")]
    pub fn did_save(&self, file_path: &Path) -> Result<(), LspError> {
        let uri = path_to_uri(file_path);
        let params = serde_json::json!({
            "textDocument": {
                "uri": uri,
            }
        });
        self.send_notification("textDocument/didSave", params)?;
        self.pull.ask_all();
        Ok(())
    }

    #[note("client-lspclient-is-open")]
    pub fn is_open(&self, file_path: &Path) -> bool {
        let uri = path_to_uri(file_path);
        self.documents.lock().is_ok_and(|d| d.contains_key(&uri))
    }

    #[note("client-lspclient-did-close")]
    pub fn did_close(&self, file_path: &Path) -> Result<(), LspError> {
        let uri = path_to_uri(file_path);
        if let Ok(mut docs) = self.documents.lock() {
            docs.remove(&uri);
        }
        self.pull.forget(file_path);
        let params = serde_json::json!({
            "textDocument": {
                "uri": uri,
            }
        });
        self.send_notification("textDocument/didClose", params)
    }

    #[note("client-lspclient-request-completion")]
    pub fn request_completion(
        &self,
        file_path: &Path,
        line: u32,
        character: u32,
        timeout: Duration,
    ) -> Result<Vec<CompletionItem>, LspError> {
        let uri = path_to_uri(file_path);
        let params = serde_json::json!({
            "textDocument": {
                "uri": uri,
            },
            "position": {
                "line": line,
                "character": character,
            }
        });

        let value = self.send_request("textDocument/completion", params, timeout)?;
        if value.is_null() {
            return Ok(Vec::new());
        }

        let resp: CompletionResponse = serde_json::from_value(value)?;
        Ok(resp.into_items())
    }

    #[note("client-lspclient-request-completion-list")]
    pub fn request_completion_list(
        &self,
        file_path: &Path,
        at: Position,
        context: serde_json::Value,
        timeout: Duration,
        superseded: &mut dyn FnMut() -> bool,
    ) -> Result<(Vec<CompletionItem>, bool), LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "position": { "line": at.line, "character": at.character },
            "context": context
        });
        let value =
            self.send_request_until("textDocument/completion", params, timeout, superseded)?;
        if value.is_null() {
            return Ok((Vec::new(), false));
        }
        let resp: CompletionResponse = serde_json::from_value(value)?;
        let incomplete = resp.is_incomplete();
        Ok((resp.into_items(), incomplete))
    }

    #[note("client-lspclient-request-hover")]
    pub fn request_hover(
        &self,
        file_path: &Path,
        line: u32,
        character: u32,
        timeout: Duration,
    ) -> Result<Option<Hover>, LspError> {
        let uri = path_to_uri(file_path);
        let params = serde_json::json!({
            "textDocument": {
                "uri": uri,
            },
            "position": {
                "line": line,
                "character": character,
            }
        });

        let value = self.send_request("textDocument/hover", params, timeout)?;
        if value.is_null() {
            return Ok(None);
        }

        let hover: Hover = serde_json::from_value(value)?;
        Ok(Some(hover))
    }

    #[note("client-lspclient-request-definition")]
    pub fn request_definition(
        &self,
        file_path: &Path,
        line: u32,
        character: u32,
        timeout: Duration,
    ) -> Result<Vec<Location>, LspError> {
        let uri = path_to_uri(file_path);
        let params = serde_json::json!({
            "textDocument": {
                "uri": uri,
            },
            "position": {
                "line": line,
                "character": character,
            }
        });

        let value = self.send_request("textDocument/definition", params, timeout)?;
        Ok(crate::protocol::locations_from(value))
    }

    #[note("client-lspclient-request-declaration")]
    pub fn request_declaration(
        &self,
        file_path: &Path,
        at: Position,
        timeout: Duration,
    ) -> Result<Vec<Location>, LspError> {
        self.request_places("textDocument/declaration", file_path, at, None, timeout)
    }

    #[note("client-lspclient-request-type-definition")]
    pub fn request_type_definition(
        &self,
        file_path: &Path,
        at: Position,
        timeout: Duration,
    ) -> Result<Vec<Location>, LspError> {
        self.request_places("textDocument/typeDefinition", file_path, at, None, timeout)
    }

    #[note("client-lspclient-request-implementation")]
    pub fn request_implementation(
        &self,
        file_path: &Path,
        at: Position,
        timeout: Duration,
    ) -> Result<Vec<Location>, LspError> {
        self.request_places("textDocument/implementation", file_path, at, None, timeout)
    }

    #[note("client-lspclient-request-references")]
    pub fn request_references(
        &self,
        file_path: &Path,
        at: Position,
        include_declaration: bool,
        timeout: Duration,
    ) -> Result<Vec<Location>, LspError> {
        let context = serde_json::json!({ "includeDeclaration": include_declaration });
        self.request_places(
            "textDocument/references",
            file_path,
            at,
            Some(context),
            timeout,
        )
    }

    #[note("client-lspclient-request-places")]
    fn request_places(
        &self,
        method: &str,
        file_path: &Path,
        at: Position,
        context: Option<serde_json::Value>,
        timeout: Duration,
    ) -> Result<Vec<Location>, LspError> {
        let mut params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "position": { "line": at.line, "character": at.character }
        });
        if let Some(context) = context {
            params["context"] = context;
        }
        let value = self.send_request(method, params, timeout)?;
        Ok(crate::protocol::locations_from(value))
    }

    #[note("client-lspclient-request-prepare-rename")]
    pub fn request_prepare_rename(
        &self,
        file_path: &Path,
        at: Position,
        timeout: Duration,
    ) -> Result<Option<Range>, LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "position": { "line": at.line, "character": at.character }
        });
        let value = self.send_request("textDocument/prepareRename", params, timeout)?;
        // `Range` | `{ range, placeholder }` | `{ defaultBehavior }` | `null`.
        let range = value.get("range").cloned().unwrap_or(value);
        Ok(serde_json::from_value(range).ok())
    }

    #[note("client-lspclient-request-rename")]
    pub fn request_rename(
        &self,
        file_path: &Path,
        at: Position,
        new_name: &str,
        timeout: Duration,
    ) -> Result<crate::edit::WorkspaceEdit, LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "position": { "line": at.line, "character": at.character },
            "newName": new_name
        });
        let value = self.send_request("textDocument/rename", params, timeout)?;
        crate::edit::workspace_edit_from(value).map_err(LspError::Unsupported)
    }

    #[note("client-lspclient-request-formatting")]
    pub fn request_formatting(
        &self,
        file_path: &Path,
        tab_size: u32,
        insert_spaces: bool,
        timeout: Duration,
    ) -> Result<Vec<crate::edit::TextEdit>, LspError> {
        let params = serde_json::json!({
            "textDocument": { "uri": path_to_uri(file_path) },
            "options": { "tabSize": tab_size, "insertSpaces": insert_spaces }
        });
        let value = self.send_request("textDocument/formatting", params, timeout)?;
        Ok(serde_json::from_value(value).unwrap_or_default())
    }

    #[note("client-lspclient-status-name")]
    pub fn status_name(&self) -> &'static str {
        self.source.status_name()
    }

    #[note("client-lspclient-server-version")]
    pub fn server_version(&self) -> Option<String> {
        self.server_version.lock().ok()?.clone()
    }

    #[note("client-lspclient-reload-workspace")]
    pub fn reload_workspace(&self) -> Result<(), LspError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let body = serde_json::to_string(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "rust-analyzer/reloadWorkspace",
            "params": null,
        }))?;
        self.writer_tx
            .send(body)
            .map_err(|_| LspError::ChannelClosed)
    }

    #[note("client-lspclient-shutdown")]
    pub fn shutdown(&self) {
        let child = self.child.lock().ok().and_then(|mut c| c.take());
        self.is_alive.store(false, Ordering::SeqCst);
        let Some(mut child) = child else {
            return;
        };
        let _ = self.send_request("shutdown", serde_json::Value::Null, SHUTDOWN_WAIT);
        let _ = self.send_notification("exit", serde_json::Value::Null);
        let asked = Instant::now();
        while asked.elapsed() < EXIT_WAIT && matches!(child.try_wait(), Ok(None)) {
            std::thread::sleep(Duration::from_millis(20));
        }
        let _ = child.kill();
        if let Some(tree) = self.tree.lock().ok().and_then(|mut t| t.take()) {
            tree.end();
        }
        let _ = child.wait();
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[note("client-answer-server-request")]
pub(crate) fn answer_server_request(
    payload: &str,
    reply: &Sender<String>,
    inlay_refresh: &AtomicBool,
    code_lens_refresh: &AtomicBool,
) -> bool {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(payload) else {
        return false;
    };
    let (Some(id), Some(method)) = (val.get("id"), val.get("method").and_then(|m| m.as_str()))
    else {
        return false;
    };
    if method == "workspace/inlayHint/refresh" {
        inlay_refresh.store(true, Ordering::SeqCst);
    } else if method == "workspace/codeLens/refresh" {
        code_lens_refresh.store(true, Ordering::SeqCst);
    }
    let answer = serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null });
    let _ = reply.send(answer.to_string());
    true
}

#[note("client-track-progress")]
pub(crate) fn track_progress(
    payload: &str,
    work: &Mutex<std::collections::BTreeMap<String, String>>,
    inlay_refresh: &AtomicBool,
) -> bool {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(payload) else {
        return false;
    };
    if val.get("method").and_then(|m| m.as_str()) != Some("$/progress") {
        return false;
    }
    let params = &val["params"];
    let token = params["token"].to_string();
    let value = &params["value"];
    let Ok(mut work) = work.lock() else {
        return true;
    };
    match value["kind"].as_str() {
        Some("end") => {
            work.remove(&token);
            if work.is_empty() {
                inlay_refresh.store(true, Ordering::SeqCst);
            }
        }
        Some(kind) => {
            // One line per token, rebuilt from its parts each time: the title
            // from `begin`, the latest message and percentage from any report.
            // Measured on 01/10/2026: appending gave `Indexing (0%) (5%) · …`
            // and a proc-macro report carried a full `C:\…\x.dll` path.
            let previous = work.get(&token).cloned().unwrap_or_default();
            let mut parts = previous.splitn(2, '\u{1}');
            let mut title = parts.next().unwrap_or("").to_string();
            if kind == "begin" {
                title = value["title"].as_str().unwrap_or("").to_string();
            }
            let message = value["message"]
                .as_str()
                .filter(|m| !m.contains(":\\") && !m.contains(":/"))
                .map(|m| m.chars().take(40).collect::<String>());
            let percent = value["percentage"].as_u64();
            let mut line = title.clone();
            if let Some(m) = message {
                line = format!("{line} {m}");
            }
            if let Some(p) = percent {
                line = format!("{line} ({p}%)");
            }
            work.insert(token, format!("{title}\u{1}{line}"));
        }
        None => {}
    }
    true
}

#[note("client-file-key")]
pub fn file_key(file: &Path) -> String {
    let key = file.to_string_lossy().replace('\\', "/");
    if cfg!(windows) {
        key.to_lowercase()
    } else {
        key
    }
}

#[note("client-problem-counts-of")]
fn problem_counts_of(
    kept: &HashMap<String, Vec<serde_json::Value>>,
) -> Vec<(String, usize, usize)> {
    let severity = |list: &[serde_json::Value], n: u64| {
        let of = |d: &&serde_json::Value| d.get("severity").and_then(|s| s.as_u64()) == Some(n);
        list.iter().filter(of).count()
    };
    let mut out: Vec<_> = kept
        .iter()
        .map(|(file, list)| (file.clone(), severity(list, 1), severity(list, 2)))
        .filter(|(_, e, w)| e + w > 0)
        .collect();
    out.sort();
    out
}

fn handle_incoming_message(
    payload: &str,
    pending: &Arc<Mutex<HashMap<u64, ResponseCallback>>>,
    diag_tx: &Sender<PublishDiagnosticsParams>,
    says: &crate::server_says::Watch,
) {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(payload) else {
        return;
    };

    // Case 1: Response to a request (has `id`)
    if let Some(id_val) = val.get("id")
        && let Some(id) = id_val.as_u64()
    {
        let callback = {
            let mut lock = pending.lock().unwrap();
            lock.remove(&id)
        };

        if let Some(cb) = callback {
            if let Some(err) = val.get("error") {
                let code = err.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
                let message = err
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown RPC error")
                    .to_string();
                cb(Err(LspError::RpcError { code, message }));
            } else {
                let result = val
                    .get("result")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                cb(Ok(result));
            }
        }
        return;
    }

    // Case 2: Server notification
    let Some(method) = val.get("method").and_then(|m| m.as_str()) else {
        return;
    };
    let Some(params) = val.get("params") else {
        return;
    };

    if method == "textDocument/publishDiagnostics"
        && let Ok(diag_params) = serde_json::from_value::<PublishDiagnosticsParams>(params.clone())
    {
        let _ = diag_tx.send(diag_params);
        return;
    }

    // Everything else the server says about ITSELF. Dropping these was how
    // "the project did not load" became "no references found".
    says.heard(method, params);
}

#[cfg(test)]
#[path = "test/cancel/test.rs"]
mod cancel_tests;

#[cfg(test)]
#[path = "test/problems/test.rs"]
mod problem_tests;
