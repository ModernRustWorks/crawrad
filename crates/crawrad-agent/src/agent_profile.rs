//! Canonical AI Agent Profiles for the Agent Client Protocol (ACP).
//!
//! Each supported agent in CrawRAD has an explicit `AgentProfile` defining:
//! - Stable identifier persisted in config (`agent_id`)
//! - Friendly UI display label
//! - Default executable binary and endpoint
//! - Multi-turn session continuity semantics (`Pinned`, `Captured`, or `None`)
//! - Prompt delivery mechanism (`Stdin` or `BareStdin`)
//! - Dynamic or verified static model catalog (queryable via `ModelSource`)

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

#[note("agent-profile-sessionsupport")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionSupport {
    /// Agent does not support continuous session flags (each turn is standalone).
    None,
    /// Agent accepts an externally pinned UUID via `--session-id` on turn 1 and `--resume` on turn 2+ (e.g. Claude Code).
    Pinned,
    /// Agent CLI generates its own conversation ID on turn 1, captured and resumed via `--conversation <id>` (e.g. Gemini / Antigravity CLI).
    Captured,
}

#[note("agent-profile-promptdelivery")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptDelivery {
    /// Prompt passed via stdin pipe, preventing Windows 32,766 character command line limit.
    Stdin,
    /// Prompt on stdin with NO prompt flag: the CLI reads stdin when it gets no
    /// `-p` (`agy`). It used to be `-p <prompt>` in argv, which a prompt with the
    /// project context outgrew: Windows refuses the process (error 206) and the
    /// turn came back as an empty response.
    BareStdin,
}

#[note("agent-profile-modelspec")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelSpec {
    /// Exact identifier passed to `--model` (e.g. "sonnet", "gemini-3.8-flash-high").
    pub id: String,
    /// Friendly UI display label.
    pub display_name: String,
    /// Contextual description, provider and capability notes.
    pub description: String,
    /// Whether the model supports extended reasoning / thinking blocks.
    pub supports_reasoning: bool,
}

#[note("agent-profile-modelsource")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelSource {
    /// Static list of verified models.
    Static(&'static [StaticModelSpec]),
    /// Dynamic agent query with static fallback if CLI query fails.
    AgentCommand {
        subcommand: &'static str,
        fallback: &'static [StaticModelSpec],
    },
}

#[note("agent-profile-staticmodelspec")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticModelSpec {
    pub id: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub supports_reasoning: bool,
}

impl StaticModelSpec {
    pub fn to_model_spec(self) -> ModelSpec {
        ModelSpec {
            id: self.id.to_string(),
            display_name: self.display_name.to_string(),
            description: self.description.to_string(),
            supports_reasoning: self.supports_reasoning,
        }
    }
}

#[note("agent-profile-agentprofile")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProfile {
    /// Stable canonical identifier stored in settings (e.g. "claude", "gemini", "codex").
    pub id: &'static str,
    /// Official ACP registry identifier (e.g. "claude-acp", "antigravity-acp").
    pub registry_id: &'static str,
    /// Friendly UI display label.
    pub display_name: &'static str,
    /// Default binary executable name to resolve.
    pub default_binary: &'static str,
    /// Default canonical endpoint.
    pub default_endpoint: &'static str,
    /// Session continuity semantics.
    pub session: SessionSupport,
    /// Prompt delivery mechanism.
    pub prompt_delivery: PromptDelivery,
    /// Prompt delivery via stdin flag for backwards compatibility.
    pub prompt_via_stdin: bool,
    /// Model catalog source.
    pub model_source: ModelSource,
}

static MODEL_CACHE: OnceLock<Mutex<HashMap<String, Vec<ModelSpec>>>> = OnceLock::new();
static FETCH_IN_FLIGHT: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
static BACKGROUND_FETCH: AtomicBool = AtomicBool::new(true);

fn model_cache() -> &'static Mutex<HashMap<String, Vec<ModelSpec>>> {
    MODEL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn fetch_in_flight() -> &'static Mutex<HashSet<String>> {
    FETCH_IN_FLIGHT.get_or_init(|| Mutex::new(HashSet::new()))
}

impl AgentProfile {
    #[note("agent-profile-agentprofile-set-background-fetch")]
    pub fn set_background_fetch(enabled: bool) {
        BACKGROUND_FETCH.store(enabled, Ordering::SeqCst);
    }

    #[note("agent-profile-agentprofile-models")]
    pub fn models(&self) -> Vec<ModelSpec> {
        match &self.model_source {
            ModelSource::Static(specs) => specs.iter().map(|s| s.to_model_spec()).collect(),
            ModelSource::AgentCommand { fallback, .. } => {
                if let Ok(cache) = model_cache().lock()
                    && let Some(cached) = cache.get(self.id)
                {
                    return cached.clone();
                }
                self.trigger_background_fetch_once();
                fallback.iter().map(|s| s.to_model_spec()).collect()
            }
        }
    }

    #[note("agent-profile-agentprofile-trigger-background-fetch-once")]
    pub fn trigger_background_fetch_once(&self) {
        if cfg!(test) || !BACKGROUND_FETCH.load(Ordering::SeqCst) {
            return;
        }

        if let ModelSource::AgentCommand { subcommand, .. } = self.model_source {
            let mut in_flight = match fetch_in_flight().lock() {
                Ok(guard) => guard,
                Err(_) => return,
            };
            if in_flight.insert(self.id.to_string()) {
                let id = self.id.to_string();
                let bin = resolve_binary_path(self.default_binary);
                let sub = subcommand.to_string();
                std::thread::spawn(move || {
                    if let Ok(queried) = query_agent_models(&bin, &sub)
                        && !queried.is_empty()
                        && let Ok(mut cache) = model_cache().lock()
                    {
                        cache.insert(id, queried);
                    }
                });
            }
        }
    }

    #[note("agent-profile-agentprofile-refresh-models-in-background")]
    pub fn refresh_models_in_background(&self) {
        if let ModelSource::AgentCommand { subcommand, .. } = self.model_source {
            let id = self.id.to_string();
            let bin = resolve_binary_path(self.default_binary);
            let sub = subcommand.to_string();
            std::thread::spawn(move || {
                if let Ok(queried) = query_agent_models(&bin, &sub)
                    && !queried.is_empty()
                    && let Ok(mut cache) = model_cache().lock()
                {
                    cache.insert(id, queried);
                }
            });
        }
    }

    #[note("agent-profile-agentprofile-set-cached-models")]
    pub fn set_cached_models(&self, models: Vec<ModelSpec>) {
        if let Ok(mut cache) = model_cache().lock() {
            cache.insert(self.id.to_string(), models);
        }
    }

    #[note("agent-profile-agentprofile-clear-cache")]
    pub fn clear_cache() {
        if let Ok(mut cache) = model_cache().lock() {
            cache.clear();
        }
        if let Ok(mut in_flight) = fetch_in_flight().lock() {
            in_flight.clear();
        }
    }
}

// -----------------------------------------------------------------------------
// Model Catalogs
// -----------------------------------------------------------------------------

#[note("agent-profile-claude-models")]
pub const CLAUDE_MODELS: &[StaticModelSpec] = &[
    StaticModelSpec {
        id: "sonnet",
        display_name: "Sonnet (Claude Default)",
        description: "Standard Frontier & Hybrid Reasoning (Anthropic)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "opus",
        display_name: "Opus (Deep Thinking)",
        description: "Frontier Deep Analysis (Anthropic)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "haiku",
        display_name: "Haiku (Fast & Compact)",
        description: "Lightweight High Speed (Anthropic)",
        supports_reasoning: false,
    },
];

#[note("agent-profile-gemini-agy-models")]
pub const GEMINI_AGY_MODELS: &[StaticModelSpec] = &[
    StaticModelSpec {
        id: "gemini-3.8-flash-high",
        display_name: "Gemini 3.8 Flash (High)",
        description: "Next-Gen 1M High Reasoning (Google)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "gemini-3.8-flash-medium",
        display_name: "Gemini 3.8 Flash (Medium)",
        description: "Next-Gen 1M Balanced Reasoning (Google)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "gemini-3.7-flash-high",
        display_name: "Gemini 3.7 Flash (High)",
        description: "Hybrid Reasoning 1M (Google)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "gemini-3.1-pro-high",
        display_name: "Gemini 3.1 Pro (High)",
        description: "Frontier 2M Reasoning (Google)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "claude-sonnet-4-6",
        display_name: "Claude Sonnet 4.6 (via agy)",
        description: "Anthropic Frontier Hosted via Antigravity",
        supports_reasoning: false,
    },
    StaticModelSpec {
        id: "claude-opus-4-6-thinking",
        display_name: "Claude Opus 4.6 Thinking (via agy)",
        description: "Anthropic Thinking Hosted via Antigravity",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "gpt-oss-120b-medium",
        display_name: "GPT-OSS 120B (via agy)",
        description: "Open-Weights MoE Balanced Reasoning",
        supports_reasoning: true,
    },
];

#[note("agent-profile-openai-models")]
pub const OPENAI_MODELS: &[StaticModelSpec] = &[
    StaticModelSpec {
        id: "gpt-5.6-sol",
        display_name: "GPT-5.6 Sol (Codex Default)",
        description: "OpenAI Flagship Coding Model",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "o3-mini",
        display_name: "o3-mini",
        description: "Fast Reasoning Engine (OpenAI)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "o1",
        display_name: "o1",
        description: "Deep Reasoning Flagship (OpenAI)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "gpt-4o",
        display_name: "GPT-4o",
        description: "Omni Multimodal Flagship (OpenAI)",
        supports_reasoning: false,
    },
    StaticModelSpec {
        id: "gpt-4o-mini",
        display_name: "GPT-4o Mini",
        description: "Fast Affordable (OpenAI)",
        supports_reasoning: false,
    },
];

#[note("agent-profile-opencode-models")]
pub const OPENCODE_MODELS: &[StaticModelSpec] = &[
    StaticModelSpec {
        id: "qwen2.5-coder:7b",
        display_name: "Qwen 2.5 Coder 7B",
        description: "Local Code Specialist (Alibaba)",
        supports_reasoning: false,
    },
    StaticModelSpec {
        id: "deepseek-coder-v2",
        display_name: "DeepSeek Coder v2",
        description: "Local MoE Architecture (DeepSeek)",
        supports_reasoning: true,
    },
    StaticModelSpec {
        id: "llama-3.3-70b",
        display_name: "Llama 3.3 70B",
        description: "Local Flagship (Meta)",
        supports_reasoning: true,
    },
];

#[note("agent-profile-profiles")]
pub const PROFILES: &[AgentProfile] = &[
    AgentProfile {
        id: "claude",
        registry_id: "claude-acp",
        display_name: "Claude Agent",
        default_binary: "claude",
        default_endpoint: "stdio:claude-agent",
        session: SessionSupport::Pinned,
        prompt_delivery: PromptDelivery::Stdin,
        prompt_via_stdin: true,
        model_source: ModelSource::Static(CLAUDE_MODELS),
    },
    AgentProfile {
        id: "gemini",
        registry_id: "antigravity-acp",
        display_name: "Gemini (Antigravity)",
        default_binary: "agy",
        default_endpoint: "stdio:agy",
        session: SessionSupport::Captured,
        prompt_delivery: PromptDelivery::BareStdin,
        prompt_via_stdin: true,
        model_source: ModelSource::AgentCommand {
            subcommand: "models",
            fallback: GEMINI_AGY_MODELS,
        },
    },
    AgentProfile {
        id: "codex",
        registry_id: "codex-acp",
        display_name: "OpenAI / Codex ACP Bridge",
        default_binary: "codex",
        default_endpoint: "stdio:codex",
        session: SessionSupport::Captured,
        prompt_delivery: PromptDelivery::Stdin,
        prompt_via_stdin: true,
        model_source: ModelSource::Static(OPENAI_MODELS),
    },
    AgentProfile {
        id: "opencode",
        registry_id: "opencode",
        display_name: "OpenCode / Local (Ollama)",
        default_binary: "opencode",
        default_endpoint: "stdio:opencode",
        session: SessionSupport::None,
        prompt_delivery: PromptDelivery::Stdin,
        prompt_via_stdin: true,
        model_source: ModelSource::Static(OPENCODE_MODELS),
    },
    AgentProfile {
        id: "custom",
        registry_id: "",
        display_name: "Custom ACP Server",
        default_binary: "",
        default_endpoint: "",
        session: SessionSupport::None,
        prompt_delivery: PromptDelivery::Stdin,
        prompt_via_stdin: true,
        model_source: ModelSource::Static(CLAUDE_MODELS),
    },
];

#[note("agent-profile-find-profile-by-id")]
pub fn find_profile_by_id(id: &str) -> Option<&'static AgentProfile> {
    PROFILES.iter().find(|p| p.id.eq_ignore_ascii_case(id))
}

#[note("agent-profile-find-profile-by-display-name")]
pub fn find_profile_by_display_name(name: &str) -> Option<&'static AgentProfile> {
    PROFILES
        .iter()
        .find(|p| p.display_name.eq_ignore_ascii_case(name))
}

#[note("agent-profile-find-profile-by-binary")]
pub fn find_profile_by_binary(binary: &str) -> Option<&'static AgentProfile> {
    PROFILES
        .iter()
        .find(|p| p.default_binary.eq_ignore_ascii_case(binary))
}

#[note("agent-profile-resolve-profile")]
pub fn resolve_profile(id_or_name: &str) -> &'static AgentProfile {
    find_profile_by_id(id_or_name)
        .or_else(|| find_profile_by_display_name(id_or_name))
        .or_else(|| find_profile_by_binary(id_or_name))
        .unwrap_or(&PROFILES[0])
}

#[note("agent-profile-profile-for")]
pub fn profile_for(agent_id: &str, provider: &str) -> &'static AgentProfile {
    find_profile_by_id(agent_id).unwrap_or_else(|| resolve_profile(provider))
}

#[note("agent-profile-query-agent-models")]
pub fn query_agent_models(binary: &Path, subcommand: &str) -> Result<Vec<ModelSpec>, String> {
    let mut cmd = crawrad_os::process::no_console(binary);
    cmd.args([subcommand]);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to spawn `{}`: {e}", binary.display()))?;

    if !output.status.success() {
        return Err("Command returned non-zero status".to_string());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut models = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let id = trimmed.split_whitespace().next().unwrap_or(trimmed);
        if id.is_empty() || id.eq_ignore_ascii_case("auto") {
            continue;
        }

        let is_reasoning = id.contains("thinking")
            || id.contains("high")
            || id.contains("reasoning")
            || id.contains("pro");

        models.push(ModelSpec {
            id: id.to_string(),
            display_name: id.to_string(),
            description: format!("Available via `{}`", binary.display()),
            supports_reasoning: is_reasoning,
        });
    }

    if models.is_empty() {
        Err("No models parsed from agent output".to_string())
    } else {
        Ok(models)
    }
}

#[note("agent-profile-build-agent-args")]
pub fn build_agent_args(
    profile: &AgentProfile,
    model_id: Option<&str>,
    session_id: Option<&str>,
    is_resume: bool,
) -> Vec<String> {
    if profile.id.eq_ignore_ascii_case("codex")
        || profile.default_binary.eq_ignore_ascii_case("codex")
    {
        let mut args = vec!["exec".to_string()];
        if is_resume && let Some(sid) = session_id {
            let s = sid.trim();
            if !s.is_empty() {
                args.push("resume".to_string());
                args.push(s.to_string());
            }
        }
        args.push("--json".to_string());
        if let Some(m) = model_id {
            let mut trimmed = m.trim().to_string();
            if trimmed.to_lowercase().starts_with("gtp-") {
                trimmed = format!("gpt-{}", &trimmed[4..]);
            }
            if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("auto") {
                args.push("--model".to_string());
                args.push(trimmed);
            }
        }
        return args;
    }

    let mut args = Vec::new();

    if profile.prompt_delivery == PromptDelivery::Stdin {
        args.push("-p".to_string());
        args.push("--output-format".to_string());
        args.push("text".to_string());
    } else {
        args.push("--output-format".to_string());
        args.push("json".to_string());
    }

    if let Some(m) = model_id {
        let mut trimmed = m.trim().to_string();
        if trimmed.to_lowercase().starts_with("gtp-") {
            trimmed = format!("gpt-{}", &trimmed[4..]);
        }
        if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("auto") {
            args.push("--model".to_string());
            args.push(trimmed);
        }
    }

    match profile.session {
        SessionSupport::Pinned => {
            if let Some(sid) = session_id {
                if is_resume {
                    args.push("--resume".to_string());
                } else {
                    args.push("--session-id".to_string());
                }
                args.push(sid.to_string());
            }
        }
        SessionSupport::Captured => {
            if let Some(sid) = session_id
                && is_resume
                && !sid.trim().is_empty()
            {
                args.push("--conversation".to_string());
                args.push(sid.to_string());
            }
        }
        SessionSupport::None => {}
    }

    args
}

#[note("agent-profile-build-agent-invocation-args")]
pub fn build_agent_invocation_args(
    profile: &AgentProfile,
    prompt: &str,
    model_id: Option<&str>,
    session_id: Option<&str>,
    is_resume: bool,
) -> (Vec<String>, String) {
    if profile.id.eq_ignore_ascii_case("codex")
        || profile.default_binary.eq_ignore_ascii_case("codex")
    {
        let mut args = vec!["exec".to_string()];
        if is_resume && let Some(sid) = session_id {
            let s = sid.trim();
            if !s.is_empty() {
                args.push("resume".to_string());
                args.push(s.to_string());
            }
        }
        args.push("--json".to_string());
        if let Some(m) = model_id {
            let mut trimmed = m.trim().to_string();
            if trimmed.to_lowercase().starts_with("gtp-") {
                trimmed = format!("gpt-{}", &trimmed[4..]);
            }
            if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("auto") {
                args.push("--model".to_string());
                args.push(trimmed);
            }
        }
        match profile.prompt_delivery {
            PromptDelivery::Stdin | PromptDelivery::BareStdin => {
                args.push("-".to_string());
                return (args, prompt.to_string());
            }
        }
    }

    let mut args = Vec::new();
    let stdin_data = match profile.prompt_delivery {
        PromptDelivery::Stdin => {
            args.push("-p".to_string());
            args.push("--output-format".to_string());
            args.push("text".to_string());
            prompt.to_string()
        }
        PromptDelivery::BareStdin => {
            args.push("--output-format".to_string());
            args.push("json".to_string());
            prompt.to_string()
        }
    };

    if let Some(m) = model_id {
        let mut trimmed = m.trim().to_string();
        if trimmed.to_lowercase().starts_with("gtp-") {
            trimmed = format!("gpt-{}", &trimmed[4..]);
        }
        if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("auto") {
            args.push("--model".to_string());
            args.push(trimmed);
        }
    }

    match profile.session {
        SessionSupport::Pinned => {
            if let Some(sid) = session_id {
                if is_resume {
                    args.push("--resume".to_string());
                } else {
                    args.push("--session-id".to_string());
                }
                args.push(sid.to_string());
            }
        }
        SessionSupport::Captured => {
            if let Some(sid) = session_id
                && is_resume
                && !sid.trim().is_empty()
            {
                args.push("--conversation".to_string());
                args.push(sid.to_string());
            }
        }
        SessionSupport::None => {}
    }

    (args, stdin_data)
}

#[note("agent-profile-resolve-binary-path")]
pub fn resolve_binary_path(binary: &str) -> PathBuf {
    let candidate = Path::new(binary);

    #[cfg(windows)]
    let is_valid_win_exec = |p: &Path| -> bool {
        if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            ext_lower == "exe" || ext_lower == "cmd" || ext_lower == "bat" || ext_lower == "com"
        } else {
            false
        }
    };

    #[cfg(windows)]
    if candidate.is_file() && is_valid_win_exec(candidate) {
        return candidate.to_path_buf();
    }
    #[cfg(not(windows))]
    if candidate.is_file() {
        return candidate.to_path_buf();
    }

    // 1. Search in PATH
    if let Some(path_var) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_var) {
            #[cfg(windows)]
            {
                let with_exe = dir.join(format!("{binary}.exe"));
                if with_exe.is_file() {
                    return with_exe;
                }
                let with_cmd = dir.join(format!("{binary}.cmd"));
                if with_cmd.is_file() {
                    return with_cmd;
                }
                let with_bat = dir.join(format!("{binary}.bat"));
                if with_bat.is_file() {
                    return with_bat;
                }
                let direct = dir.join(binary);
                if direct.is_file() && is_valid_win_exec(&direct) {
                    return direct;
                }
            }
            #[cfg(not(windows))]
            {
                let direct = dir.join(binary);
                if direct.is_file() {
                    return direct;
                }
            }
        }
    }

    // 2. Known standard user locations
    #[cfg(windows)]
    {
        if binary == "agy" {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                let p = PathBuf::from(local_app_data)
                    .join("agy")
                    .join("bin")
                    .join("agy.exe");
                if p.is_file() {
                    return p;
                }
            }
            if let Some(home) = std::env::var_os("USERPROFILE") {
                let p = PathBuf::from(home)
                    .join("AppData")
                    .join("Local")
                    .join("agy")
                    .join("bin")
                    .join("agy.exe");
                if p.is_file() {
                    return p;
                }
            }
        } else if binary == "claude" {
            if let Some(home) = std::env::var_os("USERPROFILE") {
                let home_path = PathBuf::from(home);
                let p = home_path.join(".local").join("bin").join("claude.exe");
                if p.is_file() {
                    return p;
                }
                let p_cmd = home_path.join(".local").join("bin").join("claude.cmd");
                if p_cmd.is_file() {
                    return p_cmd;
                }
            }
        } else if binary == "codex" {
            if let Some(app_data) = std::env::var_os("APPDATA") {
                let p = PathBuf::from(app_data).join("npm").join("codex.cmd");
                if p.is_file() {
                    return p;
                }
            }
            if let Some(home) = std::env::var_os("USERPROFILE") {
                let p = PathBuf::from(home)
                    .join("AppData")
                    .join("Roaming")
                    .join("npm")
                    .join("codex.cmd");
                if p.is_file() {
                    return p;
                }
            }
        }
    }

    #[cfg(not(windows))]
    {
        if let Some(home) = std::env::var_os("HOME") {
            let p = PathBuf::from(home).join(".local").join("bin").join(binary);
            if p.is_file() {
                return p;
            }
        }
    }

    PathBuf::from(binary)
}

#[note("agent-profile-build-platform-command")]
pub fn build_platform_command(bin_path: &Path, args: &[String]) -> Command {
    #[cfg(windows)]
    {
        if let Some(ext) = bin_path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            if ext_lower == "cmd" || ext_lower == "bat" {
                let mut cmd = crawrad_os::process::no_console("cmd.exe");
                cmd.arg("/c");
                cmd.arg(bin_path);
                cmd.args(args);
                return cmd;
            }
        }
    }
    let mut cmd = crawrad_os::process::no_console(bin_path);
    cmd.args(args);
    cmd
}
