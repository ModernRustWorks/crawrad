//! The Build Manager's settings (moved out of the shell): the
//! targets, the profile, the cargo verb, the dialog's mode and tab, and the
//! state the dialog edits. Plain data — the window draws it.

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    Win32,
    Win64,
    Linux64,
    MacIntel,
    MacSilicon,
    AndroidArm64,
    IosArm64,
    Wasm32,
}

impl TargetPlatform {
    pub const ALL: &'static [Self] = &[
        Self::Win32,
        Self::Win64,
        Self::Linux64,
        Self::MacIntel,
        Self::MacSilicon,
        Self::AndroidArm64,
        Self::IosArm64,
        Self::Wasm32,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Win32 => "Windows 32-bit (x86)",
            Self::Win64 => "Windows 64-bit (x64)",
            Self::Linux64 => "Linux 64-bit (x64)",
            Self::MacIntel => "macOS (Intel)",
            Self::MacSilicon => "macOS (Apple Silicon)",
            Self::AndroidArm64 => "Android (ARM64)",
            Self::IosArm64 => "iOS (ARM64)",
            Self::Wasm32 => "WebAssembly (wasm32)",
        }
    }

    pub fn triple(self) -> &'static str {
        match self {
            Self::Win32 => "i686-pc-windows-msvc",
            Self::Win64 => "x86_64-pc-windows-msvc",
            Self::Linux64 => "x86_64-unknown-linux-gnu",
            Self::MacIntel => "x86_64-apple-darwin",
            Self::MacSilicon => "aarch64-apple-darwin",
            Self::AndroidArm64 => "aarch64-linux-android",
            Self::IosArm64 => "aarch64-apple-ios",
            Self::Wasm32 => "wasm32-unknown-unknown",
        }
    }

    #[note("build-settings-targetplatform-icon")]
    pub fn icon(self) -> &'static str {
        match self {
            Self::Win32 | Self::Win64 => "⊞",
            Self::Linux64 => "🐧",
            Self::MacIntel | Self::MacSilicon => "🍎",
            Self::AndroidArm64 => "🤖",
            Self::IosArm64 => "📱",
            Self::Wasm32 => "🌐",
        }
    }

    #[note("build-settings-targetplatform-check-list")]
    pub fn check_list(selected: &[Self]) -> (Vec<String>, Vec<String>, Vec<bool>) {
        let rows = Self::ALL
            .iter()
            .map(|t| format!("{}  {}", t.icon(), t.label()));
        let triples = Self::ALL.iter().map(|t| t.triple().to_string());
        let on = Self::ALL.iter().map(|t| selected.contains(t));
        (rows.collect(), triples.collect(), on.collect())
    }

    #[note("build-settings-targetplatform-set-in")]
    pub fn set_in(selected: &mut Vec<Self>, index: usize, on: bool) {
        let Some(&t) = Self::ALL.get(index) else {
            return;
        };
        selected.retain(|&x| x != t);
        if on {
            selected.push(t);
        }
    }

    pub fn chip_label(self) -> &'static str {
        match self {
            Self::Win32 => "Windows 32-bit (x86)",
            Self::Win64 => "Windows 64-bit (x64)",
            Self::Linux64 => "Linux 64-bit (x64)",
            Self::MacIntel => "macOS (Intel)",
            Self::MacSilicon => "macOS (Apple Silicon)",
            Self::AndroidArm64 => "Android (ARM64)",
            Self::IosArm64 => "iOS (ARM64)",
            Self::Wasm32 => "WebAssembly (wasm32)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildProfile {
    #[default]
    Debug,
    Release,
}

impl BuildProfile {
    pub fn label(self) -> &'static str {
        match self {
            Self::Debug => "Debug",
            Self::Release => "Release",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Debug => Self::Release,
            Self::Release => Self::Debug,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CargoCommand {
    #[default]
    Build,
    Check,
    Clean,
}

impl CargoCommand {
    pub fn cargo_verb(&self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Check => "check",
            Self::Clean => "clean",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildMode {
    #[default]
    Build,
    Run,
    Check,
    Clean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildTab {
    #[default]
    Output,
    Warnings,
    Errors,
}

use std::time::{Duration, Instant};

use crate::build_progress::BuildProgress;

#[note("build-settings-already-building")]
pub const ALREADY_BUILDING: &str = "already building — wait for it to finish";

#[note("build-settings-refusal-hold")]
pub const REFUSAL_HOLD: Duration = Duration::from_secs(3);
use crawrad_note::note;
use crawrad_os::tasks::TaskId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildPage {
    #[default]
    Options,
    Command(BuildMode),
    Size,
}

#[note("build-settings-page-title")]
pub fn page_title(m: BuildMode) -> &'static str {
    match m {
        BuildMode::Build => "Build",
        BuildMode::Run => "Run",
        BuildMode::Check => "Check",
        BuildMode::Clean => "Clean",
    }
}

#[note("build-settings-buildstate")]
#[derive(Debug, Clone)]
pub struct BuildState {
    pub dialog: bool,
    pub profile: BuildProfile,
    pub selected_targets: Vec<TargetPlatform>,
    pub clean_before: bool,
    pub workspace: bool,
    pub verbose: bool,
    pub show_messages: bool,
    pub features: String,
    pub all_features: bool,
    pub no_default_features: bool,
    pub extra_args: String,
    pub env_vars: Vec<(String, String)>,
    pub mode: BuildMode,
    pub tab: BuildTab,
    pub warnings_count: usize,
    pub errors_count: usize,
    pub console_output: String,
    pub page: BuildPage,
    pub runs: Vec<(BuildMode, BuildProgress)>,
    pub running: Option<(BuildMode, TaskId, Instant)>,
    pub finished_at: Option<(BuildMode, Instant)>,
    pub page_switched_at: Option<Instant>,
    /// When a command was last refused because a build was running.
    pub refused_at: Option<Instant>,
    pub last_release_symbols: Vec<crate::release_size::Symbol>,
    pub size_unseen: bool,
    pub last_release: Option<crate::release_size::ReleaseCard>,
    pub release_card_dismissed: bool,
}

impl Default for BuildState {
    fn default() -> Self {
        Self {
            dialog: false,
            profile: BuildProfile::default(),
            selected_targets: Vec::new(),
            clean_before: true,
            workspace: false,
            verbose: false,
            show_messages: true,
            features: String::new(),
            all_features: false,
            no_default_features: false,
            extra_args: String::new(),
            env_vars: Vec::new(),
            mode: BuildMode::default(),
            tab: BuildTab::default(),
            warnings_count: 0,
            errors_count: 0,
            console_output: String::new(),
            page: BuildPage::default(),
            runs: Vec::new(),
            running: None,
            finished_at: None,
            page_switched_at: None,
            refused_at: None,
            last_release_symbols: Vec::new(),
            size_unseen: false,
            last_release: None,
            release_card_dismissed: false,
        }
    }
}

impl BuildState {
    #[note("build-settings-buildstate-run")]
    pub fn run(&self, m: BuildMode) -> Option<&BuildProgress> {
        self.runs
            .iter()
            .find(|(mode, _)| *mode == m)
            .map(|(_, p)| p)
    }

    #[note("build-settings-buildstate-run-mut")]
    pub fn run_mut(&mut self, m: BuildMode) -> Option<&mut BuildProgress> {
        self.runs
            .iter_mut()
            .find(|(mode, _)| *mode == m)
            .map(|(_, p)| p)
    }

    #[note("build-settings-buildstate-start-run")]
    pub fn start_run(&mut self, m: BuildMode, p: BuildProgress) {
        self.last_release = None;
        self.release_card_dismissed = false;
        if let Some((_, existing)) = self.runs.iter_mut().find(|(mode, _)| *mode == m) {
            *existing = p;
        } else {
            self.runs.push((m, p));
        }
    }

    #[note("build-settings-buildstate-switch-page")]
    pub fn switch_page(&mut self, page: BuildPage) {
        if self.page != page {
            self.page = page;
            self.page_switched_at = Some(Instant::now());
        }
        if page == BuildPage::Size {
            self.size_unseen = false;
        }
    }

    #[note("build-settings-buildstate-record-finish")]
    pub fn record_finish(&mut self, m: BuildMode, at: Instant) {
        self.finished_at = Some((m, at));
    }

    #[note("build-settings-buildstate-needs-tick")]
    pub fn needs_tick(&self) -> bool {
        if self.running.is_some() {
            return true;
        }
        if let Some((_, fin)) = self.finished_at
            && fin.elapsed() < crate::build_busy::CARD_LINGER
        {
            return true;
        }
        if let Some(sw) = self.page_switched_at
            && sw.elapsed() < crate::build_busy::PAGE_TURN
        {
            return true;
        }
        false
    }

    #[note("build-settings-buildstate-card-now")]
    pub fn card_now(&self) -> Option<crate::build_busy::Card> {
        if let Some((mode, _, start)) = self.running {
            self.run(mode)
                .and_then(|p| crate::build_busy::card(p, start.elapsed(), None))
        } else if let Some((mode, fin_at)) = self.finished_at {
            self.run(mode)
                .and_then(|p| crate::build_busy::card(p, fin_at.elapsed(), Some(fin_at.elapsed())))
        } else {
            None
        }
    }

    #[note("build-settings-buildstate-rail-label")]
    pub fn rail_label(&self, mode: BuildMode) -> String {
        let normal = match mode {
            BuildMode::Build => "🔨 Build",
            BuildMode::Run => "▶ Run",
            BuildMode::Check => "📝 Check",
            BuildMode::Clean => "🧹 Clean",
        };
        if let Some((m, _, start)) = self.running
            && m == mode
        {
            let timing = crawrad_component::pieces::BusyTiming::default();
            if let Some(busy) = crate::build_busy::busy_label(mode, &timing, start.elapsed(), None)
            {
                return busy;
            }
        }
        normal.to_string()
    }

    #[note("build-settings-buildstate-page-offset")]
    pub fn page_offset(&self) -> f64 {
        self.page_switched_at
            .map(|t| crate::build_busy::page_turn_offset(t.elapsed()))
            .unwrap_or(0.0)
    }

    #[note("build-settings-buildstate-after-finish")]
    pub fn after_finish(&mut self, mode: BuildMode, now: Instant) -> bool {
        self.record_finish(mode, now);
        if let Some(run) = self.run(mode)
            && crate::build_busy::should_open_on_finish(mode, run, self.dialog)
        {
            self.switch_page(BuildPage::Command(mode));
            self.tab = BuildTab::Errors;
            return true;
        }
        false
    }

    #[note("build-settings-buildstate-is-mode-enabled")]
    pub fn is_mode_enabled(&self, mode: BuildMode) -> bool {
        self.running.as_ref().is_none_or(|(m, _, _)| *m == mode)
    }

    #[note("build-settings-buildstate-active-or-last-mode")]
    pub fn active_or_last_mode(&self) -> BuildMode {
        self.running
            .as_ref()
            .map(|(m, _, _)| *m)
            .or_else(|| self.finished_at.map(|(m, _)| m))
            .unwrap_or(self.mode)
    }

    #[note("build-settings-buildstate-status-line-now")]
    pub fn status_line_now(&self) -> Option<String> {
        let (mode, _, start) = self.running?;
        if self
            .refused_at
            .is_some_and(|at| at.elapsed() < REFUSAL_HOLD)
        {
            return Some(ALREADY_BUILDING.into());
        }
        let run = self.run(mode)?;
        Some(run.status_line(start.elapsed()))
    }

    #[note("build-settings-buildstate-refuse-busy")]
    pub fn refuse_busy(&mut self) -> String {
        self.refused_at = Some(Instant::now());
        ALREADY_BUILDING.into()
    }

    #[note("build-settings-buildstate-take-running")]
    pub fn take_running(&mut self) -> (BuildMode, std::time::Duration) {
        match self.running.take() {
            Some((m, _, start)) => (m, start.elapsed()),
            None => (self.mode, std::time::Duration::ZERO),
        }
    }

    #[note("build-settings-buildstate-finish-failed-run")]
    pub fn finish_failed_run(&mut self, mode: BuildMode, elapsed: std::time::Duration, err: &str) {
        if let Some(run) = self.run_mut(mode) {
            run.finish(crate::build_progress::Finish {
                success: false,
                cancelled: false,
                errors: 1,
                warnings: 0,
                elapsed,
            });
            run.append_log_line("");
            run.append_log_line(format!("Error: {err}"));
        }
    }

    #[note("build-settings-buildstate-update-diagnostics")]
    pub fn update_diagnostics(&mut self, diags: &[crate::diagnostics_model::Diagnostic]) {
        self.errors_count = diags
            .iter()
            .filter(|d| d.severity == crate::diagnostics_model::Severity::Error)
            .count();
        self.warnings_count = diags
            .iter()
            .filter(|d| d.severity == crate::diagnostics_model::Severity::Warning)
            .count();
    }

    #[note("build-settings-buildstate-command-view-info")]
    pub fn command_view_info(&self, mode: BuildMode) -> (String, String, String, Option<f64>) {
        let is_running = self.running.as_ref().is_some_and(|(m, _, _)| *m == mode);
        let run_opt = self.run(mode);
        let headline = run_opt
            .map(|r| r.headline())
            .unwrap_or_else(|| format!("{} not run yet", page_title(mode)));
        let counter = run_opt.and_then(|r| r.counter()).unwrap_or_default();
        let elapsed = if is_running {
            self.running
                .as_ref()
                .map(|(_, _, s)| s.elapsed())
                .unwrap_or_default()
        } else {
            run_opt
                .and_then(|r| r.finished())
                .map(|f| f.elapsed)
                .unwrap_or_default()
        };
        let clock = if run_opt.is_some() {
            crate::build_progress::BuildProgress::clock(elapsed)
        } else {
            String::new()
        };
        let fraction = run_opt.and_then(|r| r.fraction());
        (headline, counter, clock, fraction)
    }

    #[note("build-settings-buildstate-target-chip-labels")]
    pub fn target_chip_labels(&self) -> Vec<String> {
        self.selected_targets
            .iter()
            .map(|t| t.chip_label().to_string())
            .collect()
    }

    #[note("build-settings-buildstate-meta-line")]
    pub fn meta_line(&self) -> String {
        let mut parts = Vec::new();
        let prof = self.profile.label();
        if !prof.is_empty() {
            parts.push(prof.to_string());
        }
        let plat = crate::build_report::platform_label(&self.selected_targets);
        if !plat.is_empty() {
            parts.push(plat);
        }
        let env_part = self
            .env_vars
            .iter()
            .filter(|(k, _)| !k.trim().is_empty())
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(" ");
        if !env_part.is_empty() {
            parts.push(env_part);
        }
        parts.join(" · ")
    }

    fn has_cross_target(&self) -> bool {
        let Some(t) = self.selected_targets.first() else {
            return false;
        };
        let is_host = match t {
            TargetPlatform::Win64 => cfg!(all(target_os = "windows", target_arch = "x86_64")),
            TargetPlatform::Win32 => cfg!(all(target_os = "windows", target_arch = "x86")),
            TargetPlatform::Linux64 => cfg!(all(target_os = "linux", target_arch = "x86_64")),
            TargetPlatform::MacSilicon => {
                cfg!(all(target_os = "macos", target_arch = "aarch64"))
            }
            TargetPlatform::MacIntel => cfg!(all(target_os = "macos", target_arch = "x86_64")),
            _ => false,
        };
        !is_host || self.selected_targets.len() > 1
    }

    /// Whether this build targets the host machine without cross-compilation flags.
    pub fn is_host_build(&self) -> bool {
        !self.has_cross_target()
            && !self
                .extra_args
                .split_whitespace()
                .any(|w| w == "--target" || w.starts_with("--target="))
    }

    /// The build log's closing summary of `report`, plus the Release size line when this was a
    /// finished Release build (which is then measured and recorded).
    pub fn closing_summary(
        &mut self,
        report: &crate::build_report::Report,
        cancelled: bool,
        root: Option<&Path>,
    ) -> Vec<String> {
        let mut lines = crate::build_report::summary(report);
        lines.extend(self.record_release_size(report, cancelled, root));
        lines
    }

    /// Measures and records the Release build `report` describes; returns the line for the
    /// build log, or `None` when it was not a finished single-executable Release build.
    pub fn record_release_size(
        &mut self,
        report: &crate::build_report::Report,
        cancelled: bool,
        root: Option<&Path>,
    ) -> Option<String> {
        use crate::release_size;
        let package = report.package.as_str();
        if report.mode != BuildMode::Build
            || !report.success
            || cancelled
            || self.profile != BuildProfile::Release
        {
            return None;
        }
        let [(exe, _)] = report.executables.as_slice() else {
            return None;
        };
        let Some(root) = root else {
            return Some("Release size not recorded: no project root".to_string());
        };
        // Only a build that asked the linker for the map may read it: a map left by an
        // earlier build would describe another executable.
        let map_text = self
            .wants_map(CargoCommand::Build)
            .then(|| release_size::map_file(&root.join("target"), package))
            .and_then(|map| std::fs::read_to_string(map).ok());
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let (current, symbols) =
            match release_size::measure_with_symbols(package, exe, map_text.as_deref(), at) {
                Ok(measured) => measured,
                Err(e) => return Some(format!("Release size not recorded: {e}")),
            };
        // The Size page's Functions tab reads them; the map is not read twice.
        self.last_release_symbols = symbols;
        self.size_unseen = true;
        let history = release_size::read_history(root);
        let elapsed = self
            .run(report.mode)
            .and_then(|r| r.finished())
            .map(|f| f.elapsed)
            .unwrap_or_default();
        self.last_release = Some(release_size::release_card(&history, &current, elapsed));
        if self.dialog {
            self.release_card_dismissed = true;
        }
        let line = release_size::size_line(&history, &current);
        match release_size::record(root, current) {
            Ok(()) => Some(line),
            Err(e) => Some(format!("Release size not recorded: {e}")),
        }
    }

    /// Returns the release size summary text for the status bar.
    pub fn status_text(&self) -> Option<String> {
        self.last_release.as_ref().map(|r| r.status_text())
    }

    /// Whether a build should ask the linker for a map file.
    pub fn wants_map(&self, cmd: CargoCommand) -> bool {
        cmd == CargoCommand::Build
            && self.profile == BuildProfile::Release
            && cfg!(all(target_os = "windows", target_env = "msvc"))
            && self.is_host_build()
            && !self.extra_args.split_whitespace().any(|a| a == "--")
    }

    #[note("build-settings-buildstate-cargo-call")]
    pub fn cargo_call(
        &self,
        cmd: CargoCommand,
        package: &str,
        root: &Path,
    ) -> (crawrad_os::process::CargoCall, String) {
        let wants_map = self.wants_map(cmd);
        let verbo = if wants_map { "rustc" } else { cmd.cargo_verb() };
        let mut flags = Vec::new();

        if self.profile == BuildProfile::Release {
            flags.push("--release".to_string());
        }
        if self.workspace {
            flags.push("--workspace".to_string());
        }
        if self.verbose {
            flags.push("--verbose".to_string());
        }
        if self.all_features {
            flags.push("--all-features".to_string());
        }
        if self.no_default_features {
            flags.push("--no-default-features".to_string());
        }
        let feat = self.features.trim();
        if !feat.is_empty() {
            flags.push(format!("--features {feat}"));
        }
        let args = self.extra_args.trim();
        if !args.is_empty() {
            flags.push(args.to_string());
        }
        if self.has_cross_target()
            && let Some(t) = self.selected_targets.first()
        {
            flags.push(format!("--target {}", t.triple()));
        }
        // The rustc args go AFTER `--` and are never split on spaces: the map path may hold one
        // (a project under `Users/Ana Lima/`).
        let mut rustc_args = Vec::new();
        if wants_map {
            flags.push(format!("--bin {package}"));
            rustc_args.push("--".to_string());
            let map = crate::release_size::map_file(&root.join("target"), package);
            rustc_args.extend(crate::release_size::map_link_args(&map));
        }

        let env_prefix = if self.env_vars.is_empty() {
            String::new()
        } else {
            let envs: Vec<_> = self
                .env_vars
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect();
            format!("{} ", envs.join(" "))
        };

        let flags_str: String = flags
            .iter()
            .chain(&rustc_args)
            .map(|f| format!(" {f}"))
            .collect();

        let cmd_line = format!("{env_prefix}cargo {verbo} -p {package}{flags_str}");

        let call = if flags.is_empty() && rustc_args.is_empty() {
            match cmd {
                CargoCommand::Build => crawrad_os::process::CargoCall::build(package, root),
                CargoCommand::Check => crawrad_os::process::CargoCall::check(package, root),
                CargoCommand::Clean => crawrad_os::process::CargoCall::clean(package, root),
            }
        } else {
            let mut full_args = vec![verbo.to_string(), "-p".to_string(), package.to_string()];
            if cmd == CargoCommand::Build || cmd == CargoCommand::Check {
                full_args.push("--message-format=json".to_string());
            }
            for f in &flags {
                for part in f.split_whitespace() {
                    full_args.push(part.to_string());
                }
            }
            full_args.extend(rustc_args);
            crawrad_os::process::CargoCall::new(full_args, root)
        };

        (call, cmd_line)
    }

    #[note("build-settings-buildstate-sort-targets")]
    pub fn sort_targets(&mut self) {
        self.selected_targets.sort_by_key(|t| {
            TargetPlatform::ALL
                .iter()
                .position(|x| x == t)
                .unwrap_or(99)
        });
    }

    #[note("build-settings-buildstate-ensure-default-targets")]
    pub fn ensure_default_targets(&mut self) {
        if self.selected_targets.is_empty()
            || self.selected_targets == [TargetPlatform::Win32, TargetPlatform::Win64]
        {
            #[cfg(target_os = "windows")]
            {
                self.selected_targets = vec![TargetPlatform::Win64];
            }
            #[cfg(target_os = "linux")]
            {
                self.selected_targets = vec![TargetPlatform::Linux64];
            }
            #[cfg(target_os = "macos")]
            {
                self.selected_targets = vec![TargetPlatform::MacSilicon];
            }
            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            {
                self.selected_targets = vec![];
            }
        }
    }
}

#[cfg(test)]
#[path = "test/build_settings/test.rs"]
mod tests;
