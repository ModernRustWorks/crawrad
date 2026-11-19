//! First-start environment check model and installation planning.
//!
//! Checks the presence and suitability of the core development tools required or
//! recommended by CrawRAD: the Rust toolchain, rust-analyzer, debugger, and WebAssembly target.

use std::path::PathBuf;

/// The minimum supported Rust version required for CrawRAD projects (Rust edition 2024).
pub const MINIMUM_RUST_VERSION: &str = "1.85.0";
/// `MINIMUM_RUST_VERSION` as (major, minor), for comparison.
const MINIMUM_RUST: (u64, u64) = (1, 85);

/// Tools checked during the environment check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tool {
    /// System build tools (native linker, C compiler and system libraries).
    BuildTools,
    /// Rust compiler (`rustc`) and package manager (`cargo`).
    RustToolchain,
    /// Language server for code completion and diagnostics (`rust-analyzer`).
    RustAnalyzer,
    /// DAP-compatible debugger (`lldb-dap`).
    Debugger,
    /// WebAssembly target for browser applications (`wasm32-unknown-unknown`).
    WebTarget,
}

impl Tool {
    /// Whether this tool is strictly required to open and build projects.
    pub fn required(self) -> bool {
        matches!(
            self,
            Tool::BuildTools | Tool::RustToolchain | Tool::RustAnalyzer
        )
    }

    /// Display title for the tool in dialogs and reports.
    pub fn title(self) -> &'static str {
        match self {
            Tool::BuildTools => "System build tools",
            Tool::RustToolchain => "Rust toolchain",
            Tool::RustAnalyzer => "rust-analyzer",
            Tool::Debugger => "Debugger",
            Tool::WebTarget => "Web target",
        }
    }

    /// One-line summary of what the tool is used for.
    pub fn purpose(self) -> &'static str {
        match self {
            Tool::BuildTools => "The linker and system libraries Rust needs to build an app.",
            Tool::RustToolchain => {
                "Builds and runs every project. Needs 1.85 or newer (edition 2024)."
            }
            Tool::RustAnalyzer => "Completion, hover, go to definition and errors as you type.",
            Tool::Debugger => "Breakpoints and stepping. Run works without it.",
            Tool::WebTarget => "Only for Web App projects.",
        }
    }

    /// Host-specific detail describing what build tools are needed.
    pub fn detail(self, host: Host) -> &'static str {
        match (self, host) {
            (Tool::BuildTools, Host::WindowsX64 | Host::WindowsArm64) => {
                "Visual Studio C++ Build Tools (MSVC linker and Windows SDK)."
            }
            (Tool::BuildTools, Host::MacX64 | Host::MacArm64) => "Xcode Command Line Tools.",
            (Tool::BuildTools, Host::LinuxX64 | Host::LinuxArm64) => {
                "A C compiler (cc) and the X11, Wayland and ALSA development libraries."
            }
            _ => "",
        }
    }
}

/// The state of an inspected tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolState {
    /// The tool is installed and satisfies minimum version requirements.
    Installed { version: String },
    /// The tool is present but its version is below the required minimum.
    TooOld {
        version: String,
        minimum: &'static str,
    },
    /// The tool was not found on the machine.
    Missing,
}

/// Status of a specific tool after environment probing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolStatus {
    pub tool: Tool,
    pub state: ToolState,
}

/// Everything the environment check needs from the machine, allowing tests to mock it.
pub trait Probe {
    /// stdout of `<program> <args>` when it ran and exited with code 0, else `None`.
    fn output(&self, program: &str, args: &[&str]) -> Option<String>;
    /// Path to the `rust-analyzer` executable, if found.
    fn rust_analyzer(&self) -> Option<PathBuf>;
    /// Path to the `lldb-dap` executable, if found.
    fn lldb_dap(&self) -> Option<PathBuf>;
    /// Whether Microsoft Visual C++ Build Tools (MSVC linker and Windows SDK) are installed.
    fn msvc_installed(&self) -> bool;
    /// The host operating system and architecture target, if known.
    fn host(&self) -> Option<Host>;
}

/// Host operating system and CPU architecture target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Host {
    WindowsX64,
    WindowsArm64,
    LinuxX64,
    LinuxArm64,
    MacX64,
    MacArm64,
}

impl Host {
    /// Detects the host platform from target OS and target architecture configuration.
    pub fn current() -> Option<Self> {
        if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
            Some(Host::WindowsX64)
        } else if cfg!(target_os = "windows") && cfg!(target_arch = "aarch64") {
            Some(Host::WindowsArm64)
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            Some(Host::LinuxX64)
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
            Some(Host::LinuxArm64)
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
            Some(Host::MacX64)
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
            Some(Host::MacArm64)
        } else {
            None
        }
    }

    /// The target triple used by rustup-init for this host.
    pub const fn triple(self) -> &'static str {
        match self {
            Host::WindowsX64 => "x86_64-pc-windows-msvc",
            Host::WindowsArm64 => "aarch64-pc-windows-msvc",
            Host::LinuxX64 => "x86_64-unknown-linux-gnu",
            Host::LinuxArm64 => "aarch64-unknown-linux-gnu",
            Host::MacX64 => "x86_64-apple-darwin",
            Host::MacArm64 => "aarch64-apple-darwin",
        }
    }

    /// The name of the rustup installer executable on this host.
    pub const fn rustup_init_file_name(self) -> &'static str {
        match self {
            Host::WindowsX64 | Host::WindowsArm64 => "rustup-init.exe",
            Host::LinuxX64 | Host::LinuxArm64 | Host::MacX64 | Host::MacArm64 => "rustup-init",
        }
    }

    /// The official download URL for `rustup-init` on this host.
    pub fn rustup_init_url(self) -> String {
        format!(
            "https://static.rust-lang.org/rustup/dist/{}/{}",
            self.triple(),
            self.rustup_init_file_name()
        )
    }
}

/// A step in an installation plan to set up missing or outdated development tools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallStep {
    /// Download an installer executable and run it with arguments.
    DownloadAndRun {
        url: String,
        file_name: &'static str,
        args: Vec<&'static str>,
    },
    /// Run a command already available on the system.
    Run {
        program: &'static str,
        args: Vec<&'static str>,
    },
    /// Open a web page in the default browser for user-driven installation.
    OpenUrl { url: &'static str },
    /// Copy a command to the clipboard for the user to run manually (e.g. requires sudo).
    CopyCommand { command: &'static str },
    /// Manual instructions when automatic installation is not available.
    Manual { tool: Tool, how: &'static str },
}

/// Inspects the host machine through `probe` and returns the status of all 5 tools in enum order.
pub fn check(probe: &dyn Probe) -> Vec<ToolStatus> {
    vec![
        ToolStatus {
            tool: Tool::BuildTools,
            state: check_build_tools(probe),
        },
        ToolStatus {
            tool: Tool::RustToolchain,
            state: check_rust_toolchain(probe),
        },
        ToolStatus {
            tool: Tool::RustAnalyzer,
            state: check_rust_analyzer(probe),
        },
        ToolStatus {
            tool: Tool::Debugger,
            state: check_debugger(probe),
        },
        ToolStatus {
            tool: Tool::WebTarget,
            state: check_web_target(probe),
        },
    ]
}

/// Returns `true` if any required tool is not installed (either missing or too old).
pub fn needs_dialog(statuses: &[ToolStatus]) -> bool {
    statuses
        .iter()
        .any(|s| s.tool.required() && !matches!(s.state, ToolState::Installed { .. }))
}

/// Generates an installation plan for tools that are missing or outdated.
///
/// Order: Build tools first, then Rust toolchain, then rust-analyzer, then web target, then debugger.
pub fn install_plan(statuses: &[ToolStatus], host: Host) -> Vec<InstallStep> {
    let mut plan = Vec::new();

    // 0. Build tools first
    if is_missing(statuses, Tool::BuildTools) {
        match host {
            Host::WindowsX64 | Host::WindowsArm64 => {
                plan.push(InstallStep::OpenUrl {
                    url: "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
                });
            }
            Host::MacX64 | Host::MacArm64 => {
                plan.push(InstallStep::Run {
                    program: "xcode-select",
                    args: vec!["--install"],
                });
            }
            Host::LinuxX64 | Host::LinuxArm64 => {
                plan.push(InstallStep::CopyCommand {
                    command: "sudo apt install build-essential pkg-config libx11-dev libxcursor-dev libxkbcommon-dev libasound2-dev",
                });
            }
        }
    }

    // 1. Rust first (when Missing or TooOld; TooOld uses `Run rustup update stable` instead of downloading)
    if let Some(status) = statuses.iter().find(|s| s.tool == Tool::RustToolchain) {
        match status.state {
            ToolState::Missing => {
                plan.push(InstallStep::DownloadAndRun {
                    url: host.rustup_init_url(),
                    file_name: host.rustup_init_file_name(),
                    args: vec!["-y", "--profile", "default"],
                });
            }
            ToolState::TooOld { .. } => {
                plan.push(InstallStep::Run {
                    program: "rustup",
                    args: vec!["update", "stable"],
                });
            }
            ToolState::Installed { .. } => {}
        }
    }

    // 2. rust-analyzer
    if is_missing(statuses, Tool::RustAnalyzer) {
        plan.push(InstallStep::Run {
            program: "rustup",
            args: vec!["component", "add", "rust-analyzer"],
        });
    }

    // 3. Web target
    if is_missing(statuses, Tool::WebTarget) {
        plan.push(InstallStep::Run {
            program: "rustup",
            args: vec!["target", "add", "wasm32-unknown-unknown"],
        });
    }

    // 4. Debugger
    if is_missing(statuses, Tool::Debugger) {
        plan.push(InstallStep::Manual {
            tool: Tool::Debugger,
            how: "Install lldb-dap via your system package manager or LLVM distribution.",
        });
    }

    plan
}

/// The tool is in the list and not installed.
fn is_missing(statuses: &[ToolStatus], tool: Tool) -> bool {
    statuses
        .iter()
        .any(|s| s.tool == tool && !matches!(s.state, ToolState::Installed { .. }))
}

fn extract_leading_u64(s: &str) -> Option<u64> {
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u64>().ok()
    }
}

fn parse_version_tuple(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.');
    let major = extract_leading_u64(parts.next()?)?;
    let minor = extract_leading_u64(parts.next()?)?;
    let patch = match parts.next() {
        Some(p) => extract_leading_u64(p).unwrap_or(0),
        None => 0,
    };
    Some((major, minor, patch))
}

fn check_build_tools(probe: &dyn Probe) -> ToolState {
    let Some(host) = probe.host() else {
        return ToolState::Installed {
            version: String::new(),
        };
    };
    let installed = match host {
        Host::WindowsX64 | Host::WindowsArm64 => probe.msvc_installed(),
        Host::MacX64 | Host::MacArm64 => probe.output("xcode-select", &["-p"]).is_some(),
        Host::LinuxX64 | Host::LinuxArm64 => {
            probe.output("cc", &["--version"]).is_some()
                && probe
                    .output(
                        "pkg-config",
                        &["--exists", "x11", "xcursor", "xkbcommon", "alsa"],
                    )
                    .is_some()
        }
    };
    if installed {
        ToolState::Installed {
            version: String::new(),
        }
    } else {
        ToolState::Missing
    }
}

fn check_rust_toolchain(probe: &dyn Probe) -> ToolState {
    let Some(rustc_out) = probe.output("rustc", &["--version"]) else {
        return ToolState::Missing;
    };
    if probe.output("cargo", &["--version"]).is_none() {
        return ToolState::Missing;
    }

    let trimmed = rustc_out.trim();
    let ver_candidate = trimmed
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("rustc ")
                .and_then(|rest| rest.split_whitespace().next())
        })
        .unwrap_or(trimmed);

    if let Some((major, minor, _patch)) = parse_version_tuple(ver_candidate) {
        if (major, minor) < MINIMUM_RUST {
            ToolState::TooOld {
                version: ver_candidate.to_string(),
                minimum: MINIMUM_RUST_VERSION,
            }
        } else {
            ToolState::Installed {
                version: ver_candidate.to_string(),
            }
        }
    } else {
        // A version that does not parse counts as Installed with the raw text.
        ToolState::Installed {
            version: trimmed.to_string(),
        }
    }
}

fn parse_rust_analyzer_version(raw: &str) -> String {
    let trimmed = raw.trim();
    for line in trimmed.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("rust-analyzer ")
            && let Some(first) = rest.split_whitespace().next()
        {
            return first.to_string();
        }
    }
    if let Some(first) = trimmed.split_whitespace().next() {
        return first.to_string();
    }
    trimmed.to_string()
}

fn check_rust_analyzer(probe: &dyn Probe) -> ToolState {
    let Some(path) = probe.rust_analyzer() else {
        return ToolState::Missing;
    };
    let path_str = path.to_str().unwrap_or("rust-analyzer");
    let version_out = probe
        .output(path_str, &["--version"])
        .or_else(|| probe.output("rust-analyzer", &["--version"]));
    // rustup puts a `rust-analyzer` proxy on PATH even when the component is not
    // installed; the proxy is found but fails to run. Only a server that answers counts.
    match version_out {
        Some(out) => ToolState::Installed {
            version: parse_rust_analyzer_version(&out),
        },
        None => ToolState::Missing,
    }
}

fn check_debugger(probe: &dyn Probe) -> ToolState {
    if probe.lldb_dap().is_some() {
        ToolState::Installed {
            version: String::new(),
        }
    } else {
        ToolState::Missing
    }
}

fn check_web_target(probe: &dyn Probe) -> ToolState {
    let Some(out) = probe.output("rustup", &["target", "list", "--installed"]) else {
        return ToolState::Missing;
    };
    if out
        .lines()
        .any(|l| l.split_whitespace().any(|w| w == "wasm32-unknown-unknown"))
    {
        ToolState::Installed {
            version: String::new(),
        }
    } else {
        ToolState::Missing
    }
}

#[cfg(test)]
#[path = "test/environment/test.rs"]
mod test;
