use std::collections::HashMap;
use std::path::PathBuf;

use super::*;

#[derive(Default)]
struct FakeProbe {
    outputs: HashMap<(String, Vec<String>), String>,
    rust_analyzer_path: Option<PathBuf>,
    lldb_dap_path: Option<PathBuf>,
    msvc: bool,
    host_target: Option<Host>,
}

impl FakeProbe {
    fn with_all_installed() -> Self {
        let mut probe = Self {
            msvc: true,
            host_target: Some(Host::WindowsX64),
            ..Self::default()
        };
        probe.set_output(
            "rustc",
            &["--version"],
            "rustc 1.95.0 (59807616 2026-02-15)",
        );
        probe.set_output(
            "cargo",
            &["--version"],
            "cargo 1.95.0 (59807616 2026-02-15)",
        );
        probe.rust_analyzer_path = Some(PathBuf::from("rust-analyzer"));
        probe.set_output(
            "rust-analyzer",
            &["--version"],
            "rust-analyzer 1.95.0 (59807616 2026-02-15)",
        );
        probe.lldb_dap_path = Some(PathBuf::from("lldb-dap"));
        probe.set_output(
            "rustup",
            &["target", "list", "--installed"],
            "wasm32-unknown-unknown\nx86_64-pc-windows-msvc\n",
        );
        probe
    }

    fn set_output(&mut self, program: &str, args: &[&str], stdout: &str) {
        self.outputs.insert(
            (
                program.to_string(),
                args.iter().map(|s| s.to_string()).collect(),
            ),
            stdout.to_string(),
        );
    }
}

impl Probe for FakeProbe {
    fn output(&self, program: &str, args: &[&str]) -> Option<String> {
        let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        self.outputs.get(&(program.to_string(), args_vec)).cloned()
    }

    fn rust_analyzer(&self) -> Option<PathBuf> {
        self.rust_analyzer_path.clone()
    }

    fn lldb_dap(&self) -> Option<PathBuf> {
        self.lldb_dap_path.clone()
    }

    fn msvc_installed(&self) -> bool {
        self.msvc
    }

    fn host(&self) -> Option<Host> {
        self.host_target
    }
}

fn status_of(statuses: &[ToolStatus], tool: Tool) -> &ToolStatus {
    statuses
        .iter()
        .find(|s| s.tool == tool)
        .unwrap_or_else(|| panic!("tool {tool:?} not found in statuses"))
}

#[test]
fn everything_installed_no_dialog_empty_plan() {
    let probe = FakeProbe::with_all_installed();
    let statuses = check(&probe);
    assert_eq!(statuses.len(), 5);
    assert_eq!(statuses[0].tool, Tool::BuildTools);
    assert_eq!(
        statuses[0].state,
        ToolState::Installed {
            version: String::new(),
        }
    );
    assert_eq!(statuses[1].tool, Tool::RustToolchain);
    assert_eq!(
        statuses[1].state,
        ToolState::Installed {
            version: "1.95.0".to_string(),
        }
    );
    assert_eq!(statuses[2].tool, Tool::RustAnalyzer);
    assert_eq!(
        statuses[2].state,
        ToolState::Installed {
            version: "1.95.0".to_string(),
        }
    );
    assert_eq!(statuses[3].tool, Tool::Debugger);
    assert_eq!(
        statuses[3].state,
        ToolState::Installed {
            version: String::new(),
        }
    );
    assert_eq!(statuses[4].tool, Tool::WebTarget);
    assert_eq!(
        statuses[4].state,
        ToolState::Installed {
            version: String::new(),
        }
    );

    assert!(!needs_dialog(&statuses));
    assert!(install_plan(&statuses, Host::WindowsX64).is_empty());
}

#[test]
fn no_rustc_needs_dialog_and_downloads_correct_triple_per_host() {
    let mut probe = FakeProbe::with_all_installed();
    probe
        .outputs
        .remove(&("rustc".to_string(), vec!["--version".to_string()]));

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::Missing
    );
    assert!(needs_dialog(&statuses));

    let hosts = [
        (
            Host::WindowsX64,
            "x86_64-pc-windows-msvc",
            "rustup-init.exe",
        ),
        (
            Host::WindowsArm64,
            "aarch64-pc-windows-msvc",
            "rustup-init.exe",
        ),
        (Host::LinuxX64, "x86_64-unknown-linux-gnu", "rustup-init"),
        (Host::LinuxArm64, "aarch64-unknown-linux-gnu", "rustup-init"),
        (Host::MacX64, "x86_64-apple-darwin", "rustup-init"),
        (Host::MacArm64, "aarch64-apple-darwin", "rustup-init"),
    ];

    for (host, triple, file_name) in hosts {
        let plan = install_plan(&statuses, host);
        assert!(!plan.is_empty());
        let expected_url = format!("https://static.rust-lang.org/rustup/dist/{triple}/{file_name}");
        match &plan[0] {
            InstallStep::DownloadAndRun {
                url,
                file_name: f,
                args,
            } => {
                assert_eq!(url, &expected_url);
                assert_eq!(*f, file_name);
                assert_eq!(args, &["-y", "--profile", "default"]);
            }
            other => panic!("expected DownloadAndRun, got {other:?}"),
        }
    }
}

#[test]
fn rustc_1_80_is_too_old_and_plan_uses_rustup_update_stable() {
    let mut probe = FakeProbe::with_all_installed();
    probe.set_output(
        "rustc",
        &["--version"],
        "rustc 1.80.0 (59807616 2024-07-25)",
    );

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::TooOld {
            version: "1.80.0".to_string(),
            minimum: "1.85.0",
        }
    );
    assert!(needs_dialog(&statuses));

    let plan = install_plan(&statuses, Host::WindowsX64);
    assert_eq!(plan.len(), 1);
    assert_eq!(
        plan[0],
        InstallStep::Run {
            program: "rustup",
            args: vec!["update", "stable"],
        }
    );
}

#[test]
fn rust_analyzer_missing_only_needs_dialog_and_runs_component_add() {
    let mut probe = FakeProbe::with_all_installed();
    probe.rust_analyzer_path = None;
    probe
        .outputs
        .remove(&("rust-analyzer".to_string(), vec!["--version".to_string()]));

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustAnalyzer).state,
        ToolState::Missing
    );
    assert!(needs_dialog(&statuses));

    let plan = install_plan(&statuses, Host::LinuxX64);
    assert_eq!(plan.len(), 1);
    assert_eq!(
        plan[0],
        InstallStep::Run {
            program: "rustup",
            args: vec!["component", "add", "rust-analyzer"],
        }
    );
}

#[test]
fn only_optional_tools_missing_no_dialog() {
    let mut probe = FakeProbe::with_all_installed();
    probe.lldb_dap_path = None;
    probe.set_output(
        "rustup",
        &["target", "list", "--installed"],
        "x86_64-pc-windows-msvc\n",
    );

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::Debugger).state,
        ToolState::Missing
    );
    assert_eq!(
        status_of(&statuses, Tool::WebTarget).state,
        ToolState::Missing
    );

    assert!(!needs_dialog(&statuses));

    let plan = install_plan(&statuses, Host::WindowsX64);
    assert_eq!(plan.len(), 2);
    assert_eq!(
        plan[0],
        InstallStep::Run {
            program: "rustup",
            args: vec!["target", "add", "wasm32-unknown-unknown"],
        }
    );
    match &plan[1] {
        InstallStep::Manual { tool, .. } => {
            assert_eq!(*tool, Tool::Debugger);
        }
        other => panic!("expected Manual for debugger, got {other:?}"),
    }
}

#[test]
fn version_parsing_semver_and_garbage() {
    let mut probe = FakeProbe::with_all_installed();

    // 1.100.0 > 1.85.0 -> Installed
    probe.set_output(
        "rustc",
        &["--version"],
        "rustc 1.100.0 (abcdef12 2026-10-01)",
    );
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::Installed {
            version: "1.100.0".to_string()
        }
    );

    // Exactly 1.85.0 -> Installed
    probe.set_output(
        "rustc",
        &["--version"],
        "rustc 1.85.0 (00000000 2025-02-20)",
    );
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::Installed {
            version: "1.85.0".to_string()
        }
    );

    // 1.84.99 < 1.85.0 -> TooOld
    probe.set_output("rustc", &["--version"], "rustc 1.84.99");
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::TooOld {
            version: "1.84.99".to_string(),
            minimum: "1.85.0",
        }
    );

    // Garbage text -> counts as Installed with raw text
    probe.set_output("rustc", &["--version"], "some garbage unparseable output");
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::Installed {
            version: "some garbage unparseable output".to_string()
        }
    );
}

#[test]
fn cargo_missing_makes_rust_toolchain_missing() {
    let mut probe = FakeProbe::with_all_installed();
    probe
        .outputs
        .remove(&("cargo".to_string(), vec!["--version".to_string()]));
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustToolchain).state,
        ToolState::Missing
    );
}

#[test]
fn rustup_target_list_not_running_makes_web_target_missing() {
    let mut probe = FakeProbe::with_all_installed();
    probe.outputs.remove(&(
        "rustup".to_string(),
        vec![
            "target".to_string(),
            "list".to_string(),
            "--installed".to_string(),
        ],
    ));
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::WebTarget).state,
        ToolState::Missing
    );
}

#[test]
fn rust_analyzer_proxy_that_does_not_run_is_missing() {
    // rustup's proxy is on PATH without the component: found, but `--version` fails.
    let mut probe = FakeProbe::with_all_installed();
    probe
        .outputs
        .retain(|(program, _), _| !program.contains("rust-analyzer"));
    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::RustAnalyzer).state,
        ToolState::Missing
    );
    assert!(needs_dialog(&statuses));
}

#[test]
fn rustc_1_84_is_too_old_and_1_85_is_enough() {
    for (raw, too_old) in [
        ("rustc 1.84.1 (e71f9a9a9 2025-01-27)", true),
        ("rustc 1.85.0 (4d91de4e4 2025-02-17)", false),
    ] {
        let mut probe = FakeProbe::with_all_installed();
        probe.set_output("rustc", &["--version"], raw);
        let statuses = check(&probe);
        let state = &status_of(&statuses, Tool::RustToolchain).state;
        assert_eq!(matches!(state, ToolState::TooOld { .. }), too_old, "{raw}");
    }
}

#[test]
fn tool_properties_and_host_current() {
    assert!(Tool::BuildTools.required());
    assert!(Tool::RustToolchain.required());
    assert!(Tool::RustAnalyzer.required());
    assert!(!Tool::Debugger.required());
    assert!(!Tool::WebTarget.required());

    assert_eq!(Tool::BuildTools.title(), "System build tools");
    assert_eq!(Tool::RustToolchain.title(), "Rust toolchain");
    assert_eq!(Tool::RustAnalyzer.title(), "rust-analyzer");
    assert_eq!(Tool::Debugger.title(), "Debugger");
    assert_eq!(Tool::WebTarget.title(), "Web target");

    assert_eq!(
        Tool::BuildTools.purpose(),
        "The linker and system libraries Rust needs to build an app."
    );
    assert!(!Tool::RustToolchain.purpose().is_empty());
    assert!(!Tool::RustAnalyzer.purpose().is_empty());
    assert!(!Tool::Debugger.purpose().is_empty());
    assert!(!Tool::WebTarget.purpose().is_empty());

    assert_eq!(
        Tool::BuildTools.detail(Host::WindowsX64),
        "Visual Studio C++ Build Tools (MSVC linker and Windows SDK)."
    );
    assert_eq!(
        Tool::BuildTools.detail(Host::WindowsArm64),
        "Visual Studio C++ Build Tools (MSVC linker and Windows SDK)."
    );
    assert_eq!(
        Tool::BuildTools.detail(Host::MacX64),
        "Xcode Command Line Tools."
    );
    assert_eq!(
        Tool::BuildTools.detail(Host::MacArm64),
        "Xcode Command Line Tools."
    );
    assert_eq!(
        Tool::BuildTools.detail(Host::LinuxX64),
        "A C compiler (cc) and the X11, Wayland and ALSA development libraries."
    );
    assert_eq!(
        Tool::BuildTools.detail(Host::LinuxArm64),
        "A C compiler (cc) and the X11, Wayland and ALSA development libraries."
    );
    assert_eq!(Tool::RustToolchain.detail(Host::WindowsX64), "");

    assert!(Host::current().is_some());
}

#[test]
fn windows_without_msvc_needs_dialog_and_plan_opens_url() {
    let mut probe = FakeProbe::with_all_installed();
    probe.host_target = Some(Host::WindowsX64);
    probe.msvc = false;

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::BuildTools).state,
        ToolState::Missing
    );
    assert!(needs_dialog(&statuses));

    let plan = install_plan(&statuses, Host::WindowsX64);
    assert!(!plan.is_empty());
    assert_eq!(
        plan[0],
        InstallStep::OpenUrl {
            url: "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
        }
    );
}

#[test]
fn macos_without_xcode_select_needs_dialog_and_plan_runs_installer() {
    let mut probe = FakeProbe::with_all_installed();
    probe.host_target = Some(Host::MacX64);
    // xcode-select is not in outputs, so output("xcode-select", &["-p"]) returns None.

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::BuildTools).state,
        ToolState::Missing
    );
    assert!(needs_dialog(&statuses));

    let plan = install_plan(&statuses, Host::MacX64);
    assert!(!plan.is_empty());
    assert_eq!(
        plan[0],
        InstallStep::Run {
            program: "xcode-select",
            args: vec!["--install"],
        }
    );

    // When xcode-select -p is found -> Installed
    probe.set_output(
        "xcode-select",
        &["-p"],
        "/Library/Developer/CommandLineTools\n",
    );
    let statuses_ok = check(&probe);
    assert_eq!(
        status_of(&statuses_ok, Tool::BuildTools).state,
        ToolState::Installed {
            version: String::new(),
        }
    );
}

#[test]
fn linux_with_cc_but_pkg_config_failing_is_missing_and_plan_copies_command() {
    let mut probe = FakeProbe::with_all_installed();
    probe.host_target = Some(Host::LinuxX64);
    probe.set_output("cc", &["--version"], "cc (Ubuntu 11.4.0) 11.4.0\n");
    // pkg-config is not in outputs, so output(...) returns None.

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::BuildTools).state,
        ToolState::Missing
    );
    assert!(needs_dialog(&statuses));

    let plan = install_plan(&statuses, Host::LinuxX64);
    assert!(!plan.is_empty());
    assert_eq!(
        plan[0],
        InstallStep::CopyCommand {
            command: "sudo apt install build-essential pkg-config libx11-dev libxcursor-dev libxkbcommon-dev libasound2-dev",
        }
    );

    // When pkg-config also succeeds -> Installed
    probe.set_output(
        "pkg-config",
        &["--exists", "x11", "xcursor", "xkbcommon", "alsa"],
        "",
    );
    let statuses_ok = check(&probe);
    assert_eq!(
        status_of(&statuses_ok, Tool::BuildTools).state,
        ToolState::Installed {
            version: String::new(),
        }
    );
}

#[test]
fn unknown_host_none_build_tools_installed() {
    let mut probe = FakeProbe::with_all_installed();
    probe.host_target = None;

    let statuses = check(&probe);
    assert_eq!(
        status_of(&statuses, Tool::BuildTools).state,
        ToolState::Installed {
            version: String::new(),
        }
    );
}
