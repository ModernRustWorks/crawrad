//! The rust-analyzer pill: its phase, its detail, its spin.

use super::*;

/// **Progress makes the pill busy and fills the cell beside it; without
/// progress it is ready, with the idle note or nothing.**
#[test]
fn a_running_server_is_busy_only_while_it_reports_progress() {
    let busy = LspCell::running(SERVER, Some("Indexing (35%) 412/1180"), None);
    assert_eq!(
        (busy.phase, busy.detail.as_str()),
        (LspPhase::Busy, "Indexing (35%) 412/1180")
    );
    let idle = LspCell::running(SERVER, None, None);
    assert_eq!((idle.phase, idle.detail.as_str()), (LspPhase::Ready, ""));
    let note = LspCell::running(SERVER, None, Some("not generated"));
    assert_eq!(
        (note.phase, note.detail.as_str()),
        (LspPhase::Ready, "not generated")
    );
    assert_eq!(note.name, SERVER);
}

/// **Starting spins; no server is a failure that says why.**
#[test]
fn starting_spins_and_down_says_why() {
    assert_eq!(LspCell::starting(SERVER).phase, LspPhase::Busy);
    let down = LspCell::down(SERVER, "offline");
    assert_eq!(
        (down.phase, down.detail.as_str()),
        (LspPhase::Failed, "offline")
    );
}

/// **The shader and the MCP tell the three phases apart.**
#[test]
fn the_three_phases_have_their_own_code_and_name() {
    let all = [LspPhase::Busy, LspPhase::Ready, LspPhase::Failed];
    assert_eq!(all.map(LspPhase::code), [0.0, 1.0, 2.0]);
    assert_eq!(all.map(LspPhase::name), ["busy", "ready", "failed"]);
}

/// **One revolution every 0.9 s.**
#[test]
fn the_arc_turns_once_every_nine_tenths_of_a_second() {
    for (seconds, turned) in [(0.0, 0.0), (0.45, 0.5), (0.9, 0.0), (1.35, 0.5)] {
        assert!((turn(seconds) - turned).abs() < 1e-9, "{seconds}");
    }
}

#[test]
fn lsp_cell_for_client_status() {
    let fallback = LspCell::down("lsp", "offline");
    assert_eq!(LspCell::for_client_status(&fallback, None, None), fallback);
    let down = LspCell::for_client_status(&fallback, Some(("lsp", false, None)), None);
    assert_eq!(down.phase, LspPhase::Failed);
    assert_eq!(down.detail, "exited");
    let ready = LspCell::for_client_status(&fallback, Some(("lsp", true, None)), Some("note"));
    assert_eq!(ready.phase, LspPhase::Ready);
    assert_eq!(ready.detail, "note");
}
