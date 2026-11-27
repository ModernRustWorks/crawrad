use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Instant;

pub use super::agent_profile::{
    AgentProfile, CLAUDE_MODELS, GEMINI_AGY_MODELS, ModelSource, ModelSpec, OPENAI_MODELS,
    OPENCODE_MODELS, PROFILES, PromptDelivery, SessionSupport, StaticModelSpec, build_agent_args,
    build_agent_invocation_args, build_platform_command, find_profile_by_display_name,
    find_profile_by_id, query_agent_models, resolve_binary_path, resolve_profile,
};
use crawrad_note::note;

#[note("acp-acpmodelinfo")]
pub type AcpModelInfo = ModelSpec;

#[note("acp-cliinvocation")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliInvocation {
    pub binary: String,
    pub args: Vec<String>,
    pub stdin_data: String,
    pub cwd: Option<PathBuf>,
}

#[note("acp-build-cli-args-for-session")]
pub fn build_cli_args_for_session(
    binary: &str,
    prompt: &str,
    model: Option<&str>,
    session: Option<(&str, bool)>,
    support: SessionSupport,
) -> CliInvocation {
    let profile = resolve_profile(binary);
    let mut invocation = build_cli_invocation_with_options(
        binary,
        prompt,
        model,
        session,
        support,
        profile.prompt_delivery,
        None,
    );
    invocation.binary = binary.to_string();
    invocation
}

#[note("acp-build-cli-invocation-with-options")]
pub fn build_cli_invocation_with_options(
    binary: &str,
    prompt: &str,
    model: Option<&str>,
    session: Option<(&str, bool)>,
    support: SessionSupport,
    delivery: PromptDelivery,
    cwd: Option<PathBuf>,
) -> CliInvocation {
    let mut prof = resolve_profile(binary).clone();
    prof.session = support;
    prof.prompt_delivery = delivery;

    let (session_id, is_resume) = match session {
        Some((sid, resume)) => (Some(sid), resume),
        None => (None, false),
    };

    let (args, stdin_data) =
        build_agent_invocation_args(&prof, prompt, model, session_id, is_resume);

    CliInvocation {
        binary: binary.to_string(),
        args,
        stdin_data,
        cwd,
    }
}

#[note("acp-execute-invocation")]
pub fn execute_invocation(
    invocation: &CliInvocation,
    resolve_bin: impl Fn(&str) -> PathBuf,
) -> Result<String, String> {
    let bin_path = resolve_bin(&invocation.binary);
    let mut cmd = build_platform_command(&bin_path, &invocation.args);
    if let Some(ref cwd) = invocation.cwd {
        cmd.current_dir(cwd);
    }
    if !invocation.stdin_data.is_empty() {
        cmd.stdin(Stdio::piped());
    } else {
        cmd.stdin(Stdio::null());
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to execute `{}` (path: {}): {e}",
            invocation.binary,
            bin_path.display()
        )
    })?;

    if !invocation.stdin_data.is_empty()
        && let Some(mut stdin) = child.stdin.take()
    {
        stdin
            .write_all(invocation.stdin_data.as_bytes())
            .map_err(|e| {
                format!(
                    "Failed to write prompt to `{}` stdin: {e}",
                    invocation.binary
                )
            })?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for `{}`: {e}", invocation.binary))?;

    let stdout_raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr_raw = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if !output.status.success() {
        let err_msg = if !stderr_raw.is_empty() {
            stderr_raw
        } else if !stdout_raw.is_empty() {
            stdout_raw
        } else {
            format!("Process exited with status {}", output.status)
        };
        return Err(format!("`{}` error: {err_msg}", invocation.binary));
    }

    if !stderr_raw.is_empty() && stderr_raw.to_lowercase().contains("warning: conversation") {
        Ok(format!("{stderr_raw}\n{stdout_raw}"))
    } else {
        Ok(stdout_raw)
    }
}

#[note("acp-acpflag")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpFlag {
    AutoApproveReads,
    NativeTools,
}

impl AcpFlag {
    #[note("acp-acpflag-all")]
    pub const ALL: [Self; 2] = [Self::AutoApproveReads, Self::NativeTools];

    #[note("acp-acpflag-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::AutoApproveReads => "Approve read operations without asking",
            Self::NativeTools => "Let the agent use its own tools (off by default)",
        }
    }

    #[note("acp-acpflag-widget")]
    pub fn widget(self) -> &'static str {
        match self {
            Self::AutoApproveReads => "opt_ai_auto_approve",
            Self::NativeTools => "opt_ai_native_tools",
        }
    }

    #[note("acp-acpflag-get")]
    pub fn get(self, c: &AcpConfig) -> bool {
        match self {
            Self::AutoApproveReads => c.auto_approve_reads,
            Self::NativeTools => c.allow_native_agent_tools,
        }
    }

    #[note("acp-acpflag-field")]
    pub fn field(self, c: &mut AcpConfig) -> &mut bool {
        match self {
            Self::AutoApproveReads => &mut c.auto_approve_reads,
            Self::NativeTools => &mut c.allow_native_agent_tools,
        }
    }
}

#[note("acp-acpconfig")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcpConfig {
    /// Stable canonical agent identifier (e.g. "claude", "gemini", "codex").
    #[serde(default)]
    pub agent_id: String,
    /// Active provider display name (e.g. "Claude Agent", "Gemini (Antigravity)").
    pub provider: String,
    /// Default model identifier (e.g. "sonnet", "gemini-3.8-flash-high").
    pub model: String,
    /// ACP server executable command or network endpoint URL.
    pub endpoint: String,
    /// Arguments passed to the ACP server executable command.
    #[serde(default)]
    pub endpoint_args: Vec<String>,
    /// Whether to auto-approve read operations requested by the agent without confirmation dialogs.
    pub auto_approve_reads: bool,
    /// Whether to auto-approve implementation plans without manual confirmation.
    pub auto_approve_plans: bool,
    /// Whether extended thinking capability is enabled during query execution.
    pub enable_thinking: bool,
    /// Whether native CLI agent tools are allowed (default: false, strict security per ADR 0029).
    #[serde(default)]
    pub allow_native_agent_tools: bool,
    /// Mode for reviewing and applying agent code edits (wait_for_approval, show_and_apply, apply_directly).
    #[serde(default)]
    pub diff_review_mode: super::diff_review::DiffReviewMode,
    /// Saved model selection history per provider.
    #[serde(default)]
    pub models_per_provider: HashMap<String, String>,
    /// Custom models list per provider.
    #[serde(default)]
    pub custom_models_by_provider: HashMap<String, Vec<String>>,
    /// Custom models list for third-party or custom ACP servers (legacy).
    #[serde(default)]
    pub custom_models: Vec<String>,
    /// Saved configuration option choices per agent (agent_id -> config_id -> value).
    #[serde(default)]
    pub session_choices: HashMap<String, HashMap<String, String>>,
    /// Preserved unknown JSON keys from third-party writers (read-modify-write).
    #[serde(skip)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

impl Default for AcpConfig {
    fn default() -> Self {
        Self {
            agent_id: "claude".to_string(),
            provider: "Claude Agent".to_string(),
            model: "auto".to_string(),
            endpoint: "stdio:claude-agent".to_string(),
            endpoint_args: Vec::new(),
            auto_approve_reads: true,
            auto_approve_plans: false,
            enable_thinking: true,
            allow_native_agent_tools: false,
            diff_review_mode: super::diff_review::DiffReviewMode::WaitForApproval,
            models_per_provider: HashMap::new(),
            custom_models_by_provider: HashMap::new(),
            custom_models: Vec::new(),
            session_choices: HashMap::new(),
            extra_fields: HashMap::new(),
        }
    }
}

#[cfg(test)]
static CONFIG_OVERRIDE_DIR: std::sync::RwLock<Option<PathBuf>> = std::sync::RwLock::new(None);

impl AcpConfig {
    #[note("acp-acpconfig-record-session-choice")]
    pub fn record_session_choice(&mut self, config_id: &str, value: &str) {
        self.session_choices
            .entry(self.agent_id.clone())
            .or_default()
            .insert(config_id.to_string(), value.to_string());
    }

    pub const PROVIDERS: &'static [&'static str] = &[
        "Claude Agent",
        "Gemini (Antigravity)",
        "OpenAI / Codex ACP Bridge",
        "OpenCode / Local (Ollama)",
        "Custom ACP Server",
    ];

    #[note("acp-acpconfig-provider-labels")]
    pub fn provider_labels() -> Vec<String> {
        Self::PROVIDERS.iter().map(|s| s.to_string()).collect()
    }

    #[note("acp-acpconfig-provider-index")]
    pub fn provider_index(&self) -> Option<usize> {
        Self::PROVIDERS.iter().position(|p| *p == self.provider)
    }

    pub const MODELS: &'static [&'static str] = &[
        "auto",
        "sonnet",
        "opus",
        "haiku",
        "gemini-3.8-flash-high",
        "gemini-3.8-flash-medium",
        "gemini-3.7-flash-high",
        "gemini-3.1-pro-high",
        "claude-sonnet-4-6",
        "claude-opus-4-6-thinking",
        "gpt-oss-120b-medium",
    ];

    #[note("acp-acpconfig-models-for-provider-and-endpoint")]
    pub fn models_for_provider_and_endpoint(
        &self,
        provider: &str,
        _endpoint: &str,
    ) -> Vec<AcpModelInfo> {
        let profile = resolve_profile(provider);
        let mut list = Vec::new();
        list.push(ModelSpec {
            id: "auto".to_string(),
            display_name: "Auto (Agent Default)".to_string(),
            description: "Default model configured in agent account".to_string(),
            supports_reasoning: true,
        });

        for m in profile.models() {
            if !m.id.eq_ignore_ascii_case("auto")
                && !list
                    .iter()
                    .any(|existing| existing.id.eq_ignore_ascii_case(&m.id))
            {
                list.push(m);
            }
        }

        if let Some(custom_list) = self.custom_models_by_provider.get(provider) {
            for cm in custom_list {
                if !cm.eq_ignore_ascii_case("auto")
                    && !list
                        .iter()
                        .any(|existing| existing.id.eq_ignore_ascii_case(cm))
                {
                    list.push(ModelSpec {
                        id: cm.clone(),
                        display_name: cm.clone(),
                        description: "Custom Model".into(),
                        supports_reasoning: false,
                    });
                }
            }
        }

        if profile.id == "custom" || profile.id == "opencode" {
            for cm in &self.custom_models {
                if !cm.eq_ignore_ascii_case("auto")
                    && !list
                        .iter()
                        .any(|existing| existing.id.eq_ignore_ascii_case(cm))
                {
                    list.push(ModelSpec {
                        id: cm.clone(),
                        display_name: cm.clone(),
                        description: "Custom Model".into(),
                        supports_reasoning: false,
                    });
                }
            }
        }

        list
    }

    #[note("acp-acpconfig-add-custom-model-for-provider")]
    pub fn add_custom_model_for_provider(&mut self, provider: &str, model_id: &str) {
        let trimmed = model_id.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("auto") {
            return;
        }
        let entry = self
            .custom_models_by_provider
            .entry(provider.to_string())
            .or_default();
        if !entry.iter().any(|m| m.eq_ignore_ascii_case(trimmed)) {
            entry.push(trimmed.to_string());
        }
        self.model = trimmed.to_string();
        self.models_per_provider
            .insert(provider.to_string(), trimmed.to_string());
    }

    #[note("acp-acpconfig-remove-model-for-provider")]
    pub fn remove_model_for_provider(&mut self, provider: &str, model_id: &str) {
        if model_id.eq_ignore_ascii_case("auto") {
            return;
        }
        if let Some(entry) = self.custom_models_by_provider.get_mut(provider) {
            entry.retain(|m| !m.eq_ignore_ascii_case(model_id));
        }
        self.custom_models
            .retain(|m| !m.eq_ignore_ascii_case(model_id));
        let available = self.models_for_provider_and_endpoint(provider, &self.endpoint);
        if self.model.eq_ignore_ascii_case(model_id) {
            self.model = available
                .first()
                .map(|m| m.id.clone())
                .unwrap_or_else(|| "auto".to_string());
            self.models_per_provider
                .insert(provider.to_string(), self.model.clone());
        }
    }

    #[note("acp-acpconfig-models-for-active-provider")]
    pub fn models_for_active_provider(&self) -> Vec<AcpModelInfo> {
        self.models_for_provider_and_endpoint(&self.provider, &self.endpoint)
    }

    #[note("acp-acpconfig-model-at-position")]
    pub fn model_at_position(&self, i: usize) -> Option<String> {
        self.models_for_active_provider()
            .get(i)
            .map(|m| m.id.clone())
    }

    #[note("acp-acpconfig-refresh-active-models-async")]
    pub fn refresh_active_models_async(&self) {
        let profile = resolve_profile(&self.provider);
        profile.refresh_models_in_background();
    }

    #[note("acp-acpconfig-switch-provider")]
    pub fn switch_provider(&mut self, new_provider: &str) {
        if !self.provider.is_empty() && !self.model.is_empty() {
            self.models_per_provider
                .insert(self.provider.clone(), self.model.clone());
        }

        let prof = resolve_profile(new_provider);
        self.agent_id = prof.id.to_string();
        self.provider = prof.display_name.to_string();
        self.endpoint = prof.default_endpoint.to_string();

        if let Some(saved_model) = self.models_per_provider.get(self.provider.as_str()) {
            self.model = saved_model.clone();
        } else {
            self.model = "auto".to_string();
        }
    }

    #[note("acp-acpconfig-test-connection")]
    pub fn test_connection(&self) -> Result<String, String> {
        Self::test_connection_for(&self.provider, &self.endpoint)
    }

    #[note("acp-acpconfig-test-connection-for")]
    pub fn test_connection_for(provider: &str, endpoint: &str) -> Result<String, String> {
        let profile = resolve_profile(provider);

        if !profile.default_binary.is_empty() {
            Self::execute_version_probe(profile.default_binary)
        } else {
            let endpoint_trimmed = endpoint.trim();
            if endpoint_trimmed.is_empty() {
                return Err("ACP endpoint is not configured".to_string());
            }

            let bin_name = endpoint_trimmed
                .strip_prefix("stdio:")
                .unwrap_or(endpoint_trimmed)
                .split_whitespace()
                .next()
                .unwrap_or(endpoint_trimmed);

            let bin_path = resolve_binary_path(bin_name);
            let mut cmd = build_platform_command(&bin_path, &["--version".to_string()]);
            cmd.stdin(Stdio::null());

            match cmd.output() {
                Ok(out) if out.status.success() => {
                    let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    Ok(format!("🟢 Connected to `{provider}` ({ver}) · Ready"))
                }
                Ok(out) => {
                    let err = String::from_utf8_lossy(&out.stderr);
                    Err(format!("`{bin_name}` returned error: {}", err.trim()))
                }
                Err(e) => Err(format!(
                    "Agent executable `{bin_name}` ({provider}) not found or not in PATH: {e}"
                )),
            }
        }
    }

    #[note("acp-acpconfig-execute-version-probe")]
    pub fn execute_version_probe(binary: &str) -> Result<String, String> {
        let start = Instant::now();
        let bin_path = resolve_binary_path(binary);
        let mut cmd = build_platform_command(&bin_path, &["--version".to_string()]);
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let out = cmd.output().map_err(|e| {
            format!(
                "Agent CLI `{binary}` not found at {}: {e}",
                bin_path.display()
            )
        })?;

        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("`{binary}` returned error: {}", err.trim()));
        }

        let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let elapsed = start.elapsed().as_millis();
        Ok(format!(
            "🟢 Connected to `{binary}` ({ver}) · {elapsed}ms · Ready"
        ))
    }

    #[note("acp-acpconfig-resolve-cli-path")]
    pub fn resolve_cli_path(binary: &str) -> PathBuf {
        resolve_binary_path(binary)
    }

    #[note("acp-acpconfig-execute-prompt")]
    pub fn execute_prompt(&self, prompt: &str) -> Result<String, String> {
        self.execute_prompt_with_session(prompt, None)
    }

    #[note("acp-acpconfig-create-acp-client")]
    pub fn create_acp_client(
        &self,
        cwd: Option<&Path>,
    ) -> Result<super::client::AcpClient, String> {
        self.create_acp_client_with_permissions(cwd, None)
    }

    #[note("acp-acpconfig-create-acp-client-with-permissions")]
    pub fn create_acp_client_with_permissions(
        &self,
        cwd: Option<&Path>,
        perms: Option<std::sync::Arc<std::sync::Mutex<super::permission::SessionPermissions>>>,
    ) -> Result<super::client::AcpClient, String> {
        let profile =
            find_profile_by_id(&self.agent_id).unwrap_or_else(|| resolve_profile(&self.provider));

        let (bin_name, args) = if profile.id == "custom" {
            if !self.endpoint_args.is_empty() {
                let prog = self
                    .endpoint
                    .strip_prefix("stdio:")
                    .unwrap_or(&self.endpoint)
                    .trim();
                if prog.is_empty() {
                    return Err("Custom ACP server endpoint is empty".to_string());
                }
                (prog.to_string(), self.endpoint_args.clone())
            } else {
                let parts: Vec<&str> = self.endpoint.split_whitespace().collect();
                if parts.is_empty() {
                    return Err("Custom ACP server endpoint is empty".to_string());
                }
                let prog = parts[0].strip_prefix("stdio:").unwrap_or(parts[0]);
                (
                    prog.to_string(),
                    parts[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                )
            }
        } else {
            let reg_id = profile.registry_id;
            let entry = super::acp_registry::registry_agent(reg_id).ok_or_else(|| {
                format!(
                    "{} is not installed — Options › AI Assistant › ACP catalog › Install",
                    profile.display_name
                )
            })?;
            let launch = super::acp_registry::launch_for(
                &entry,
                super::acp_registry::current_platform(),
                &super::acp_registry::agents_dir(),
            )
            .map_err(|_| {
                format!(
                    "{} is not installed — Options › AI Assistant › ACP catalog › Install",
                    profile.display_name
                )
            })?;
            if entry.distribution_kind() == Some("binary") && !Path::new(&launch.program).is_file()
            {
                return Err(format!(
                    "{} is not installed — Options › AI Assistant › ACP catalog › Install",
                    profile.display_name
                ));
            }
            (launch.program, launch.args)
        };

        let bin_path = resolve_binary_path(&bin_name);
        super::client::AcpClient::spawn_full(
            &bin_path,
            &args,
            cwd,
            self.auto_approve_reads,
            self.allow_native_agent_tools,
            perms,
        )
        .inspect(|client| client.set_review_edits(self.reviews_edits()))
    }

    #[note("acp-acpconfig-reviews-edits")]
    pub fn reviews_edits(&self) -> bool {
        self.diff_review_mode == super::diff_review::DiffReviewMode::WaitForApproval
    }

    #[note("acp-acpconfig-execute-prompt-with-session")]
    pub fn execute_prompt_with_session(
        &self,
        prompt: &str,
        session: Option<(&str, bool)>,
    ) -> Result<String, String> {
        self.execute_prompt_with_session_and_cwd(prompt, session, None)
    }

    #[note("acp-acpconfig-build-invocation-for-prompt")]
    pub fn build_invocation_for_prompt(
        &self,
        prompt: &str,
        session: Option<(&str, bool)>,
        cwd: Option<&Path>,
    ) -> Result<CliInvocation, String> {
        let profile =
            find_profile_by_id(&self.agent_id).unwrap_or_else(|| resolve_profile(&self.provider));

        let (binary, extra_args) = if profile.id == "custom" {
            let parts: Vec<&str> = self.endpoint.split_whitespace().collect();
            if parts.is_empty() {
                return Err("Custom ACP server endpoint is empty".to_string());
            }
            (
                parts[0].to_string(),
                parts[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            )
        } else {
            if profile.default_binary.is_empty() {
                return Err(format!(
                    "Direct prompt execution not supported for provider `{}`.",
                    self.provider
                ));
            }
            (profile.default_binary.to_string(), Vec::new())
        };

        let mut invocation = build_cli_invocation_with_options(
            &binary,
            prompt,
            Some(&self.model),
            session,
            profile.session,
            profile.prompt_delivery,
            cwd.map(PathBuf::from),
        );
        if !extra_args.is_empty() {
            let mut combined = extra_args;
            combined.append(&mut invocation.args);
            invocation.args = combined;
        }
        Ok(invocation)
    }

    #[note("acp-acpconfig-execute-prompt-with-session-and-cwd")]
    pub fn execute_prompt_with_session_and_cwd(
        &self,
        prompt: &str,
        session: Option<(&str, bool)>,
        cwd: Option<&Path>,
    ) -> Result<String, String> {
        let invocation = self.build_invocation_for_prompt(prompt, session, cwd)?;
        execute_invocation(&invocation, resolve_binary_path)
    }

    #[note("acp-acpconfig-execute-cli-query")]
    pub fn execute_cli_query(binary: &str, prompt: &str) -> Result<String, String> {
        Self::execute_cli_query_with_model(binary, prompt, None)
    }

    #[note("acp-acpconfig-execute-cli-query-with-model")]
    pub fn execute_cli_query_with_model(
        binary: &str,
        prompt: &str,
        model: Option<&str>,
    ) -> Result<String, String> {
        let invocation =
            build_cli_args_for_session(binary, prompt, model, None, SessionSupport::None);
        execute_invocation(&invocation, resolve_binary_path)
    }

    #[note("acp-acpconfig-test-live-ping")]
    pub fn test_live_ping(&self) -> Result<String, String> {
        let profile =
            find_profile_by_id(&self.agent_id).unwrap_or_else(|| resolve_profile(&self.provider));

        if profile.default_binary.is_empty() {
            return Err(format!(
                "Live ping only supported for executable agent backends (provider: {})",
                self.provider
            ));
        }

        let start = Instant::now();
        let resp = Self::execute_cli_query(profile.default_binary, "Respond only with: 'PONG ok'")?;
        let elapsed = start.elapsed().as_millis();
        Ok(format!(
            "🟢 Live ping ok via `{}` (response: '{resp}') · {elapsed}ms",
            profile.default_binary
        ))
    }

    #[cfg(test)]
    pub fn set_config_dir_override(dir: Option<PathBuf>) {
        if let Ok(mut lock) = CONFIG_OVERRIDE_DIR.write() {
            *lock = dir;
        }
    }

    fn active_appdata_dir() -> Option<PathBuf> {
        #[cfg(test)]
        if let Ok(lock) = CONFIG_OVERRIDE_DIR.read()
            && let Some(ref dir) = *lock
        {
            return Some(dir.clone());
        }
        std::env::var_os("APPDATA").map(PathBuf::from)
    }

    /// Returns the configuration file path for writing.
    /// Always points to CrawRAD's configuration path, never to legacy Aefos.
    pub fn config_path_for_write() -> PathBuf {
        Self::config_path_for_write_from(Self::active_appdata_dir().as_deref())
    }

    /// Returns the configuration file path for reading.
    /// If CrawRAD config does not exist but legacy Aefos config exists, returns legacy Aefos path.
    pub fn config_path_for_read() -> PathBuf {
        Self::config_path_for_read_from(Self::active_appdata_dir().as_deref())
    }

    pub fn config_path_for_write_from(app_data: Option<&Path>) -> PathBuf {
        if let Some(base) = app_data {
            base.join("CrawRAD").join("acp_config.json")
        } else if let Some(home) =
            std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))
        {
            PathBuf::from(home).join(".crawrad").join("acp_config.json")
        } else {
            PathBuf::from("acp_config.json")
        }
    }

    pub fn config_path_for_read_from(app_data: Option<&Path>) -> PathBuf {
        if let Some(base) = app_data {
            let crawrad = base.join("CrawRAD").join("acp_config.json");
            if !crawrad.exists() {
                let legacy = base.join("Aefos").join("acp_config.json");
                if legacy.exists() {
                    return legacy;
                }
            }
            crawrad
        } else if let Some(home) =
            std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))
        {
            let base = PathBuf::from(home);
            let crawrad = base.join(".crawrad").join("acp_config.json");
            if !crawrad.exists() {
                let legacy = base.join(".aefos").join("acp_config.json");
                if legacy.exists() {
                    return legacy;
                }
            }
            crawrad
        } else {
            PathBuf::from("acp_config.json")
        }
    }

    /// Legacy alias for `config_path_for_read`.
    pub fn default_path() -> PathBuf {
        Self::config_path_for_read()
    }

    pub fn load() -> Self {
        Self::load_from_file(&Self::config_path_for_read()).unwrap_or_default()
    }

    #[note("acp-acpconfig-load-from-file")]
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let val: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let mut cfg = Self::default();
        let mut extra_fields = HashMap::new();

        if let serde_json::Value::Object(map) = val {
            for (k, v) in map {
                match k.as_str() {
                    "agent_id" => {
                        if let Some(s) = v.as_str() {
                            cfg.agent_id = s.to_string();
                        }
                    }
                    "provider" => {
                        if let Some(s) = v.as_str() {
                            cfg.provider = s.to_string();
                        }
                    }
                    "model" => {
                        if let Some(s) = v.as_str() {
                            cfg.model = s.to_string();
                        }
                    }
                    "endpoint" => {
                        if let Some(s) = v.as_str() {
                            cfg.endpoint = s.to_string();
                        }
                    }
                    "endpoint_args" => {
                        if let Ok(args) = serde_json::from_value(v) {
                            cfg.endpoint_args = args;
                        }
                    }
                    "auth_token" => {
                        // Ignored per ADR 0029 (plaintext tokens eliminated)
                    }
                    "auto_approve_reads" => {
                        if let Some(b) = v.as_bool() {
                            cfg.auto_approve_reads = b;
                        }
                    }
                    "auto_approve_plans" => {
                        if let Some(b) = v.as_bool() {
                            cfg.auto_approve_plans = b;
                        }
                    }
                    "enable_thinking" => {
                        if let Some(b) = v.as_bool() {
                            cfg.enable_thinking = b;
                        }
                    }
                    "allow_native_agent_tools" => {
                        if let Some(b) = v.as_bool() {
                            cfg.allow_native_agent_tools = b;
                        }
                    }
                    "diff_review_mode" => {
                        if let Some(s) = v.as_str() {
                            cfg.diff_review_mode = super::diff_review::DiffReviewMode::parse(s);
                        }
                    }
                    "models_per_provider" => {
                        if let Ok(m) = serde_json::from_value(v) {
                            cfg.models_per_provider = m;
                        }
                    }
                    "custom_models_by_provider" => {
                        if let Ok(c) = serde_json::from_value(v) {
                            cfg.custom_models_by_provider = c;
                        }
                    }
                    "custom_models" => {
                        if let Ok(c) = serde_json::from_value(v) {
                            cfg.custom_models = c;
                        }
                    }
                    _ => {
                        extra_fields.insert(k, v);
                    }
                }
            }
        }
        cfg.extra_fields = extra_fields;

        if cfg.agent_id.is_empty() && !cfg.provider.is_empty() {
            let prof = resolve_profile(&cfg.provider);
            cfg.agent_id = prof.id.to_string();
        } else if !cfg.agent_id.is_empty()
            && let Some(prof) = find_profile_by_id(&cfg.agent_id)
        {
            cfg.provider = prof.display_name.to_string();
        }

        Ok(cfg)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.save_to_file(&Self::config_path_for_write())
    }

    #[note("acp-acpconfig-save-to-file")]
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 1. Read existing config from disk if present to preserve third-party keys
        let mut root_val = if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                serde_json::from_str::<serde_json::Value>(&content)
                    .unwrap_or_else(|_| serde_json::json!({}))
            } else {
                serde_json::json!({})
            }
        } else {
            serde_json::json!({})
        };

        if !root_val.is_object() {
            root_val = serde_json::json!({});
        }

        // Preserve all existing extra fields
        for (k, v) in &self.extra_fields {
            root_val[k] = v.clone();
        }

        // Merge our known properties
        root_val["agent_id"] = serde_json::Value::String(self.agent_id.clone());
        root_val["provider"] = serde_json::Value::String(self.provider.clone());
        root_val["model"] = serde_json::Value::String(self.model.clone());
        root_val["endpoint"] = serde_json::Value::String(self.endpoint.clone());
        root_val["endpoint_args"] =
            serde_json::to_value(&self.endpoint_args).unwrap_or_else(|_| serde_json::json!([]));
        if let serde_json::Value::Object(ref mut map) = root_val {
            map.remove("auth_token");
        }
        root_val["auto_approve_reads"] = serde_json::Value::Bool(self.auto_approve_reads);
        root_val["auto_approve_plans"] = serde_json::Value::Bool(self.auto_approve_plans);
        root_val["enable_thinking"] = serde_json::Value::Bool(self.enable_thinking);
        root_val["allow_native_agent_tools"] =
            serde_json::Value::Bool(self.allow_native_agent_tools);
        root_val["diff_review_mode"] =
            serde_json::Value::String(self.diff_review_mode.as_str().to_string());
        root_val["models_per_provider"] = serde_json::to_value(&self.models_per_provider)
            .unwrap_or_else(|_| serde_json::json!({}));
        root_val["custom_models_by_provider"] =
            serde_json::to_value(&self.custom_models_by_provider)
                .unwrap_or_else(|_| serde_json::json!({}));
        root_val["custom_models"] =
            serde_json::to_value(&self.custom_models).unwrap_or_else(|_| serde_json::json!([]));

        let json = serde_json::to_string_pretty(&root_val).map_err(std::io::Error::other)?;

        // 2. Atomic write via temporary file
        let tmp_path = path.with_extension(format!("tmp.{}", std::process::id()));
        std::fs::write(&tmp_path, json.as_bytes())?;

        #[cfg(windows)]
        {
            if path.exists() {
                let _ = std::fs::remove_file(path);
            }
        }

        std::fs::rename(&tmp_path, path)?;
        Ok(())
    }
}

#[note("acp-format-model-label-pure")]
pub fn format_model_label_pure(model: &str) -> String {
    match model {
        "auto" | "" => "Auto ▾".to_string(),
        "claude-3-7-sonnet" | "sonnet" => "Sonnet 3.7 ▾".to_string(),
        "claude-sonnet-4-6" => "Sonnet 4.6 ▾".to_string(),
        "claude-opus-4-7" | "opus" => "Opus 4.7 ▾".to_string(),
        "claude-3-5-haiku" | "haiku" => "Haiku 3.5 ▾".to_string(),
        "Claude-Opus-5" => "Opus 5 ▾".to_string(),
        "Claude-Sonnet-5" => "Sonnet 5 ▾".to_string(),
        "Claude-Fable-5-1" => "Fable 5.1 ▾".to_string(),
        "o3-mini" => "o3-mini ▾".to_string(),
        "o1" => "o1 ▾".to_string(),
        "gpt-4o" => "GPT-4o ▾".to_string(),
        "gpt-4o-mini" => "GPT-4o mini ▾".to_string(),
        "gemini-2.5-pro" => "Gemini 2.5 ▾".to_string(),
        "gemini-2.5-flash" => "Gemini 2.5 Flash ▾".to_string(),
        "gemini-3.8-flash" => "Gemini 3.8 ▾".to_string(),
        "qwen2.5-coder:7b" => "Qwen 2.5 ▾".to_string(),
        "deepseek-coder-v2:16b" => "DeepSeek V2 ▾".to_string(),
        m if m.chars().count() > 14 => {
            let prefix: String = m.chars().take(12).collect();
            format!("{prefix}… ▾")
        }
        m => format!("{m} ▾"),
    }
}

#[cfg(test)]
#[path = "test/acp/test.rs"]
mod tests;
