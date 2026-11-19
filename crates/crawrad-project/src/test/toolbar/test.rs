//! Tests for toolbar commands and their roles.

use crate::icons::Chrome;
use crate::toolbar::Command;

#[test]
fn build_command_is_positioned_right_before_run() {
    let build_index = Command::ALL
        .iter()
        .position(|&command| command == Command::Build)
        .expect("Build command should exist in Command::ALL");
    let run_index = Command::ALL
        .iter()
        .position(|&command| command == Command::Run)
        .expect("Run command should exist in Command::ALL");
    assert_eq!(
        build_index + 1,
        run_index,
        "Command::Build must be positioned immediately before Command::Run in Command::ALL"
    );
}

#[test]
fn build_command_has_expected_action_and_icon() {
    assert_eq!(Command::Build.action(), "project.build_manager");
    assert_eq!(Command::Build.icon(), Chrome::Build);
}

#[test]
fn build_chrome_role_glyph_and_name() {
    assert_eq!(Chrome::Build.glyph(), "\u{eb6d}");
    assert_eq!(Chrome::Build.codicon_name(), "tools");
}
