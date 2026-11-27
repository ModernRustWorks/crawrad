use super::*;
use std::path::PathBuf;
use std::time::Duration;

fn make_test_config(endpoint: &str) -> AcpConfig {
    AcpConfig {
        endpoint: endpoint.to_string(),
        endpoint_args: vec!["--stdio".to_string()],
        ..Default::default()
    }
}

fn make_base_hooks() -> DiagnosticHooks {
    DiagnosticHooks {
        resolve_program: Box::new(|_| Ok(PathBuf::from("/bin/mock-agent"))),
        check_network: Box::new(|| Ok(Duration::from_millis(10))),
        spawn_process: Box::new(|_, _| Ok("Process started (PID 100)".to_string())),
        check_handshake: Box::new(|_| Ok(Duration::from_millis(20))),
        find_installed_agents: Box::new(|| {
            vec![
                "Claude Agent".to_string(),
                "Gemini (Antigravity)".to_string(),
            ]
        }),
        config_path: Box::new(|| PathBuf::from("/home/user/.crawrad/acp_config.json")),
    }
}

#[test]
fn test_diagnostic_program_absent() {
    let mut hooks = make_base_hooks();
    hooks.resolve_program = Box::new(|prog| Err(format!("Binary '{prog}' was not found in PATH")));

    let cfg = make_test_config("stdio:nonexistent-agent");
    let report = run_diagnostic(&cfg, &hooks);

    assert_eq!(report.checks.len(), 4);
    assert_eq!(report.checks[0].step, DiagnosticStep::ProgramExists);
    assert_eq!(report.checks[0].status, CheckStatus::Failed);
    assert!(report.checks[0].detail.contains("nonexistent-agent"));

    assert_eq!(report.checks[1].step, DiagnosticStep::NetworkReachable);
    assert_eq!(report.checks[1].status, CheckStatus::NotAttempted);

    assert_eq!(report.checks[2].step, DiagnosticStep::ProcessStarted);
    assert_eq!(report.checks[2].status, CheckStatus::NotAttempted);

    assert_eq!(report.checks[3].step, DiagnosticStep::HandshakeResponded);
    assert_eq!(report.checks[3].status, CheckStatus::NotAttempted);

    assert!(!report.is_success());
    assert_eq!(
        report.failed_step().unwrap().step,
        DiagnosticStep::ProgramExists
    );

    // Verify offered actions contain CopyError and detected agents
    assert!(
        report
            .actions
            .iter()
            .any(|a| matches!(a, DiagnosticAction::CopyError(_)))
    );
    assert!(
        report
            .actions
            .iter()
            .any(|a| matches!(a, DiagnosticAction::UseAgent(name) if name == "Claude Agent"))
    );
    assert!(report.actions.contains(&DiagnosticAction::Retry));
    assert!(report.actions.contains(&DiagnosticAction::SelectAnother));
    assert!(report.actions.contains(&DiagnosticAction::OpenConfigFile));

    // Config path was reported
    assert_eq!(
        report.config_path,
        PathBuf::from("/home/user/.crawrad/acp_config.json")
    );
}

#[test]
fn test_diagnostic_network_offline() {
    let mut hooks = make_base_hooks();
    hooks.check_network =
        Box::new(|| Err("Network unreachable: DNS resolution failed".to_string()));

    let cfg = make_test_config("npx -y @modelcontextprotocol/server-everything");
    let report = run_diagnostic(&cfg, &hooks);

    assert_eq!(report.checks.len(), 4);
    assert_eq!(report.checks[0].step, DiagnosticStep::ProgramExists);
    assert_eq!(report.checks[0].status, CheckStatus::Passed);

    assert_eq!(report.checks[1].step, DiagnosticStep::NetworkReachable);
    assert_eq!(report.checks[1].status, CheckStatus::Failed);
    assert!(report.checks[1].detail.contains("Network unreachable"));

    assert_eq!(report.checks[2].step, DiagnosticStep::ProcessStarted);
    assert_eq!(report.checks[2].status, CheckStatus::NotAttempted);

    assert_eq!(report.checks[3].step, DiagnosticStep::HandshakeResponded);
    assert_eq!(report.checks[3].status, CheckStatus::NotAttempted);

    assert!(!report.is_success());
    assert_eq!(
        report.failed_step().unwrap().step,
        DiagnosticStep::NetworkReachable
    );
    assert!(report.actions.iter().any(
        |a| matches!(a, DiagnosticAction::CopyError(err) if err.contains("Network unreachable"))
    ));
}

#[test]
fn test_diagnostic_process_dies() {
    let mut hooks = make_base_hooks();
    hooks.spawn_process = Box::new(|_, _| {
        Err("Process exited with exit code 1: missing API key credentials".to_string())
    });

    let cfg = make_test_config("stdio:claude-agent");
    let report = run_diagnostic(&cfg, &hooks);

    assert_eq!(report.checks.len(), 4);
    assert_eq!(report.checks[0].step, DiagnosticStep::ProgramExists);
    assert_eq!(report.checks[0].status, CheckStatus::Passed);

    assert_eq!(report.checks[1].step, DiagnosticStep::NetworkReachable);
    assert_eq!(report.checks[1].status, CheckStatus::NotAttempted); // local binary

    assert_eq!(report.checks[2].step, DiagnosticStep::ProcessStarted);
    assert_eq!(report.checks[2].status, CheckStatus::Failed);
    assert!(
        report.checks[2]
            .detail
            .contains("missing API key credentials")
    );

    assert_eq!(report.checks[3].step, DiagnosticStep::HandshakeResponded);
    assert_eq!(report.checks[3].status, CheckStatus::NotAttempted);

    assert!(!report.is_success());
    assert_eq!(
        report.failed_step().unwrap().step,
        DiagnosticStep::ProcessStarted
    );
}

#[test]
fn test_diagnostic_handshake_unresponsive() {
    let mut hooks = make_base_hooks();
    hooks.check_handshake =
        Box::new(|_| Err("Handshake timed out after 5000ms with no JSON-RPC response".to_string()));

    let cfg = make_test_config("stdio:custom-acp");
    let report = run_diagnostic(&cfg, &hooks);

    assert_eq!(report.checks.len(), 4);
    assert_eq!(report.checks[0].step, DiagnosticStep::ProgramExists);
    assert_eq!(report.checks[0].status, CheckStatus::Passed);

    assert_eq!(report.checks[1].step, DiagnosticStep::NetworkReachable);
    assert_eq!(report.checks[1].status, CheckStatus::NotAttempted);

    assert_eq!(report.checks[2].step, DiagnosticStep::ProcessStarted);
    assert_eq!(report.checks[2].status, CheckStatus::Passed);

    assert_eq!(report.checks[3].step, DiagnosticStep::HandshakeResponded);
    assert_eq!(report.checks[3].status, CheckStatus::Failed);
    assert!(report.checks[3].detail.contains("Handshake timed out"));

    assert!(!report.is_success());
    assert_eq!(
        report.failed_step().unwrap().step,
        DiagnosticStep::HandshakeResponded
    );
    assert!(report.actions.iter().any(
        |a| matches!(a, DiagnosticAction::CopyError(err) if err.contains("Handshake timed out"))
    ));
}

#[test]
fn test_diagnostic_happy_path_local_binary() {
    let hooks = make_base_hooks();
    let cfg = make_test_config("stdio:claude-agent");
    let report = run_diagnostic(&cfg, &hooks);

    assert_eq!(report.checks.len(), 4);
    assert_eq!(report.checks[0].status, CheckStatus::Passed);
    assert_eq!(report.checks[1].status, CheckStatus::NotAttempted); // local binary
    assert_eq!(report.checks[2].status, CheckStatus::Passed);
    assert_eq!(report.checks[3].status, CheckStatus::Passed);

    assert!(report.is_success());
    assert!(report.failed_step().is_none());

    // On success, CopyError is not offered
    assert!(
        !report
            .actions
            .iter()
            .any(|a| matches!(a, DiagnosticAction::CopyError(_)))
    );
    assert!(report.actions.contains(&DiagnosticAction::Retry));
    assert!(report.actions.contains(&DiagnosticAction::SelectAnother));
    assert!(report.actions.contains(&DiagnosticAction::OpenConfigFile));
}

#[test]
fn test_diagnostic_happy_path_network_binary() {
    let hooks = make_base_hooks();
    let cfg = make_test_config("npx -y @modelcontextprotocol/server");
    let report = run_diagnostic(&cfg, &hooks);

    assert_eq!(report.checks.len(), 4);
    assert_eq!(report.checks[0].status, CheckStatus::Passed);
    assert_eq!(report.checks[1].status, CheckStatus::Passed); // npx check executed and passed
    assert_eq!(report.checks[2].status, CheckStatus::Passed);
    assert_eq!(report.checks[3].status, CheckStatus::Passed);

    assert!(report.is_success());
}

#[test]
fn test_diagnostic_action_labels() {
    assert_eq!(DiagnosticAction::Retry.label(), "Try again");
    assert_eq!(
        DiagnosticAction::SelectAnother.label(),
        "Select another agent"
    );
    assert_eq!(DiagnosticAction::OpenConfigFile.label(), "Open config file");
    assert_eq!(
        DiagnosticAction::CopyError("fail".into()).label(),
        "Copy error details"
    );
    assert_eq!(
        DiagnosticAction::UseAgent("Codex".into()).label(),
        "Use detected agent `Codex`"
    );
}

/// No action may be produced that the dialog cannot draw.
///
/// The agent registry grows over time, so the detected list is unbounded while
/// the dialog's button slots are fixed widgets. Without this bound, an extra
/// detected agent pushes `Copy error details` past the last slot and it is
/// simply never drawn -- the user sees a panel that quietly lost an option,
/// which is the complaint that started this screen.
#[test]
fn the_report_never_carries_more_actions_than_the_dialog_can_draw() {
    use crate::acp_diagnostic::{MAX_DIAGNOSTIC_ACTIONS, MAX_QUICK_PICK_AGENTS};

    let muitos: Vec<String> = (0..50).map(|i| format!("agent-{i}")).collect();
    let mut hooks = DiagnosticHooks::live();
    let found_agents = muitos.clone();
    hooks.find_installed_agents = Box::new(move || found_agents.clone());
    hooks.resolve_program = Box::new(|_| Err("not found".to_string()));

    let cfg = AcpConfig {
        endpoint: "nao-existe-mesmo".to_string(),
        ..Default::default()
    };
    let report = run_diagnostic(&cfg, &hooks);

    assert!(
        report.actions.len() <= MAX_DIAGNOSTIC_ACTIONS,
        "the report carries {} actions but the dialog can draw only {}; the \
         extra ones would vanish with no message",
        report.actions.len(),
        MAX_DIAGNOSTIC_ACTIONS
    );

    let shortcut_count = report
        .actions
        .iter()
        .filter(|a| matches!(a, DiagnosticAction::UseAgent(_)))
        .count();
    assert_eq!(
        shortcut_count, MAX_QUICK_PICK_AGENTS,
        "with 50 agents detected, exactly the quick-pick ones are offered"
    );

    // The agents left out must stay reachable, or capping would hide them.
    assert!(
        report.actions.contains(&DiagnosticAction::SelectAnother),
        "the other 46 agents are only reachable through `Select another agent`, \
         so that action must always be there"
    );
}
