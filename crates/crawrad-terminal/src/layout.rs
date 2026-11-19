use crawrad_note::note;
#[note("layout-splitmode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMode {
    Single,
    Horizontal,
    Vertical,
    Grid,
}

#[note("layout-terminalpane")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalPane {
    pub pane_id: usize,
    pub session_id: String,
    pub is_focused: bool,
}

#[note("layout-panelayout")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneLayout {
    pub mode: SplitMode,
    pub panes: Vec<TerminalPane>,
    pub focused_index: usize,
}

impl Default for PaneLayout {
    fn default() -> Self {
        Self {
            mode: SplitMode::Single,
            panes: vec![TerminalPane {
                pane_id: 0,
                session_id: "welcome".to_string(),
                is_focused: true,
            }],
            focused_index: 0,
        }
    }
}

impl PaneLayout {
    pub fn split_horizontal(&mut self, session_id: impl Into<String>) {
        let new_id = self.panes.len();
        self.panes.push(TerminalPane {
            pane_id: new_id,
            session_id: session_id.into(),
            is_focused: true,
        });
        self.mode = match self.mode {
            SplitMode::Single => SplitMode::Horizontal,
            SplitMode::Vertical => SplitMode::Grid,
            other => other,
        };
        self.focused_index = new_id;
        self.update_focus();
    }

    pub fn split_vertical(&mut self, session_id: impl Into<String>) {
        let new_id = self.panes.len();
        self.panes.push(TerminalPane {
            pane_id: new_id,
            session_id: session_id.into(),
            is_focused: true,
        });
        self.mode = match self.mode {
            SplitMode::Single => SplitMode::Vertical,
            SplitMode::Horizontal => SplitMode::Grid,
            other => other,
        };
        self.focused_index = new_id;
        self.update_focus();
    }

    pub fn close_pane(&mut self, index: usize) {
        if self.panes.len() > 1 && index < self.panes.len() {
            self.panes.remove(index);
            if self.panes.len() == 1 {
                self.mode = SplitMode::Single;
            }
            if self.focused_index >= self.panes.len() {
                self.focused_index = self.panes.len() - 1;
            }
            self.update_focus();
        }
    }

    fn update_focus(&mut self) {
        for (i, p) in self.panes.iter_mut().enumerate() {
            p.is_focused = i == self.focused_index;
        }
    }
}
