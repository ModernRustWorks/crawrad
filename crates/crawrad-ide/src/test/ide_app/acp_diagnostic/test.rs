//! Unit tests for ACP agent diagnostic dialog presentation and decisions.

use super::*;
use crawrad_agent::{
    CheckStatus, DiagnosticAction, DiagnosticCheck, DiagnosticReport, DiagnosticStep,
};
use std::path::PathBuf;

#[test]
fn test_diagnostic_report_lines_order_and_status_marks() {
    let report = DiagnosticReport {
        checks: vec![
            DiagnosticCheck {
                step: DiagnosticStep::ProgramExists,
                status: CheckStatus::Passed,
                detail: "Found at /usr/bin/claude".to_string(),
            },
            DiagnosticCheck {
                step: DiagnosticStep::NetworkReachable,
                status: CheckStatus::Passed,
                detail: "Reachable (12ms)".to_string(),
            },
            DiagnosticCheck {
                step: DiagnosticStep::ProcessStarted,
                status: CheckStatus::Failed,
                detail: "Executable format error".to_string(),
            },
            DiagnosticCheck {
                step: DiagnosticStep::HandshakeResponded,
                status: CheckStatus::NotAttempted,
                detail: "Skipped".to_string(),
            },
        ],
        config_path: PathBuf::from("/home/user/.config/acp/config.toml"),
        detected_agents: vec![
            "Claude Agent".to_string(),
            "Gemini (Antigravity)".to_string(),
        ],
        actions: vec![
            DiagnosticAction::Retry,
            DiagnosticAction::SelectAnother,
            DiagnosticAction::OpenConfigFile,
        ],
    };

    let lines = format_check_lines(&report);
    assert_eq!(
        lines.len(),
        4,
        "all 4 checks must be formatted into lines in order"
    );

    // Check 0: ProgramExists - Passed
    assert!(
        lines[0].starts_with("✓"),
        "Passed status must use '✓' mark, got: {}",
        lines[0]
    );
    assert!(
        lines[0].contains(DiagnosticStep::ProgramExists.label()),
        "Line 0 must contain step label"
    );
    assert!(
        lines[0].contains("Found at /usr/bin/claude"),
        "Line 0 must contain check detail"
    );

    // Check 1: NetworkReachable - Passed
    assert!(
        lines[1].starts_with("✓"),
        "Passed status must use '✓' mark, got: {}",
        lines[1]
    );
    assert!(
        lines[1].contains(DiagnosticStep::NetworkReachable.label()),
        "Line 1 must contain step label"
    );
    assert!(
        lines[1].contains("Reachable (12ms)"),
        "Line 1 must contain check detail"
    );

    // Check 2: ProcessStarted - Failed
    assert!(
        lines[2].starts_with("✕"),
        "Failed status must use '✕' mark, got: {}",
        lines[2]
    );
    assert!(
        lines[2].contains(DiagnosticStep::ProcessStarted.label()),
        "Line 2 must contain step label"
    );
    assert!(
        lines[2].contains("Executable format error"),
        "Line 2 must contain check detail"
    );

    // Check 3: HandshakeResponded - NotAttempted
    assert!(
        lines[3].starts_with("—"),
        "NotAttempted status must use '—' (em-dash) mark, got: {}",
        lines[3]
    );
    assert!(
        lines[3].contains(DiagnosticStep::HandshakeResponded.label()),
        "Line 3 must contain step label"
    );
    assert!(
        lines[3].contains("Skipped"),
        "Line 3 must contain check detail"
    );
}

#[test]
fn test_all_diagnostic_action_variants_have_corresponding_buttons() {
    let all_action_variants = vec![
        DiagnosticAction::UseAgent("claude-agent".to_string()),
        DiagnosticAction::UseAgent("agy".to_string()),
        DiagnosticAction::Retry,
        DiagnosticAction::SelectAnother,
        DiagnosticAction::OpenConfigFile,
        DiagnosticAction::CopyError("Process terminated with exit code 127".to_string()),
    ];

    let report = DiagnosticReport {
        checks: vec![],
        config_path: PathBuf::from("/etc/acp/config.json"),
        detected_agents: vec!["claude-agent".to_string(), "agy".to_string()],
        actions: all_action_variants.clone(),
    };

    let labels = action_button_labels(&report);
    assert_eq!(
        labels.len(),
        all_action_variants.len(),
        "number of buttons must match number of actions exactly"
    );

    for (action, label) in all_action_variants.iter().zip(labels.iter()) {
        assert_eq!(
            label,
            &action.label(),
            "button label must match action.label() with no extra text"
        );
        assert!(
            !label.trim().is_empty(),
            "action button label must not be empty"
        );
    }

    // Explicit check for every variant's specific label text
    assert_eq!(
        DiagnosticAction::UseAgent("claude-agent".to_string()).label(),
        "Use detected agent `claude-agent`"
    );
    assert_eq!(DiagnosticAction::Retry.label(), "Try again");
    assert_eq!(
        DiagnosticAction::SelectAnother.label(),
        "Select another agent"
    );
    assert_eq!(DiagnosticAction::OpenConfigFile.label(), "Open config file");
    assert_eq!(
        DiagnosticAction::CopyError("err".to_string()).label(),
        "Copy error details"
    );
}

#[test]
fn test_chip_click_decision_only_opens_when_agent_failing() {
    use crawrad_agent::chat_layout::ChipLink;

    // Normal successful ACP connection: must NOT open diagnostic
    assert!(
        !should_open_diagnostic_on_chip_click(ChipLink::Acp, false),
        "Clicking chip when agent is healthy (ACP) must not open diagnostic"
    );

    // Waiting for first response: must NOT open diagnostic
    assert!(
        !should_open_diagnostic_on_chip_click(ChipLink::Waiting, false),
        "Clicking chip when agent is waiting/connecting must not open diagnostic"
    );

    // Failed turn: MUST open diagnostic
    assert!(
        should_open_diagnostic_on_chip_click(ChipLink::Failed, false),
        "Clicking chip with failed status must open diagnostic"
    );

    // CLI fallback due to ACP unavailable: MUST open diagnostic
    assert!(
        should_open_diagnostic_on_chip_click(ChipLink::Cli, false),
        "Clicking chip with CLI fallback (ACP unavailable) must open diagnostic"
    );

    // Live failure detected (e.g. warm-up failure) even if chip link not yet updated: MUST open diagnostic
    assert!(
        should_open_diagnostic_on_chip_click(ChipLink::Acp, true),
        "Clicking chip when live status reports failure must open diagnostic"
    );
    assert!(
        should_open_diagnostic_on_chip_click(ChipLink::Waiting, true),
        "Clicking chip when live status reports failure must open diagnostic"
    );
}
