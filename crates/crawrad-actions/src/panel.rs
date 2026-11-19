//! The dockable panels of the IDE, and the commands that rearrange them.
//!
//! # Why the panel list lives in the catalogue and not in a shell
//!
//! ADR 0014 (Amendment 9) made every panel the same thing: a dockable tab.
//! Explorer, Components, Object Inspector, Package Manager, Design and Chat
//! stopped being regions with an owner and became **items with an identity**,
//! and the owner's rule of 15/09/2026 says what that identity is for:
//!
//! > *"Do not forget that everything in the IDE must have actions, so that the
//! > list can later be used as tools for the MCP."*
//!
//! A panel an agent cannot name is a panel an agent cannot open. So the list
//! of panels is **catalogue data**, next to the actions that move them, and
//! every shell maps [`Panel`] to whatever it draws. The alternative — each
//! shell owning its own list — is the two-lists defect this house keeps
//! paying for: the second list is born out of date and nothing complains.
//!
//! # What this module does NOT know
//!
//! Where a panel is right now, whether it is open, and what it looks like.
//! None of that can live here: `crawrad-actions` has no window ([ADR 0005]), and
//! the same catalogue is read by a shell, by the keyboard and by the MCP. The
//! commands below are **declarative** — they say what should happen, and the
//! shell that owns a dock is the one that makes it happen.
//!
//! That is also why no command here has a predicate that reads layout state:
//! moving a panel to a side always works, resetting always works, and showing
//! a panel that is already showing is a focus. A predicate that needed to know
//! the current arrangement would have to travel through `Context`, and then
//! the MCP would need a window to ask.
//!

use crawrad_note::note;
#[note("panel-panel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Panel {
    /// The generation plan and who owns each file (ADR 0016).
    Explorer,
    /// The Toolbox and the Structure tree, stacked — the one panel that holds
    /// two, because whoever is drawing wants both at once.
    Components,
    /// The properties and the events of what is selected.
    Inspector,
    /// The crates.io package store.
    Packages,
    /// The form itself: the design canvas and the code behind it.
    Design,
    /// The agent session (ADR 0029). A mockup until the transport lands.
    Chat,
    /// Integrated multi-pane terminal environment.
    Terminal,
    /// Run and Debug: the debugger's controls docked on top, and
    /// Variables, Watch, Call Stack and Breakpoints stacked below them.
    Debug,
    /// What Find All References found, grouped by file.
    References,
    /// The build errors and warnings, grouped by file.
    Problems,
    /// The trees the extensions fill — TODOs, lints, anything.
    Extensions,
    /// Every documentation, note and comment of the project, grouped by file.
    Notes,
    /// The project's `cargo test`, as a tree package › file › test.
    Tests,
    /// What the picked test printed, under the editor.
    TestOutput,
    /// What is wrong with the open form, under the editor.
    FormCheck,
}

impl Panel {
    #[note("panel-panel-all")]
    pub const ALL: &'static [Self] = &[
        Self::Explorer,
        Self::Components,
        Self::Inspector,
        Self::Packages,
        Self::Design,
        Self::Chat,
        Self::Terminal,
        Self::Debug,
        Self::References,
        Self::Problems,
        Self::Extensions,
        Self::Notes,
        Self::Tests,
        Self::TestOutput,
        Self::FormCheck,
    ];

    #[note("panel-panel-tab-name")]
    pub fn tab_name(self) -> String {
        format!("tab_{}", self.id())
    }

    #[note("panel-panel-template-name")]
    pub fn template_name(self) -> String {
        let id = self.id();
        format!("Panel{}{}", id[..1].to_uppercase(), &id[1..])
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Explorer => "explorer",
            Self::Components => "components",
            Self::Inspector => "inspector",
            Self::Packages => "packages",
            Self::Design => "design",
            Self::Chat => "chat",
            Self::Terminal => "terminal",
            Self::Debug => "debug",
            Self::References => "references",
            Self::Problems => "problems",
            Self::Extensions => "extensions",
            Self::Notes => "notes",
            Self::Tests => "tests",
            Self::TestOutput => "testoutput",
            Self::FormCheck => "formcheck",
        }
    }

    #[note("panel-panel-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Explorer => "Explorer",
            Self::Components => "Components",
            Self::Inspector => "Object Inspector",
            Self::Packages => "Package Manager",
            Self::Design => "Design",
            Self::Chat => "Chat",
            Self::Terminal => "Terminal",
            Self::Debug => "Run and Debug",
            Self::References => "References",
            Self::Problems => "Problems",
            Self::Extensions => "Extensions",
            Self::Notes => "Notes",
            Self::Tests => "Tests",
            Self::TestOutput => "Test output",
            Self::FormCheck => "Form check",
        }
    }

    #[note("panel-panel-from-id")]
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|p| p.id() == id)
    }
}

#[note("panel-dockside")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockSide {
    Left,
    Right,
    Bottom,
}

impl DockSide {
    #[note("panel-dockside-all")]
    pub const ALL: &'static [Self] = &[Self::Left, Self::Right, Self::Bottom];

    #[note("panel-dockside-id")]
    pub fn id(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Bottom => "bottom",
        }
    }

    #[note("panel-dockside-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Bottom => "Bottom",
        }
    }

    #[note("panel-dockside-from-id")]
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|s| s.id() == id)
    }
}

#[note("panel-layoutcommand")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCommand {
    /// Open the panel if it was closed, and focus it.
    ///
    /// Open-and-focus is one command and not two because there is no useful
    /// difference for a caller: a panel behind another tab and a panel that
    /// was closed are both "not where I can see it", and an agent that had to
    /// choose between `open` and `focus` would have to know which — which
    /// means asking a window.
    Show(Panel),
    /// Close the tab. The panel keeps existing; it is the rail and this
    /// command that bring it back.
    Hide(Panel),
    /// Show it if it is not the visible tab, hide it if it is — the gesture of
    /// the activity rail, and the same one a modern editor has.
    Toggle(Panel),
    /// Dock the panel against one edge of the window.
    ///
    /// This is the command Amendment 10 puts **first**: dragging a tab goes down
    /// to the drag-and-drop of the operating system and has no arm on Linux
    /// (`makepad-platform-1.0.0/src/os/linux/linux_x11.rs`, generic arm), so
    /// the drag is comfort where the platform gives it and this is the door
    /// that is always open — for the keyboard, for the MCP, and for a test
    /// with no screen.
    Move { panel: Panel, side: DockSide },
    /// Put the panel in the same tab group as another one.
    Stack { panel: Panel, onto: Panel },
    /// Float the panel **inside** the IDE window (Amendment 8).
    ///
    /// Not a separate system window: the dock of the framework has no `Float`
    /// variant (`makepad-widgets-1.0.0/src/dock.rs:315-334` — `Splitter`,
    /// `Tabs`, `Tab`); letting a panel leave the window is future work.
    Float(Panel),
    /// Back to the arrangement the IDE ships with.
    ///
    /// Amendment 9 calls it obligatory and not decoration, and the reason is
    /// written there: an arrangement that persists without a way back is a
    /// trap — the user drags everything to the wrong place once and has no
    /// way to undo it.
    Reset,
}

impl LayoutCommand {
    #[note("panel-layoutcommand-panel")]
    pub fn panel(self) -> Option<Panel> {
        match self {
            Self::Show(p) | Self::Hide(p) | Self::Toggle(p) | Self::Float(p) => Some(p),
            Self::Move { panel, .. } | Self::Stack { panel, .. } => Some(panel),
            Self::Reset => None,
        }
    }
}

#[note("panel-profilecommand")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileCommand {
    /// Put the IDE as the profile has it.
    Apply(String),
    /// Keep the IDE as it is now under the name, replacing one of that name.
    Save(String),
    /// Forget the saved profile.
    Delete(String),
}

#[cfg(test)]
#[path = "test/panel/test.rs"]
mod tests;
