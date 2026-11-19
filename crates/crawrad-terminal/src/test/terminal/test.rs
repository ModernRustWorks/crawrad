use crate::{PaneLayout, ShellType, SplitMode, TerminalCatalog, TerminalModel};

#[test]
fn terminal_model_lifecycle() {
    let mut term = TerminalModel::new();
    assert_eq!(term.sessions.len(), 1);
    assert!(term.active_session().unwrap().is_welcome);

    let idx = term.open_shell(ShellType::Pwsh, "D:\\CrawRAD");
    assert_eq!(idx, 1);
    assert_eq!(term.sessions.len(), 2);
    assert_eq!(term.active_session().unwrap().title, "pwsh");
    assert_eq!(term.active_session().unwrap().cwd, "D:\\CrawRAD");

    term.toggle_ai_drawer();
    assert!(term.ai_drawer_open);

    term.toggle_actions();
    assert!(term.actions_panel_open);
    assert!(!term.snippets_panel_open);

    term.toggle_snippets();
    assert!(term.snippets_panel_open);
    assert!(!term.actions_panel_open);

    term.toggle_search();
    assert!(term.search_open);

    term.close_session(idx);
    assert_eq!(term.sessions.len(), 1);
    assert!(term.active_session().unwrap().is_welcome);
}

#[test]
fn terminal_pane_splitting() {
    let mut layout = PaneLayout::default();
    assert_eq!(layout.mode, SplitMode::Single);
    assert_eq!(layout.panes.len(), 1);

    layout.split_horizontal("pwsh-1");
    assert_eq!(layout.mode, SplitMode::Horizontal);
    assert_eq!(layout.panes.len(), 2);

    layout.split_vertical("cmd-2");
    assert_eq!(layout.mode, SplitMode::Grid);
    assert_eq!(layout.panes.len(), 3);

    layout.close_pane(2);
    assert_eq!(layout.panes.len(), 2);
}

#[test]
fn terminal_model_multi_shell_and_splits() {
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

#[test]
fn terminal_catalog_defaults() {
    let actions = TerminalCatalog::default_actions();
    assert!(!actions.is_empty());
    assert!(actions.iter().any(|a| a.id == "git_pull"));
    assert!(actions.iter().any(|a| a.id == "build_debug"));

    let snippets = TerminalCatalog::default_snippets();
    assert!(!snippets.is_empty());
    assert!(snippets.iter().any(|s| s.id == "cargo_test"));
}

#[test]
fn terminal_session_append_output() {
    let mut term = TerminalModel::new();
    term.open_shell(ShellType::Pwsh, "C:\\Test");
    let session = term.active_session_mut().unwrap();
    let initial_len = session.buffer.len();
    session.append_output("line 1\nline 2");
    assert_eq!(session.buffer.len(), initial_len + 2);
}
