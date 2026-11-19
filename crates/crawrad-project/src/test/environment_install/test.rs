use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use super::*;
use crate::environment::{Host, InstallStep, Tool};

/// Tests that change CARGO_HOME run one at a time: the process environment is shared.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct EnvGuard {
    key: &'static str,
    old: Option<std::ffi::OsString>,
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.old {
            Some(val) => crawrad_os::set_env_for_children(self.key, val.to_str().unwrap_or("")),
            None => crawrad_os::set_env_for_children(self.key, ""),
        }
    }
}

#[test]
fn resolve_in_found_with_and_without_exe_and_not_found() {
    let dir = std::env::temp_dir().join("crawrad-test-resolve-in");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    // Create tool_with_exe.exe
    std::fs::write(dir.join("tool_with_exe.exe"), b"").unwrap();
    // Create tool_bare (without .exe)
    std::fs::write(dir.join("tool_bare"), b"").unwrap();

    // Found with .exe suffix
    let res_exe = resolve_in(Some(&dir), "tool_with_exe", ".exe");
    assert_eq!(res_exe, dir.join("tool_with_exe.exe"));

    // Found already specifying .exe
    let res_exe_direct = resolve_in(Some(&dir), "tool_with_exe.exe", ".exe");
    assert_eq!(res_exe_direct, dir.join("tool_with_exe.exe"));

    // Found without .exe suffix
    let res_bare = resolve_in(Some(&dir), "tool_bare", ".exe");
    assert_eq!(res_bare, dir.join("tool_bare"));

    // Not found in directory -> bare name
    let res_not_found = resolve_in(Some(&dir), "tool_missing", ".exe");
    assert_eq!(res_not_found, PathBuf::from("tool_missing"));

    // No bin dir provided -> bare name
    let res_none = resolve_in(None, "tool_with_exe", ".exe");
    assert_eq!(res_none, PathBuf::from("tool_with_exe"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cargo_bin_dir_honours_cargo_home() {
    let _env = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let old_cargo_home = std::env::var_os("CARGO_HOME");
    let _guard = EnvGuard {
        key: "CARGO_HOME",
        old: old_cargo_home,
    };

    let temp_dir = std::env::temp_dir().join("crawrad-test-custom-cargo-home");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    crawrad_os::set_env_for_children("CARGO_HOME", temp_dir.to_str().unwrap());

    let bin_dir = cargo_bin_dir();
    assert_eq!(bin_dir, Some(temp_dir.join("bin")));

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn run_plan_untrusted_url_refused_and_nothing_written() {
    let work_dir = std::env::temp_dir().join("crawrad-test-untrusted-url");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    let plan = vec![InstallStep::DownloadAndRun {
        url: "http://127.0.0.1:8080/evil.exe".to_string(),
        file_name: "evil.exe",
        args: vec![],
    }];
    let cancel = AtomicBool::new(false);
    let mut events = Vec::new();

    let result = run_plan(&plan, &work_dir, &cancel, &mut |ev| events.push(ev));

    assert_eq!(
        result,
        Err(InstallError::UntrustedUrl(
            "http://127.0.0.1:8080/evil.exe".to_string()
        ))
    );
    assert!(events.is_empty());
    assert!(!work_dir.join("evil.exe").exists());
    let entries = std::fs::read_dir(&work_dir).unwrap().count();
    assert_eq!(entries, 0);

    let _ = std::fs::remove_dir_all(&work_dir);
}

#[test]
fn run_plan_run_step_emits_events_and_nonexistent_fails() {
    let work_dir = std::env::temp_dir().join("crawrad-test-run-step");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    // 1. Success case: rustc --version
    let plan = vec![InstallStep::Run {
        program: "rustc",
        args: vec!["--version"],
    }];
    let cancel = AtomicBool::new(false);
    let mut events = Vec::new();
    let result = run_plan(&plan, &work_dir, &cancel, &mut |ev| events.push(ev));
    assert!(result.is_ok());

    let started = events.iter().any(|ev| {
        matches!(
            ev,
            InstallEvent::Started {
                step: 0,
                title
            } if title == "rustc --version"
        )
    });
    assert!(started);

    let lines: Vec<&String> = events
        .iter()
        .filter_map(|ev| match ev {
            InstallEvent::Line { step: 0, text } => Some(text),
            _ => None,
        })
        .collect();
    assert!(!lines.is_empty());
    assert!(lines[0].contains("rustc"));

    let finished = events.iter().any(|ev| {
        matches!(
            ev,
            InstallEvent::Finished {
                step: 0,
                ok: true,
                ..
            }
        )
    });
    assert!(finished);

    // 2. Failure case: nonexistent program
    let bad_plan = vec![InstallStep::Run {
        program: "nonexistent-tool-crawrad-test-xyz",
        args: vec!["--help"],
    }];
    let mut bad_events = Vec::new();
    let bad_result = run_plan(&bad_plan, &work_dir, &cancel, &mut |ev| bad_events.push(ev));

    assert!(matches!(
        bad_result,
        Err(InstallError::StepFailed { step: 0, .. })
    ));
    let bad_finished = bad_events.iter().any(|ev| {
        matches!(
            ev,
            InstallEvent::Finished {
                step: 0,
                ok: false,
                ..
            }
        )
    });
    assert!(bad_finished);

    let _ = std::fs::remove_dir_all(&work_dir);
}

#[test]
fn run_plan_cancelled_before_anything_runs() {
    let work_dir = std::env::temp_dir().join("crawrad-test-cancel-init");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    let plan = vec![InstallStep::Run {
        program: "rustc",
        args: vec!["--version"],
    }];
    let cancel = AtomicBool::new(true);
    let mut events = Vec::new();

    let result = run_plan(&plan, &work_dir, &cancel, &mut |ev| events.push(ev));
    assert_eq!(result, Err(InstallError::Cancelled));
    assert!(events.is_empty());

    let _ = std::fs::remove_dir_all(&work_dir);
}

#[test]
fn run_plan_manual_step_emits_finished_ok_false_and_continues() {
    let work_dir = std::env::temp_dir().join("crawrad-test-manual-step");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    let manual_msg = "Install lldb-dap through your distribution package manager.";
    let plan = vec![
        InstallStep::Manual {
            tool: Tool::Debugger,
            how: manual_msg,
        },
        InstallStep::Run {
            program: "rustc",
            args: vec!["--version"],
        },
    ];
    let cancel = AtomicBool::new(false);
    let mut events = Vec::new();

    let result = run_plan(&plan, &work_dir, &cancel, &mut |ev| events.push(ev));
    assert!(result.is_ok());

    // Step 0: Manual
    let step0_started = events
        .iter()
        .any(|ev| matches!(ev, InstallEvent::Started { step: 0, .. }));
    assert!(step0_started);
    let step0_finished = events.iter().any(|ev| {
        matches!(
            ev,
            InstallEvent::Finished {
                step: 0,
                ok: false,
                message,
            } if message == manual_msg
        )
    });
    assert!(step0_finished);

    // Step 1: Run continued and succeeded
    let step1_started = events
        .iter()
        .any(|ev| matches!(ev, InstallEvent::Started { step: 1, .. }));
    assert!(step1_started);
    let step1_finished = events.iter().any(|ev| {
        matches!(
            ev,
            InstallEvent::Finished {
                step: 1,
                ok: true,
                ..
            }
        )
    });
    assert!(step1_finished);

    let _ = std::fs::remove_dir_all(&work_dir);
}

#[test]
fn system_probe_probes_rust_analyzer_and_lldb_dap() {
    let probe = SystemProbe {
        rust_analyzer: || Some(PathBuf::from("/custom/rust-analyzer")),
        lldb_dap: || Some(PathBuf::from("/custom/lldb-dap")),
    };

    assert_eq!(
        probe.rust_analyzer(),
        Some(PathBuf::from("/custom/rust-analyzer"))
    );
    assert_eq!(probe.lldb_dap(), Some(PathBuf::from("/custom/lldb-dap")));
    assert_eq!(probe.host(), Host::current());
    let _ = probe.msvc_installed();

    let out = probe.output("rustc", &["--version"]);
    assert!(out.is_some());
    assert!(out.unwrap().contains("rustc"));

    let bad_out = probe.output("nonexistent-tool-xyz-123", &["--version"]);
    assert!(bad_out.is_none());
}

#[test]
fn system_probe_rust_analyzer_fallback_to_cargo_bin() {
    let dir = std::env::temp_dir().join("crawrad-test-ra-fallback");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let bin_dir = dir.join("bin");
    std::fs::create_dir_all(&bin_dir).unwrap();

    let exe_suffix = if cfg!(windows) { ".exe" } else { "" };
    let ra_name = format!("rust-analyzer{exe_suffix}");
    std::fs::write(bin_dir.join(&ra_name), b"").unwrap();

    let _env = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let old_cargo_home = std::env::var_os("CARGO_HOME");
    let _guard = EnvGuard {
        key: "CARGO_HOME",
        old: old_cargo_home,
    };
    crawrad_os::set_env_for_children("CARGO_HOME", dir.to_str().unwrap());

    let probe = SystemProbe::default();
    assert_eq!(probe.rust_analyzer(), Some(bin_dir.join(&ra_name)));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn install_error_display_formatting() {
    let err_url = InstallError::UntrustedUrl("http://evil.com".to_string());
    assert_eq!(
        err_url.to_string(),
        "untrusted download URL: http://evil.com"
    );

    let err_cancel = InstallError::Cancelled;
    assert_eq!(err_cancel.to_string(), "installation cancelled");

    let err_step = InstallError::StepFailed {
        step: 2,
        message: "process exited with code 1".to_string(),
    };
    assert_eq!(
        err_step.to_string(),
        "step 2 failed: process exited with code 1"
    );
}

#[test]
fn run_plan_with_open_url_and_run_step() {
    let work_dir = std::env::temp_dir().join("crawrad-test-open-url-step");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    let plan = vec![
        InstallStep::OpenUrl {
            url: "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
        },
        InstallStep::Run {
            program: "rustc",
            args: vec!["--version"],
        },
    ];
    let cancel = AtomicBool::new(false);
    let mut events = Vec::new();

    let result = run_plan(&plan, &work_dir, &cancel, &mut |ev| events.push(ev));
    assert!(result.is_ok());

    let finished_events: Vec<&InstallEvent> = events
        .iter()
        .filter(|ev| matches!(ev, InstallEvent::Finished { .. }))
        .collect();
    assert_eq!(finished_events.len(), 2);

    match finished_events[0] {
        InstallEvent::Finished {
            step: 0,
            ok: false,
            message,
        } => {
            assert_eq!(
                message,
                "https://visualstudio.microsoft.com/visual-cpp-build-tools/"
            );
        }
        other => panic!("expected step 0 Finished ok: false, got {other:?}"),
    }

    match finished_events[1] {
        InstallEvent::Finished {
            step: 1, ok: true, ..
        } => {}
        other => panic!("expected step 1 Finished ok: true, got {other:?}"),
    }

    let _ = std::fs::remove_dir_all(&work_dir);
}

#[test]
fn run_plan_with_copy_command_step() {
    let work_dir = std::env::temp_dir().join("crawrad-test-copy-cmd-step");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    let cmd = "sudo apt install build-essential";
    let plan = vec![
        InstallStep::CopyCommand { command: cmd },
        InstallStep::Run {
            program: "rustc",
            args: vec!["--version"],
        },
    ];
    let cancel = AtomicBool::new(false);
    let mut events = Vec::new();

    let result = run_plan(&plan, &work_dir, &cancel, &mut |ev| events.push(ev));
    assert!(result.is_ok());

    let finished_events: Vec<&InstallEvent> = events
        .iter()
        .filter(|ev| matches!(ev, InstallEvent::Finished { .. }))
        .collect();
    assert_eq!(finished_events.len(), 2);

    match finished_events[0] {
        InstallEvent::Finished {
            step: 0,
            ok: false,
            message,
        } => {
            assert_eq!(message, cmd);
        }
        other => panic!("expected step 0 Finished ok: false, got {other:?}"),
    }

    match finished_events[1] {
        InstallEvent::Finished {
            step: 1, ok: true, ..
        } => {}
        other => panic!("expected step 1 Finished ok: true, got {other:?}"),
    }

    let _ = std::fs::remove_dir_all(&work_dir);
}
