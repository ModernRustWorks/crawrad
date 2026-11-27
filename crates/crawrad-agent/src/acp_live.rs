//! Continuous live ACP agent lifecycle and warm-up.
#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::acp::AcpConfig;
use crate::acp_status::{StatusFeed, StatusLine};
use crate::acp_turn::{TurnEnd, collect_turn};
use crate::chat_layout::{Via, cli_fallback_notice};
use crate::client::{AcpClient, bridge_next_to_this_exe, ide_mcp_server};
use crate::file_write::FileWriteRequest;
use crate::markdown::ToolCallRecord;
use crate::permission::{PermissionRequest, SessionPermissions};
use crawrad_note::note;

#[note("acp-live-livestatus")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiveStatus {
    /// Not started yet (clean state).
    Idle,
    /// Background warm-up is in flight.
    Connecting,
    /// ACP client is connected and ready with an established session.
    Ready { session: String },
    /// Warm-up or connection failed.
    Failed(String),
}

#[note("acp-live-liveagent")]
pub struct LiveAgent {
    client: Option<Arc<AcpClient>>,
    status: LiveStatus,
    agent_key: String,
    warming: Option<WarmRx>,
    choosing: Option<Receiver<Result<Vec<Value>, String>>>,
    pending_choice: Option<(String, String)>,
    last_error: Option<String>,
    /// The ACP session the next warm-up resumes, if the agent can load it.
    resume: Option<String>,
    /// The warm-up failed while the agent catalog was still empty (first run,
    /// download in flight): try again once the catalog has arrived.
    retry_when_catalog: bool,
}

#[note("acp-live-warmrx")]
pub type WarmRx = Receiver<Result<(Arc<AcpClient>, String), String>>;

#[note("acp-live-ide-mcp-servers")]
pub fn ide_mcp_servers() -> Vec<Value> {
    let instance = std::env::var("CRAWRAD_INSTANCE").unwrap_or_else(|_| "default".to_string());
    vec![ide_mcp_server(&bridge_next_to_this_exe(), &instance)]
}

impl Default for LiveAgent {
    fn default() -> Self {
        Self {
            client: None,
            status: LiveStatus::Idle,
            agent_key: String::new(),
            warming: None,
            choosing: None,
            pending_choice: None,
            last_error: None,
            resume: None,
            retry_when_catalog: false,
        }
    }
}

impl LiveAgent {
    #[note("acp-live-liveagent-new")]
    pub fn new() -> Self {
        Self::default()
    }

    #[note("acp-live-liveagent-key-of")]
    pub fn key_of(config: &AcpConfig) -> String {
        format!(
            "{}:{}:{}",
            config.agent_id,
            config.endpoint,
            config.endpoint_args.join(" ")
        )
    }

    #[note("acp-live-liveagent-warm")]
    pub fn warm(
        config: &AcpConfig,
        cwd: Option<PathBuf>,
        mcp_servers: Vec<Value>,
        perms: Arc<Mutex<SessionPermissions>>,
    ) -> WarmRx {
        Self::warm_resuming(config, cwd, mcp_servers, perms, None)
    }

    #[note("acp-live-liveagent-warm-resuming")]
    pub fn warm_resuming(
        config: &AcpConfig,
        cwd: Option<PathBuf>,
        mcp_servers: Vec<Value>,
        perms: Arc<Mutex<SessionPermissions>>,
        resume: Option<String>,
    ) -> WarmRx {
        let (tx, rx) = channel();
        let cfg = config.clone();
        let thread_tx = tx.clone();
        let handle = std::thread::Builder::new()
            .name("acp-live-warm".to_string())
            .spawn(move || {
                let res = (|| {
                    let client =
                        cfg.create_acp_client_with_permissions(cwd.as_deref(), Some(perms))?;
                    client.initialize()?;
                    let loaded = resume.filter(|_| client.can_load_session()).and_then(|id| {
                        client
                            .load_session(&id, cwd.as_deref(), mcp_servers.clone())
                            .ok()
                    });
                    let session_id = match loaded {
                        Some(id) => id,
                        None => client.new_session(cwd.as_deref(), mcp_servers)?,
                    };
                    if let Some(saved) = cfg.session_choices.get(&cfg.agent_id) {
                        for (cid, val) in saved {
                            let _ = client.set_config_option(cid, val);
                        }
                    }
                    Ok((Arc::new(client), session_id))
                })();
                let _ = thread_tx.send(res);
            });
        if let Err(e) = handle {
            let _ = tx.send(Err(format!("Failed to spawn ACP warm thread: {e}")));
        }
        rx
    }

    /// The live failure, when the warm-up or the connection failed.
    ///
    /// The chip and the hero footer both read this, so they cannot give the
    /// panel two different verdicts at the same time.
    pub fn live_failure(&self) -> Option<&str> {
        match self.status() {
            LiveStatus::Failed(e) => Some(e.as_str()),
            _ => None,
        }
    }

    #[note("acp-live-liveagent-status")]
    pub fn status(&self) -> &LiveStatus {
        &self.status
    }

    #[note("acp-live-liveagent-agent-key")]
    pub fn agent_key(&self) -> &str {
        &self.agent_key
    }

    #[note("acp-live-liveagent-last-error")]
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    #[note("acp-live-liveagent-client")]
    pub fn client(&self) -> Option<Arc<AcpClient>> {
        if matches!(self.status, LiveStatus::Ready { .. }) {
            self.client.clone()
        } else {
            None
        }
    }

    #[note("acp-live-liveagent-config-options")]
    pub fn config_options(&self) -> Vec<Value> {
        let Some(client) = self.client() else {
            return Vec::new();
        };
        let mut opts = client.config_options();
        if let Some((ref cid, ref val)) = self.pending_choice {
            for opt in &mut opts {
                if opt.get("id").and_then(|i| i.as_str()) == Some(cid.as_str())
                    && let Some(map) = opt.as_object_mut()
                {
                    map.insert("currentValue".to_string(), Value::String(val.clone()));
                }
            }
        }
        opts
    }

    #[note("acp-live-liveagent-choose")]
    pub fn choose(&mut self, config_id: &str, value: &str) {
        let Some(client) = self.client() else { return };
        self.pending_choice = Some((config_id.to_string(), value.to_string()));
        let (tx, rx) = channel();
        let cid = config_id.to_string();
        let val = value.to_string();
        let handle = std::thread::Builder::new()
            .name("acp-live-choose".to_string())
            .spawn(move || {
                let res = client.set_config_option(&cid, &val);
                let _ = tx.send(res);
            });
        if let Err(e) = handle {
            self.pending_choice = None;
            self.last_error = Some(format!("Failed to spawn choose thread: {e}"));
        } else {
            self.choosing = Some(rx);
        }
    }

    #[note("acp-live-liveagent-set-connecting")]
    pub fn set_connecting(&mut self, agent_key: String) {
        self.client = None;
        self.status = LiveStatus::Connecting;
        self.agent_key = agent_key;
    }

    #[note("acp-live-liveagent-set-ready")]
    pub fn set_ready(&mut self, client: Arc<AcpClient>, session: String) {
        self.client = Some(client);
        self.status = LiveStatus::Ready { session };
        self.last_error = None;
    }

    #[note("acp-live-liveagent-set-failed")]
    pub fn set_failed(&mut self, reason: String) {
        self.client = None;
        self.status = LiveStatus::Failed(reason);
        self.choosing = None;
        self.pending_choice = None;
    }

    #[note("acp-live-liveagent-reset")]
    pub fn reset(&mut self) {
        self.client = None;
        self.warming = None;
        self.choosing = None;
        self.pending_choice = None;
        self.last_error = None;
        self.status = LiveStatus::Idle;
        self.agent_key.clear();
        self.retry_when_catalog = false;
    }

    #[note("acp-live-liveagent-ensure-warm")]
    pub fn ensure_warm(
        &mut self,
        config: &AcpConfig,
        cwd: Option<PathBuf>,
        perms: Arc<Mutex<SessionPermissions>>,
    ) -> bool {
        let folder = cwd.as_deref().map(|p| p.display().to_string());
        let key = format!("{}|{}", Self::key_of(config), folder.unwrap_or_default());
        if self.agent_key != key {
            self.reset();
        }
        let failed = matches!(self.status, LiveStatus::Failed(_));
        if self.client.is_some() || self.warming.is_some() || failed {
            return false;
        }
        self.set_connecting(key);
        let resume = self.resume.take();
        self.warming = Some(Self::warm_resuming(
            config,
            cwd,
            ide_mcp_servers(),
            perms,
            resume,
        ));
        true
    }

    #[note("acp-live-liveagent-resume-next")]
    pub fn resume_next(&mut self, session: Option<String>) {
        self.resume = session;
    }

    #[note("acp-live-liveagent-session-id")]
    pub fn session_id(&self) -> Option<&str> {
        match &self.status {
            LiveStatus::Ready { session } => Some(session),
            _ => None,
        }
    }

    #[note("acp-live-liveagent-can-resume")]
    pub fn can_resume(&self) -> bool {
        self.client().is_some_and(|c| c.can_load_session())
    }

    #[note("acp-live-liveagent-poll")]
    pub fn poll(&mut self) -> bool {
        let mut changed = false;
        if self.retry_when_catalog && crate::acp_registry::registry_agent_count() > 0 {
            self.retry_when_catalog = false;
            self.client = None;
            self.status = LiveStatus::Idle;
            changed = true;
        }
        if let Some(answer) = self.warming.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.warming = None;
            match answer {
                Ok((client, session)) => self.set_ready(client, session),
                Err(e) => {
                    self.retry_when_catalog = crate::acp_registry::registry_agent_count() == 0;
                    self.set_failed(e);
                }
            }
            changed = true;
        }
        if let Some(answer) = self.choosing.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.choosing = None;
            self.pending_choice = None;
            match answer {
                Ok(_) => {
                    self.last_error = None;
                }
                Err(e) => {
                    self.last_error = Some(e);
                }
            }
            changed = true;
        }
        changed
    }
}

#[note("acp-live-turnrequest")]
pub struct TurnRequest {
    pub config: AcpConfig,
    /// The conversation's live client, when it is Ready.
    pub live: Option<Arc<AcpClient>>,
    pub prompt: String,
    pub images: Vec<crate::chat_attachment::ImageAttachment>,
    pub cwd: Option<PathBuf>,
    pub buffers: BTreeMap<PathBuf, String>,
    pub perms: Arc<Mutex<SessionPermissions>>,
    /// The CLI fallback's session: its id and whether it resumes.
    pub cli_session: Option<(String, bool)>,
    /// The active agent's name, as the status line says it.
    pub agent: String,
    /// Where the live status line goes while the turn runs.
    pub status: Sender<StatusLine>,
}

#[note("acp-live-turnresult")]
pub type TurnResult = Result<(String, Vec<ToolCallRecord>, Via, StatusLine), String>;

#[note("acp-live-run-turn")]
pub fn run_turn(
    req: TurnRequest,
    is_cancelled: impl Fn() -> bool,
    on_perm_request: impl Fn(Arc<PermissionRequest>),
    on_write: impl FnMut(Arc<FileWriteRequest>),
) -> TurnResult {
    let cwd = req.cwd.as_deref();
    let mut status = StatusFeed::start(&req.agent, false, req.status.clone());
    let feed = |client: &AcpClient| {
        for (path, text) in &req.buffers {
            client.set_live_buffer(path.to_string_lossy().to_string(), text.clone());
        }
    };
    if let Some(client) = &req.live {
        feed(client);
        let servers = ide_mcp_servers();
        return execute_client_turn(
            client,
            &req.prompt,
            &req.images,
            cwd,
            servers,
            &mut status,
            is_cancelled,
            on_perm_request,
            on_write,
        );
    }
    let acp_err = match req
        .config
        .create_acp_client_with_permissions(cwd, Some(req.perms.clone()))
    {
        Err(e) => e,
        Ok(client) => {
            feed(&client);
            let client = Arc::new(client);
            let turn = client.initialize().and_then(|_| {
                let servers = ide_mcp_servers();
                execute_client_turn(
                    &client,
                    &req.prompt,
                    &req.images,
                    cwd,
                    servers,
                    &mut status,
                    is_cancelled,
                    on_perm_request,
                    on_write,
                )
            });
            match turn {
                Ok(turn) => return Ok(turn),
                Err(e) => e,
            }
        }
    };
    let notice = cli_fallback_notice(&acp_err);
    status.fall_back_to_cli();
    let session = req
        .cli_session
        .as_ref()
        .map(|(id, resume)| (id.as_str(), *resume));
    let mut prompt = req.prompt.clone();
    for img in &req.images {
        if !prompt.is_empty() {
            prompt.push('\n');
        }
        prompt.push_str(&img.prompt_note());
    }
    req.config
        .execute_prompt_with_session_and_cwd(&prompt, session, cwd)
        .map(|text| (text, Vec::new(), Via::Cli { notice }, status.finish()))
}

#[note("acp-live-execute-client-turn")]
pub fn execute_client_turn(
    client: &Arc<AcpClient>,
    prompt: &str,
    images: &[crate::chat_attachment::ImageAttachment],
    cwd: Option<&Path>,
    mcp_servers: Vec<Value>,
    status: &mut StatusFeed,
    is_cancelled: impl Fn() -> bool,
    on_perm_request: impl Fn(Arc<PermissionRequest>),
    on_write: impl FnMut(Arc<FileWriteRequest>),
) -> TurnResult {
    let stream_rx = client.prompt(prompt, images, cwd, mcp_servers)?;
    let on_status = |event: &crate::client::AcpStreamEvent| status.on_event(event);
    match collect_turn(
        &stream_rx,
        is_cancelled,
        on_status,
        on_perm_request,
        on_write,
    ) {
        TurnEnd::Reply(text, tools) => {
            let session = client.active_session_id();
            Ok((text, tools, Via::Acp { session }, status.finish()))
        }
        TurnEnd::Cancelled => {
            let _ = client.cancel();
            Err("Cancelled by user".to_string())
        }
        TurnEnd::Failed(err) => Err(err),
    }
}

#[cfg(test)]
#[path = "test/acp_live/test.rs"]
mod tests;
