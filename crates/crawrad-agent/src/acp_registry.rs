//! Official ACP (Agent Client Protocol) Registry & Catalog.
//!
//! The single source of truth is the official CDN:
//! `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
//!
//! On startup the disk cache is loaded instantly; a background thread
//! fetches the latest version and overwrites the cache atomically.
//! When there is no cache AND no network, the list is empty and a
//! status message explains the situation — no agents are invented.

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock, RwLock};

pub const CDN_REGISTRY_URL: &str =
    "https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json";

#[note("acp-registry-acp-portal-url")]
pub const ACP_PORTAL_URL: &str = "https://agentclientprotocol.com";

#[note("acp-registry-acp-registry-search-url")]
pub const ACP_REGISTRY_SEARCH_URL: &str =
    "https://github.com/anthropics/agent-client-protocol/blob/main/registry/registry.json";

// ──────────────────────────────────── data model ─────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcpAgentEntry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    pub author: String,
    pub badge: String,
    pub description: String,
    pub package_cmd: String,
    pub transport: String,
    pub provider_key: String,
    pub default_model: String,
    pub supported_models: Vec<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub distribution: Option<serde_json::Value>,
}

#[note("acp-registry-acplaunch")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcpLaunch {
    pub program: String,
    pub args: Vec<String>,
}

#[note("acp-registry-current-platform")]
pub fn current_platform() -> &'static str {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        "windows-x86_64"
    }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        "windows-aarch64"
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        "linux-x86_64"
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        "linux-aarch64"
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        "darwin-x86_64"
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        "darwin-aarch64"
    }
    #[cfg(not(any(
        all(
            target_os = "windows",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "linux",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "macos",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
    )))]
    {
        "unknown"
    }
}

#[note("acp-registry-agents-dir")]
pub fn agents_dir() -> PathBuf {
    if let Some(app_data) = std::env::var_os("APPDATA") {
        PathBuf::from(app_data).join("CrawRAD").join("agents")
    } else if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))
    {
        PathBuf::from(home).join(".crawrad").join("agents")
    } else {
        PathBuf::from("agents")
    }
}

#[note("acp-registry-launch-for")]
pub fn launch_for(
    entry: &AcpAgentEntry,
    platform: &str,
    agents_dir: &std::path::Path,
) -> Result<AcpLaunch, String> {
    let dist = entry
        .distribution
        .as_ref()
        .ok_or_else(|| format!("{} has no build for {}", entry.name, platform))?;

    if let Some(npx) = dist.get("npx") {
        let pkg = npx
            .get("package")
            .and_then(|p| p.as_str())
            .ok_or_else(|| format!("{} has invalid npx package", entry.name))?;
        let mut args = vec!["-y".to_string(), pkg.to_string()];
        if let Some(extra_args) = npx.get("args").and_then(|a| a.as_array()) {
            for arg in extra_args {
                if let Some(s) = arg.as_str() {
                    args.push(s.to_string());
                }
            }
        }
        Ok(AcpLaunch {
            program: "npx".to_string(),
            args,
        })
    } else if let Some(uvx) = dist.get("uvx") {
        let pkg = uvx
            .get("package")
            .and_then(|p| p.as_str())
            .ok_or_else(|| format!("{} has invalid uvx package", entry.name))?;
        let mut args = vec![pkg.to_string()];
        if let Some(extra_args) = uvx.get("args").and_then(|a| a.as_array()) {
            for arg in extra_args {
                if let Some(s) = arg.as_str() {
                    args.push(s.to_string());
                }
            }
        }
        Ok(AcpLaunch {
            program: "uvx".to_string(),
            args,
        })
    } else if let Some(binary) = dist.get("binary") {
        let plat_val = binary
            .get(platform)
            .ok_or_else(|| format!("{} has no build for {}", entry.name, platform))?;
        let raw_cmd = plat_val
            .get("cmd")
            .and_then(|c| c.as_str())
            .ok_or_else(|| format!("{} binary entry has no cmd", entry.name))?;
        let clean_cmd = raw_cmd.strip_prefix("./").unwrap_or(raw_cmd);
        let ver = entry.version.as_deref().unwrap_or("latest");
        let program_path = agents_dir.join(&entry.id).join(ver).join(clean_cmd);
        let mut args = Vec::new();
        if let Some(extra_args) = plat_val.get("args").and_then(|a| a.as_array()) {
            for arg in extra_args {
                if let Some(s) = arg.as_str() {
                    args.push(s.to_string());
                }
            }
        }
        Ok(AcpLaunch {
            program: program_path.to_string_lossy().to_string(),
            args,
        })
    } else {
        Err(format!("{} has no build for {}", entry.name, platform))
    }
}

#[note("acp-registry-is-executable-on-path")]
pub fn is_executable_on_path(binary_name: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        let extensions: &[&str] = if cfg!(windows) {
            &[".cmd", ".exe", ".bat", ""]
        } else {
            &[""]
        };

        for dir in std::env::split_paths(&path_var) {
            for ext in extensions {
                let candidate = dir.join(format!("{binary_name}{ext}"));
                if candidate.is_file() {
                    return true;
                }
            }
        }
    }
    false
}

impl AcpAgentEntry {
    #[note("acp-registry-acpagententry-distribution-kind")]
    pub fn distribution_kind(&self) -> Option<&'static str> {
        let dist = self.distribution.as_ref()?;
        if dist.get("npx").is_some() {
            Some("npx")
        } else if dist.get("uvx").is_some() {
            Some("uvx")
        } else if dist.get("binary").is_some() {
            Some("binary")
        } else {
            None
        }
    }

    #[note("acp-registry-acpagententry-is-installed-for")]
    pub fn is_installed_for(&self, platform: &str, agents_dir: &std::path::Path) -> bool {
        match self.distribution_kind() {
            Some("npx") => is_executable_on_path("npx"),
            Some("uvx") => is_executable_on_path("uvx"),
            Some("binary") => {
                if let Ok(launch) = launch_for(self, platform, agents_dir) {
                    std::path::Path::new(&launch.program).is_file()
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    #[note("acp-registry-acpagententry-is-installed")]
    pub fn is_installed(&self) -> bool {
        self.is_installed_for(current_platform(), &agents_dir())
    }

    #[note("acp-registry-acpagententry-is-active")]
    pub fn is_active(&self, endpoint: &str, provider: &str) -> bool {
        if !endpoint.is_empty() {
            if let Ok(launch) = launch_for(self, current_platform(), &agents_dir())
                && endpoint == launch.program
            {
                return true;
            }
            endpoint == self.package_cmd
        } else {
            provider == self.provider_key
        }
    }

    #[note("acp-registry-acpagententry-action-button-text")]
    pub fn action_button_text(&self, is_active: bool) -> &'static str {
        if is_active {
            "In use"
        } else {
            let platform = current_platform();
            let agents = agents_dir();
            match self.distribution_kind() {
                Some("binary") => {
                    if let Ok(launch) = launch_for(self, platform, &agents) {
                        if std::path::Path::new(&launch.program).is_file() {
                            "Connect"
                        } else {
                            "Install"
                        }
                    } else {
                        "Install"
                    }
                }
                Some("npx") => {
                    if is_executable_on_path("npx") {
                        "Connect"
                    } else {
                        "Needs Node.js"
                    }
                }
                Some("uvx") => {
                    if is_executable_on_path("uvx") {
                        "Connect"
                    } else {
                        "Needs uv"
                    }
                }
                _ => "Connect",
            }
        }
    }

    #[note("acp-registry-acpagententry-repo-button-text")]
    pub fn repo_button_text(&self) -> &'static str {
        if self.repository.is_some() {
            "GitHub ↗"
        } else if self.website.is_some() {
            "Docs ↗"
        } else {
            "Registry ↗"
        }
    }

    #[note("acp-registry-acpagententry-target-url")]
    pub fn target_url(&self) -> &str {
        self.repository
            .as_deref()
            .or(self.website.as_deref())
            .unwrap_or(ACP_PORTAL_URL)
    }

    #[note("acp-registry-acpagententry-connect-to-config")]
    pub fn connect_to_config(&self, cfg: &mut crate::AcpConfig) -> Result<String, String> {
        let launch = launch_for(self, current_platform(), &agents_dir())?;
        if self.distribution_kind() == Some("binary")
            && !std::path::Path::new(&launch.program).is_file()
        {
            return Err(format!("Install {} first", self.name));
        }
        cfg.agent_id = "custom".to_string();
        cfg.endpoint = launch.program.clone();
        cfg.endpoint_args = launch.args.clone();
        cfg.provider = self.provider_key.clone();
        if !self.default_model.is_empty() {
            cfg.model = self.default_model.clone();
        }
        cfg.save()
            .map_err(|e| format!("Failed to save ACP config: {e}"))?;
        Ok(format!(
            "✓ Connected to '{}' (Provider: {}, Endpoint: {})",
            self.name, self.provider_key, launch.program
        ))
    }
}

// ──────────────────────────────────── CDN payload ────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CdnRegistryPayload {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub agents: Vec<CdnAgentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CdnAgentItem {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub license_url: Option<String>,
    #[serde(default)]
    pub distribution: Option<serde_json::Value>,
    #[serde(default)]
    pub icon: Option<String>,
}

impl From<CdnAgentItem> for AcpAgentEntry {
    fn from(item: CdnAgentItem) -> Self {
        let author = item
            .authors
            .first()
            .cloned()
            .unwrap_or_else(|| "Community".to_string());
        let badge = match item.id.as_str() {
            "claude-code" | "gemini-cli" | "codex-acp" => "Official".to_string(),
            _ => "Registry".to_string(),
        };
        let desc = item.description.unwrap_or_default();
        let package_cmd = if let Some(dist) = &item.distribution {
            if let Some(npx) = dist
                .get("npx")
                .and_then(|v| v.get("package"))
                .and_then(|p| p.as_str())
            {
                npx.to_string()
            } else if let Some(bin) = dist
                .get("binary")
                .and_then(|v| v.as_object())
                .and_then(|o| o.values().next())
                .and_then(|v| v.get("cmd"))
                .and_then(|c| c.as_str())
            {
                bin.trim_start_matches("./").to_string()
            } else {
                item.id.clone()
            }
        } else {
            item.id.clone()
        };

        let mut tags = Vec::new();
        tags.push(item.id.clone());
        for a in &item.authors {
            tags.push(a.clone());
        }
        if let Some(lic) = &item.license {
            tags.push(lic.clone());
        }
        let provider_key = format!("{} (ACP)", item.name);
        Self {
            id: item.id,
            name: item.name,
            version: item.version,
            author,
            badge,
            description: desc,
            package_cmd,
            transport: "stdio".to_string(),
            provider_key,
            default_model: "auto".to_string(),
            supported_models: vec!["auto".to_string()],
            tags,
            repository: item.repository,
            website: item.website,
            distribution: item.distribution,
        }
    }
}

// ──────────────────────────────────── disk cache ─────────

fn registry_cache_path() -> Option<PathBuf> {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let dir = PathBuf::from(appdata).join("CrawRAD");
        let _ = std::fs::create_dir_all(&dir);
        Some(dir.join("registry_cache.json"))
    } else if let Ok(home) = std::env::var("HOME") {
        let dir = PathBuf::from(home).join(".crawrad");
        let _ = std::fs::create_dir_all(&dir);
        Some(dir.join("registry_cache.json"))
    } else {
        None
    }
}

fn load_cache_from_disk() -> Option<Vec<AcpAgentEntry>> {
    let cache_path = registry_cache_path()?;
    if !cache_path.is_file() {
        return None;
    }
    let conteudo = std::fs::read_to_string(cache_path).ok()?;
    let payload: CdnRegistryPayload = serde_json::from_str(&conteudo).ok()?;
    Some(
        payload
            .agents
            .into_iter()
            .map(AcpAgentEntry::from)
            .collect(),
    )
}

fn save_cache_to_disk(payload_json: &str) {
    if let Some(cache_path) = registry_cache_path() {
        let _ = std::fs::write(cache_path, payload_json);
    }
}

#[note("acp-registry-cache-timestamp")]
pub fn cache_timestamp() -> Option<String> {
    let cache_path = registry_cache_path()?;
    let meta = std::fs::metadata(cache_path).ok()?;
    let modified = meta.modified().ok()?;
    let elapsed = modified.elapsed().ok()?;
    let secs = elapsed.as_secs();
    let text = if secs < 60 {
        "just now".to_string()
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86400 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86400)
    };
    Some(text)
}

// ──────────────────────────────────── global store ───────

struct RegistryState {
    agents: Vec<AcpAgentEntry>,
    status: RegistryStatus,
}

#[derive(Clone, PartialEq, Eq)]
pub enum RegistryStatus {
    /// Loaded from cache, background update pending or done.
    Cached,
    /// Fresh data fetched from CDN.
    Live,
    /// No cache AND no network.
    Empty,
    /// Had cache but network fetch failed (offline).
    Offline,
}

static REGISTRY_STORE: OnceLock<RwLock<RegistryState>> = OnceLock::new();
static FETCH_STARTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn store() -> &'static RwLock<RegistryState> {
    REGISTRY_STORE.get_or_init(|| {
        if let Some(do_disco) = load_cache_from_disk() {
            RwLock::new(RegistryState {
                agents: do_disco,
                status: RegistryStatus::Cached,
            })
        } else {
            RwLock::new(RegistryState {
                agents: Vec::new(),
                status: RegistryStatus::Empty,
            })
        }
    })
}

// ──────────────────────────────────── background fetch ───

#[note("acp-registry-spawn-background-refresh")]
pub fn spawn_background_refresh() {
    if FETCH_STARTED.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(|| {
        let agente = ureq::Agent::config_builder()
            .tls_config(
                ureq::tls::TlsConfig::builder()
                    .provider(ureq::tls::TlsProvider::Rustls)
                    .unversioned_rustls_crypto_provider(Arc::new(
                        rustls_graviola::default_provider(),
                    ))
                    .build(),
            )
            .build()
            .new_agent();

        match agente
            .get(CDN_REGISTRY_URL)
            .header("User-Agent", "CrawRAD-IDE/0.1")
            .call()
        {
            Ok(mut resp) => {
                if let Ok(body) = resp.body_mut().read_to_string()
                    && let Ok(payload) = serde_json::from_str::<CdnRegistryPayload>(&body)
                {
                    save_cache_to_disk(&body);
                    let fresh: Vec<AcpAgentEntry> = payload
                        .agents
                        .into_iter()
                        .map(AcpAgentEntry::from)
                        .collect();
                    if let Ok(mut lock) = store().write() {
                        lock.agents = fresh;
                        lock.status = RegistryStatus::Live;
                    }
                }
            }
            Err(_) => {
                if let Ok(mut lock) = store().write() {
                    if lock.status == RegistryStatus::Empty {
                        // No cache AND no network — stay empty.
                    } else {
                        lock.status = RegistryStatus::Offline;
                    }
                }
            }
        }
    });
}

// ──────────────────────────────────── public API ─────────

#[note("acp-registry-search-registry")]
pub fn search_registry(query: &str) -> Vec<AcpAgentEntry> {
    spawn_background_refresh();
    let q = query.trim().to_lowercase();
    let lock = store().read().unwrap();

    if q.is_empty() {
        return lock.agents.clone();
    }

    lock.agents
        .iter()
        .filter(|a| {
            a.name.to_lowercase().contains(&q)
                || a.id.to_lowercase().contains(&q)
                || a.author.to_lowercase().contains(&q)
                || a.description.to_lowercase().contains(&q)
                || a.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
        .cloned()
        .collect()
}

#[note("acp-registry-registry-status")]
pub fn registry_status() -> RegistryStatus {
    store().read().unwrap().status.clone()
}

#[note("acp-registry-registry-agent-count")]
pub fn registry_agent_count() -> usize {
    store().read().unwrap().agents.len()
}

#[note("acp-registry-registry-subtitle")]
pub fn registry_subtitle() -> String {
    let count = registry_agent_count();
    let status = registry_status();
    let ts = cache_timestamp().unwrap_or_else(|| "never".to_string());

    match status {
        RegistryStatus::Live => format!("{count} agents · updated {ts}"),
        RegistryStatus::Cached => format!("{count} agents · cached {ts}"),
        RegistryStatus::Offline => {
            format!("{count} agents · offline — showing cached list from {ts}")
        }
        RegistryStatus::Empty => {
            "No agents — connect to the internet to fetch the registry".to_string()
        }
    }
}

#[note("acp-registry-registry-browse-url")]
pub fn registry_browse_url(query: &str) -> String {
    let q = query.trim();
    if q.is_empty() {
        ACP_PORTAL_URL.to_string()
    } else {
        let encoded: String = q
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                    c.to_string()
                } else if c == ' ' {
                    "+".to_string()
                } else {
                    format!("%{:02X}", c as u32)
                }
            })
            .collect();
        format!(
            "https://github.com/anthropics/agent-client-protocol/blob/main/registry/registry.json#:~:text={}",
            encoded
        )
    }
}

#[note("acp-registry-registry-link-text")]
pub fn registry_link_text(query: &str) -> String {
    let q = query.trim();
    if q.is_empty() {
        "Browse all agents on the Official ACP Portal (Web) →".to_string()
    } else {
        format!("Search \"{q}\" on Official ACP Registry (Web) →")
    }
}

#[note("acp-registry-acpagentoperationalstatus")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcpAgentOperationalStatus {
    pub active_name: String,
    pub active_version: String,
    pub command: String,
    pub status_title: String,
    pub status_detail: String,
    pub target_url: String,
    pub is_installed: bool,
}

#[note("acp-registry-active-entry")]
pub fn active_entry<'a>(
    cfg: &crate::AcpConfig,
    agents: &'a [AcpAgentEntry],
) -> Option<&'a AcpAgentEntry> {
    let by_profile = crate::agent_profile::find_profile_by_id(&cfg.agent_id)
        .filter(|p| p.id != "custom")
        .and_then(|p| agents.iter().find(|a| a.id == p.registry_id));
    by_profile.or_else(|| {
        agents
            .iter()
            .find(|a| a.is_active(&cfg.endpoint, &cfg.provider))
    })
}

#[note("acp-registry-agentstate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Ready,
    NotInstalled,
    SignedOut,
    Failed,
}

impl AgentState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::NotInstalled => "Not installed",
            Self::SignedOut => "Signed out",
            Self::Failed => "Failed",
        }
    }

    pub fn fix(self) -> Option<&'static str> {
        match self {
            Self::Ready => None,
            Self::NotInstalled => Some("Install"),
            Self::SignedOut => Some("Sign in"),
            Self::Failed => Some("Diagnose"),
        }
    }

    pub fn hint(self, connected: bool, program: &str) -> String {
        match self {
            Self::Ready => {
                if connected {
                    "Connected and answering in the chat.".to_string()
                } else {
                    "Installed and ready.".to_string()
                }
            }
            Self::NotInstalled => format!("The command {program} was not found."),
            Self::SignedOut => "The agent asks you to sign in.".to_string(),
            Self::Failed => "The agent failed to connect or start.".to_string(),
        }
    }
}

#[note("acp-registry-agent-state")]
pub fn agent_state(installed: bool, connected: bool, last_error: Option<&str>) -> AgentState {
    if connected {
        AgentState::Ready
    } else if last_error.is_some_and(crate::client::is_auth_required) {
        AgentState::SignedOut
    } else if !installed {
        AgentState::NotInstalled
    } else if last_error.is_some() {
        AgentState::Failed
    } else {
        AgentState::Ready
    }
}

pub fn chat_status_line(state: AgentState, connected: bool) -> &'static str {
    match (state, connected) {
        (AgentState::Ready, true) => "🟢 Connected · ready",
        (AgentState::Ready, false) => "🟢 Ready · connects on your first message",
        (AgentState::SignedOut, true) | (AgentState::SignedOut, false) => {
            "🟡 Signed out · sign in at Tools › Options › AI"
        }
        (AgentState::NotInstalled, true) | (AgentState::NotInstalled, false) => {
            "⚪ No AI agent installed · set one up at Tools › Options › AI"
        }
        (AgentState::Failed, true) | (AgentState::Failed, false) => {
            "🔴 Connection failed · sending messages will not work"
        }
    }
}

#[note("acp-registry-agentcard")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCard {
    pub status: AcpAgentOperationalStatus,
    pub state: AgentState,
    /// The line under the status.
    pub hint: String,
    /// The last connection message, else the registered command.
    pub detail: String,
}

#[note("acp-registry-agent-card")]
pub fn agent_card(cfg: &crate::AcpConfig, status_msg: Option<&str>, connected: bool) -> AgentCard {
    let status = get_operational_status(cfg, status_msg);
    let state = agent_state(status.is_installed, connected, status_msg);
    let program = status.command.trim();
    let program = program.strip_prefix("stdio:").unwrap_or(program);
    let hint = state.hint(connected, program.split_whitespace().next().unwrap_or(""));
    let detail = status_msg.unwrap_or(&status.status_detail).to_string();
    AgentCard {
        status,
        state,
        hint,
        detail,
    }
}

#[note("acp-registry-get-operational-status")]
pub fn get_operational_status(
    cfg: &crate::AcpConfig,
    custom_status_msg: Option<&str>,
) -> AcpAgentOperationalStatus {
    let agents = search_registry("");
    let (active_name, active_version, target_url, is_installed) =
        if let Some(agent) = active_entry(cfg, &agents) {
            (
                agent.name.clone(),
                format!("(v{})", agent.badge),
                agent.target_url().to_string(),
                agent.is_installed(),
            )
        } else {
            let name = if cfg.provider.is_empty() {
                "Custom Agent".to_string()
            } else {
                cfg.provider.clone()
            };
            // A custom agent is installed when the command it runs is there;
            // its link is the ACP portal, never another vendor's docs.
            let program = cfg.endpoint.trim();
            let program = program.strip_prefix("stdio:").unwrap_or(program);
            let program = program.split_whitespace().next().unwrap_or("");
            let found = !program.is_empty()
                && (std::path::Path::new(program).is_file() || is_executable_on_path(program));
            (name, String::new(), ACP_PORTAL_URL.to_string(), found)
        };

    let command = cfg.endpoint.clone();

    let status_title = if let Some(msg) = custom_status_msg {
        msg.to_string()
    } else if is_installed {
        format!("{active_name} is installed and ready.")
    } else {
        format!("{active_name} is not installed ({command}).")
    };

    let status_detail = format!("Registered command: {}", cfg.endpoint);

    AcpAgentOperationalStatus {
        active_name,
        active_version,
        command,
        status_title,
        status_detail,
        target_url,
        is_installed,
    }
}

// ──────────────────────────────────── parse helper ───────

#[note("acp-registry-parse-registry-json")]
pub fn parse_registry_json(json: &str) -> Result<Vec<AcpAgentEntry>, String> {
    let payload: CdnRegistryPayload =
        serde_json::from_str(json).map_err(|e| format!("Invalid registry JSON: {e}"))?;
    Ok(payload
        .agents
        .into_iter()
        .map(AcpAgentEntry::from)
        .collect())
}

#[note("acp-registry-registry-agent")]
pub fn registry_agent(id: &str) -> Option<AcpAgentEntry> {
    store()
        .read()
        .ok()?
        .agents
        .iter()
        .find(|a| a.id.eq_ignore_ascii_case(id))
        .cloned()
}

#[note("acp-registry-install-binary-agent")]
pub fn install_binary_agent(
    entry: &AcpAgentEntry,
    platform: &str,
    agents_dir: &std::path::Path,
) -> Result<PathBuf, String> {
    match entry.distribution_kind() {
        Some("npx") => {
            return Err(format!(
                "{} runs through npx; nothing to install",
                entry.name
            ));
        }
        Some("uvx") => {
            return Err(format!(
                "{} runs through uvx; nothing to install",
                entry.name
            ));
        }
        Some("binary") => {}
        _ => {
            return Err(format!("{} has no binary distribution", entry.name));
        }
    }

    let dist = entry
        .distribution
        .as_ref()
        .ok_or_else(|| format!("{} has no distribution", entry.name))?;
    let binary = dist
        .get("binary")
        .ok_or_else(|| format!("{} has no binary distribution", entry.name))?;
    let plat_val = binary
        .get(platform)
        .ok_or_else(|| format!("{} has no build for {}", entry.name, platform))?;
    let archive_url = plat_val
        .get("archive")
        .and_then(|a| a.as_str())
        .ok_or_else(|| format!("{} has no archive URL for {}", entry.name, platform))?;

    let ver = entry.version.as_deref().unwrap_or("latest");
    let target_dir = agents_dir.join(&entry.id).join(ver);

    if !target_dir.starts_with(agents_dir) {
        return Err("Path traversal forbidden".to_string());
    }
    std::fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create directory {}: {e}", target_dir.display()))?;

    // A fixed name, never one taken from the URL: a `..` there would leave `agents_dir`.
    let archive_path = target_dir.join("agent-archive.download");

    let agent = ureq::Agent::config_builder()
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .provider(ureq::tls::TlsProvider::Rustls)
                .unversioned_rustls_crypto_provider(Arc::new(rustls_graviola::default_provider()))
                .build(),
        )
        .build()
        .new_agent();

    let mut resp = agent
        .get(archive_url)
        .header("User-Agent", "CrawRAD-IDE/0.1")
        .call()
        .map_err(|e| format!("Failed to download {archive_url}: {e}"))?;

    // Streamed to disk: an agent archive is >100 MB (Antigravity's is 124 MB),
    // past ureq's 10 MB default for a body read into memory.
    let mut file = std::fs::File::create(&archive_path)
        .map_err(|e| format!("Failed to create the archive file: {e}"))?;
    std::io::copy(&mut resp.body_mut().as_reader(), &mut file)
        .map_err(|e| format!("Failed to download the archive body: {e}"))?;
    drop(file);

    let extract_status = crawrad_os::process::no_console("tar")
        .args([
            "-xf",
            &archive_path.to_string_lossy(),
            "-C",
            &target_dir.to_string_lossy(),
        ])
        .status()
        .map_err(|e| format!("Failed to run tar: {e}"))?;

    let _ = std::fs::remove_file(&archive_path);

    if !extract_status.success() {
        return Err(format!(
            "tar extraction failed with status {extract_status}"
        ));
    }

    let launch = launch_for(entry, platform, agents_dir)?;
    let prog_path = PathBuf::from(&launch.program);
    if !prog_path.is_file() {
        return Err(format!(
            "Installed program '{}' does not exist after extraction",
            launch.program
        ));
    }

    Ok(prog_path)
}

#[note("acp-registry-agentclickresult")]
#[derive(Debug)]
pub enum AgentClickResult {
    Connected(String),
    InstallStarted(String, std::sync::mpsc::Receiver<Result<String, String>>),
    NeedsRequirement(String),
    Ignored,
}

#[note("acp-registry-click-agent-action")]
pub fn click_agent_action(agent: &AcpAgentEntry, cfg: &mut crate::AcpConfig) -> AgentClickResult {
    let is_active = agent.is_active(&cfg.endpoint, &cfg.provider);
    let label = agent.action_button_text(is_active);
    match label {
        "In use" => AgentClickResult::Ignored,
        "Needs Node.js" => AgentClickResult::NeedsRequirement(
            "Node.js (npx) is not found in PATH. Install Node.js from https://nodejs.org/"
                .to_string(),
        ),
        "Needs uv" => AgentClickResult::NeedsRequirement(
            "uv (uvx) is not found in PATH. Install uv from https://docs.astral.sh/uv/".to_string(),
        ),
        "Install" => {
            let name = agent.name.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            let entry = agent.clone();
            std::thread::spawn(move || {
                let res =
                    install_binary_agent(&entry, current_platform(), &agents_dir()).map(|_| name);
                let _ = tx.send(res);
            });
            AgentClickResult::InstallStarted(agent.name.clone(), rx)
        }
        _ => match agent.connect_to_config(cfg) {
            Ok(msg) => AgentClickResult::Connected(msg),
            Err(err) => AgentClickResult::NeedsRequirement(err),
        },
    }
}

#[note("acp-registry-list-agents-json")]
pub fn list_agents_json(
    cfg: &crate::AcpConfig,
    platform: &str,
    agents_dir: &std::path::Path,
) -> serde_json::Value {
    let agents = search_registry("");
    let items: Vec<serde_json::Value> = agents
        .iter()
        .map(|a| {
            let active = a.is_active(&cfg.endpoint, &cfg.provider);
            let installed = a.is_installed_for(platform, agents_dir);
            let kind = a.distribution_kind().unwrap_or("unknown");
            let action = a.action_button_text(active);
            serde_json::json!({
                "id": a.id,
                "name": a.name,
                "version": a.version.as_deref().unwrap_or(""),
                "kind": kind,
                "installed": installed,
                "active": active,
                "action": action,
            })
        })
        .collect();
    serde_json::Value::Array(items)
}

#[note("acp-registry-acptoolcall")]
pub struct AcpToolCall {
    pub result: crawrad_mcp::protocol::ToolCallResult,
    pub audit: &'static str,
    pub install: Option<(String, std::sync::mpsc::Receiver<Result<String, String>>)>,
}

#[note("acp-registry-mcp-call")]
pub fn mcp_call(name: &str, args: &serde_json::Value, cfg: &mut crate::AcpConfig) -> AcpToolCall {
    use crawrad_mcp::protocol::ToolCallResult as R;
    let done = |result, audit| AcpToolCall {
        result,
        audit,
        install: None,
    };
    if name == "acp.list_agents" {
        return done(
            R::success_json(mcp_list_agents(cfg), "ACP agents listed."),
            "applied",
        );
    }
    let Some(id) = crawrad_mcp::cargo_tool::nonempty(args, "id") else {
        return done(R::error_text("missing-id", "id is required"), "error");
    };
    match name {
        "acp.install_agent" => match mcp_install_agent(id) {
            Ok((agent, rx)) => AcpToolCall {
                result: R::success_json(
                    serde_json::json!({ "status": "started", "id": id }),
                    "Installation started in background.",
                ),
                audit: "applied",
                install: Some((agent, rx)),
            },
            Err(e) => done(R::error_text("install-error", e), "error"),
        },
        _ => match mcp_connect_agent(id, cfg) {
            Ok(val) => done(R::success_json(val, "Agent connected."), "applied"),
            Err(e) => done(R::error_text("connect-error", e), "error"),
        },
    }
}

pub fn mcp_list_agents(cfg: &crate::AcpConfig) -> serde_json::Value {
    list_agents_json(cfg, current_platform(), &agents_dir())
}

#[note("acp-registry-mcp-install-agent")]
pub fn mcp_install_agent(
    id: &str,
) -> Result<(String, std::sync::mpsc::Receiver<Result<String, String>>), String> {
    let entry = registry_agent(id).ok_or_else(|| format!("Agent '{id}' not found in registry"))?;
    if entry.distribution_kind() != Some("binary") {
        return Err(format!(
            "Agent '{}' runs through npx/uvx; nothing to install",
            entry.name
        ));
    }
    let name = entry.name.clone();
    let name_ret = name.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let res = install_binary_agent(&entry, current_platform(), &agents_dir()).map(|_| name);
        let _ = tx.send(res);
    });
    Ok((name_ret, rx))
}

#[note("acp-registry-mcp-connect-agent")]
pub fn mcp_connect_agent(
    id: &str,
    cfg: &mut crate::AcpConfig,
) -> Result<serde_json::Value, String> {
    let entry = registry_agent(id).ok_or_else(|| format!("Agent '{id}' not found in registry"))?;
    entry.connect_to_config(cfg)?;
    Ok(serde_json::json!({
        "agent_id": cfg.agent_id,
        "endpoint": cfg.endpoint,
        "endpoint_args": cfg.endpoint_args,
    }))
}

#[cfg(test)]
#[path = "test/acp_registry/test.rs"]
mod tests;
