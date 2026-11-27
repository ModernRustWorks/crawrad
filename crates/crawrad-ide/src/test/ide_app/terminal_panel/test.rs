//! Tests and static gates for the Integrated Terminal panel (Aefos Terminal).

use super::casca::CASCA;
use crawrad_terminal::{PaneLayout, ShellType, SplitMode, TerminalModel};

#[test]
fn terminal_panel_is_in_dock_dsl() {
    assert!(
        CASCA.contains("PanelTerminal = <CrawradPanelTerminal> {}"),
        "PanelTerminal template is not instantiated in the shell dock"
    );
    assert!(
        CASCA.contains("tab_terminal = Tab {"),
        "tab_terminal tab is not declared in the shell dock"
    );
}

#[test]
fn terminal_panel_has_actions_and_ai_drawer() {
    let panels = include_str!("../../../ide_app/panels/terminal.rs");
    assert!(
        panels.contains("pub CrawradPanelTerminal = <CrawradPanel> {"),
        "CrawradPanelTerminal DSL disappeared from panels/terminal.rs"
    );
    assert!(
        panels.contains("btn_term_new"),
        "button btn_term_new is not declared in the terminal"
    );
    assert!(
        panels.contains("btn_term_ai"),
        "button btn_term_ai is not declared in the terminal"
    );
    assert!(
        panels.contains("term_ai_drawer"),
        "drawer term_ai_drawer is not declared in the terminal"
    );
    assert!(
        panels.contains("term_search_bar"),
        "bar term_search_bar is not declared in the terminal"
    );
    assert!(
        panels.contains("term_sidebar_left"),
        "side bar term_sidebar_left for actions/snippets disappeared"
    );
}

#[test]
fn terminal_model_supports_multiple_shells_and_splits() {
    let mut term = TerminalModel::new();
    assert_eq!(term.sessions.len(), 1);
    assert!(term.active_session().unwrap().is_welcome);

    term.open_shell(ShellType::Pwsh, "D:\\CrawRAD");
    assert_eq!(term.active_session().unwrap().title, "pwsh");

    term.open_shell(ShellType::Cmd, "D:\\CrawRAD");
    assert_eq!(term.active_session().unwrap().title, "cmd");

    term.open_shell(ShellType::Wsl, "/mnt/d/CrawRAD");
    assert_eq!(term.active_session().unwrap().title, "wsl");

    term.open_shell(ShellType::GitBash, "/d/CrawRAD");
    assert_eq!(term.active_session().unwrap().title, "git-bash");

    assert_eq!(term.sessions.len(), 5);

    let mut layout = PaneLayout::default();
    layout.split_horizontal("pwsh-1");
    layout.split_vertical("cmd-1");
    assert_eq!(layout.mode, SplitMode::Grid);
    assert_eq!(layout.panes.len(), 3);
}
