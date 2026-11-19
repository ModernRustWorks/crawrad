//! Unit tests for EnvironmentSession and its pure helpers.

use crate::environment::{Host, InstallStep, Probe, Tool, ToolState, ToolStatus};
use crate::environment_session::{
    EnvironmentSession, Pump, TOOL_ORDER, button_label, heading_text, install_all_count,
    state_word, status_color, tool_install_step, tool_mono_tag,
};

#[test]
fn heading_text_for_zero_missing() {
    assert_eq!(heading_text(0), "Everything is ready");
}

#[test]
fn heading_text_for_one_missing() {
    assert_eq!(heading_text(1), "1 required tool is missing");
}

#[test]
fn heading_text_for_two_missing() {
    assert_eq!(heading_text(2), "2 required tools are missing");
    assert_eq!(heading_text(3), "3 required tools are missing");
}

#[test]
fn button_label_per_install_step() {
    let download_step = InstallStep::DownloadAndRun {
        url: "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
            .to_string(),
        file_name: "rustup-init.exe",
        args: vec!["-y"],
    };
    assert_eq!(button_label(&download_step), "Download and install");

    let run_step = InstallStep::Run {
        program: "rustup",
        args: vec!["component", "add", "rust-analyzer"],
    };
    assert_eq!(button_label(&run_step), "Install");

    let open_url_step = InstallStep::OpenUrl {
        url: "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
    };
    assert_eq!(button_label(&open_url_step), "Open download page");

    let copy_cmd_step = InstallStep::CopyCommand {
        command: "sudo apt install build-essential",
    };
    assert_eq!(button_label(&copy_cmd_step), "Copy command");

    let manual_step = InstallStep::Manual {
        tool: Tool::Debugger,
        how: "Install lldb-dap via your system package manager or LLVM distribution.",
    };
    assert_eq!(button_label(&manual_step), "How to install");
}

#[test]
fn install_all_count_excludes_openurl_copycommand_manual() {
    let mixed_plan = vec![
        InstallStep::OpenUrl {
            url: "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
        },
        InstallStep::DownloadAndRun {
            url: "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
                .to_string(),
            file_name: "rustup-init.exe",
            args: vec!["-y"],
        },
        InstallStep::Run {
            program: "rustup",
            args: vec!["component", "add", "rust-analyzer"],
        },
        InstallStep::CopyCommand {
            command: "sudo apt install build-essential",
        },
        InstallStep::Manual {
            tool: Tool::Debugger,
            how: "Install lldb-dap via system package manager.",
        },
    ];

    assert_eq!(mixed_plan.len(), 5);
    assert_eq!(install_all_count(&mixed_plan), 2);

    let non_interactive_only = vec![
        InstallStep::OpenUrl {
            url: "https://visualstudio.microsoft.com/",
        },
        InstallStep::CopyCommand {
            command: "sudo apt install build-essential",
        },
        InstallStep::Manual {
            tool: Tool::Debugger,
            how: "Install manually.",
        },
    ];
    assert_eq!(install_all_count(&non_interactive_only), 0);

    let empty_plan: Vec<InstallStep> = Vec::new();
    assert_eq!(install_all_count(&empty_plan), 0);
}

#[test]
fn tool_mono_tags() {
    assert_eq!(tool_mono_tag(Tool::BuildTools), "");
    assert_eq!(tool_mono_tag(Tool::RustToolchain), "rustc · cargo");
    assert_eq!(tool_mono_tag(Tool::RustAnalyzer), "rust-analyzer");
    assert_eq!(tool_mono_tag(Tool::Debugger), "lldb-dap");
    assert_eq!(tool_mono_tag(Tool::WebTarget), "wasm32-unknown-unknown");
}

#[test]
fn state_word_formats() {
    let installed_with_version = ToolState::Installed {
        version: "1.85.0".to_string(),
    };
    assert_eq!(
        state_word(&installed_with_version, Tool::RustToolchain, false),
        "Installed 1.85.0"
    );

    let installed_empty_version = ToolState::Installed {
        version: String::new(),
    };
    assert_eq!(
        state_word(&installed_empty_version, Tool::BuildTools, false),
        "Installed"
    );

    let too_old = ToolState::TooOld {
        version: "1.82.0".to_string(),
        minimum: "1.85.0",
    };
    assert_eq!(
        state_word(&too_old, Tool::RustToolchain, false),
        "Too old 1.82.0, needs 1.85"
    );

    let missing_required = ToolState::Missing;
    assert_eq!(
        state_word(&missing_required, Tool::RustToolchain, false),
        "Missing"
    );
    assert_eq!(
        state_word(&missing_required, Tool::BuildTools, false),
        "Missing"
    );
    assert_eq!(
        state_word(&missing_required, Tool::RustAnalyzer, false),
        "Missing"
    );

    let missing_optional = ToolState::Missing;
    assert_eq!(
        state_word(&missing_optional, Tool::Debugger, false),
        "Optional"
    );
    assert_eq!(
        state_word(&missing_optional, Tool::WebTarget, false),
        "Optional"
    );

    let running = ToolState::Missing;
    assert_eq!(
        state_word(&running, Tool::RustToolchain, true),
        "Installing"
    );
}

#[test]
fn tool_install_step_resolution() {
    let statuses = vec![
        ToolStatus {
            tool: Tool::BuildTools,
            state: ToolState::Missing,
        },
        ToolStatus {
            tool: Tool::RustToolchain,
            state: ToolState::Missing,
        },
        ToolStatus {
            tool: Tool::RustAnalyzer,
            state: ToolState::Missing,
        },
        ToolStatus {
            tool: Tool::Debugger,
            state: ToolState::Installed {
                version: String::new(),
            },
        },
        ToolStatus {
            tool: Tool::WebTarget,
            state: ToolState::Missing,
        },
    ];

    assert_eq!(
        tool_install_step(Tool::Debugger, &statuses, Host::WindowsX64),
        None
    );

    assert!(matches!(
        tool_install_step(Tool::BuildTools, &statuses, Host::WindowsX64),
        Some(InstallStep::OpenUrl { .. })
    ));

    assert!(matches!(
        tool_install_step(Tool::BuildTools, &statuses, Host::LinuxX64),
        Some(InstallStep::CopyCommand { .. })
    ));

    assert!(matches!(
        tool_install_step(Tool::BuildTools, &statuses, Host::MacX64),
        Some(InstallStep::Run { .. })
    ));

    assert!(matches!(
        tool_install_step(Tool::RustToolchain, &statuses, Host::WindowsX64),
        Some(InstallStep::DownloadAndRun { .. })
    ));

    assert!(matches!(
        tool_install_step(Tool::RustAnalyzer, &statuses, Host::WindowsX64),
        Some(InstallStep::Run { .. })
    ));
}

#[test]
fn status_colors() {
    let installed = ToolState::Installed {
        version: String::new(),
    };
    assert_eq!(
        status_color(&installed, Tool::BuildTools, false),
        [0.498, 0.831, 0.627, 1.0]
    );

    let running = ToolState::Missing;
    assert_eq!(
        status_color(&running, Tool::BuildTools, true),
        [0.941, 0.702, 0.365, 1.0]
    );

    let missing_req = ToolState::Missing;
    assert_eq!(
        status_color(&missing_req, Tool::BuildTools, false),
        [1.0, 0.541, 0.502, 1.0]
    );

    let missing_opt = ToolState::Missing;
    assert_eq!(
        status_color(&missing_opt, Tool::Debugger, false),
        [0.596, 0.596, 0.608, 1.0]
    );
}

struct FakeProbe;

impl Probe for FakeProbe {
    fn output(&self, _program: &str, _args: &[&str]) -> Option<String> {
        None
    }

    fn rust_analyzer(&self) -> Option<std::path::PathBuf> {
        None
    }

    fn lldb_dap(&self) -> Option<std::path::PathBuf> {
        None
    }

    fn msvc_installed(&self) -> bool {
        false
    }

    fn host(&self) -> Option<Host> {
        Some(Host::WindowsX64)
    }
}

fn fake_probe_factory() -> Box<dyn Probe + Send> {
    Box::new(FakeProbe)
}

#[test]
fn poll_after_fake_probe_delivers() {
    let mut session = EnvironmentSession::default();
    session.start_probe(fake_probe_factory);
    assert!(session.is_busy());

    let mut delivered = false;
    for _ in 0..100 {
        if session.poll() {
            delivered = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(delivered);
    assert_eq!(session.statuses.len(), 5);
    assert!(!session.is_busy());
}

#[test]
fn view_row_count_and_heading() {
    let session = EnvironmentSession::default();
    let (head, rows) = session.view(Host::WindowsX64);
    assert_eq!(rows.len(), 5);
    assert_eq!(head.0, "Checking your tools\u{2026}");
}

#[test]
fn install_all_on_plan_of_only_non_interactive_starts_nothing() {
    // Only the debugger is missing, and its step is manual on every host. (Missing build
    // tools are not: on macOS they install by running `xcode-select --install`.)
    let mut session = EnvironmentSession {
        statuses: vec![
            ToolStatus {
                tool: Tool::BuildTools,
                state: ToolState::Installed {
                    version: String::new(),
                },
            },
            ToolStatus {
                tool: Tool::RustToolchain,
                state: ToolState::Installed {
                    version: "1.85.0".to_string(),
                },
            },
            ToolStatus {
                tool: Tool::RustAnalyzer,
                state: ToolState::Installed {
                    version: "1.85.0".to_string(),
                },
            },
            ToolStatus {
                tool: Tool::Debugger,
                state: ToolState::Missing,
            },
            ToolStatus {
                tool: Tool::WebTarget,
                state: ToolState::Installed {
                    version: String::new(),
                },
            },
        ],
        ..Default::default()
    };

    session.install_all();
    assert!(!session.is_busy());
    assert!(session.install_cancel.is_none());
    assert!(session.active_plan.is_empty());
}

#[test]
fn begin_startup_probes_only_once() {
    let mut session = EnvironmentSession::default();
    assert!(session.begin_startup(fake_probe_factory));
    assert!(session.waiting_startup_decision);
    assert!(!session.begin_startup(fake_probe_factory));
}

#[test]
fn pump_is_idle_without_news() {
    let mut session = EnvironmentSession::default();
    assert_eq!(session.pump(), Pump::Idle);
}

#[test]
fn press_primary_out_of_range_does_nothing() {
    let mut session = EnvironmentSession::default();
    assert_eq!(session.press_primary(TOOL_ORDER.len()), None);
    assert_eq!(session.copied_tool, None);
}

#[test]
fn nothing_missing_hides_intro_and_install_all() {
    let installed = |tool| ToolStatus {
        tool,
        state: ToolState::Installed {
            version: "1.95.0".to_string(),
        },
    };
    let session = EnvironmentSession {
        statuses: TOOL_ORDER.iter().map(|t| installed(*t)).collect(),
        ..Default::default()
    };
    let (head, _) = session.view(Host::WindowsX64);
    assert_eq!(head.0, "Everything is ready");
    assert!(head.1.is_empty() && head.2.is_empty());
}
