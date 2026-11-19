#![forbid(unsafe_code)]

pub mod actions;
pub mod layout;
pub mod session;

pub use actions::{TerminalAction, TerminalCatalog, TerminalSnippet};
use crawrad_note::note;
pub use layout::{PaneLayout, SplitMode, TerminalPane};
pub use session::{ShellType, TerminalSession};

#[note("terminalmodel")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalModel {
    pub sessions: Vec<TerminalSession>,
    pub active_session_idx: usize,
    pub layout: PaneLayout,
    pub actions: Vec<TerminalAction>,
    pub snippets: Vec<TerminalSnippet>,
    pub ai_drawer_open: bool,
    pub actions_panel_open: bool,
    pub snippets_panel_open: bool,
    pub search_open: bool,
    pub search_query: String,
}

impl Default for TerminalModel {
    fn default() -> Self {
        Self {
            sessions: vec![TerminalSession::new_welcome()],
            active_session_idx: 0,
            layout: PaneLayout::default(),
            actions: TerminalCatalog::default_actions(),
            snippets: TerminalCatalog::default_snippets(),
            ai_drawer_open: false,
            actions_panel_open: false,
            snippets_panel_open: false,
            search_open: false,
            search_query: String::new(),
        }
    }
}

impl TerminalModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_shell(&mut self, shell: ShellType, cwd: &str) -> usize {
        let session = TerminalSession::new_shell(shell, cwd);
        self.sessions.push(session);
        self.active_session_idx = self.sessions.len() - 1;
        self.active_session_idx
    }

    pub fn close_session(&mut self, index: usize) {
        if self.sessions.len() > 1 && index < self.sessions.len() {
            self.sessions.remove(index);
            if self.active_session_idx >= self.sessions.len() {
                self.active_session_idx = self.sessions.len() - 1;
            }
        }
    }

    pub fn active_session(&self) -> Option<&TerminalSession> {
        self.sessions.get(self.active_session_idx)
    }

    pub fn active_session_mut(&mut self) -> Option<&mut TerminalSession> {
        self.sessions.get_mut(self.active_session_idx)
    }

    pub fn toggle_ai_drawer(&mut self) {
        self.ai_drawer_open = !self.ai_drawer_open;
    }

    pub fn toggle_actions(&mut self) {
        self.actions_panel_open = !self.actions_panel_open;
        if self.actions_panel_open {
            self.snippets_panel_open = false;
        }
    }

    pub fn toggle_snippets(&mut self) {
        self.snippets_panel_open = !self.snippets_panel_open;
        if self.snippets_panel_open {
            self.actions_panel_open = false;
        }
    }

    pub fn toggle_search(&mut self) {
        self.search_open = !self.search_open;
    }
}

#[cfg(test)]
#[path = "test/terminal/test.rs"]
mod test;
