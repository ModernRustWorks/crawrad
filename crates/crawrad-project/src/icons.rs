//! The ICONS of the Makepad shell — the chrome ones, and the catalogue ones.
//!
//! # Two sets, because there are two questions
//!
//! *What does this button DO?* is answered by [`Chrome`], a glyph of the
//! Codicons font. *Which component is this?* is answered by [`ClassIcon`], a
//! drawing on a 16x16 grid. They are the same two sets the egui shell already
//! ships (`crates/crawrad-ide/src/icones.rs`), and they never mix in one list.
//!
//! # Why Codicons and not the Font Awesome that ships with makepad-widgets
//!
//! The rail used to draw Font Awesome Solid, which `makepad-widgets` embeds as
//! `THEME_FONT_ICONS`
//! (`makepad-widgets-1.0.0/src/theme_desktop_dark.rs:410-415`). It was the
//! cheapest thing that worked, and it is the wrong set here for two reasons,
//! both MEASURED by reading the `cmap` of both files:
//!
//! 1. **It is not the set the owner approved.** The egui shell — the one he
//!    judges by — draws Codicons, and the roles below are the same thirteen it
//!    names (ADR 0026). Two shells of one IDE drawing two icon families is the
//!    "set that is not a set" he already rejected once.
//! 2. **Font Awesome cannot share a font family with text.** It maps the whole
//!    latin alphabet: `U+0041`..`U+005A` and `U+0061`..`U+007A` are icons in
//!    it, 81 code points below `U+0100` in total. Codicons maps **zero** below
//!    `U+0100` — its 639 glyphs live in `U+EA60`..`U+ECE7`. Since a font family
//!    in Makepad is a per-glyph fallback chain
//!    (`makepad-draw-1.0.0/src/text/shaper.rs:80-99`: a glyph that resolves to
//!    id 0 is re-shaped with the remaining fonts), Codicons can sit in front of
//!    a text face and Font Awesome cannot — it would eat every letter.
//!
//! # The code points go through RUST, never through the DSL
//!
//! The `live_design!` tokenizer knows three escapes — `\\`, `\"` and `\n`
//! (`makepad-live-tokenizer-1.0.0/src/tokenizer.rs:530-550`) — so a
//! `"\u{eb4b}"` written in the DSL shows up spelled out on screen. Writing the
//! raw character instead works, and hides the value from anyone reading the
//! file. Here it is Rust, `\u{}` means what it says, and the shell writes the
//! text at startup.
//!
//! # These tables are duplicates, and the duplication is nailed down
//!
//! The egui shell cannot be imported from here: `icones.rs` is built on
//! `eframe::egui` and the Makepad binary does not carry it. So the values live
//! twice and a test compares them character by character
//! (`src/test/icones/test.rs`), the same arrangement `ide_app/tema.rs` uses
//! against `tema.rs`. Duplication a test holds together beats sharing that is
//! only claimed.

use crawrad_note::note;
#[note("icons-rail-role")]
pub fn rail_role(panel: crawrad_actions::Panel) -> Option<Chrome> {
    use crawrad_actions::Panel as P;
    match panel {
        P::Explorer => Some(Chrome::Explorer),
        P::Components => Some(Chrome::Components),
        P::Inspector => Some(Chrome::Inspector),
        P::Packages => Some(Chrome::Packages),
        P::Debug => Some(Chrome::Debug),
        P::Extensions => Some(Chrome::ExtensionViews),
        P::Tests => Some(Chrome::Tests),
        P::Design
        | P::Chat
        | P::Terminal
        | P::References
        | P::Problems
        | P::Notes
        | P::TestOutput
        | P::FormCheck => None,
    }
}

#[note("icons-rail-glyph")]
pub fn rail_glyph(panel: crawrad_actions::Panel) -> &'static str {
    rail_role(panel).map(|r| r.glyph()).unwrap_or("")
}

pub const IC_NOTA: &str = concat!(
    "One row per chrome role, at 12 / 18 / 24 pt, with the Codicons name. ",
    "The egui shell compares TWO families here because it has two; this shell ",
    "has one (ADR 0026), so what is left is the sheet that shows a glyph ",
    "actually rendering.",
);

#[note("icons-chrome")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chrome {
    Undo,
    Redo,
    Save,
    SaveAll,
    /// Build manager action.
    Build,
    Run,
    /// The designer's Preview: the form answers clicks and typing.
    Preview,
    /// The rail: the generation plan and who owns each file.
    Explorer,
    /// The rail: the palette and the tree.
    Components,
    /// The rail: properties and events of the selection.
    Inspector,
    /// The rail: the crates.io store.
    Packages,
    /// The rail: the trees the extensions fill.
    ExtensionViews,
    /// The rail: the project's tests.
    Tests,
    /// The foot of the rail — and it is not a rail destination.
    Settings,
    /// **The `+` of a panel header** — create a new one of whatever this panel
    /// holds.
    ///
    /// # It was `NewChat`, and the gate is why it is not any more
    ///
    /// The collection header needed a `+` too, and it came in as a second role
    /// with the same Codicons `add`. `no_chrome_role_repeats_another_ones_drawing`
    /// refused it, and the refusal was right for a reason worth writing down:
    /// **two roles that draw the same thing are usually one role with two
    /// names**. Starting a conversation and adding an item to a collection are
    /// the same gesture — *create a new one of what is here* — and a set that
    /// says so with one symbol is a set.
    ///
    /// Collapsing it beat giving the collection a different glyph, which would
    /// have meant picking a drawing that means something else just to get past
    /// a guard.
    ///
    /// Its two users today: the AI panel's new conversation, and the `+` of a
    /// collection in the Structure header — which exists because a collection
    /// item is out of the palette on purpose (`Action` carries `palette: ""`),
    /// so until 18/09/2026 there was **no way at all** to create one.
    New,
    /// The AI panel: wipe the conversation.
    ClearChat,
    /// **Close what is in front** — the `X` of a panel header and of an editor tab.
    ///
    /// It has been renamed twice, and both times by the same rule. It was
    /// `CloseChat` while the AI panel was the only thing with a close button;
    /// the dock (ADR 0014, Emenda 9) gave every panel one, and it became
    /// `ClosePanel`. The editor tab strip gave the same `X` to something that
    /// is not a panel at all, so it is now just `Close`.
    ///
    /// A role named after one of its users gets read as *"the X of the chat"*
    /// by whoever adds the next — and the drawing never changed: it is
    /// Codicons `close` all three times.
    Close,
    /// **An editor tab with unsaved work** — the dot VS Code puts where the
    /// `X` is.
    ///
    /// Here it sits BESIDE the name instead of replacing the `X`, and the
    /// difference is measured on this shell and not on that one: VS Code can
    /// swap the two because it brings the `X` back when the pointer is over
    /// the tab, and nothing in this strip reacts to hover yet. Swapping now
    /// would hide the close button of exactly the tab you most need to close.
    Unsaved,
    /// The tick of a menu row that mirrors a STATE — `View > Explorer`.
    ///
    /// Not a button: it is the only chrome role nobody clicks. It is here
    /// anyway because it is a drawing of the same set, and a check mark
    /// written as a loose `"OK"` glyph somewhere in the shell would be the
    /// very "glyph that is not part of a set" the owner already rejected once
    /// (ADR 0026) — `U+2713` is not in the text face either, so it would come
    /// out as an empty box.
    Check,
    /// The error count in the status bar — how many the last build found.
    ///
    /// It is here and not in the Explorer set because it counts a BUILD, not a
    /// file: the Explorer's `WARNING` marks a leftover file on disk, which is
    /// a different fact that only happens to look alarming too.
    Error,
    /// The warning count in the status bar.
    Warning,
    /// The zoom of the canvas, in the status bar.
    Zoom,
    /// Chat tools: attach file / context (paperclip)
    Attach,
    /// Chat tools: memory / instructions (lightbulb sparkle)
    Memory,
    /// Chat tools: MCP servers (plug)
    Mcp,
    /// Chat header: conversation history (clock)
    History,
    /// Chat tools: AI provider / agent (hubot)
    Agent,
    /// Editor tab actions: AI / sparkle action
    Sparkle,
    /// Editor tab actions: Split editor
    SplitHorizontal,
    /// Editor tab actions: More actions / three dots
    More,
    /// Chevron down icon (collapsible expand)
    ChevronDown,
    /// Chevron right icon (collapsible contract)
    ChevronRight,
    /// Arrow right / send action
    ArrowRight,
    /// Edit item action (pencil)
    Edit,
    /// Queue / layers / stack icon
    Queue,
    /// Explorer: New file
    NewFile,
    /// Explorer: New folder
    NewFolder,
    /// Explorer: New form
    NewForm,
    /// Explorer: Refresh
    Refresh,
    /// Explorer: Collapse all
    CollapseAll,
    /// A menu notice: what is locked here, and why.
    Lock,
    /// The rail: Run and Debug.
    Debug,
    /// Run and Debug: continue the program.
    DebugContinue,
    /// Run and Debug: pause the program.
    DebugPause,
    /// Run and Debug: step over the line.
    StepOver,
    /// Run and Debug: step into the call.
    StepInto,
    /// Run and Debug: step out of the function.
    StepOut,
    /// Run and Debug: stop the program.
    DebugStop,
    /// Run under the debugger: the filled triangle of the Run button and menu.
    Start,
}

impl Chrome {
    #[note("icons-chrome-all")]
    pub const ALL: &'static [Self] = &[
        Self::Undo,
        Self::Redo,
        Self::Save,
        Self::SaveAll,
        Self::Build,
        Self::Run,
        Self::Preview,
        Self::Explorer,
        Self::Components,
        Self::Inspector,
        Self::Packages,
        Self::ExtensionViews,
        Self::Tests,
        Self::Settings,
        Self::New,
        Self::ClearChat,
        Self::Close,
        Self::Unsaved,
        Self::Check,
        Self::Error,
        Self::Warning,
        Self::Zoom,
        Self::Attach,
        Self::Memory,
        Self::Mcp,
        Self::History,
        Self::Agent,
        Self::Sparkle,
        Self::SplitHorizontal,
        Self::More,
        Self::ChevronDown,
        Self::ChevronRight,
        Self::ArrowRight,
        Self::Edit,
        Self::Queue,
        Self::NewFile,
        Self::NewFolder,
        Self::NewForm,
        Self::Refresh,
        Self::CollapseAll,
        Self::Lock,
        Self::Debug,
        Self::DebugContinue,
        Self::DebugPause,
        Self::StepOver,
        Self::StepInto,
        Self::StepOut,
        Self::DebugStop,
        Self::Start,
    ];

    #[note("icons-chrome-glyph")]
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Undo => "\u{eae2}",
            Self::Redo => "\u{ebb0}",
            Self::Save => "\u{eb4b}",
            Self::SaveAll => "\u{eb49}",
            Self::Build => "\u{eb6d}",
            Self::Run => "\u{eb2c}",
            Self::Preview => "\u{ea70}",
            Self::Explorer => "\u{eaf0}",
            Self::Components => "\u{eb9c}",
            Self::Inspector => "\u{eb65}",
            Self::Packages => "\u{eb29}",
            Self::ExtensionViews => "\u{eae6}",
            Self::Tests => "\u{ea79}",
            Self::Settings => "\u{eb51}",
            Self::New => "\u{ea60}",
            Self::ClearChat => "\u{ea81}",
            Self::Close => "\u{ea76}",
            Self::Unsaved => "\u{ea71}",
            Self::Check => "\u{eab2}",
            Self::Error => "\u{ea87}",
            Self::Warning => "\u{ea6c}",
            Self::Zoom => "\u{eb81}",
            Self::Attach => "\u{ec34}",
            Self::Memory => "\u{ec1f}",
            Self::Mcp => "\u{eb2d}",
            Self::History => "\u{ea82}",
            Self::Agent => "\u{eb08}",
            Self::Sparkle => "\u{ec10}",
            Self::SplitHorizontal => "\u{eb56}",
            Self::More => "\u{ea7c}",
            Self::ChevronDown => "\u{eab4}",
            Self::ChevronRight => "\u{eab6}",
            Self::ArrowRight => "\u{ea9c}",
            Self::Edit => "\u{ea73}",
            Self::Queue => "\u{ebd2}",
            Self::NewFile => "\u{ea7f}",
            Self::NewFolder => "\u{ea80}",
            Self::NewForm => "\u{eb7f}",
            Self::Refresh => "\u{eb37}",
            Self::CollapseAll => "\u{eac5}",
            Self::Lock => "\u{ea75}",
            Self::Debug => "\u{eb91}",
            Self::DebugContinue => "\u{eacf}",
            Self::DebugPause => "\u{ead1}",
            Self::StepOver => "\u{ead6}",
            Self::StepInto => "\u{ead4}",
            Self::StepOut => "\u{ead5}",
            Self::DebugStop => "\u{ead7}",
            Self::Start => "\u{eb70}",
        }
    }

    #[note("icons-chrome-for-action")]
    pub fn for_action(action: &str) -> Option<Self> {
        match action {
            "project.run" => Some(Self::Start),
            "project.run_without_debugging" => Some(Self::Run),
            _ => None,
        }
    }

    #[note("icons-chrome-codicon-name")]
    pub fn codicon_name(self) -> &'static str {
        match self {
            Self::Undo => "discard",
            Self::Redo => "redo",
            Self::Save => "save",
            Self::SaveAll => "save-all",
            Self::Build => "tools",
            Self::Run => "play",
            Self::Preview => "eye",
            Self::Explorer => "files",
            Self::Components => "library",
            Self::Inspector => "symbol-property",
            Self::Packages => "package",
            Self::ExtensionViews => "extensions",
            Self::Tests => "beaker",
            Self::Settings => "settings-gear",
            Self::New => "add",
            Self::ClearChat => "trash",
            Self::Close => "close",
            Self::Unsaved => "circle-filled",
            Self::Check => "check",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Zoom => "zoom-in",
            Self::Attach => "attach",
            Self::Memory => "lightbulb-sparkle",
            Self::Mcp => "plug",
            Self::History => "history",
            Self::Agent => "hubot",
            Self::Sparkle => "sparkle",
            Self::SplitHorizontal => "split-horizontal",
            Self::More => "ellipsis",
            Self::ChevronDown => "chevron-down",
            Self::ChevronRight => "chevron-right",
            Self::ArrowRight => "arrow-right",
            Self::Edit => "edit",
            Self::Queue => "layers",
            Self::NewFile => "new-file",
            Self::NewFolder => "new-folder",
            Self::NewForm => "window",
            Self::Refresh => "refresh",
            Self::CollapseAll => "collapse-all",
            Self::Lock => "lock",
            Self::Debug => "debug-alt",
            Self::DebugContinue => "debug-continue",
            Self::DebugPause => "debug-pause",
            Self::StepOver => "debug-step-over",
            Self::StepInto => "debug-step-into",
            Self::StepOut => "debug-step-out",
            Self::DebugStop => "debug-stop",
            Self::Start => "triangle-right",
        }
    }
}

#[note("icons-classicon")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassIcon {
    /// One action of an `ActionList`. Only the Structure shows it.
    Action,
    ActionList,
    Button,
    CheckBox,
    ComboBox,
    Edit,
    Form,
    GroupBox,
    ImageList,
    Label,
    ListBox,
    Memo,
    PageControl,
    Panel,
    RadioButton,
    ScrollBar,
    ScrollBox,
    TabSheet,
    Timer,
    TreeView,
    Image,
    Grid,
    BitBtn,
    SpeedButton,
    MainMenu,
    PopupMenu,
    ToolBar,
    StatusBar,
    ProgressBar,
    TrackBar,
    PaintBox,
    Shape,
    OpenDialog,
    SaveDialog,
    Splitter,
    RadioGroup,
    SpinEdit,
    Toggle,
    ColorBox,
    LinkLabel,
    Bevel,
    CategoryPanel,
    ListView,
    MessageDialog,
    Card,
    Badge,
    Chips,
    Breadcrumb,
    Pagination,
    Drawer,
    MultiSelect,
    CheckListBox,
    MaskEdit,
    DateTimePicker,
}

impl ClassIcon {
    #[note("icons-classicon-all")]
    pub const ALL: &'static [Self] = &[
        Self::Action,
        Self::ActionList,
        Self::Button,
        Self::CheckBox,
        Self::ComboBox,
        Self::Edit,
        Self::Form,
        Self::GroupBox,
        Self::ImageList,
        Self::Label,
        Self::ListBox,
        Self::Memo,
        Self::PageControl,
        Self::Panel,
        Self::RadioButton,
        Self::ScrollBar,
        Self::ScrollBox,
        Self::TabSheet,
        Self::Timer,
        Self::TreeView,
        Self::Image,
        Self::Grid,
        Self::BitBtn,
        Self::SpeedButton,
        Self::MainMenu,
        Self::PopupMenu,
        Self::ToolBar,
        Self::StatusBar,
        Self::ProgressBar,
        Self::TrackBar,
        Self::PaintBox,
        Self::Shape,
        Self::OpenDialog,
        Self::SaveDialog,
        Self::Splitter,
        Self::RadioGroup,
        Self::SpinEdit,
        Self::Toggle,
        Self::ColorBox,
        Self::LinkLabel,
        Self::Bevel,
        Self::CategoryPanel,
        Self::ListView,
        Self::MessageDialog,
        Self::Card,
        Self::Badge,
        Self::Chips,
        Self::Breadcrumb,
        Self::Pagination,
        Self::Drawer,
        Self::MultiSelect,
        Self::CheckListBox,
        Self::MaskEdit,
        Self::DateTimePicker,
    ];

    #[note("icons-classicon-class")]
    pub fn class(self) -> &'static str {
        match self {
            Self::Action => "Action",
            Self::ActionList => "ActionList",
            Self::Button => "Button",
            Self::CheckBox => "CheckBox",
            Self::ComboBox => "ComboBox",
            Self::Edit => "Edit",
            Self::Form => "Form",
            Self::GroupBox => "GroupBox",
            Self::ImageList => "ImageList",
            Self::Label => "Label",
            Self::ListBox => "ListBox",
            Self::Memo => "Memo",
            Self::PageControl => "PageControl",
            Self::Panel => "Panel",
            Self::RadioButton => "RadioButton",
            Self::ScrollBar => "ScrollBar",
            Self::ScrollBox => "ScrollBox",
            Self::TabSheet => "TabSheet",
            Self::Timer => "Timer",
            Self::TreeView => "TreeView",
            Self::Image => "Image",
            Self::Grid => "Grid",
            Self::BitBtn => "BitBtn",
            Self::SpeedButton => "SpeedButton",
            Self::MainMenu => "MainMenu",
            Self::PopupMenu => "PopupMenu",
            Self::ToolBar => "ToolBar",
            Self::StatusBar => "StatusBar",
            Self::ProgressBar => "ProgressBar",
            Self::TrackBar => "TrackBar",
            Self::PaintBox => "PaintBox",
            Self::Shape => "Shape",
            Self::OpenDialog => "OpenDialog",
            Self::SaveDialog => "SaveDialog",
            Self::Splitter => "Splitter",
            Self::RadioGroup => "RadioGroup",
            Self::SpinEdit => "SpinEdit",
            Self::Toggle => "Toggle",
            Self::ColorBox => "ColorBox",
            Self::LinkLabel => "LinkLabel",
            Self::Bevel => "Bevel",
            Self::CategoryPanel => "CategoryPanel",
            Self::ListView => "ListView",
            Self::MessageDialog => "MessageDialog",
            Self::Card => "Card",
            Self::Badge => "Badge",
            Self::Chips => "Chips",
            Self::Breadcrumb => "Breadcrumb",
            Self::Pagination => "Pagination",
            Self::Drawer => "Drawer",
            Self::MultiSelect => "MultiSelect",
            Self::CheckListBox => "CheckListBox",
            Self::MaskEdit => "MaskEdit",
            Self::DateTimePicker => "DateTimePicker",
        }
    }

    #[note("icons-classicon-of-class")]
    pub fn of_class(class: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|i| i.class() == class)
    }

    #[note("icons-classicon-kind")]
    pub fn kind(self) -> f64 {
        Self::ALL
            .iter()
            .position(|i| *i == self)
            .expect("every variant is in ALL") as f64
    }

    #[note("icons-classicon-steps")]
    pub fn steps(self) -> &'static [crawrad_ir::icon::IconStep] {
        use crawrad_ir::icon::IconStep::*;
        match self {
            Self::Action => &[
                MoveTo(4.0, 3.5),
                LineTo(12.5, 8.0),
                LineTo(4.0, 12.5),
                ClosePath,
                Fill,
            ],
            Self::ActionList => &[
                MoveTo(1.0, 1.5),
                LineTo(4.5, 3.5),
                LineTo(1.0, 5.5),
                ClosePath,
                Fill,
                MoveTo(1.0, 6.0),
                LineTo(4.5, 8.0),
                LineTo(1.0, 10.0),
                ClosePath,
                Fill,
                MoveTo(1.0, 10.5),
                LineTo(4.5, 12.5),
                LineTo(1.0, 14.5),
                ClosePath,
                Fill,
                MoveTo(6.0, 3.5),
                LineTo(15.0, 3.5),
                MoveTo(6.0, 8.0),
                LineTo(15.0, 8.0),
                MoveTo(6.0, 12.5),
                LineTo(15.0, 12.5),
                Stroke,
            ],
            Self::Button => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 1.25,
                },
                Stroke,
                MoveTo(4.5, 8.0),
                LineTo(11.5, 8.0),
                Stroke,
            ],
            Self::CheckBox => &[
                Box {
                    x: 2.0,
                    y: 2.0,
                    w: 12.0,
                    h: 12.0,
                    r: 1.0,
                },
                Stroke,
                MoveTo(4.8, 8.4),
                LineTo(7.0, 10.6),
                LineTo(11.2, 5.2),
                Stroke,
            ],
            Self::ComboBox => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(10.5, 3.0),
                LineTo(10.5, 13.0),
                MoveTo(11.8, 7.2),
                LineTo(12.75, 9.2),
                LineTo(13.7, 7.2),
                Stroke,
            ],
            Self::Edit => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(4.0, 5.5),
                LineTo(4.0, 10.5),
                Stroke,
            ],
            // The Edit's box, its positions waiting as blanks
            Self::MaskEdit => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(3.5, 10.5),
                LineTo(5.5, 10.5),
                MoveTo(7.0, 10.5),
                LineTo(9.0, 10.5),
                MoveTo(10.5, 10.5),
                LineTo(12.5, 10.5),
                Stroke,
            ],
            // A calendar page: the box, its two rings, the header and
            // the grid of days
            Self::DateTimePicker => &[
                Box {
                    x: 1.5,
                    y: 2.5,
                    w: 13.0,
                    h: 12.0,
                    r: 0.8,
                },
                Stroke,
                MoveTo(4.5, 1.2),
                LineTo(4.5, 3.2),
                MoveTo(11.5, 1.2),
                LineTo(11.5, 3.2),
                MoveTo(1.5, 6.0),
                LineTo(14.5, 6.0),
                MoveTo(1.5, 10.25),
                LineTo(14.5, 10.25),
                MoveTo(5.8, 6.0),
                LineTo(5.8, 14.5),
                MoveTo(10.2, 6.0),
                LineTo(10.2, 14.5),
                Stroke,
            ],
            Self::Form => &[
                Box {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(1.0, 5.5),
                LineTo(15.0, 5.5),
                Stroke,
                Circle {
                    x: 11.6,
                    y: 3.75,
                    r: 0.7,
                },
                Circle {
                    x: 13.8,
                    y: 3.75,
                    r: 0.7,
                },
                Fill,
            ],
            Self::GroupBox => &[
                MoveTo(1.0, 3.5),
                LineTo(4.0, 3.5),
                MoveTo(10.0, 3.5),
                LineTo(15.0, 3.5),
                MoveTo(1.0, 3.5),
                LineTo(1.0, 14.0),
                MoveTo(15.0, 3.5),
                LineTo(15.0, 14.0),
                MoveTo(1.0, 14.0),
                LineTo(15.0, 14.0),
                Stroke,
            ],
            Self::ImageList => &[
                Box {
                    x: 5.0,
                    y: 1.5,
                    w: 10.0,
                    h: 8.0,
                    r: 0.5,
                },
                Stroke,
                Box {
                    x: 1.0,
                    y: 6.5,
                    w: 10.0,
                    h: 8.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(2.0, 13.2),
                LineTo(4.9, 10.0),
                LineTo(6.6, 12.0),
                LineTo(8.2, 10.4),
                LineTo(10.0, 13.2),
                Stroke,
                Circle {
                    x: 3.6,
                    y: 9.0,
                    r: 0.9,
                },
                Fill,
            ],
            Self::Label => &[
                MoveTo(2.0, 14.0),
                LineTo(8.0, 2.2),
                LineTo(14.0, 14.0),
                MoveTo(4.6, 10.0),
                LineTo(11.4, 10.0),
                Stroke,
            ],
            Self::ListBox => &[
                Box {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(3.2, 8.4),
                LineTo(12.8, 8.4),
                MoveTo(3.2, 11.4),
                LineTo(12.8, 11.4),
                Stroke,
                Box {
                    x: 2.4,
                    y: 3.6,
                    w: 11.2,
                    h: 2.0,
                    r: 0.25,
                },
                Fill,
            ],
            Self::Memo => &[
                Box {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(3.0, 5.0),
                LineTo(13.0, 5.0),
                MoveTo(3.0, 8.0),
                LineTo(11.5, 8.0),
                MoveTo(3.0, 11.0),
                LineTo(8.5, 11.0),
                Stroke,
            ],
            Self::PageControl => &[
                MoveTo(1.0, 7.0),
                LineTo(1.0, 3.5),
                LineTo(6.5, 3.5),
                LineTo(6.5, 7.0),
                MoveTo(7.6, 7.0),
                LineTo(7.6, 5.0),
                LineTo(13.0, 5.0),
                LineTo(13.0, 7.0),
                Stroke,
                Box {
                    x: 1.0,
                    y: 7.0,
                    w: 14.0,
                    h: 7.0,
                    r: 0.5,
                },
                Stroke,
            ],
            Self::Panel => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 0.5,
                },
                Stroke,
                Box {
                    x: 3.6,
                    y: 5.3,
                    w: 8.8,
                    h: 5.4,
                    r: 0.25,
                },
                Stroke,
            ],
            Self::RadioButton => &[
                Circle {
                    x: 8.0,
                    y: 8.0,
                    r: 5.4,
                },
                Stroke,
                Circle {
                    x: 8.0,
                    y: 8.0,
                    r: 2.4,
                },
                Fill,
            ],
            Self::ScrollBar => &[
                Box {
                    x: 1.0,
                    y: 2.5,
                    w: 14.0,
                    h: 11.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(4.0, 2.5),
                LineTo(4.0, 13.5),
                MoveTo(3.2, 6.5),
                LineTo(2.0, 8.0),
                LineTo(3.2, 9.5),
                MoveTo(12.0, 2.5),
                LineTo(12.0, 13.5),
                MoveTo(12.8, 6.5),
                LineTo(14.0, 8.0),
                LineTo(12.8, 9.5),
                Stroke,
                Box {
                    x: 6.5,
                    y: 5.5,
                    w: 3.0,
                    h: 5.0,
                    r: 0.25,
                },
                Fill,
            ],
            Self::ScrollBox => &[
                Box {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(12.0, 2.0),
                LineTo(12.0, 11.0),
                MoveTo(1.0, 11.0),
                LineTo(15.0, 11.0),
                Stroke,
                Box {
                    x: 12.5,
                    y: 4.0,
                    w: 2.0,
                    h: 4.5,
                    r: 0.25,
                },
                Box {
                    x: 4.0,
                    y: 11.5,
                    w: 4.5,
                    h: 2.0,
                    r: 0.25,
                },
                Fill,
            ],
            Self::TabSheet => &[
                MoveTo(1.0, 7.0),
                LineTo(1.0, 3.5),
                LineTo(8.0, 3.5),
                LineTo(8.0, 7.0),
                Stroke,
                Box {
                    x: 1.0,
                    y: 7.0,
                    w: 14.0,
                    h: 7.0,
                    r: 0.5,
                },
                Stroke,
            ],
            Self::Timer => &[
                Circle {
                    x: 8.0,
                    y: 8.0,
                    r: 5.4,
                },
                Stroke,
                MoveTo(8.0, 8.0),
                LineTo(8.0, 4.6),
                MoveTo(8.0, 8.0),
                LineTo(10.8, 8.8),
                Stroke,
            ],
            Self::TreeView => &[
                Box {
                    x: 1.0,
                    y: 2.4,
                    w: 4.0,
                    h: 4.0,
                    r: 0.25,
                },
                Stroke,
                MoveTo(2.2, 4.4),
                LineTo(3.8, 4.4),
                MoveTo(6.6, 4.4),
                LineTo(15.0, 4.4),
                MoveTo(3.0, 6.4),
                LineTo(3.0, 12.6),
                MoveTo(3.0, 9.0),
                LineTo(5.6, 9.0),
                MoveTo(7.0, 9.0),
                LineTo(15.0, 9.0),
                MoveTo(3.0, 12.6),
                LineTo(5.6, 12.6),
                MoveTo(7.0, 12.6),
                LineTo(15.0, 12.6),
                Stroke,
            ],
            Self::Image => &[
                Box {
                    x: 1.5,
                    y: 2.0,
                    w: 13.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(3.0, 12.0),
                LineTo(6.5, 7.5),
                LineTo(9.0, 10.5),
                LineTo(11.0, 8.5),
                LineTo(13.0, 12.0),
                Stroke,
                Circle {
                    x: 5.0,
                    y: 5.0,
                    r: 1.0,
                },
                Fill,
            ],
            Self::Grid => &[
                Box {
                    x: 1.5,
                    y: 2.0,
                    w: 13.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(1.5, 6.0),
                LineTo(14.5, 6.0),
                MoveTo(1.5, 10.0),
                LineTo(14.5, 10.0),
                MoveTo(6.0, 2.0),
                LineTo(6.0, 14.0),
                MoveTo(10.5, 2.0),
                LineTo(10.5, 14.0),
                Stroke,
            ],
            Self::BitBtn => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 1.25,
                },
                Stroke,
                Box {
                    x: 3.0,
                    y: 5.5,
                    w: 3.5,
                    h: 5.0,
                    r: 0.5,
                },
                Fill,
                MoveTo(8.5, 8.0),
                LineTo(13.0, 8.0),
                Stroke,
            ],
            Self::SpeedButton => &[
                Box {
                    x: 1.5,
                    y: 1.5,
                    w: 13.0,
                    h: 13.0,
                    r: 1.0,
                },
                Stroke,
                MoveTo(8.5, 3.5),
                LineTo(5.0, 8.5),
                LineTo(8.0, 8.5),
                LineTo(7.5, 12.5),
                LineTo(11.0, 7.5),
                LineTo(8.0, 7.5),
                ClosePath,
                Fill,
            ],
            Self::MainMenu => &[
                Box {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(1.0, 5.5),
                LineTo(15.0, 5.5),
                Stroke,
                Box {
                    x: 2.5,
                    y: 3.0,
                    w: 2.5,
                    h: 1.5,
                    r: 0.2,
                },
                Fill,
                Box {
                    x: 6.0,
                    y: 3.0,
                    w: 2.5,
                    h: 1.5,
                    r: 0.2,
                },
                Fill,
                Box {
                    x: 9.5,
                    y: 3.0,
                    w: 2.5,
                    h: 1.5,
                    r: 0.2,
                },
                Fill,
            ],
            Self::PopupMenu => &[
                Box {
                    x: 2.0,
                    y: 2.0,
                    w: 12.0,
                    h: 12.0,
                    r: 1.0,
                },
                Stroke,
                MoveTo(4.0, 5.0),
                LineTo(12.0, 5.0),
                MoveTo(4.0, 8.0),
                LineTo(12.0, 8.0),
                MoveTo(4.0, 11.0),
                LineTo(12.0, 11.0),
                Stroke,
                Box {
                    x: 4.0,
                    y: 4.2,
                    w: 1.6,
                    h: 1.6,
                    r: 0.2,
                },
                Fill,
            ],
            Self::ToolBar => &[
                Box {
                    x: 1.0,
                    y: 4.0,
                    w: 14.0,
                    h: 8.0,
                    r: 0.5,
                },
                Stroke,
                Box {
                    x: 2.5,
                    y: 5.5,
                    w: 3.0,
                    h: 5.0,
                    r: 0.25,
                },
                Stroke,
                Box {
                    x: 6.5,
                    y: 5.5,
                    w: 3.0,
                    h: 5.0,
                    r: 0.25,
                },
                Stroke,
                Box {
                    x: 10.5,
                    y: 5.5,
                    w: 3.0,
                    h: 5.0,
                    r: 0.25,
                },
                Stroke,
            ],
            Self::StatusBar => &[
                Box {
                    x: 1.0,
                    y: 6.0,
                    w: 14.0,
                    h: 7.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(6.0, 6.0),
                LineTo(6.0, 13.0),
                MoveTo(11.0, 6.0),
                LineTo(11.0, 13.0),
                Stroke,
                MoveTo(2.5, 9.5),
                LineTo(4.5, 9.5),
                Stroke,
            ],
            Self::ProgressBar => &[
                Box {
                    x: 1.0,
                    y: 4.5,
                    w: 14.0,
                    h: 7.0,
                    r: 1.0,
                },
                Stroke,
                Box {
                    x: 2.0,
                    y: 5.5,
                    w: 7.0,
                    h: 5.0,
                    r: 0.5,
                },
                Fill,
            ],
            Self::TrackBar => &[
                MoveTo(1.5, 8.0),
                LineTo(14.5, 8.0),
                Stroke,
                MoveTo(2.0, 6.0),
                LineTo(2.0, 10.0),
                MoveTo(14.0, 6.0),
                LineTo(14.0, 10.0),
                Stroke,
                Box {
                    x: 6.5,
                    y: 4.0,
                    w: 3.0,
                    h: 8.0,
                    r: 0.5,
                },
                Fill,
            ],
            Self::PaintBox => &[
                Box {
                    x: 1.5,
                    y: 1.5,
                    w: 13.0,
                    h: 13.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(3.0, 13.0),
                LineTo(13.0, 3.0),
                Stroke,
                Circle {
                    x: 5.0,
                    y: 5.0,
                    r: 1.5,
                },
                Fill,
            ],
            Self::Shape => &[
                Circle {
                    x: 8.0,
                    y: 8.0,
                    r: 5.5,
                },
                Stroke,
                MoveTo(8.0, 2.5),
                LineTo(13.5, 8.0),
                LineTo(8.0, 13.5),
                LineTo(2.5, 8.0),
                ClosePath,
                Stroke,
            ],
            Self::OpenDialog => &[
                MoveTo(1.5, 4.0),
                LineTo(5.5, 4.0),
                LineTo(7.0, 6.0),
                LineTo(14.5, 6.0),
                LineTo(14.5, 13.5),
                LineTo(1.5, 13.5),
                ClosePath,
                Stroke,
                MoveTo(8.0, 12.0),
                LineTo(8.0, 8.0),
                LineTo(6.0, 9.5),
                MoveTo(8.0, 8.0),
                LineTo(10.0, 9.5),
                Stroke,
            ],
            Self::SaveDialog => &[
                Box {
                    x: 2.0,
                    y: 2.0,
                    w: 12.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                Box {
                    x: 4.5,
                    y: 2.5,
                    w: 7.0,
                    h: 4.5,
                    r: 0.25,
                },
                Fill,
                Box {
                    x: 4.5,
                    y: 9.0,
                    w: 7.0,
                    h: 4.5,
                    r: 0.25,
                },
                Stroke,
            ],
            Self::Splitter => &[
                Box {
                    x: 7.0,
                    y: 1.5,
                    w: 2.0,
                    h: 13.0,
                    r: 0.5,
                },
                Fill,
                MoveTo(4.5, 8.0),
                LineTo(2.0, 8.0),
                LineTo(3.5, 6.0),
                MoveTo(2.0, 8.0),
                LineTo(3.5, 10.0),
                MoveTo(11.5, 8.0),
                LineTo(14.0, 8.0),
                LineTo(12.5, 6.0),
                MoveTo(14.0, 8.0),
                LineTo(12.5, 10.0),
                Stroke,
            ],
            Self::RadioGroup => &[
                MoveTo(1.0, 3.5),
                LineTo(4.0, 3.5),
                MoveTo(10.0, 3.5),
                LineTo(15.0, 3.5),
                MoveTo(1.0, 3.5),
                LineTo(1.0, 14.0),
                MoveTo(15.0, 3.5),
                LineTo(15.0, 14.0),
                MoveTo(1.0, 14.0),
                LineTo(15.0, 14.0),
                Stroke,
                Circle {
                    x: 5.5,
                    y: 7.5,
                    r: 2.0,
                },
                Stroke,
                Circle {
                    x: 5.5,
                    y: 11.5,
                    r: 2.0,
                },
                Stroke,
                Circle {
                    x: 5.5,
                    y: 11.5,
                    r: 0.9,
                },
                Fill,
            ],
            Self::SpinEdit => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 14.0,
                    h: 10.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(4.0, 5.5),
                LineTo(4.0, 10.5),
                Stroke,
                MoveTo(11.0, 7.0),
                LineTo(12.5, 5.0),
                LineTo(14.0, 7.0),
                MoveTo(11.0, 9.0),
                LineTo(12.5, 11.0),
                LineTo(14.0, 9.0),
                Stroke,
            ],
            Self::Toggle => &[
                Box {
                    x: 1.0,
                    y: 4.5,
                    w: 14.0,
                    h: 7.0,
                    r: 3.5,
                },
                Stroke,
                Circle {
                    x: 11.5,
                    y: 8.0,
                    r: 2.3,
                },
                Fill,
            ],
            Self::ColorBox => &[
                Box {
                    x: 1.5,
                    y: 4.5,
                    w: 6.0,
                    h: 7.0,
                    r: 1.0,
                },
                Fill,
                Box {
                    x: 1.0,
                    y: 4.0,
                    w: 14.0,
                    h: 8.0,
                    r: 1.5,
                },
                Stroke,
                MoveTo(10.0, 7.0),
                LineTo(11.5, 9.0),
                LineTo(13.0, 7.0),
                Stroke,
            ],
            Self::LinkLabel => &[
                MoveTo(2.0, 9.5),
                LineTo(8.5, 9.5),
                MoveTo(9.5, 3.0),
                LineTo(14.0, 3.0),
                LineTo(14.0, 7.5),
                MoveTo(14.0, 3.0),
                LineTo(8.5, 8.5),
                MoveTo(2.0, 12.5),
                LineTo(14.0, 12.5),
                Stroke,
            ],
            Self::Bevel => &[
                MoveTo(1.5, 13.5),
                LineTo(1.5, 2.5),
                LineTo(14.5, 2.5),
                Stroke,
                MoveTo(4.0, 11.0),
                LineTo(12.0, 11.0),
                LineTo(12.0, 5.0),
                Stroke,
            ],
            Self::CategoryPanel => &[
                Rect {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 4.0,
                },
                Fill,
                Rect {
                    x: 1.5,
                    y: 6.0,
                    w: 13.0,
                    h: 8.0,
                },
                Stroke,
                MoveTo(4.0, 9.0),
                LineTo(12.0, 9.0),
                MoveTo(4.0, 11.5),
                LineTo(10.0, 11.5),
                Stroke,
            ],
            Self::ListView => &[
                Rect {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 3.5,
                },
                Fill,
                Box {
                    x: 1.0,
                    y: 2.0,
                    w: 14.0,
                    h: 12.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(8.0, 2.0),
                LineTo(8.0, 14.0),
                MoveTo(1.0, 8.5),
                LineTo(15.0, 8.5),
                Stroke,
            ],
            Self::MessageDialog => &[
                Box {
                    x: 1.5,
                    y: 2.5,
                    w: 13.0,
                    h: 11.0,
                    r: 1.0,
                },
                Stroke,
                Circle {
                    x: 8.0,
                    y: 6.0,
                    r: 0.8,
                },
                Fill,
                MoveTo(8.0, 8.0),
                LineTo(8.0, 11.0),
                Stroke,
            ],
            Self::Card => &[
                Box {
                    x: 1.5,
                    y: 2.0,
                    w: 13.0,
                    h: 12.0,
                    r: 2.0,
                },
                Stroke,
                Rect {
                    x: 2.5,
                    y: 2.5,
                    w: 11.0,
                    h: 3.5,
                },
                Fill,
                MoveTo(3.5, 9.0),
                LineTo(12.5, 9.0),
                MoveTo(3.5, 11.5),
                LineTo(9.5, 11.5),
                Stroke,
            ],
            Self::Badge => &[
                Box {
                    x: 1.5,
                    y: 5.0,
                    w: 13.0,
                    h: 6.0,
                    r: 3.0,
                },
                Stroke,
                MoveTo(4.5, 8.0),
                LineTo(11.5, 8.0),
                Stroke,
            ],
            Self::Chips => &[
                Box {
                    x: 1.0,
                    y: 3.0,
                    w: 6.0,
                    h: 4.0,
                    r: 2.0,
                },
                Fill,
                Box {
                    x: 8.0,
                    y: 3.0,
                    w: 7.0,
                    h: 4.0,
                    r: 2.0,
                },
                Stroke,
                Box {
                    x: 1.0,
                    y: 9.0,
                    w: 7.0,
                    h: 4.0,
                    r: 2.0,
                },
                Stroke,
                Box {
                    x: 9.0,
                    y: 9.0,
                    w: 6.0,
                    h: 4.0,
                    r: 2.0,
                },
                Fill,
            ],
            Self::Breadcrumb => &[
                MoveTo(2.0, 8.0),
                LineTo(5.0, 8.0),
                Stroke,
                MoveTo(7.0, 6.5),
                LineTo(8.5, 8.0),
                LineTo(7.0, 9.5),
                Stroke,
                MoveTo(10.5, 8.0),
                LineTo(14.0, 8.0),
                Stroke,
            ],
            Self::Pagination => &[
                MoveTo(3.5, 6.0),
                LineTo(2.0, 8.0),
                LineTo(3.5, 10.0),
                Stroke,
                Box {
                    x: 5.0,
                    y: 5.5,
                    w: 3.0,
                    h: 5.0,
                    r: 0.5,
                },
                Fill,
                Box {
                    x: 9.0,
                    y: 5.5,
                    w: 3.0,
                    h: 5.0,
                    r: 0.5,
                },
                Stroke,
                MoveTo(13.0, 6.0),
                LineTo(14.5, 8.0),
                LineTo(13.0, 10.0),
                Stroke,
            ],
            Self::Drawer => &[
                Rect {
                    x: 1.5,
                    y: 2.0,
                    w: 13.0,
                    h: 12.0,
                },
                Stroke,
                Rect {
                    x: 1.5,
                    y: 2.0,
                    w: 6.0,
                    h: 12.0,
                },
                Fill,
            ],
            Self::MultiSelect => &[
                Box {
                    x: 1.0,
                    y: 3.5,
                    w: 14.0,
                    h: 9.0,
                    r: 1.0,
                },
                Stroke,
                Box {
                    x: 2.5,
                    y: 5.5,
                    w: 4.0,
                    h: 5.0,
                    r: 0.5,
                },
                Fill,
                MoveTo(11.0, 7.0),
                LineTo(12.5, 9.0),
                LineTo(14.0, 7.0),
                Stroke,
            ],
            Self::CheckListBox => &[
                Box {
                    x: 1.5,
                    y: 1.5,
                    w: 13.0,
                    h: 13.0,
                    r: 1.0,
                },
                Stroke,
                Rect {
                    x: 3.5,
                    y: 3.5,
                    w: 2.5,
                    h: 2.5,
                },
                Stroke,
                MoveTo(7.5, 4.75),
                LineTo(12.5, 4.75),
                Stroke,
                MoveTo(3.3, 8.2),
                LineTo(4.5, 9.3),
                LineTo(6.3, 6.9),
                Stroke,
                MoveTo(7.5, 8.0),
                LineTo(12.5, 8.0),
                Stroke,
                Rect {
                    x: 3.5,
                    y: 10.0,
                    w: 2.5,
                    h: 2.5,
                },
                Stroke,
                MoveTo(7.5, 11.25),
                LineTo(12.5, 11.25),
                Stroke,
            ],
        }
    }
}

#[note("icons-chevron-closed")]
pub const CHEVRON_CLOSED: &str = crate::codicons::CHEVRON_CLOSED;
pub const CHEVRON_FECHADO: &str = CHEVRON_CLOSED;

#[note("icons-chevron-open")]
pub const CHEVRON_OPEN: &str = crate::codicons::CHEVRON_OPEN;
pub const CHEVRON_ABERTO: &str = CHEVRON_OPEN;

#[note("icons-warning")]
pub const WARNING: &str = "\u{ea6c}";
pub const AVISO: &str = WARNING;
