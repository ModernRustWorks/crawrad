//! # crawrad-actions — the IDE action catalogue
//!
//! Everything the IDE knows how to do is a **named action**: `edit.copy`,
//! `layout.align.left`, `order.bring_to_front`. The list is the catalogue, and
//! the catalogue has three readers:
//!
//! 1. **the interface** — menus, toolbars, and shortcuts come from here,
//!    instead of each button carrying its own copy of the rule;
//! 2. **the keyboard** — the shortcut is an action attribute, not a loose
//!    `match`;
//! 3. **the RTA and MCP** (§12 of the study) — each action becomes a tool that
//!    the agent sees, with id, label, and description already written.
//!
//! It is the classic `TActionList` read through the lens of chapter 12: the
//! automation surface and the interface surface become the same thing, which is
//! only possible because **the action does not know the UI**. It receives a
//! [`Context`] and returns a declarative [`Effect`] — whoever applies it is
//! the caller, whether the egui shell or an agent on the other side of the
//! protocol.

// This crate has no `unsafe`, and the compiler now guarantees this. CrawRAD
// does not have A SINGLE line of unsafe code: the FFI adapter was removed in
// ADR 0013 — the rest of the system is safe Rust, and this boundary is just as
// important as that of ADR 0002.
#![forbid(unsafe_code)]

use crawrad_designer::arrangement::{self, AlignMode, Distribute, Equalize, Reference, ZOrder};
use crawrad_designer::clipboard::{self, Clipboard};
use crawrad_doc::{Command, Doc, Transaction};
use crawrad_ir::{NodeId, Rect, Value};

/// Who requested the change. Re-exported because whoever builds the [`Context`]
/// must state this, and not every catalogue caller depends on `crawrad-doc` directly —
/// the MCP server, when it exists, depends on this crate and nothing else.
pub use crawrad_doc::Author;
use crawrad_registry::Registry;
use std::collections::BTreeMap;
use std::path::PathBuf;

// THE PANEL ARRANGEMENT. In its own module, and in English, because it is a new
// item (ADR 0030): the catalogue now publishes the panel list, and that is what
// the MCP reads to know what an agent can open, move, or dock.
pub mod action_list;
pub mod color_dialog;
pub mod grid_columns;
pub mod i18n;
pub mod items_editor;
pub mod key_routing;
pub mod menu;
pub mod style_rule_check;
pub use menu::{Menu, Row, menu_path};
pub mod panel;
pub mod reference;
pub use panel::{DockSide, LayoutCommand, Panel, ProfileCommand};
pub mod dialog_fields;
pub mod project_log;
pub mod project_options_dialog;

// WHAT IS OPEN INSIDE THE EXPLORER. In its own module by the same criterion as
// `panel`: it is an effect family, and it is what the MCP reads to know that
// folding a folder and opening the legend are things an agent can request.
pub mod explorer;
pub use explorer::{ExplorerCommand, WorkspaceRole};

// WHAT IS OPEN INSIDE THE OBJECT INSPECTOR. In its own module by the same
// criterion as `explorer`: it is an effect family, and it is what the MCP reads
// to know that opening a collection editor and folding margins are things an
// agent can request.
pub mod inspector;
pub mod inspector_model;
pub use inspector::InspectorCommand;
pub use inspector_model::{
    BOUND_TO_DATASET, DATA_CATEGORY, binding_mode_name, parse_binding_mode, rows_with_columns,
};

// THE UNIFIED IDE CAPABILITY REGISTRY.
// Unifies native actions and extension commands with strict ID validation,
// typed JSON schemas, and dispatch with audit logging.
pub mod capability;
pub use capability::{
    AuditEntry, Capability, CapabilityError, CapabilityExecutor, CapabilityParam,
    CapabilityRegistry, CapabilitySource,
};

mod coerce;
pub use coerce::coerce_to_property;
use crawrad_note::note;

#[note("context")]
pub struct Context<'a> {
    /// The open document — and `None` is a STATE, not a failure.
    ///
    /// Since [ADR 0017] the IDE exists without a document: first run, folder
    /// without a form, pending choice between several. The shell before that
    /// simply **did not build** the `Context` in that state, and the entire
    /// catalogue died in silence along with it — including the three FOLDER
    /// actions, which need no document at all. In a console project, created
    /// by `New Project…` itself, `Open code` did nothing and gave no notice.
    ///
    /// `Option` here is what separates the two families without a second code
    /// path: whoever is about the form gives up, whoever is about the folder
    /// proceeds.
    ///
    pub doc: Option<&'a Doc>,
    pub registry: &'a Registry,
    pub clipboard: &'a Clipboard,
    /// What the action receives from outside. Empty for the vast majority — they
    /// operate on the selection —, and populated by the few that need a parameter.
    ///
    /// It was the declared loophole of ADR 0005:
    /// without this, an action with an argument had to become a loose function,
    /// and the catalogue — which is the source of the MCP — began to lie about
    /// what the IDE knows how to do.
    pub args: Args,
    /// What is known about the **project** — what is on disk, and not in the
    /// document.
    ///
    /// It entered due to a measured defect: `Run` was offered in a `--lib`
    /// project, and after a green build `cargo run -p <lib>` answered
    /// `error: a bin target must be available for cargo run`.
    /// [ADR 0018] predicted this in so many words — *"what it costs is not the
    /// flag, it is the IDE stopping offering `Run` to whoever has nothing to
    /// run"* — and the bill was not paid when `Library` entered the list.
    ///
    /// Here and not in the shell because **the predicate belongs to the
    /// catalogue** (ADR 0005): whoever decides whether `Run` is valid is the
    /// `enabled` of the action, for the toolbar, for the keyboard, and for the
    /// MCP at the same time. An `if` in the shell before calling the action
    /// would be a fourth place deciding what the action already knows how to
    /// say.
    ///
    pub project: Project,
    /// **Who** is asking — and this is what signs the transaction in audit.
    ///
    /// [`Author`] has existed in `crawrad-doc` since day one, and its documentation
    /// promises to answer "who touched this?" — a question no classic IDE
    /// answers well. When this field was born, the enum did not have **a single
    /// producer in production code**: 32 `Transaction::user` and zero
    /// `Author::Plugin | Agent`. Wiring the MCP in that state would make history
    /// say `user` for everything an agent did — the promise broken precisely
    /// in the case where it matters.
    ///
    /// The shell continues sending [`Author::User`], because it is true: the
    /// one who clicked was the user. Whoever has to say otherwise is the other
    /// side of the protocol, and now it has a place to do so.
    pub author: Author,
    /// **Is there text typed in the editor and not yet saved?**
    ///
    /// It entered when the code editor became a real editor (17/09/2026). Until
    /// then `file.save` evaluated `has_document` and that was enough, because
    /// the only saveable thing was the form.
    ///
    /// Leaving it like that would cause `Ctrl+S` to die in silence in a case
    /// that [ADR 0017] itself created: the Explorer opens any `.rs` in the
    /// project, and a project may have no form at all. The user would type,
    /// press `Ctrl+S`, and the IDE would answer nothing.
    ///
    /// Here and not in a condition in the shell because **the predicate belongs
    /// to the catalogue** (ADR 0005): whoever decides whether `Save` is valid
    /// is the `enabled` of the action — for the menu, for the keyboard, and
    /// for the MCP at the same time.
    ///
    pub editor_dirty: bool,
    /// The code editor's text side: is anything selected, is there text on
    /// the system clipboard: Cut/Copy need a selection, Paste a text.
    pub text: EditorText,
}

#[note("editortext")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EditorText {
    pub selection: bool,
    pub clipboard: bool,
    /// The code editor is in front (not the designer): the menu lists the
    /// text twin of Cut/Copy/Paste/Select All, not the components' one.
    pub code_in_front: bool,
    /// The file in front is the open form's unit: `F12` (Toggle Form/Unit)
    /// has a form to go to.
    pub unit_has_form: bool,
}

impl EditorText {
    /// The editor side as the catalog sees it: selection, clipboard and form unit
    /// count only while the code is in front.
    pub fn for_view(
        code_in_front: bool,
        has_selection: bool,
        has_clipboard: bool,
        is_form_unit: bool,
    ) -> Self {
        Self {
            selection: code_in_front && has_selection,
            clipboard: code_in_front && has_clipboard,
            code_in_front,
            unit_has_form: code_in_front && is_form_unit,
        }
    }
}

#[note("project")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Project {
    /// Does the package that `Run` would run have an executable target?
    pub has_binary: bool,
    /// The project has a form to inherit from: a console one has none.
    pub has_forms: bool,
    /// A project is open: the Explorer's commands are about its folder.
    pub open: bool,
    /// Is the open root a workspace with multiple members?
    pub is_workspace: bool,
    /// Where the debugger is: Step needs a stopped program, Pause a running one.
    pub debug: DebugSession,
}

#[note("debugsession")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DebugSession {
    #[default]
    None,
    Running,
    Stopped,
}

/// What an action needs before it can run: why it is greyed when it is.
#[note("need")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Need {
    Nothing,
    Project,
    FormInProject,
    Binary,
    Form,
    OpenFile,
    Selection,
    CodeEditor,
    TextSelection,
    TextClipboard,
    Clipboard,
    Changes,
    DebugRunning,
    DebugStopped,
    DebugSession,
}

impl Need {
    /// Tells whether the need is met in this context.
    #[note("need-met")]
    pub fn met(self, c: &Context) -> bool {
        match self {
            Need::Nothing => true,
            Need::Project => c.project.open,
            Need::FormInProject => c.project.has_forms,
            Need::Binary => c.project.has_binary,
            Need::Form => c.doc.is_some(),
            Need::OpenFile => c.doc.is_some() || c.text.code_in_front,
            Need::Selection => c.doc.is_some_and(|d| !d.selection().is_empty()),
            Need::CodeEditor => c.text.code_in_front,
            Need::TextSelection => c.text.selection,
            Need::TextClipboard => c.text.clipboard,
            Need::Clipboard => !c.clipboard.is_empty(),
            Need::Changes => c.doc.is_some() || c.editor_dirty,
            Need::DebugRunning => c.project.debug == DebugSession::Running,
            Need::DebugStopped => c.project.debug == DebugSession::Stopped,
            Need::DebugSession => c.project.debug != DebugSession::None,
        }
    }

    /// What an agent calls to meet the need before it resends the action.
    #[note("need-remedy")]
    pub fn remedy(self) -> &'static str {
        match self {
            Need::Project | Need::FormInProject => "Call 'project.open' and resend.",
            Need::Form => "Call 'project.open' with a form path and resend.",
            Need::OpenFile => {
                "Call 'editor.open_file' or 'project.open' with a form path and resend."
            }
            Need::Selection => "Call 'designer.select' and resend.",
            Need::CodeEditor => "Call 'editor.open_file' and resend.",
            Need::DebugRunning | Need::DebugStopped | Need::DebugSession => {
                "Start it with 'ide.run_action' on 'project.run' and resend."
            }
            _ => "Change the IDE state and resend.",
        }
    }

    /// The gate of an action whose need alone decides.
    #[note("need-only")]
    pub fn only(self) -> Gate {
        Gate {
            need: self,
            check: |_| true,
        }
    }

    /// The gate of an action that needs this and passes `check` too.
    #[note("need-and")]
    pub fn and(self, check: fn(&Context) -> bool) -> Gate {
        Gate { need: self, check }
    }

    /// The missing thing, as it ends the sentence *"<Action> needs …"*.
    #[note("need-reason")]
    pub fn reason(self) -> &'static str {
        match self {
            Need::Nothing => "nothing",
            Need::Project => "an open project",
            Need::FormInProject => "a form in the project",
            Need::Binary => "a binary target (this project is a library)",
            Need::Form => "a form open in the designer",
            Need::OpenFile => "a file open in the editor or the designer",
            Need::Selection => "a selected component",
            Need::CodeEditor => "a code editor in front",
            Need::TextSelection => "selected text",
            Need::TextClipboard => "text in the clipboard",
            Need::Clipboard => "copied components",
            Need::Changes => "something to save",
            Need::DebugRunning => "a running program under the debugger",
            Need::DebugStopped => "the debugger stopped at a line",
            Need::DebugSession => "a debug session",
        }
    }
}

/// When an action may run: its [`Need`], refined by an optional check.
#[note("gate")]
#[derive(Clone, Copy)]
pub struct Gate {
    pub need: Need,
    check: fn(&Context) -> bool,
}

impl Gate {
    /// Tells whether the action may run in this context.
    #[note("gate-allows")]
    pub fn allows(&self, c: &Context) -> bool {
        self.need.met(c) && (self.check)(c)
    }
}

impl Default for Project {
    #[note("project-default")]
    fn default() -> Self {
        Self {
            has_binary: true,
            has_forms: true,
            open: true,
            is_workspace: false,
            debug: DebugSession::None,
        }
    }
}

#[note("args")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Args {
    values: BTreeMap<String, Value>,
}

impl Args {
    pub fn none() -> Self {
        Self::default()
    }

    #[note("args-one")]
    pub fn one(name: impl Into<String>, value: Value) -> Self {
        Self::none().and(name, value)
    }

    #[note("args-and")]
    pub fn and(mut self, name: impl Into<String>, value: Value) -> Self {
        self.values.insert(name.into(), value);
        self
    }

    #[note("args-text")]
    pub fn text(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self::one(name, Value::Str(text.into()))
    }

    #[note("args-texts")]
    pub fn texts<I, S>(name: impl Into<String>, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::one(
            name,
            Value::List(items.into_iter().map(|s| Value::Str(s.into())).collect()),
        )
    }

    #[note("args-from-map")]
    pub fn from_map(values: BTreeMap<String, Value>) -> Self {
        Self { values }
    }

    #[note("args-value")]
    pub fn value(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    pub fn as_text(&self, name: &str) -> Option<&str> {
        self.values.get(name)?.as_str()
    }

    #[note("args-as-texts")]
    pub fn as_texts(&self, name: &str) -> Vec<&str> {
        match self.values.get(name) {
            Some(Value::List(v)) => v.iter().filter_map(Value::as_str).collect(),
            _ => Vec::new(),
        }
    }

    #[note("args-as-number")]
    pub fn as_number(&self, name: &str) -> Option<f64> {
        match self.values.get(name)? {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    #[note("args-as-bool")]
    pub fn as_bool(&self, name: &str) -> Option<bool> {
        match self.values.get(name)? {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[note("pathinproject")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathInProject(String);

impl PathInProject {
    #[note("pathinproject-new")]
    pub fn new(raw: &str) -> Option<Self> {
        use std::path::Component;

        // THE SAME VERDICT ON EVERY SYSTEM. Measured in a Linux container on
        // 01/10/2026: there `\` separates nothing and `C:` is no prefix, so
        // `C:algo.txt` and `\\servidor\share\x.rs` passed as plain names. A
        // project moves between systems; a Windows path is refused everywhere
        // and a `\` is a separator everywhere.
        let bytes = raw.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            return None;
        }
        let portable = raw.replace('\\', "/");
        let mut parts: Vec<&std::ffi::OsStr> = Vec::new();
        for c in std::path::Path::new(&portable).components() {
            match c {
                // The two that re-anchor. They are the entire family of the hole, and that
                // is why refusal is by VARIANT and not by text.
                Component::Prefix(_) | Component::RootDir => return None,
                Component::CurDir => {}
                // Ascending beyond the start escapes the root — the shell joins this
                // path with it, and `Path::starts_with` is lexical: it would say
                // that `<root>\..\..\x` starts at `<root>`, because `..`
                // remains there as a component. Who resolves it is this stack.
                Component::ParentDir => {
                    parts.pop()?;
                }
                Component::Normal(n) => parts.push(n),
            }
        }
        if parts.is_empty() {
            // Empty path (or just `.`) is not "a project file" — it is the
            // absence of an argument, and whoever wants the folder sends `None`.
            return None;
        }
        let clean: std::path::PathBuf = parts.iter().collect();
        Some(Self(clean.to_string_lossy().into_owned()))
    }

    #[note("pathinproject-as-str")]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PathInProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[note("paramkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    /// Free-form text (string).
    String,
    /// Signed integer.
    Integer,
    /// Floating-point or general number.
    Number,
    /// Boolean flag.
    Boolean,
    /// Identifier of a component in the design hierarchy.
    ComponentId,
    /// Closed set of string variants.
    Enum(&'static [&'static str]),
    /// Arbitrary JSON value or dynamic property value.
    Any,
}

#[note("param")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Param {
    pub name: &'static str,
    pub kind: ParamKind,
    pub description: &'static str,
    /// Receives several at once?
    pub list: bool,
    /// Does the action do anything **without** it?
    ///
    /// The difference is behavioural, not taste, and it separates the two
    /// actions with parameters that exist today: `project.open_editor` without
    /// argument opens the folder — which is what the `Run` menu always did —, while
    /// `action.standard` without argument returns `Effect::none()`.
    ///
    /// Who reads this are two: the MCP, which publishes tool signature and
    /// needs to say *required* or not; and the menu, which **has nowhere to type an
    /// argument** and therefore cannot draw an action that requires one. Without
    /// this field, the Edit menu showed two `Standard actions...` — the one of the
    /// catalogue, lit and inert, and the real one eleven lines below.
    pub required: bool,
    /// The values this parameter accepts, when the catalogue can name them
    /// all — and `None` when it takes free text (a node id, a caption, a
    /// coordinate).
    ///
    /// # This exists for the MCP first
    ///
    /// A tool that does not say which values it takes is a tool the agent has
    /// to GUESS, and a guess that misses arrives as `no such panel:
    /// `inspetor`` — one round trip wasted on something the signature could
    /// have said up front. The owner's rule of 15/09/2026 puts every command
    /// of the IDE in this catalogue so it can become a tool; a parameter with
    /// a closed set of values is exactly the part of a signature a protocol
    /// can publish as an enumeration.
    ///
    /// # And it is what lets a MENU draw the command whole
    ///
    /// [`Action::requires_argument`] keeps an action that requires an argument
    /// out of a menu, because a menu has nowhere to type one. A domain lifts
    /// that for the case where the menu can stand in for the typing: it draws
    /// **one row per value**, and each row carries its own argument. That is
    /// how `View ▸ Explorer` exists at all — see [`Domain::row_per_value`].
    pub domain: Option<Domain>,
}

#[note("choice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choice {
    /// What the caller passes as the argument.
    pub id: &'static str,
    /// What a menu row — or a picker, or a tool description — shows.
    pub label: &'static str,
}

#[note("domain")]
#[derive(Debug, Clone, Copy)]
pub struct Domain {
    /// Every accepted value, in the order a menu should show them.
    pub values: fn() -> Vec<Choice>,
    /// May a MENU collect this argument by drawing one row per value?
    ///
    /// # Why this is not simply "the parameter has a domain"
    ///
    /// Six panels times the four one-argument layout commands is twenty-four
    /// rows in the `View` menu, and twenty-two of them nobody asked for.
    /// `layout.toggle_panel` is the one where a row per value is the **whole
    /// command and reads as a state**: a row with a tick beside it IS a
    /// toggle, which is how a classic RAD IDE and modern editors both draw the list of panels.
    /// `show`, `hide` and `float` say the same thing three times over and
    /// belong to the doors that take an argument — the panel context menu and
    /// the protocol.
    ///
    /// So the flag sits on the PARAMETER because that is the question it
    /// answers: *can a menu supply this argument by showing it, instead of
    /// asking for it?*
    pub row_per_value: bool,
}

/// Two domains are the same when they OFFER the same thing.
///
/// Written out instead of derived, and the compiler is the one that asked:
/// deriving it compares the function POINTERS, which `rustc` warns about
/// because the address of one function is not guaranteed to be unique across
/// codegen units. Comparing the values they produce is both lint-clean and the
/// question anyone actually has — a domain that yields the same six panels is
/// the same domain, whichever function built the list.
impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self.row_per_value == other.row_per_value && (self.values)() == (other.values)()
    }
}

impl Eq for Domain {}

impl Param {
    #[note("param-choices")]
    pub fn choices(&self) -> Vec<Choice> {
        self.domain.map(|d| (d.values)()).unwrap_or_default()
    }

    #[note("param-json-schema")]
    pub fn json_schema(&self) -> serde_json::Value {
        let mut item = match self.kind {
            ParamKind::String => serde_json::json!({
                "type": "string"
            }),
            ParamKind::Integer => serde_json::json!({
                "type": "integer"
            }),
            ParamKind::Number => serde_json::json!({
                "type": "number"
            }),
            ParamKind::Boolean => serde_json::json!({
                "type": "boolean"
            }),
            ParamKind::ComponentId => serde_json::json!({
                "type": "string"
            }),
            ParamKind::Enum(variants) => {
                let list: Vec<serde_json::Value> = if !variants.is_empty() {
                    variants.iter().map(|s| serde_json::json!(s)).collect()
                } else {
                    self.choices()
                        .into_iter()
                        .map(|c| serde_json::json!(c.id))
                        .collect()
                };
                if list.is_empty() {
                    serde_json::json!({ "type": "string" })
                } else {
                    serde_json::json!({
                        "type": "string",
                        "enum": list
                    })
                }
            }
            ParamKind::Any => serde_json::json!({}),
        };

        if self.list {
            serde_json::json!({
                "type": "array",
                "description": self.description,
                "items": item
            })
        } else {
            if let Some(obj) = item.as_object_mut() {
                obj.insert(
                    "description".to_string(),
                    serde_json::json!(self.description),
                );
            }
            item
        }
    }
}

impl Action {
    #[note("action-view-affinity")]
    pub fn view_affinity(&self) -> ViewAffinity {
        self.view
    }

    #[note("action-input-schema")]
    pub fn input_schema(&self) -> serde_json::Value {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for p in self.params {
            properties.insert(p.name.to_string(), p.json_schema());
            if p.required {
                required.push(serde_json::json!(p.name));
            }
        }

        let mut schema = serde_json::Map::new();
        schema.insert("type".to_string(), serde_json::json!("object"));
        schema.insert(
            "properties".to_string(),
            serde_json::Value::Object(properties),
        );
        schema.insert("required".to_string(), serde_json::Value::Array(required));

        serde_json::Value::Object(schema)
    }

    #[note("action-requires-argument")]
    pub fn requires_argument(&self) -> bool {
        self.params.iter().any(|p| p.required)
    }

    #[note("action-row-per-value")]
    pub fn row_per_value(&self) -> Option<&'static Param> {
        let mut required = self.params.iter().filter(|p| p.required);
        let only = required.next()?;
        if required.next().is_some() {
            return None;
        }
        only.domain.filter(|d| d.row_per_value).map(|_| only)
    }
}

impl<'a> Context<'a> {
    #[note("context-ir")]
    fn ir(&self) -> Option<&'a crawrad_ir::Document> {
        self.doc.map(|d| d.ir())
    }

    fn sel(&self) -> Vec<NodeId> {
        self.doc.map(|d| d.selection().to_vec()).unwrap_or_default()
    }

    #[note("context-new-id")]
    fn new_id(&self, class: &str) -> String {
        self.registry.suggest_id(class, |i| {
            self.ir()
                .is_some_and(|ir| ir.node(i).is_some() || ir.node_by_name(i).is_some())
        })
    }
}

#[note("effect")]
#[derive(Debug, Default)]
pub struct Effect {
    pub transaction: Option<Transaction>,
    pub selection: Option<Vec<NodeId>>,
    /// Text to copy to the system clipboard.
    pub copy_text: Option<String>,
    pub status: Option<String>,
    /// Actions that touch history, which the IDE handles separately.
    pub history: Option<History>,
    /// Asks the IDE to save or reload the file.
    pub file: Option<FileEffect>,
    /// Asks the IDE to change what is in the center of the window.
    pub view: Option<ViewEffect>,
    /// Asks the IDE to change a designer mode.
    pub designer: Option<DesignerMode>,
    /// Asks the IDE to change canvas zoom.
    pub zoom: Option<Zoom>,
    /// Asks the IDE to change the zoom of the **ENTIRE INTERFACE**.
    ///
    /// # Not [`Effect::zoom`], and the difference is what the owner demanded
    ///
    /// > *"the zoom is not only in the designer area, it is in ALL OF THE IDE, WITHOUT
    /// > EXCEPTION"*
    ///
    /// [`Effect::zoom`] enlarges the FORM on the canvas — it is the magnifying glass of the
    /// design, and its `100%` means *"the design measures what the application
    /// will measure"*. This here enlarges the IDE: menu, panels, tree, Inspector,
    /// status bar and canvas together, like `Ctrl +` of any editor.
    ///
    /// Both exist, and both have their own key. Merging them would take from the IDE
    /// precisely the ruler that makes the designer worthwhile.
    pub ui_zoom: Option<Zoom>,
    /// Asks the IDE to change the **panel arrangement**.
    ///
    /// A family separate from [`ViewEffect`] by the criterion that already separates the others:
    /// `ViewEffect` says *what* is in the center of the window, and this does not change the
    /// content of anything — it changes where panels sit. An agent reading
    /// `Effect` to know what will happen reads the family and already knows: it does not
    /// write to the document, does not launch a process, does not touch the project
    /// disk.
    ///
    /// It exists because tab dragging **does not exist on Linux** —
    /// `CxOsOp::StartDragging` has no arm in `linux_x11.rs` and falls into the
    /// generic case — and because dragging is the only arrangement gesture without
    /// keyboard equivalent (ADR 0014, Amendment 10). The command comes first;
    /// dragging is the second door for it.
    pub layout: Option<LayoutCommand>,
    /// Applies, saves or deletes a desktop profile — the same family as
    /// [`Self::layout`], plus the profile's file in the configuration folder.
    pub profile: Option<ProfileCommand>,
    /// Asks the IDE to fold or unfold something **inside the Explorer**.
    ///
    /// A family separate from [`Effect::layout`] by the same criterion that separates
    /// all the others: `layout` says *where panels sit*, and this does not
    /// move any panel — it changes what is open inside one of them. The
    /// entire argument is in [`explorer`].
    pub explorer: Option<ExplorerCommand>,
    /// Asks the IDE to open, close or fold something **inside the Object
    /// Inspector**.
    ///
    /// A family separate from [`Effect::explorer`] by the same criterion that separates
    /// all the others: that one says what is open inside the Explorer, and
    /// this does not touch the file tree — it says what is open inside the
    /// properties panel. The entire argument is in [`inspector`].
    pub inspector: Option<InspectorCommand>,
    /// Asks the IDE to install a component, or to show what is
    /// installed. See [`ComponentEffect`].
    pub component: Option<ComponentEffect>,
    /// Asks the IDE for a debugger gesture. See [`DebugCommand`].
    pub debug: Option<DebugCommand>,
    /// Asks the IDE to act on its language server. See [`LspCommand`].
    pub lsp: Option<LspCommand>,
    /// Asks the code editor to act. See [`EditorCommand`].
    pub editor: Option<EditorCommand>,
    /// Asks the IDE to open an event handler in the code. See [`HandlerJump`].
    pub handler: Option<HandlerJump>,
    /// Language code for designer preview (empty for source English).
    pub language: Option<String>,
    /// Language code for project execution (empty for system language).
    pub run_language: Option<String>,
}

#[note("handlerjump")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlerJump {
    /// The function to land in.
    pub name: String,
    /// In the BASE form's unit, and not in this form's.
    pub inherited: bool,
}

#[note("bookmark-effect")]
fn bookmark_effect(c: &Context, make: fn(u8) -> EditorCommand) -> Effect {
    match c
        .args
        .as_number("number")
        .filter(|n| (1.0..=9.0).contains(n))
    {
        Some(n) => Effect {
            editor: Some(make(n as u8)),
            ..Effect::none()
        },
        None => Effect::none(),
    }
}

#[note("editorcommand")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorCommand {
    /// The caret to `line` and `column` (one-based) of the file in the editor.
    GoTo { line: u32, column: u32 },
    /// The types rust-analyzer infers, in grey inside the lines: on or off.
    ToggleInlayHints,
    /// Numbered bookmark 1–9 on the caret's line, or off it.
    ToggleBookmark(u8),
    /// The caret to numbered bookmark 1–9 of the file in the editor.
    GoToBookmark(u8),
    /// Open the completion list at the caret (`Ctrl+Space`).
    Complete,
    /// Open the code templates at the caret (`Ctrl+J`).
    Templates,
    /// Ask the language server where the symbol at the caret is defined
    /// and go there.
    GoToDefinition,
    /// Back to where the last jump left.
    NavigateBack,
    /// Forward again, to where *Back* left.
    NavigateForward,
    /// The hover card for the name at the caret.
    ShowHover,
    /// The signature of the call around the caret, its current parameter
    /// marked.
    ParameterHints,
    /// The code actions and quick fixes at the caret, as a list.
    CodeActions,
    /// Apply the code action at this index of the list offered at the caret
    ///
    ApplyCodeAction(usize),
    /// The whole file formatted by the language server, one undo step.
    FormatDocument,
    /// Every reference to the symbol at the caret, in the References panel.
    FindReferences,
    /// Rename the symbol at the caret everywhere, after a preview.
    RenameSymbol,
    /// The editor's selection to the clipboard and deleted.
    Cut,
    /// The editor's selection to the clipboard.
    Copy,
    /// The clipboard's text at the editor's selection.
    Paste,
    /// Everything in the editor selected.
    SelectAll,
    /// The editor's context menu, at the caret.
    ContextMenu,
    /// Fix the diagnostic on the caret's line and explain why in chat.
    FixExplain,
    /// Insert active file reference (@file) into the chat input without sending.
    AddFileToChat,
    /// Attach the active editor selection to the chat input without sending.
    AddSelectionToChat,
    /// Attach the active editor selection and ask chat to explain it.
    ExplainSelection,
    /// Attach the active editor selection and ask chat for a read-only review.
    ReviewSelection,
    /// Keep your version of the file and overwrite disk on next save.
    KeepMine,
    /// Discard your edits and reload the file from disk.
    ReloadFromDisk,
    /// Open diff review comparing your edits with the disk copy.
    CompareWithDisk,
}

#[note("lspcommand")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspCommand {
    /// Read the `Cargo.toml`s again.
    ReloadWorkspace,
    /// Stop the server and start a new one.
    Restart,
    /// Stop the server; the next opened file starts it again.
    Stop,
}

#[note("debugcommand")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugCommand {
    /// Toggles the breakpoint on the cursor's line — `F5` in the classic RAD IDE,
    /// `F9` in the modern editor convention, and the same as clicking on the editor glyph margin.
    ToggleBreakpoint,
    /// One step, over calls — `F8` in the classic RAD IDE, `F10` in the modern editor convention.
    StepOver,
    /// One step, stepping into call — `F7` / `F11`.
    StepInto,
    /// Step out of function — `Shift+F8` / `Shift+F11`.
    StepOut,
    /// Pauses running program — no key in the classic RAD IDE, `F6` in the modern editor convention.
    Pause,
    /// Terminates program and session — `Ctrl+F2` / `Shift+F5`.
    Stop,
    /// Compiles and runs WITHOUT debugger — `Ctrl+Shift+F9` / `Ctrl+F5`.
    RunWithoutDebugging,
    /// Adds a watch — `Ctrl+F5` in the classic key scheme. `None`: the selection, or the word
    /// under the caret, as a classic Add Watch dialog starts filled.
    AddWatch(Option<String>),
    /// Removes the watch at this position of the list.
    RemoveWatch(usize),
}

#[note("magnification")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zoom {
    /// One step inward.
    In,
    /// One step outward.
    Out,
    /// Back to 100%.
    ///
    /// It exists as its own command, not as "press the other until reaching it",
    /// because 100% is not just any zoom: it is the only one in which the design measures what
    /// the application will measure. Returning there is the gesture of *checking*.
    Reset,
}

#[note("designermode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignerMode {
    /// The classic RAD IDE's `Edit ▸ Lock Controls`.
    ///
    /// **Toggle**, and not `Lock` / `Unlock`, for the same reason as
    /// [`ViewEffect::ToggleDesignCode`]: the user gesture is a single one, back and
    /// forth, and two actions with the same chord would be the ambiguity that the test
    /// `dois_atalhos_iguais_seriam_ambiguidade` forbids.
    ToggleLock,
    /// The bar's `Preview` — the form ceases to be a design and begins to
    /// **respond**, inside the IDE itself.
    ///
    /// **Toggle**, for the same reason as lock: the gesture is a single one, back and
    /// forth, and the button label changes with state (`Preview` ↔ `Stop`).
    ///
    /// It lives here and not in [`ViewEffect`] because it does not swap what is in the center
    /// of the window: the canvas remains the canvas. What changes is the MODE in
    /// which it draws — no ruler, no handle, no grid —, and that is the same
    /// axis as the lock.
    ToggleLive,
}

#[note("vieweffect")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewEffect {
    /// `Design` ↔ `Code`, `F12` of the classic RAD IDE
    /// (ADR 0014 item 3, ADR 0015 §2).
    ///
    /// **Toggle**, and not `Show(Design)` / `Show(Code)`. The gesture described by
    /// the ADR is a single one — fingers press `F12` to go and `F12` to return
    /// —, and a key that did different things according to the active tab
    /// would require two actions in the catalogue with the same chord, which is
    /// exactly the ambiguity that the test `dois_atalhos_iguais` forbids.
    ToggleDesignCode,
    /// `Alt+F12`: the form as text, instead of the visual design.
    ///
    /// Always goes TO the source, wherever it comes from, and only returns from within it —
    /// so the key has a single response, rather than depending on where
    /// you were.
    ToggleFormSource,
    /// The build panel: diagnostics returned by `cargo`.
    ///
    /// Toggle, and not `Show`, for the same reason as `ToggleDesignCode`: the
    /// gesture is a single chord, back and forth.
    ///
    /// **The panel does not appear just because this was requested.** It appears when there are
    /// diagnostics *and* this is toggled on — drawing the empty region before having
    /// what to put in it is the defect that ADR 0014 names. Whoever intersects the two is the shell, because it owns the list.
    ToggleProblems,
    /// The **welcome screen**, over the open form —
    /// `Help ▸ Welcome` of modern editors.
    ///
    /// Toggle, and not `Show`, for the reason of the other three: the gesture is a single one,
    /// back and forth. And it exists because the screen had no way back at
    /// all: the center only draws it without a document, and the IDE reopens the last
    /// folder by itself — after first use, seeing templates required launching
    /// the IDE with the configuration directory pointing to an empty folder.
    ToggleWelcome,
    /// IDE Options dialog — `Tools ▸ Options...` of the classic RAD IDE.
    Options,
    /// Project Options dialog — `Project ▸ Options...`.
    ProjectOptions,
    /// The note at the caret: `None` opens the popup "Add documentation / Add
    /// note / Add comment" (`Tools ▸ Notes...`, the editor's context menu);
    /// a kind opens that kind's dialog (`Ctrl+Alt+D` / `N` / `M`).
    Note(Option<NoteKindChoice>),
    /// MCP servers and tools dialog — `Tools ▸ MCP Servers...`.
    Mcp,
    /// AI assistant custom instructions dialog — `Tools ▸ AI Custom Instructions...`.
    AiMemory,
    /// Environment check and toolchain installation dialog — `Help ▸ Check Environment...`.
    EnvironmentCheck,
    /// Quick file opening dialog in project — `File ▸ Quick Open...`.
    QuickOpen,
    /// Unified Command Palette dialog — `Ctrl+Shift+P` / `View ▸ Command Palette...`.
    CommandPalette,
    /// Build Manager dialog — `Project ▸ Build...` of the classic RAD IDE / Cargo Multi-target.
    Build,
    /// Code templates manager dialog — `Tools ▸ Manage Code Templates…`.
    ManageTemplates,
}

#[note("notekindchoice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteKindChoice {
    Doc,
    Note,
    Comment,
}

impl NoteKindChoice {
    #[note("notekindchoice-all")]
    pub const ALL: [Self; 3] = [Self::Doc, Self::Note, Self::Comment];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum History {
    Undo,
    Redo,
}

// Without `Copy`: `OpenInEditor` now carries a path, and a path is
// `String`. Losing `Copy` costs nothing here — `Effect` is already moved
// as a whole to whoever applies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileEffect {
    Save,
    /// Every changed file: the code of every tab, then the form.
    SaveAll,
    Reload,
    /// Release the open form — **the folder stays open**.
    ///
    /// It is not the opposite of `Open Folder…`: it is the opposite of opening a document.
    /// The state in which it leaves the IDE already exists and is normal (the `Vazio` of
    /// ADR 0017), and that is why closing is not a new path — it is the same at
    /// which first launch, folder without form and `Reload` of a
    /// file vanished from disk already stop.
    Close,
    /// Add a project to the open workspace: with a `name`, create it at
    /// once from `template` (the agent's way); without one, open the dialog.
    AddProject {
        name: Option<String>,
        template: Option<String>,
    },
    /// Add an existing project folder to the workspace (`None`: ask for it).
    AddExistingProject(Option<PathBuf>),
    /// Make a member the workspace's active project (`default-members`).
    SetActiveProject(PathBuf),
    /// Generate the Cargo project of the declared target (ADR 0004). The action decides
    /// that it is to generate; who knows how to write to disk is the shell, as in everything else
    /// here — the action does not touch files, otherwise an agent could not execute it
    /// with `preview`.
    Generate,
    /// Compile and run the generated project.
    ///
    /// Always generates beforehand: running what was on disk while the design on
    /// screen is already different would be worse than not having the button.
    Run,
    /// Open the generated project in the system editor.
    ///
    /// §9 of the study is explicit: **do not write an editor in Phases 1–3** — an
    /// "open in an external editor" button resolves this for six months. The real editor,
    /// with `ropey` and rust-analyzer, is Phase 4.
    ///
    /// The path is **relative to the generated project**, and `None` opens the folder —
    /// which is what the `Run` menu always did. Relative, and not absolute, because the
    /// action does not know where the project is: the shell knows, and it
    /// joins the two (ADR 0005).
    ///
    /// [`PathInProject`] and not `String`: this `enum` is `pub`, and whoever applies
    /// `Effect` may not be the shell of this IDE. The type is what guarantees that
    /// no applier — present or future — receives a path that did not pass
    /// validation.
    OpenInEditor(Option<PathInProject>),
    /// **Open a file in a TAB of the IDE's own editor.**
    ///
    /// It is not [`Self::OpenInEditor`], and the difference is what the owner requested
    /// on 17/09/2026: *"we need tabs for each .RS
    /// open in the editor"*. The other leaves the file in the SYSTEM editor — the
    /// solution §9 of the study chose while there was no editor
    /// here; this one opens a tab INSIDE the IDE, beside the design.
    ///
    /// Both coexist on purpose. Opening in an external editor remains what
    /// the `Run` menu does, and remains the right answer for whoever wants full
    /// rust-analyzer — Phase 4 changes that. Merging the two into an
    /// argument would produce an action whose effect depends on which shell applies it.
    ///
    /// The path is relative to the generated project, for the same reason as its sister: the
    /// action does not know where the project is, and the type guarantees that no
    /// applier receives a path that did not pass validation.
    OpenInTab(PathInProject),
    /// **Create a new project** — `New Project…`.
    ///
    /// The action creates nothing: it says to ASK. The shell asks,
    /// with its dialog, and `cargo` creates in a thread. It is the
    /// same discipline as everything else here — the action declares, the shell performs —, and
    /// is what makes `project.new` callable by an agent without needing
    /// a folder picker.
    NewProject,
    /// Open a project folder — `None`: ask for it with the folder picker.
    OpenProject(Option<PathBuf>),
    /// Open a form (`.crawform`) — `None`: ask for it with the file picker.
    OpenForm(Option<PathBuf>),
    /// Open the form the open one inherits from, beside it on disk.
    OpenBaseForm,
    /// Open the New Inherited Form dialog.
    NewInheritedForm,
    /// Open the New Component wizard.
    NewComponent,
    /// Take a folder out of the recent projects — the list, never the disk.
    ForgetRecent(PathBuf),
    /// Create a new form (.crawform) in the open project.
    NewForm,
    /// Create a new Rust source file (.rs) in the open project.
    NewFile,
    /// Bring recovery copies back as unsaved edits, by their paths in
    /// the project — empty: every copy on offer.
    Recover(Vec<PathBuf>),
    /// Delete recovery copies — empty: every copy on offer.
    DiscardRecovery(Vec<PathBuf>),
    /// Build all workspace members with `cargo build --workspace`.
    BuildAll,
    /// Clean all workspace members with `cargo clean` (exes and libraries).
    CleanAll,
}

#[note("componenteffect")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentEffect {
    /// **Install a package** — `Component ▸ Install Packages…` of the classic RAD IDE.
    /// The package folder; `None` opens a folder picker.
    Install(Option<PathBuf>),
    /// **Show what is installed — and what was REFUSED.**
    ///
    /// The second half is what matters. The loading port already collects a
    /// sentence per package that did not make it, and until 18/09/2026 **nobody
    /// showed them**: the doc of `instalados.rs` said *"it becomes a line that the
    /// shell displays"* and the shell displayed nothing. A component vanishing without
    /// explanation is the worst thing this port can do, and was exactly
    /// what happened.
    Installed,
}

impl Effect {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn tx(t: Transaction) -> Self {
        Self {
            transaction: Some(t),
            ..Default::default()
        }
    }

    pub fn with_selection(mut self, sel: Vec<NodeId>) -> Self {
        self.selection = Some(sel);
        self
    }

    pub fn with_status(mut self, s: impl Into<String>) -> Self {
        self.status = Some(s.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.transaction.is_none()
            && self.selection.is_none()
            && self.copy_text.is_none()
            && self.history.is_none()
            && self.file.is_none()
            && self.view.is_none()
            && self.designer.is_none()
            && self.zoom.is_none()
            && self.layout.is_none()
            && self.explorer.is_none()
            && self.inspector.is_none()
            && self.component.is_none()
            && self.debug.is_none()
            && self.lsp.is_none()
            && self.editor.is_none()
            && self.language.is_none()
            && self.run_language.is_none()
            // `ui_zoom` entered alongside, on 18/09/2026: it was the only
            // field of `Effect` that this function did not examine, and an effect that only
            // changed IDE zoom declared itself empty. It cost nothing until
            // today because only tests call this — and that is how a guard
            // rots.
            && self.ui_zoom.is_none()
    }
}

#[note("is-known-action")]
pub fn is_known_action(id: &str) -> bool {
    catalogue().iter().any(|a| a.id == id)
}

#[note("keymapscheme")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeymapScheme {
    /// Default scheme with the modern editor convention (F5 Run, Ctrl+Shift+B Build, etc.).
    #[default]
    Modern,
    /// Classic scheme (F9 Run, Ctrl+F9 Build, etc.).
    Classic,
}

impl KeymapScheme {
    pub const ALL: &'static [Self] = &[Self::Modern, Self::Classic];

    pub fn labels() -> Vec<String> {
        Self::ALL.iter().map(|s| s.label().to_string()).collect()
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|&s| s == self).unwrap_or(0)
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Modern => "modern",
            Self::Classic => "classic",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Modern => "Modern (Default)",
            Self::Classic => "Classic",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Modern => {
                "Modern keybindings (F5 Run, Ctrl+Shift+B Build, Ctrl+G Go to line, Ctrl+P Quick open, Ctrl+Shift+K Delete line)"
            }
            Self::Classic => {
                "Classic RAD keybindings (F9 Run, Ctrl+F9 Build, Alt+G Go to line, Ctrl+O Quick open, Ctrl+Y Delete line)"
            }
        }
    }

    #[note("keymapscheme-shortcut-table")]
    pub fn shortcut_table(self) -> Vec<(&'static str, &'static str)> {
        match self {
            Self::Modern => vec![
                ("Run Project", "F5"),
                ("Build / Generate", "Ctrl+Shift+B"),
                ("Toggle Design / Code", "F12"),
                ("Toggle Form Source", "Alt+F12"),
                ("Go to Definition", "Ctrl+F12 or Ctrl+Click"),
                ("Go Back / Forward", "Alt+Left / Alt+Right"),
                ("Show Hover", "Ctrl+K or rest the mouse on a name"),
                ("Format Document", "Shift+Alt+F"),
                ("Delete Line", "Ctrl+Shift+K"),
                ("Redo", "Ctrl+Y"),
                ("Undo", "Ctrl+Z"),
                ("Find in Editor", "Ctrl+F"),
                ("Replace in Editor", "Ctrl+H"),
                ("Go to Line", "Ctrl+G"),
                ("Quick Open (Files)", "Ctrl+P"),
                ("Find Next / Previous", "F3 / Shift+F3"),
                ("Toggle Line Comment", "Ctrl+/ or Ctrl+;"),
                ("Move Lines Up / Down", "Alt+Up / Alt+Down"),
                ("Duplicate Lines", "Shift+Alt+Up / Shift+Alt+Down"),
                ("Select Next Occurrence (Multi-cursor)", "Ctrl+D"),
                ("Fold / Unfold Code", "Ctrl+Shift+[ / Ctrl+Shift+]"),
                ("IntelliSense / Autocomplete", "Ctrl+Space"),
                ("Code Templates", "Ctrl+J"),
                ("Parameter Hints", "Ctrl+Shift+Space"),
                ("Quick Fix / Code Actions", "Ctrl+."),
                ("Sidecar Code Note", "Ctrl+Shift+N"),
                ("Problems Panel", "Ctrl+Shift+M"),
                ("Lock Controls", "Ctrl+Shift+L"),
                ("Zoom In / Out Canvas", "Ctrl+Shift+. / Ctrl+Shift+,"),
                ("Zoom 100% Canvas", "Ctrl+1"),
                ("Zoom In / Out Interface", "Ctrl+ / Ctrl-"),
            ],
            Self::Classic => vec![
                ("Run Project", "F9"),
                ("Build / Generate", "Ctrl+F9"),
                ("Toggle Design / Code", "F12"),
                ("Toggle Form Source", "Alt+F12"),
                ("Find Declaration", "Alt+Up or Ctrl+Click"),
                ("Browse Back / Forward", "Alt+Left / Alt+Right"),
                ("Show Hover", "Ctrl+K or rest the mouse on a name"),
                ("Format Source", "Ctrl+D"),
                ("Delete Line", "Ctrl+Y"),
                ("Redo", "Ctrl+Shift+Z"),
                ("Undo", "Ctrl+Z"),
                ("Find in Editor", "Ctrl+F"),
                ("Replace in Editor", "Ctrl+R"),
                ("Go to Line", "Alt+G"),
                ("Quick Open (Files)", "Ctrl+O"),
                ("Find Next / Previous", "F3 / Shift+F3"),
                ("Toggle Line Comment", "Ctrl+/ or Ctrl+Shift+C"),
                ("Move Lines Up / Down", "Ctrl+Alt+Up / Ctrl+Alt+Down"),
                ("Duplicate Lines", "Shift+Alt+Up / Shift+Alt+Down"),
                ("Select Next Occurrence (Multi-cursor)", "Ctrl+Shift+D"),
                ("Fold / Unfold Code", "Ctrl+Shift+[ / Ctrl+Shift+]"),
                ("IntelliSense / Autocomplete", "Ctrl+Space"),
                ("Code Templates", "Ctrl+J"),
                ("Parameter Hints", "Ctrl+Shift+Space"),
                ("Quick Fix / Code Actions", "Ctrl+."),
                ("Sidecar Code Note", "Ctrl+Shift+N"),
                ("Problems Panel", "Ctrl+Shift+M"),
                ("Lock Controls", "Ctrl+Shift+L"),
                ("Zoom In / Out Canvas", "Ctrl+Shift+. / Ctrl+Shift+,"),
                ("Zoom 100% Canvas", "Ctrl+1"),
                ("Zoom In / Out Interface", "Ctrl+ / Ctrl-"),
            ],
        }
    }
}

#[note("viewaffinity")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewAffinity {
    /// Action available in any view of the IDE (default).
    #[default]
    Any,
    /// Action exclusive to the Visual Form Designer.
    Design,
    /// Action exclusive to the Code Editor.
    Code,
    /// Action switching view (exempt from context guard).
    ViewSwitch,
}

#[note("place")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Place {
    /// The section in the menu; a submenu row sits in the section of its actions.
    pub section: u8,
    /// The submenu the action opens from, if any.
    pub submenu: Option<&'static str>,
    /// The section inside that submenu.
    pub sub_section: u8,
}

impl Place {
    #[note("place-top")]
    pub const TOP: Self = Self::section(0);

    pub const fn section(section: u8) -> Self {
        Self {
            section,
            submenu: None,
            sub_section: 0,
        }
    }

    pub const fn submenu(section: u8, name: &'static str, sub_section: u8) -> Self {
        Self {
            section,
            submenu: Some(name),
            sub_section,
        }
    }
}

#[note("action")]
pub struct Action {
    /// Stable identifier — this is what RTA and MCP expose.
    pub id: &'static str,
    pub label: &'static str,
    pub hint: &'static str,
    /// Group in the interface: "File", "Edit", "Align", "Order"…
    pub group: &'static str,
    /// Where it sits in its menu; see [`Place`].
    pub place: Place,
    /// Display shortcut; the IDE translates for its own keyboard.
    pub shortcut: Option<&'static str>,
    /// What the action receives. Empty in most; this is what the MCP publishes as
    /// tool signature.
    pub params: &'static [Param],
    pub view: ViewAffinity,
    /// Can it run right now? Its need, refined by a check when the need
    /// alone does not decide; the unmet need is the reason it is greyed.
    pub enabled: Gate,
    /// What to do. Never applies anything: returns the effect.
    pub execute: fn(&Context) -> Effect,
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Action")
            .field("id", &self.id)
            .field("label", &self.label)
            .finish()
    }
}

#[note("removal")]
fn removal(c: &Context, label: &str) -> Option<Transaction> {
    let doc = c.ir()?;
    let mut targets: Vec<NodeId> = Vec::new();
    for id in c.sel().into_iter().filter(|i| i != &doc.root) {
        for d in doc.subtree(&id) {
            if !targets.contains(&d) {
                targets.push(d);
            }
        }
    }
    if targets.is_empty() {
        return None;
    }

    let mut tx = Transaction::new(label, c.author.clone());
    for target in &targets {
        for referrer in doc.referrers(target) {
            if !targets.contains(&referrer)
                && let Some(n) = doc.node(&referrer)
            {
                for (key, v) in &n.props {
                    if matches!(v, crawrad_ir::Value::Ref(r) if r == target) {
                        tx.push(Command::SetProp {
                            node: referrer.clone(),
                            key: key.clone(),
                            value: Some(crawrad_ir::Value::Ref(String::new())),
                        });
                    }
                }
            }
        }
    }
    // leaf before parent
    for target in targets.into_iter().rev() {
        tx.push(Command::RemoveNode { node: target });
    }
    Some(tx)
}

// ---------------------------------------------------------------- predicates

#[note("handler-in-use")]
fn handler_in_use(ir: &crawrad_ir::Document, handler: &str) -> bool {
    ir.nodes
        .values()
        .any(|n| n.events.values().any(|h| h == handler))
}

#[note("handler-edit")]
pub fn handler_edit(
    ir: Option<&crawrad_ir::Document>,
    node: &str,
    event: &str,
    text: &str,
) -> (&'static str, Args) {
    let name = text.trim();
    let bound = ir
        .and_then(|d| d.node(node))
        .is_some_and(|n| n.events.contains_key(event));
    let args = Args::text("node", node).and("event", Value::Str(event.into()));
    if bound && !name.is_empty() && !ir.is_some_and(|d| handler_in_use(d, name)) {
        ("event.rename", args.and("name", Value::Str(name.into())))
    } else {
        ("event.bind", args.and("handler", Value::Str(text.into())))
    }
}

const INHERITED_EVENT_PARAMS: &[Param] = &[
    Param {
        name: "event",
        kind: ParamKind::String,
        description: "the event the base form handles, as the catalogue names it",
        list: false,
        required: true,
        domain: None,
    },
    Param {
        name: "node",
        kind: ParamKind::ComponentId,
        description: "id of the inherited component that owns the event",
        list: false,
        required: true,
        domain: None,
    },
];

#[note("inherited-event-action")]
fn inherited_event_action(
    id: &'static str,
    label: &'static str,
    hint: &'static str,
    execute: fn(&Context) -> Effect,
) -> Action {
    Action {
        id,
        label,
        hint,
        group: "Component",
        place: Place::TOP,
        shortcut: None,
        params: INHERITED_EVENT_PARAMS,
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute,
    }
}
const NOT_INHERITED_EVENT: &str = "that event is not one the base form handles";

#[note("inheritedevent")]
struct InheritedEvent {
    node: String,
    event: String,
    current: String,
    base: String,
}

#[note("inherited-event-of")]
fn inherited_event_of(c: &Context) -> Option<InheritedEvent> {
    let doc = c.doc?;
    let node = c.args.as_text("node")?;
    let event = c.args.as_text("event")?;
    let base = doc
        .base()?
        .node(node)?
        .events
        .get(event)?
        .trim()
        .to_string();
    let current = doc.ir().node(node)?.events.get(event);
    Some(InheritedEvent {
        node: node.to_string(),
        event: event.to_string(),
        current: current.map(|h| h.trim().to_string()).unwrap_or_default(),
        base,
    })
    .filter(|e| !e.base.is_empty() && doc.is_inherited(node))
}

#[note("path-arg")]
fn path_arg(c: &Context) -> PathBuf {
    c.args
        .as_text("path")
        .map(PathBuf::from)
        .unwrap_or_default()
}

#[note("recovery-action")]
fn recovery_action(
    id: &'static str,
    label: &'static str,
    hint: &'static str,
    execute: fn(&Context) -> Effect,
) -> Action {
    const FILES: &[Param] = &[Param {
        name: "files",
        kind: ParamKind::String,
        description: "paths in the project, e.g. src/main.rs — none: every copy",
        list: true,
        required: false,
        domain: None,
    }];
    Action {
        id,
        label,
        hint,
        group: "File",
        place: Place::TOP,
        shortcut: None,
        params: FILES,
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute,
    }
}

fn recovery_effect(c: &Context, make: fn(Vec<PathBuf>) -> FileEffect) -> Effect {
    let files = c.args.as_texts("files").into_iter().map(PathBuf::from);
    Effect {
        file: Some(make(files.collect())),
        ..Default::default()
    }
}

/// One component is selected, it is not the form and it is not inherited.
fn one_own_component(c: &Context) -> bool {
    c.doc.is_some_and(|d| match d.selection() {
        [id] => *id != d.ir().root && !d.is_inherited(id),
        _ => false,
    })
}

fn two_or_more(c: &Context) -> bool {
    c.doc.is_some_and(|d| d.selection().len() >= 2)
}

fn three_or_more(c: &Context) -> bool {
    c.doc.is_some_and(|d| d.selection().len() >= 3)
}

#[note("action-list-node")]
fn action_list_node(c: &Context) -> Option<NodeId> {
    let doc = c.ir()?;
    let id = c.doc?.selection().first()?;
    let n = doc.node(id)?;
    match n.class.as_str() {
        "ActionList" => Some(id.clone()),
        "Action" => doc
            .nodes
            .values()
            .find(|p| p.children.contains(id))
            .map(|p| p.id.clone()),
        _ => None,
    }
}

#[note("page-control")]
fn page_control(c: &Context) -> Result<NodeId, String> {
    let Some(ir) = c.ir() else {
        return Err("new page: no form is open".into());
    };
    // The ARGUMENT comes first, not the selection: on the other side of the protocol
    // there is no window at all, and whoever specifies the node must dictate the response.
    if let Some(id) = c.args.as_text("control").filter(|s| !s.is_empty()) {
        let Some(n) = ir.node(id) else {
            return Err(format!("no such component: `{id}`"));
        };
        if n.class != "PageControl" {
            return Err(format!("`{id}` is a {}, not a PageControl", n.class));
        }
        return Ok(id.to_string());
    }
    let Some(sel) = c.doc.and_then(|d| d.selection().first()) else {
        return Err("new page: select a PageControl, or name one in `control`".into());
    };
    let Some(n) = ir.node(sel) else {
        return Err(format!("no such component: `{sel}`"));
    };
    match n.class.as_str() {
        "PageControl" => Ok(sel.clone()),
        "TabSheet" => ir
            .nodes
            .values()
            .find(|p| p.children.contains(sel))
            .map(|p| p.id.clone())
            .ok_or_else(|| format!("`{sel}` is a page with no PageControl")),
        other => Err(format!("`{sel}` is a {other}, not a PageControl")),
    }
}

#[note("pages-of")]
fn pages_of(ir: &crawrad_ir::Document, control: &str) -> usize {
    ir.node(control)
        .map(|n| {
            n.children
                .iter()
                .filter(|c| ir.node(c).is_some_and(|f| f.class == "TabSheet"))
                .count()
        })
        .unwrap_or(0)
}

// ---------------------------------------------------------------- catalogue

#[note("shortcut-for-scheme")]
pub fn shortcut_for_scheme(id: &str, scheme: KeymapScheme) -> Option<&'static str> {
    match scheme {
        KeymapScheme::Modern => default_shortcut(id),
        KeymapScheme::Classic => classic_shortcut(id),
    }
}

pub fn classic_shortcut(id: &str) -> Option<&'static str> {
    match id {
        "project.build_manager" => Some("Shift+F9"),
        "project.run" => Some("F9"),
        "project.generate" => Some("Ctrl+F9"),
        "debug.toggle_breakpoint" => Some("F5"),
        "project.run_without_debugging" => Some("Ctrl+Shift+F9"),
        "debug.step_over" => Some("F8"),
        "debug.step_into" => Some("F7"),
        "debug.step_out" => Some("Shift+F8"),
        "debug.pause" => None,
        "debug.stop" => Some("Ctrl+F2"),
        "debug.add_watch" => Some("Ctrl+F5"),
        "edit.redo" => Some("Ctrl+Shift+Z"),
        "file.quick_open" => Some("Ctrl+O"),
        "view.command_palette" => Some("Ctrl+Shift+P"),
        "tools.options" => None,
        // The vendor's *Find Declaration* and *Browse back / forward*
        // (Default Keyboard Shortcuts page of the classic RAD IDE).
        "editor.goto_definition" => Some("Alt+Up"),
        "editor.navigate_back" => Some("Alt+Left"),
        "editor.navigate_forward" => Some("Alt+Right"),
        // The vendor's *Edit > Format Source* (same page).
        "editor.format_document" => Some("Ctrl+D"),
        // The vendor's *Find References* (same page).
        "editor.find_references" => Some("Ctrl+Shift+Enter"),
        // The vendor's *File > Save All* (same page).
        "file.save_all" => Some("Ctrl+Shift+S"),
        // The vendor's *Refactor > Rename* (docwiki, *Renaming a Symbol*).
        "editor.rename_symbol" => Some("Ctrl+Shift+E"),
        // ...so the designer's duplicate, ours, steps aside to Ctrl+Shift+D.
        "edit.duplicate" => Some("Ctrl+Shift+D"),
        _ => default_shortcut(id),
    }
}

pub fn default_shortcut(id: &str) -> Option<&'static str> {
    match id {
        "file.save" => Some("Ctrl+S"),
        "file.quick_open" => Some("Ctrl+P"),
        "view.command_palette" => Some("Ctrl+Shift+P"),
        "edit.undo" => Some("Ctrl+Z"),
        "edit.redo" => Some("Ctrl+Y"),
        "edit.copy" => Some("Ctrl+C"),
        "edit.cut" => Some("Ctrl+X"),
        "edit.paste" => Some("Ctrl+V"),
        "edit.duplicate" => Some("Ctrl+D"),
        "edit.delete" => Some("Del"),
        "edit.select_all" => Some("Ctrl+A"),
        // The text's twins: the same keys, the code view's. Which one
        // runs is the view in front (the shell's `keyboard::not_here`).
        // The modern editor convention and the classic IDEs' Shift+F10.
        "editor.context_menu" => Some("Shift+F10"),
        "editor.cut" => Some("Ctrl+X"),
        "editor.copy" => Some("Ctrl+C"),
        "editor.paste" => Some("Ctrl+V"),
        "editor.select_all" => Some("Ctrl+A"),
        "edit.lock_controls" => Some("Ctrl+Shift+L"),
        "project.build_manager" => Some("Ctrl+Shift+B"),
        "project.run" => Some("F5"),
        "project.generate" => None,
        "debug.toggle_breakpoint" => Some("F9"),
        "project.run_without_debugging" => Some("Ctrl+F5"),
        "debug.step_over" => Some("F10"),
        "debug.step_into" => Some("F11"),
        "debug.step_out" => Some("Shift+F11"),
        "debug.pause" => Some("F6"),
        "debug.stop" => Some("Shift+F5"),
        "view.toggle_code" => Some("F12"),
        "view.toggle_form_source" => Some("Alt+F12"),
        // The modern scheme's `F12` is ours (Design / Code); `Ctrl+F12` is the free
        // chord next to it. Back / forward follow the modern convention.
        "editor.goto_definition" => Some("Ctrl+F12"),
        "editor.navigate_back" => Some("Alt+Left"),
        "editor.navigate_forward" => Some("Alt+Right"),
        // Modern editors use `Ctrl+K Ctrl+I`; the keymap has no chords,
        // so `Ctrl+K` alone opens it (and the chord's `Ctrl+I` has no owner).
        "editor.show_hover" => Some("Ctrl+K"),
        // Modern editor *Trigger Parameter Hints*; the classic RAD IDE's *Code
        // Parameters* is the same chord, so both schemes share it.
        "editor.parameter_hints" => Some("Ctrl+Shift+Space"),
        // Modern editor *Quick Fix*, in both schemes. Not the issue's Alt+Enter:
        // on Windows that chord is DXGI's full-screen toggle and ends the
        // IDE.
        "editor.code_actions" => Some("Ctrl+."),
        // The modern editor convention.
        "editor.format_document" => Some("Shift+Alt+F"),
        // Modern editor *References: Find All References*.
        "editor.find_references" => Some("Shift+Alt+F12"),
        "view.toggle_problems" => Some("Ctrl+Shift+M"),
        "tools.options" => Some("Ctrl+,"),
        "tools.sidecar_note" => Some("Ctrl+Shift+N"),
        "editor.add_documentation" => Some("Ctrl+Alt+D"),
        "editor.add_note" => Some("Ctrl+Alt+N"),
        "editor.add_comment" => Some("Ctrl+Alt+M"),
        "view.zoom_in" => Some("Ctrl+Shift+."),
        "view.zoom_out" => Some("Ctrl+Shift+,"),
        "view.zoom_reset" => Some("Ctrl+1"),
        _ => None,
    }
}

#[note("notekindrow")]
type NoteKindRow = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    fn(&Context) -> Effect,
);

#[note("note-effect")]
fn note_effect(kind: NoteKindChoice) -> Effect {
    Effect {
        view: Some(ViewEffect::Note(Some(kind))),
        ..Effect::none()
    }
}

#[note("catalogue")]
pub fn catalogue() -> Vec<Action> {
    catalogue_with_scheme(KeymapScheme::Modern)
}

#[note("catalogue-with-scheme")]
pub fn catalogue_with_scheme(scheme: KeymapScheme) -> Vec<Action> {
    let mut v = base_catalogue();
    for a in &mut v {
        a.shortcut = shortcut_for_scheme(a.id, scheme);
    }
    v
}

/// The one argument of `component.select`: which component the Object Inspector shows.
const COMPONENT_ID_PARAM: Param = Param {
    name: "id",
    kind: ParamKind::String,
    description: "Id of the component to select",
    list: false,
    required: true,
    domain: None,
};

fn base_catalogue() -> Vec<Action> {
    let mut v = vec![
        Action {
            id: "project.new",
            label: "New Project...",
            hint: "Create a Cargo project — cargo does it, CrawRAD asks the questions",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            // ALWAYS enabled: creating a project does not depend on having an open one —
            // it is precisely what is done when there is none.
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |_| Effect {
                file: Some(FileEffect::NewProject),
                ..Default::default()
            },
        },
        Action {
            id: "project.add",
            label: "Add Project...",
            hint: "Create a new project inside the open workspace, and list it as a member",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "name",
                    kind: ParamKind::String,
                    description: "The new project; without it, the New Project dialog opens",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "template",
                    kind: ParamKind::String,
                    description: "The template id (console_app, library_crate, gui_app…); default console_app",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |ctx| Effect {
                file: Some(FileEffect::AddProject {
                    name: ctx.args.as_text("name").map(str::to_string),
                    template: ctx.args.as_text("template").map(str::to_string),
                }),
                ..Default::default()
            },
        },
        Action {
            id: "project.add_existing",
            label: "Add Existing Project...",
            hint: "List a project folder inside the workspace as one of its members",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "The project folder; without it, a folder picker opens",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |ctx| Effect {
                file: Some(FileEffect::AddExistingProject(
                    ctx.args.as_text("path").map(PathBuf::from),
                )),
                ..Default::default()
            },
        },
        Action {
            id: "project.set_active",
            label: "Set as Active Project",
            hint: "The project Run and Generate work on (the workspace's default-members)",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "The member project folder",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |ctx| Effect {
                file: Some(FileEffect::SetActiveProject(path_arg(ctx))),
                ..Default::default()
            },
        },
        Action {
            id: "project.open_folder",
            label: "Open Project...",
            hint: "Open a project folder — the one named, or the folder picker",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "The project folder; without it, a folder picker opens",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |ctx| Effect {
                file: Some(FileEffect::OpenProject(
                    ctx.args.as_text("path").map(PathBuf::from),
                )),
                ..Default::default()
            },
        },
        Action {
            id: "file.open_form",
            label: "Open Form...",
            hint: "Open a form (.crawform) — the one named, or the file picker",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "The form file; without it, a file picker opens",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |ctx| Effect {
                file: Some(FileEffect::OpenForm(
                    ctx.args.as_text("path").map(PathBuf::from),
                )),
                ..Default::default()
            },
        },
        Action {
            id: "project.forget_recent",
            label: "Remove from Recent",
            hint: "Take a folder out of the recent projects — the list, never the disk",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "The folder to take out of the list",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |ctx| match ctx.args.as_text("path").filter(|p| !p.is_empty()) {
                Some(p) => Effect {
                    file: Some(FileEffect::ForgetRecent(PathBuf::from(p))),
                    ..Default::default()
                },
                None => Effect::none().with_status("refused: `path` is required — which folder?"),
            },
        },
        Action {
            id: "file.new_form",
            label: "New Form...",
            hint: "Create a new form (.crawform) in the current project",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |_| Effect {
                file: Some(FileEffect::NewForm),
                ..Default::default()
            },
        },
        Action {
            id: "file.new_inherited_form",
            label: "New Inherited Form...",
            hint: "Create a form that inherits the components and handlers of another",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::FormInProject.only(),
            execute: |_| Effect {
                file: Some(FileEffect::NewInheritedForm),
                ..Default::default()
            },
        },
        Action {
            id: "file.new_component",
            label: "New Component...",
            hint: "Create a user component that inherits an existing one or composes several",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |_| Effect {
                file: Some(FileEffect::NewComponent),
                ..Default::default()
            },
        },
        Action {
            id: "file.new_file",
            label: "New File...",
            hint: "Create a new Rust file (.rs) in the current project",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |_| Effect {
                file: Some(FileEffect::NewFile),
                ..Default::default()
            },
        },
        Action {
            id: "file.quick_open",
            label: "Quick Open...",
            hint: "Quickly open any file in the workspace",
            group: "File",
            place: Place::TOP,
            shortcut: Some("Ctrl+P"),
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                view: Some(ViewEffect::QuickOpen),
                ..Default::default()
            },
        },
        Action {
            id: "file.save",
            label: "Save",
            hint: "Save what is in front — the edited file, or the form",
            group: "File",
            place: Place::TOP,
            shortcut: Some("Ctrl+S"),
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Changes.only(),
            execute: |_| Effect {
                file: Some(FileEffect::Save),
                ..Default::default()
            },
        },
        Action {
            id: "file.save_all",
            label: "Save All",
            hint: "Save every changed file: the code in every tab, then the form",
            group: "File",
            place: Place::TOP,
            // The modern scheme's Save All is the chord `Ctrl+K S`, which this shell
            // does not take; the classic scheme gives it its own key.
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Changes.only(),
            execute: |_| Effect {
                file: Some(FileEffect::SaveAll),
                ..Default::default()
            },
        },
        Action {
            id: "file.reload",
            label: "Reload",
            hint: "Discard changes and reload from disk",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Form.only(),
            execute: |_| Effect {
                file: Some(FileEffect::Reload),
                ..Default::default()
            },
        },
        Action {
            id: "file.close",
            label: "Close",
            // The tooltip says what does NOT happen, and it is not decoration: in a menu that has
            // `Open Folder…` three lines above, `Close` by itself reads as "close
            // everything". What it closes is the form.
            hint: "Close the form — the folder stays open",
            group: "File",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Form.only(),
            execute: |_| Effect {
                file: Some(FileEffect::Close),
                ..Default::default()
            },
        },
        recovery_action(
            "file.recover_unsaved",
            "Recover Unsaved Work",
            "Bring recovery copies back as unsaved edits — the file on disk stays as it is",
            |c| recovery_effect(c, FileEffect::Recover),
        ),
        recovery_action(
            "file.discard_recovery",
            "Discard Recovery Copies",
            "Delete recovery copies left by a crash",
            |c| recovery_effect(c, FileEffect::DiscardRecovery),
        ),
        Action {
            id: "edit.undo",
            label: "Undo",
            hint: "Undo the last change",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+Z"),
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Form.and(|c| c.doc.is_some_and(|d| d.can_undo())),
            execute: |_| Effect {
                history: Some(History::Undo),
                ..Default::default()
            },
        },
        Action {
            id: "edit.redo",
            label: "Redo",
            hint: "Redo the last undone change",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+Y"),
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Form.and(|c| c.doc.is_some_and(|d| d.can_redo())),
            execute: |_| Effect {
                history: Some(History::Redo),
                ..Default::default()
            },
        },
        Action {
            id: "edit.cut",
            label: "Cut",
            hint: "Cut the selected components",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+X"),
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let mut area = Clipboard::default();
                if area.copy(ir, &c.sel()) == 0 {
                    return Effect::none();
                }
                let Some(tx) = removal(c, "Cut") else {
                    return Effect::none();
                };
                Effect {
                    copy_text: Some(area.to_text()),
                    transaction: Some(tx),
                    selection: Some(Vec::new()),
                    ..Default::default()
                }
            },
        },
        Action {
            id: "edit.copy",
            label: "Copy",
            hint: "Copy the selected components",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+C"),
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let mut area = Clipboard::default();
                let n = area.copy(ir, &c.sel());
                if n == 0 {
                    return Effect::none().with_status("nothing to copy");
                }
                Effect {
                    copy_text: Some(area.to_text()),
                    status: Some(format!("{n} component(s) copied")),
                    ..Default::default()
                }
            },
        },
        Action {
            id: "edit.paste",
            label: "Paste",
            hint: "Paste components from the clipboard",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+V"),
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Clipboard.only(),
            execute: |c| match c
                .ir()
                .and_then(|ir| c.clipboard.paste(ir, |cl| c.new_id(cl)))
            {
                Some((tx, created)) => Effect::tx(tx).with_selection(created),
                None => Effect::none().with_status("clipboard is empty"),
            },
        },
        Action {
            id: "edit.delete",
            label: "Delete",
            hint: "Delete the selected components",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Del"),
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| match removal(c, "Delete") {
                Some(tx) => Effect::tx(tx).with_selection(Vec::new()),
                None => Effect::none(),
            },
        },
        Action {
            id: "edit.select_all",
            label: "Select all",
            hint: "Select every component on the form",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+A"),
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Form.and(|c| {
                c.ir()
                    .and_then(|ir| ir.node(&ir.root))
                    .is_some_and(|n| !n.children.is_empty())
            }),
            execute: |c| {
                let all = c
                    .ir()
                    .and_then(|ir| ir.node(&ir.root))
                    .map(|n| n.children.clone())
                    .unwrap_or_default();
                Effect::none().with_selection(all)
            },
        },
        Action {
            id: "editor.cut",
            label: "Cut",
            hint: "Cut the selected text to the clipboard",
            // The code editor's TEXT, not the designer's components (those
            // are `edit.*`, Design only). Same keys, other view.
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+X"),
            params: &[],
            view: ViewAffinity::Code,
            enabled: Need::TextSelection.only(),
            execute: |_| Effect {
                editor: Some(EditorCommand::Cut),
                ..Effect::none()
            },
        },
        Action {
            id: "editor.copy",
            label: "Copy",
            hint: "Copy the selected text to the clipboard",
            // The code editor's TEXT, not the designer's components (those
            // are `edit.*`, Design only). Same keys, other view.
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+C"),
            params: &[],
            view: ViewAffinity::Code,
            enabled: Need::TextSelection.only(),
            execute: |_| Effect {
                editor: Some(EditorCommand::Copy),
                ..Effect::none()
            },
        },
        Action {
            id: "editor.paste",
            label: "Paste",
            hint: "Paste the clipboard's text at the caret",
            // The code editor's TEXT, not the designer's components (those
            // are `edit.*`, Design only). Same keys, other view.
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+V"),
            params: &[],
            view: ViewAffinity::Code,
            enabled: Need::TextClipboard.only(),
            execute: |_| Effect {
                editor: Some(EditorCommand::Paste),
                ..Effect::none()
            },
        },
        Action {
            id: "editor.select_all",
            label: "Select all",
            hint: "Select all the text of the file",
            // The code editor's TEXT, not the designer's components (those
            // are `edit.*`, Design only). Same keys, other view.
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+A"),
            params: &[],
            view: ViewAffinity::Code,
            enabled: Need::CodeEditor.only(),
            execute: |_| Effect {
                editor: Some(EditorCommand::SelectAll),
                ..Effect::none()
            },
        },
        Action {
            id: "edit.duplicate",
            label: "Duplicate",
            hint: "Duplicate the selected components",
            group: "Edit",
            place: Place::TOP,
            shortcut: Some("Ctrl+D"),
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| match c
                .ir()
                .and_then(|ir| clipboard::duplicate(ir, &c.sel(), |cl| c.new_id(cl)))
            {
                Some((tx, created)) => Effect::tx(tx).with_selection(created),
                None => Effect::none(),
            },
        },
    ];

    v.push(Action {
        id: "action.new",
        label: "New action",
        hint: "Add an action to the selected ActionList",
        group: "Edit",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Design,
        enabled: Need::Form.and(|c| action_list_node(c).is_some()),
        execute: |c| {
            let Some(list) = action_list_node(c) else {
                return Effect::none();
            };
            let id = c.new_id("Action");
            let Some(mut n) = c.registry.instantiate(
                "Action",
                id.clone(),
                Rect::new(
                    0.0,
                    0.0,
                    crawrad_registry::ICON_SIZE.0,
                    crawrad_registry::ICON_SIZE.1,
                ),
            ) else {
                return Effect::none();
            };
            // Initial label equal to the name, as classic RAD IDE does with Action1.
            n.props
                .insert("text".into(), crawrad_ir::Value::Str(id.clone()));
            let index = c
                .ir()
                .and_then(|ir| ir.node(&list))
                .map(|l| l.children.len())
                .unwrap_or(0);
            Effect::tx(
                Transaction::new("New action", c.author.clone()).with(Command::AddNode {
                    parent: list,
                    node: n,
                    index,
                }),
            )
            .with_selection(vec![id])
        },
    });

    // NEW PAGE — `AddNode` of the `TabSheet` **and** the `active_page` that opens it,
    // in a single transaction.
    //
    // It was created to remove from the shell the mutation that the ratchet guard of
    // `crawrad-actions` counted as #1 among those escaping the catalogue, and the
    // reason for having escaped is written there: they are TWO commands in the same
    // step. Two catalogue calls would be two `Ctrl+Z`, and a created tab
    // that only opens on the second undo is worse than what existed. The answer
    // was not to compose effects — it was an action that already knows both halves, which is
    // what `action.standard` has done with N `AddNode` since forever.
    //
    // What is gained beyond the removal: the "+" on the tab strip becomes a catalogue
    // gesture, and **an agent also creates a page** (ADR 0005). It was the only
    // way to put a second tab in a `PageControl`, and it only existed for the
    // mouse.
    v.push(Action {
        id: "page.new",
        label: "New page",
        hint: "Add a page to the selected PageControl, and open it",
        // In `Edit`, next to `New action`: it is the same gesture — create a child
        // inside the selected container — and the menu draws it because none
        // of its arguments is required.
        group: "Edit",
        place: Place::TOP,
        // No chord, by the rule of ADR 0011: classic RAD IDE gives no key to *New
        // Page*, and there is no trained finger to respect.
        shortcut: None,
        params: &[
            Param {
                name: "control",
                kind: ParamKind::ComponentId,
                description: "id of the PageControl that gets the page — absent uses the selection",
                list: false,
                required: false,
                domain: None,
            },
            Param {
                name: "text",
                kind: ParamKind::String,
                description: "caption of the new page — absent names it Page2, Page3…",
                list: false,
                required: false,
                domain: None,
            },
        ],
        view: ViewAffinity::Design,
        enabled: Need::Form.and(|c| page_control(c).is_ok()),
        execute: |c| {
            let control = match page_control(c) {
                Ok(id) => id,
                Err(phrase) => return Effect::none().with_status(phrase),
            };
            let Some(ir) = c.ir() else {
                return Effect::none();
            };
            let id = c.new_id("TabSheet");
            // The rectangle is PROVISIONAL, and deliberately not a guess:
            // the page has **derived geometry** (ADR 0012, amendment) and whoever
            // decides it is the normalization, in `crawrad-designer::layout`, which runs in the
            // same turn. Calculating here "below the tab strip, filling
            // the rest" would be a second place stating the same rule — and the day
            // the strip changes height, the two diverge in silence.
            let Some(mut node) =
                c.registry
                    .instantiate("TabSheet", id.clone(), Rect::new(0.0, 0.0, 0.0, 0.0))
            else {
                return Effect::none().with_status("TabSheet is not in the catalogue");
            };
            let pages = pages_of(ir, &control);
            // `Page2`, `Page3`… as in classic RAD IDE, and the argument rules when it comes:
            // an agent creating the "Settings" tab should not have to call
            // `component.set_property` right after to name it.
            let text = c
                .args
                .as_text("text")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map_or_else(|| format!("Page{}", pages + 1), str::to_string);
            node.set_text(text);
            // The page enters at the END of the children, and its index among the tabs
            // is the number of pages that existed — the two counts are different
            // when the control has a child that is not a page.
            let index = ir.node(&control).map(|n| n.children.len()).unwrap_or(0);
            Effect::tx(
                Transaction::new("New page", c.author.clone())
                    .with(Command::AddNode {
                        parent: control.clone(),
                        node,
                        index,
                    })
                    // The new page OPENS. Creating a tab and continuing to look at the
                    // previous one would be invisible work — and in the same batch, so
                    // undo takes both halves at once.
                    .with(Command::SetProp {
                        node: control,
                        key: "active_page".into(),
                        value: Some(Value::Int(pages as i64)),
                    }),
            )
            .with_selection(vec![id])
        },
    });

    v.push(Action {
        id: "action.standard",
        label: "Standard actions...",
        hint: "Add standard actions to the selected ActionList",
        group: "Edit",
        place: Place::TOP,
        shortcut: None,
        // The first action with an argument — and the reason why `Context`
        // gained one. Before it was here returning `Effect::none()`
        // while the real work lived in a loose function: the catalogue,
        // which is the source of the MCP, said the IDE knew how to do this and an agent
        // that called it achieved nothing.
        params: &[Param {
            name: "classes",
            kind: ParamKind::String,
            description: "standard action class names, e.g. EditCopy, EditPaste",
            list: true,
            // Without any name there is nothing to create: `execute` returns
            // `Effect::none()`, and a menu item that calls this stays lit
            // without doing anything.
            required: true,
            domain: None,
        }],
        view: ViewAffinity::Any,
        enabled: Need::Form.and(|c| action_list_node(c).is_some()),
        execute: |c| {
            // By the NAME that the `Param` above publishes: the signature that the MCP
            // shows and the key read here are the same string.
            let names = c.args.as_texts("classes");
            create_standard_actions(c, &names)
        },
    });

    v.push(grid_columns::add_all_fields_action());

    v.push(Action {
        id: "edit.lock_controls",
        label: "Lock controls",
        // "in the designer", and not "with the mouse": the keyboard arrow and
        // `Align ▸ Left` are also refused. The hint promised less than the
        // command does, which amounts to the same defect of promising more.
        hint: "Stop components from being moved or resized in the designer — the Inspector still edits them",
        // In `Edit`, as in classic RAD IDE, and last in the group — which is where its
        // menu puts the command.
        group: "Edit",
        place: Place::TOP,
        // Classic RAD IDE gives no chord to this command, and modern editors do not have the command
        // — so there was no trained finger to respect, and the choice is free
        // within ADR 0011.
        //
        // `Ctrl+Shift+L` is *select all occurrences* in modern editors: a TEXT
        // editing gesture, which is precisely the case of the exception that the ADR already
        // opened for `Ctrl+D` — it means nothing on a drawing canvas, and the
        // `L` for *lock* is the mnemonic that remains. Diverging at source, stating
        // why, is better than diverging in silence.
        shortcut: Some("Ctrl+Shift+L"),
        params: &[],
        // Without a form there is no control to lock, and a switch governing
        // nothing lit on the screen is an invitation to turn it on with no effect — and to
        // think, in the next session with a form, that the IDE is broken.
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            designer: Some(DesignerMode::ToggleLock),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.toggle_code",
        label: "Toggle Design/Code",
        hint: "Switch between the form and the code behind it",
        // Group of its own, not `File`: the three above write to disk or
        // launch a process, and this one only swaps what is on screen. An agent
        // filtering the catalogue by group to know what is safe to execute
        // would get it right by group — and would get it wrong if it lived with `Generate`.
        group: "View",
        place: Place::section(0),
        // The classic RAD IDE key, and it is the reason the tab exists where it exists: the
        // fingers already know. See ADR 0014 item 3 and ADR 0011.
        shortcut: Some("F12"),
        params: &[],
        // Without a form there is neither `Design` nor `Code`: the center is the
        // welcome screen, and toggling between nothing and nothing would light up a command
        // that does nothing.
        view: ViewAffinity::ViewSwitch,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::ToggleDesignCode),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.toggle_form_source",
        label: "Toggle form source",
        hint: "Show the form as text, in place of the drawing",
        group: "View",
        place: Place::section(0),
        // The `Alt+F12` of classic RAD IDE, and the key matters because the gesture is its own:
        // there it shows the form as text, IN PLACE OF the
        // form, and not a file beside the unit.
        //
        // What is shown here is `.designer.rs`, not `.crawform`. They are
        // two texts of the same form at different stages; the generated one is
        // what `cargo`'s error message cites, and therefore is the one that serves.
        shortcut: Some("Alt+F12"),
        params: &[],
        view: ViewAffinity::ViewSwitch,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::ToggleFormSource),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.toggle_live",
        label: "Preview",
        hint: "Run the form inside the IDE — no ruler, no handles, no grid",
        // In `View`, beside `Design`/`Code`: all three answer the same
        // question — what the central surface is showing right now.
        group: "View",
        place: Place::section(0),
        // `F9` is Run in classic RAD IDE and already belongs to `project.run`. Preview has no
        // key in classic RAD IDE (there it does not even exist: the live form IS the program), so
        // there is no trained finger to respect and the chord stays empty until someone
        // asks.
        shortcut: None,
        params: &[],
        // Without a form there is nothing to run, and a lit `Preview` over nothing
        // would draw an empty canvas that looks like a bug.
        view: ViewAffinity::Any,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            designer: Some(DesignerMode::ToggleLive),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.toggle_problems",
        label: "Problems",
        hint: "Show what the last build reported, and click to go there",
        group: "View",
        place: Place::section(0),
        // The modern editor key for the same panel (ADR 0011).
        // Classic RAD IDE has no chord for Messages — it is a docked panel opened
        // from the menu —, so there was no trained finger to respect.
        shortcut: Some("Ctrl+Shift+M"),
        params: &[],
        // **Without a document too.** A console project created by
        // `New Project…` has no form and compiles just the same; tying
        // this panel to the document would wipe the compilation error precisely
        // for whoever has only code.
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::ToggleProblems),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "editor.go_to",
        label: "Go to Line and Column",
        hint: "Put the caret on a line and column of the file open in the editor",
        group: "Edit",
        place: Place::TOP,
        shortcut: None,
        params: &[
            Param {
                name: "line",
                kind: ParamKind::Integer,
                description: "the line, 1 for the first",
                list: false,
                required: true,
                domain: None,
            },
            Param {
                name: "column",
                kind: ParamKind::Integer,
                description: "the column, 1 for the first; 1 when left out",
                list: false,
                required: false,
                domain: None,
            },
        ],
        view: ViewAffinity::Any,
        enabled: Need::CodeEditor.only(),
        execute: |c| match c.args.as_number("line").filter(|n| *n >= 1.0) {
            Some(line) => Effect {
                editor: Some(EditorCommand::GoTo {
                    line: line as u32,
                    column: c
                        .args
                        .as_number("column")
                        .filter(|n| *n >= 1.0)
                        .unwrap_or(1.0) as u32,
                }),
                ..Effect::none()
            },
            None => Effect::none(),
        },
    });

    v.push(Action {
        id: "editor.complete",
        label: "Trigger Suggest",
        hint: "Open the completion list at the caret: the component's fields at once, rust-analyzer's items when they arrive",
        group: "Edit",
        place: Place::TOP,
        // `Ctrl+Space` is read by the editor's own keys (`editor/keys.rs`);
        // declaring it here too would open the list twice.
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        // Grey with nothing open: a form in the designer or code in the editor.
        enabled: Need::OpenFile.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::Complete),
            ..Effect::none()
        },
    });
    v.push(Action {
        id: "editor.templates",
        label: "Code Templates",
        hint: "Insert a ready snippet at the caret: Tab walks its stops",
        group: "Edit",
        place: Place::TOP,
        // `Ctrl+J` is read by the editor's own chords (`editor_chords`).
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::OpenFile.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::Templates),
            ..Effect::none()
        },
    });
    v.push(Action {
        id: "editor.manage_templates",
        label: "Manage Code Templates\u{2026}",
        hint: "Add, change, import and export your code templates",
        group: "Tools",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::OpenFile.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::ManageTemplates),
            ..Effect::none()
        },
    });

    // GOING TO A DEFINITION and back. In `Go` / `Refactor` and not `Edit`: the
    // `Edit` chords belong to the text editor while it has the focus
    // (`TEXT_GROUP` in the shell), and these must reach the catalog.
    for (id, label, hint, group, place, command) in [
        (
            "editor.goto_definition",
            "Go to Definition",
            "Ask rust-analyzer where the symbol at the caret is defined and go there; Ctrl+Click does the same",
            "Go",
            Place::section(0),
            EditorCommand::GoToDefinition,
        ),
        (
            "editor.navigate_back",
            "Go Back",
            "Back to where the last Go to Definition left",
            "Go",
            Place::section(1),
            EditorCommand::NavigateBack,
        ),
        (
            "editor.navigate_forward",
            "Go Forward",
            "Forward again, to where Go Back left",
            "Go",
            Place::section(1),
            EditorCommand::NavigateForward,
        ),
        (
            "editor.format_document",
            "Format Document",
            "Format the whole file with rust-analyzer (rustfmt); one Ctrl+Z undoes it. Not the generated form source",
            "Refactor",
            Place::section(1),
            EditorCommand::FormatDocument,
        ),
        (
            "editor.find_references",
            "Find References",
            "Every use of the name at the caret, its declaration included, in the References panel; the generated form source is marked",
            "Go",
            Place::section(0),
            EditorCommand::FindReferences,
        ),
        (
            "editor.rename_symbol",
            "Rename Symbol",
            "Rename the name at the caret in every file, after a preview, in memory (F2 in the code editor). A component or a handler is renamed from the Object Inspector",
            "Refactor",
            Place::section(0),
            EditorCommand::RenameSymbol,
        ),
        (
            "editor.show_hover",
            "Show Hover",
            "rust-analyzer's signature and docs for the name at the caret; resting the mouse on a name does the same",
            "Go",
            Place::section(2),
            EditorCommand::ShowHover,
        ),
        (
            "editor.parameter_hints",
            "Param Hints",
            "rust-analyzer's signature of the call around the caret, its current parameter marked; typing ( or , opens it, Esc closes it",
            "Go",
            Place::section(2),
            EditorCommand::ParameterHints,
        ),
        (
            "editor.context_menu",
            "Context Menu",
            "The code editor's context menu at the caret, as a right click opens it at the pointer",
            "Go",
            Place::section(2),
            EditorCommand::ContextMenu,
        ),
        (
            "editor.code_actions",
            "Quick Fix...",
            "rust-analyzer's code actions and quick fixes at the caret (an import, a qualified path, a rewrite); the light bulb on the line says there are some",
            "Refactor",
            Place::section(0),
            EditorCommand::CodeActions,
        ),
    ] {
        v.push(Action {
            id,
            label,
            hint,
            group,
            place,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::CodeEditor.only(),
            execute: match command {
                EditorCommand::GoToDefinition => |_| Effect {
                    editor: Some(EditorCommand::GoToDefinition),
                    ..Effect::none()
                },
                EditorCommand::NavigateBack => |_| Effect {
                    editor: Some(EditorCommand::NavigateBack),
                    ..Effect::none()
                },
                EditorCommand::ShowHover => |_| Effect {
                    editor: Some(EditorCommand::ShowHover),
                    ..Effect::none()
                },
                EditorCommand::ParameterHints => |_| Effect {
                    editor: Some(EditorCommand::ParameterHints),
                    ..Effect::none()
                },
                EditorCommand::CodeActions => |_| Effect {
                    editor: Some(EditorCommand::CodeActions),
                    ..Effect::none()
                },
                EditorCommand::FormatDocument => |_| Effect {
                    editor: Some(EditorCommand::FormatDocument),
                    ..Effect::none()
                },
                EditorCommand::FindReferences => |_| Effect {
                    editor: Some(EditorCommand::FindReferences),
                    ..Effect::none()
                },
                EditorCommand::ContextMenu => |_| Effect {
                    editor: Some(EditorCommand::ContextMenu),
                    ..Effect::none()
                },
                EditorCommand::RenameSymbol => |_| Effect {
                    editor: Some(EditorCommand::RenameSymbol),
                    ..Effect::none()
                },
                _ => |_| Effect {
                    editor: Some(EditorCommand::NavigateForward),
                    ..Effect::none()
                },
            },
        });
    }

    v.push(Action {
        id: "editor.apply_code_action",
        label: "Apply Code Action",
        hint: "Apply one of the code actions offered at the caret (editor.code_actions lists them), by its index from 0; the generated form source is refused",
        group: "Refactor",
        place: Place::section(0),
        shortcut: None,
        params: &[Param {
            name: "index",
            kind: ParamKind::Integer,
            description: "the action's place in the list, from 0",
            list: false,
            required: true,
            domain: None,
        }],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |c| match c.args.as_number("index").filter(|n| *n >= 0.0) {
            Some(n) => Effect {
                editor: Some(EditorCommand::ApplyCodeAction(n as usize)),
                ..Effect::none()
            },
            None => Effect::none(),
        },
    });

    v.push(Action {
        id: "chat.fix_explain",
        label: "Fix and Explain",
        hint: "Fix the compiler diagnostic on the caret's line and explain why in chat",
        group: "Refactor",
        place: Place::section(0),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::FixExplain),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "ai.add_file_to_chat",
        label: "Add File to Chat",
        hint: "Put @file in the chat message, without sending it",
        group: "Refactor",
        place: Place::section(2),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Code,
        enabled: Need::CodeEditor.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::AddFileToChat),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "ai.add_selection_to_chat",
        label: "Add Selection to Chat",
        hint: "Attach the selected code to the chat message, without sending it",
        group: "Refactor",
        place: Place::section(2),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Code,
        enabled: Need::CodeEditor.and(|c| c.text.code_in_front && c.text.selection),
        execute: |_| Effect {
            editor: Some(EditorCommand::AddSelectionToChat),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "ai.explain_selection",
        label: "Explain Selection",
        hint: "Attach the selected code and ask the chat to explain it",
        group: "Refactor",
        place: Place::section(2),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Code,
        enabled: Need::CodeEditor.and(|c| c.text.code_in_front && c.text.selection),
        execute: |_| Effect {
            editor: Some(EditorCommand::ExplainSelection),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "ai.review_selection",
        label: "Review Selection",
        hint: "Attach the selected code and ask the chat for a read-only review",
        group: "Refactor",
        place: Place::section(2),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Code,
        enabled: Need::CodeEditor.and(|c| c.text.code_in_front && c.text.selection),
        execute: |_| Effect {
            editor: Some(EditorCommand::ReviewSelection),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "editor.keep_mine",
        label: "Keep My Version",
        hint: "Keep your version of the file and overwrite disk on next save",
        group: "File",
        place: Place::submenu(1, "Changed on Disk", 0),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::OpenFile.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::KeepMine),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "editor.reload_from_disk",
        label: "Reload from Disk",
        hint: "Discard your edits and reload the file from disk",
        group: "File",
        place: Place::submenu(1, "Changed on Disk", 0),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::OpenFile.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::ReloadFromDisk),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "editor.compare_with_disk",
        label: "Compare with Disk",
        hint: "Open diff review comparing your edits with the disk copy",
        group: "File",
        place: Place::submenu(1, "Changed on Disk", 0),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::OpenFile.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::CompareWithDisk),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "editor.toggle_inlay_hints",
        label: "Inlay Hints",
        hint: "Show or hide the types rust-analyzer infers, in grey inside the lines",
        group: "View",
        place: Place::section(2),
        // No chord: modern editors bind none to their equivalent.
        shortcut: None,
        params: &[],
        // Code without a form has hints too.
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            editor: Some(EditorCommand::ToggleInlayHints),
            ..Effect::none()
        },
    });

    for (id, label, hint, set) in [
        (
            "editor.toggle_bookmark",
            "Toggle Bookmark",
            "Numbered bookmark 1–9 on the caret's line, or off it (Ctrl+Shift+1…9)",
            true,
        ),
        (
            "editor.go_to_bookmark",
            "Go to Bookmark",
            "The caret to numbered bookmark 1–9 of the file in the editor (Ctrl+1…9)",
            false,
        ),
    ] {
        v.push(Action {
            id,
            label,
            hint,
            group: "Edit",
            place: Place::TOP,
            // The digits are the editor's own keys: a catalogue chord cannot
            // carry the number.
            shortcut: None,
            params: &[Param {
                name: "number",
                kind: ParamKind::Integer,
                description: "the bookmark, 1 to 9",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::CodeEditor.only(),
            execute: if set {
                |c| bookmark_effect(c, EditorCommand::ToggleBookmark)
            } else {
                |c| bookmark_effect(c, EditorCommand::GoToBookmark)
            },
        });
    }

    for (id, label, hint, command) in [
        (
            "lsp.reload_workspace",
            "Reload Workspace",
            "Make rust-analyzer read the Cargo.toml files again",
            LspCommand::ReloadWorkspace,
        ),
        (
            "lsp.restart",
            "Restart Server",
            "Stop rust-analyzer and start it again",
            LspCommand::Restart,
        ),
        (
            "lsp.stop",
            "Stop Server",
            "Stop rust-analyzer; opening a Rust file starts it again",
            LspCommand::Stop,
        ),
    ] {
        v.push(Action {
            id,
            label,
            hint,
            // Not a top menu: modern editors put these behind the status bar item.
            group: "rust-analyzer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: match command {
                LspCommand::ReloadWorkspace => |_| Effect {
                    lsp: Some(LspCommand::ReloadWorkspace),
                    ..Effect::none()
                },
                LspCommand::Restart => |_| Effect {
                    lsp: Some(LspCommand::Restart),
                    ..Effect::none()
                },
                LspCommand::Stop => |_| Effect {
                    lsp: Some(LspCommand::Stop),
                    ..Effect::none()
                },
            },
        });
    }

    v.push(Action {
        id: "project.run",
        label: "Run",
        // The tooltip used to say "target: egui/RXP, **the only generator today**", and that
        // ceased to be true when the Makepad generator was added: today the
        // target comes from the saved preference, and both shells read it. A tooltip
        // that lies is worse than none — and it is not just a tooltip: it is what an
        // RTA agent reads to decide if there is a choice (ADR 0005).
        hint: "Generate, build and run — with the PROJECT's generator, not the surface selector",
        // `Run`, not `File`: the menu follows classic RAD IDE, where Run lives in Run.
        group: "Run",
        place: Place::TOP,
        // F5 is the modern convention; classic RAD IDE's F9 becomes a configurable scheme (ADR 0011).
        shortcut: Some("F5"),
        params: &[],
        // **Not valid for whoever has nothing to run** (ADR 0018, Amendment 3).
        // Measured: in a `--lib` project the build passed and `cargo run -p <lib>`
        // answered `error: a bin target must be available for cargo run`
        // — the IDE promising its central gesture and delivering a tool
        // error.
        //
        // Dimmed, not hidden: this is what modern editors do with a command that does
        // not apply, and is what classic RAD IDE does with a package's `Run`. A button that
        // vanishes leads to looking for where it went; a dimmed button with a tooltip says
        // what is missing.
        view: ViewAffinity::Any,
        enabled: Need::Binary.only(),
        execute: |_| Effect {
            file: Some(FileEffect::Run),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.toggle_breakpoint",
        label: "Toggle Breakpoint",
        hint: "Add or remove the breakpoint on the caret's line of the file in the editor",
        group: "Run",
        place: Place::TOP,
        // F9 in the modern scheme; classic scheme F5 comes from the scheme (`classic_shortcut`).
        shortcut: Some("F9"),
        params: &[],
        // Breakpoint is a CODE line: on the canvas there is no line to mark.
        view: ViewAffinity::Code,
        enabled: Need::CodeEditor.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::ToggleBreakpoint),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.add_watch",
        label: "Add Watch",
        hint: "Watch an expression in the Run and Debug panel — the selection or the word under the caret when none is given",
        group: "Run",
        place: Place::TOP,
        // No key in the modern scheme; `Ctrl+F5` comes from the classic key scheme.
        shortcut: None,
        params: &[Param {
            name: "expression",
            kind: ParamKind::String,
            description: "the expression to watch, e.g. self.edt_nome_.text",
            list: false,
            required: false,
            domain: None,
        }],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |c| Effect {
            debug: Some(DebugCommand::AddWatch(
                c.args.as_text("expression").map(str::to_string),
            )),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.remove_watch",
        label: "Remove Watch",
        hint: "Remove a watch from the Run and Debug panel by its position (0 is the first)",
        group: "Run",
        place: Place::TOP,
        shortcut: None,
        params: &[Param {
            name: "index",
            kind: ParamKind::Integer,
            description: "the watch's position in the list, 0 for the first",
            list: false,
            required: true,
            domain: None,
        }],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |c| match c.args.as_number("index").filter(|n| *n >= 0.0) {
            Some(i) => Effect {
                debug: Some(DebugCommand::RemoveWatch(i as usize)),
                ..Effect::none()
            },
            None => Effect::none(),
        },
    });

    v.push(Action {
        id: "project.run_without_debugging",
        label: "Run Without Debugging",
        hint: "Generate, build and run with no debugger attached — breakpoints are ignored",
        group: "Run",
        place: Place::TOP,
        shortcut: Some("Ctrl+F5"),
        params: &[],
        // The debugger works from any view: F8 steps with the canvas in front.
        view: ViewAffinity::Any,
        enabled: Need::Binary.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::RunWithoutDebugging),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.step_over",
        label: "Step Over",
        hint: "Run the stopped program to the next line, over any call",
        group: "Run",
        place: Place::TOP,
        shortcut: Some("F10"),
        params: &[],
        // The debugger works from any view: F8 steps with the canvas in front.
        view: ViewAffinity::Any,
        enabled: Need::DebugStopped.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::StepOver),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.step_into",
        label: "Step Into",
        hint: "Run the stopped program into the call on this line",
        group: "Run",
        place: Place::TOP,
        shortcut: Some("F11"),
        params: &[],
        // The debugger works from any view: F8 steps with the canvas in front.
        view: ViewAffinity::Any,
        enabled: Need::DebugStopped.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::StepInto),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.step_out",
        label: "Step Out",
        hint: "Run the stopped program until the current function returns",
        group: "Run",
        place: Place::TOP,
        shortcut: Some("Shift+F11"),
        params: &[],
        // The debugger works from any view: F8 steps with the canvas in front.
        view: ViewAffinity::Any,
        enabled: Need::DebugStopped.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::StepOut),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.pause",
        label: "Pause",
        hint: "Stop the running program where it is",
        group: "Run",
        place: Place::TOP,
        shortcut: Some("F6"),
        params: &[],
        // The debugger works from any view: F8 steps with the canvas in front.
        view: ViewAffinity::Any,
        enabled: Need::DebugRunning.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::Pause),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "debug.stop",
        label: "Stop Debugging",
        hint: "End the program under the debugger",
        group: "Run",
        place: Place::TOP,
        shortcut: Some("Shift+F5"),
        params: &[],
        // The debugger works from any view: F8 steps with the canvas in front.
        view: ViewAffinity::Any,
        enabled: Need::DebugSession.only(),
        execute: |_| Effect {
            debug: Some(DebugCommand::Stop),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "project.open_editor",
        label: "Open code",
        hint: "Open the generated project in your editor",
        // The `Project > View Source` of classic RAD IDE.
        group: "Project",
        place: Place::TOP,
        shortcut: None,
        // The argument was born with the Explorer (ADR 0016): clicking a file there
        // opens THAT file, not the folder. It is optional because the `Run`
        // menu continues calling the same action with no argument — two
        // actions to "open in editor" would be two places to fix.
        params: &[Param {
            name: "path",
            kind: ParamKind::String,
            description:
                "file inside the generated project, e.g. src/main.rs — empty opens the folder",
            list: false,
            // Without an argument it opens the FOLDER, which is what the `Run` menu always
            // did. That is why it remains a menu item.
            required: false,
            domain: None,
        }],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |c| {
            // Without an argument it is the folder — what the `Run` menu always did.
            let Some(raw) = c.args.as_text("path").filter(|s| !s.is_empty()) else {
                return Effect {
                    file: Some(FileEffect::OpenInEditor(None)),
                    ..Effect::none()
                };
            };
            // With an argument, it is either a path inside the project or
            // nothing happens. Refusing HERE, and not in the shell, is what guarantees that
            // the MCP and any other applier of `Effect` are covered by the
            // same rule (ADR 0005).
            match PathInProject::new(raw) {
                Some(path) => Effect {
                    file: Some(FileEffect::OpenInEditor(Some(path))),
                    ..Effect::none()
                },
                None => Effect {
                    status: Some(format!(
                        "refused: `{raw}` is not a path inside the generated project"
                    )),
                    ..Effect::none()
                },
            }
        },
    });

    v.push(Action {
        id: "editor.open",
        label: "Open in editor",
        hint: "Open a file of the project in a tab, next to the design",
        // It belongs to the `Project` group and not a new `Editor` one: what it opens is
        // a file OF the generated PROJECT, and the group is where whoever looks for this
        // already looks. A single-item group would be a drawer with one sock.
        group: "Project",
        place: Place::TOP,
        // NO SHORTCUT, and the absence is declared: `F12` already opens the form's
        // unit, which is the file opened all the time. This one here is
        // for ANY file, and choosing which calls for a click — in the
        // Explorer — or an argument, coming from an agent.
        shortcut: None,
        params: &[Param {
            name: "path",
            kind: ParamKind::String,
            description: "file inside the generated project, e.g. src/form_main.rs",
            list: false,
            // REQUIRED, unlike `project.open_editor`: without a path
            // there is no tab to open. There empty opens the FOLDER, which is a second
            // useful thing; here empty is nothing.
            required: true,
            domain: None,
        }],
        view: ViewAffinity::Code,
        enabled: Need::Nothing.only(),
        execute: |c| {
            let Some(raw) = c.args.as_text("path").filter(|s| !s.is_empty()) else {
                return Effect {
                    status: Some("refused: `editor.open` needs a path".to_string()),
                    ..Effect::none()
                };
            };
            // THE SAME VALIDATION as its sister, and for the same reason (ADR 0005):
            // refusing here is what covers the MCP and any other applier of
            // `Effect` with the same rule.
            match PathInProject::new(raw) {
                Some(path) => Effect {
                    file: Some(FileEffect::OpenInTab(path)),
                    ..Effect::none()
                },
                None => Effect {
                    status: Some(format!(
                        "refused: `{raw}` is not a path inside the generated project"
                    )),
                    ..Effect::none()
                },
            }
        },
    });

    v.push(Action {
        id: "project.generate",
        label: "Generate",
        hint: "Generate the Cargo project — with the PROJECT's generator, not the surface selector",
        // The `Project > Build` of classic RAD IDE.
        group: "Project",
        place: Place::TOP,
        shortcut: Some("Ctrl+Shift+B"),
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::FormInProject.only(),
        execute: |_| Effect {
            file: Some(FileEffect::Generate),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "project.build_manager",
        label: "Build...",
        hint: "Configure targets, profiles and build project",
        group: "Project",
        place: Place::TOP,
        shortcut: Some("Ctrl+Shift+B"),
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Project.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::Build),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "project.build_all",
        label: "Build All",
        hint: "Build all workspace members with cargo build --workspace",
        group: "Project",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Project.and(|c| c.project.is_workspace),
        execute: |_| Effect {
            file: Some(FileEffect::BuildAll),
            ..Default::default()
        },
    });

    v.push(Action {
        id: "project.clean_all",
        label: "Clean All",
        hint: "Remove what every workspace member built, with cargo clean",
        group: "Project",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Project.and(|c| c.project.is_workspace),
        execute: |_| Effect {
            file: Some(FileEffect::CleanAll),
            ..Default::default()
        },
    });

    v.push(Action {
        id: "project.options",
        label: "Options...",
        hint: "Configure project title and icon",
        group: "Project",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Project.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::ProjectOptions),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "project.run_language",
        label: "Run Language",
        hint: "Select language for project execution (empty for system language)",
        group: "Project",
        place: Place::TOP,
        shortcut: None,
        params: &[Param {
            name: "code",
            kind: ParamKind::String,
            description: "Language code for execution, or empty string for system language",
            list: false,
            required: false,
            domain: None,
        }],
        view: ViewAffinity::Any,
        enabled: Need::Binary.only(),
        execute: |c| {
            let code = c.args.as_text("code").unwrap_or_default().to_string();
            Effect {
                run_language: Some(code),
                ..Effect::none()
            }
        },
    });

    v.push(Action {
        id: "help.welcome",
        label: "Welcome Page",
        hint: "Show the welcome page — templates and recent folders",
        // Group of its own, in `Help` — which is where whoever already looked for
        // this screen once looks for it again. It is not `View`: those swap
        // how the open form is shown, and this brings a screen that speaks of the
        // IDE, not of the document.
        group: "Help",
        place: Place::TOP,
        // **No chord, and deliberately so.** ADR 0011 says the key is
        // BORROWED and not invented: modern editors bind none to
        // `Help ▸ Welcome`, and classic RAD IDE does not have the screen. A chord chosen here
        // would be the only one in the IDE without a trained finger behind it — and the day there is
        // one, it enters this line and the keyboard gets it for free.
        shortcut: None,
        params: &[],
        // With no form open the welcome page is the centre, and asking for it
        // again shows it again: it is never a dead command.
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::ToggleWelcome),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "help.environment",
        label: "Check Environment...",
        hint: "Check the Rust toolchain and build tools this IDE needs",
        group: "Help",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::EnvironmentCheck),
            ..Effect::none()
        },
    });

    // ---- the component suite ---------------------------------------------
    //
    // The `Component` of classic RAD IDE, and step §10 of ADR 0033. The two actions
    // **declare**; whoever asks is the shell dialog and whoever checks and copies is
    // `crawrad-ide::instalacao`, which is headless and has tests.
    v.push(Action {
        id: "component.install",
        label: "Install Package...",
        hint: "Install a component package — the folder with its manifest.json",
        group: "Component",
        place: Place::TOP,
        // No chord: classic RAD IDE binds no key to `Install Packages…`, and
        // ADR 0011 says the key is BORROWED and not invented.
        shortcut: None,
        params: &[Param {
            name: "path",
            kind: ParamKind::String,
            description: "The package folder (with manifest.json); without it, a folder picker opens",
            list: false,
            required: false,
            domain: None,
        }],
        // ALWAYS enabled: installing a component does not depend on an open project
        // — it is precisely what is done before having one.
        view: ViewAffinity::Design,
        enabled: Need::Nothing.only(),
        execute: |ctx| Effect {
            component: Some(ComponentEffect::Install(
                ctx.args.as_text("path").map(PathBuf::from),
            )),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "component.installed",
        label: "Installed Packages...",
        hint: "What the catalogue loaded — and what it refused, with the reason",
        group: "Component",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Design,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            component: Some(ComponentEffect::Installed),
            ..Effect::none()
        },
    });

    // ---- Tools (Tools ▸ Options...) ---------------------------------------
    v.push(Action {
        id: "tools.options",
        label: "Options...",
        hint: "Configure IDE settings, designer grid, build targets and ACP agents",
        group: "Tools",
        place: Place::TOP,
        shortcut: Some("Ctrl+,"),
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::Options),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "tools.mcp",
        label: "MCP Servers...",
        hint: "Configure connected Model Context Protocol servers and tool integrations",
        group: "Tools",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::Mcp),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "tools.ai_memory",
        label: "AI Custom Instructions...",
        hint: "Global instructions and context shared with all AI assistant sessions",
        group: "Tools",
        place: Place::TOP,
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::AiMemory),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "tools.sidecar_note",
        label: "Notes...",
        hint: "Add or edit documentation, a note or a comment at the caret",
        group: "Tools",
        place: Place::TOP,
        shortcut: Some("Ctrl+Shift+N"),
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::Note(None)),
            ..Effect::none()
        },
    });

    // One action per kind of the note popup; the popup rows say the same.
    let note_kinds: [NoteKindRow; 3] = [
        (
            "editor.add_documentation",
            "Add documentation",
            "Public docs of this item, in the rustdoc format. Shows in cargo doc and docs.rs.",
            "Ctrl+Alt+D",
            |_| note_effect(NoteKindChoice::Doc),
        ),
        (
            "editor.add_note",
            "Add note",
            "Private context for developers: why, history, decisions. Never becomes docs.",
            "Ctrl+Alt+N",
            |_| note_effect(NoteKindChoice::Note),
        ),
        (
            "editor.add_comment",
            "Add comment",
            "A short remark on this line, like a review comment. Open or resolved.",
            "Ctrl+Alt+M",
            |_| note_effect(NoteKindChoice::Comment),
        ),
    ];
    for (id, label, hint, shortcut, execute) in note_kinds {
        v.push(Action {
            id,
            label,
            hint,
            group: "Edit",
            place: Place::section(2),
            shortcut: Some(shortcut),
            params: &[],
            view: ViewAffinity::Any,
            // They act on the code at the caret.
            enabled: Need::CodeEditor.only(),
            execute,
        });
    }

    // BEFORE the zoom: the View menu draws `Panels` and then `Zoom` in one
    // section, and inside a section the catalogue order is the menu.
    v.extend(layout_actions());

    // ---- canvas zoom -----------------------------------------------------
    //
    // **The chords are NOT `Ctrl +` / `Ctrl -` / `Ctrl 0`, and cannot be.**
    // Those three belong to egui: it consumes them at the end of each pass
    // (`Options::zoom_with_keyboard`) and changes the `zoom_factor` of the whole IDE,
    // which is the slider the Settings window shows and the owner uses.
    //
    // And picking a neighbor does not help: egui's `matches_logically`
    // ignores EXTRA modifiers, so `Ctrl+Shift+=` and `Ctrl+Alt+=` also
    // fall into its handler. The whole key has to be different — measured, not
    // assumed.
    //
    // What remains, and why: `Ctrl+Shift+.` and `Ctrl+Shift+,` are `>` and
    // `<` on the keyboard, which is the pair anyone reads as "greater" and "lesser",
    // and modern editors do not use them for anything meaningful on a canvas
    // (ADR 0011).
    // `Ctrl+1` is *actual size* in Photoshop, Illustrator and Figma,
    // literally the same key. Without Shift, and that was MEASURED on screen: with
    // Shift, Windows delivers logical key `!`, egui recognizes it as
    // `Key::Exclamationmark` (it is in its table) and `Num1` never
    // arrives. `Ctrl+Shift+1` was inert on the first round, with no error at all.
    // The pair above escapes because `>` and `<` are NOT in egui's table: the
    // logical key fails, and it falls back to the physical one — `Period` and `Comma`.
    // There is a test asserting this for the entire catalogue.
    //
    // The primary gesture remains `Ctrl` + mouse wheel over the
    // canvas, which is where the hand already is.
    v.push(Action {
        id: "view.zoom_in",
        label: "Zoom in",
        hint: "Enlarge the form on the canvas — the form itself does not change",
        group: "View",
        place: Place::submenu(1, "Zoom", 0),
        shortcut: Some("Ctrl+Shift+."),
        params: &[],
        // Without a form the canvas is the welcome screen, and zooming in on a welcome
        // screen is a lit command that does nothing.
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            zoom: Some(Zoom::In),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.zoom_out",
        label: "Zoom out",
        hint: "Shrink the form on the canvas — the form itself does not change",
        group: "View",
        place: Place::submenu(1, "Zoom", 0),
        shortcut: Some("Ctrl+Shift+,"),
        params: &[],
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            zoom: Some(Zoom::Out),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.zoom_reset",
        label: "Zoom 100%",
        hint: "Back to actual size — the only zoom where the canvas measures what the app will",
        group: "View",
        place: Place::submenu(1, "Zoom", 0),
        shortcut: Some("Ctrl+1"),
        params: &[],
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |_| Effect {
            zoom: Some(Zoom::Reset),
            ..Effect::none()
        },
    });

    // ---- INTERFACE zoom --------------------------------------------------
    //
    // **Not the canvas zoom above**, and the owner separated the two in so many
    // words: *"the zoom is not only in the designer area, it is in ALL OF THE IDE, WITHOUT
    // EXCEPTION"*. The one above is the DESIGN magnifying glass; this one is `Ctrl +` of any
    // editor, which enlarges menu, panels, tree and bar together.
    //
    // **Without `shortcut`, and the absence is house rule, not an oversight.**
    // Their chord is `Ctrl +`/`Ctrl -`/`Ctrl 0`, and in the egui host these
    // keys HAVE AN OWNER: egui itself consumes them and does exactly this, through
    // `zoom_factor`. Declaring them in `shortcut` would make that shell execute the
    // action AND the host's zoom — twice the same step, per keypress.
    //
    // That is why they live in `PREFERRED_SHORTCUTS`, which is the set only the
    // shell WITHOUT key owner binds. In the egui shell the behavior is not
    // lost: it already happens, done by the host, which is what the owner
    // described — *"if you look at egui, it was always there"*.
    //
    // And they are valid WITHOUT A DOCUMENT, unlike the three above: enlarging the
    // interface is useful in a freshly opened IDE, and is when someone arranges the screen
    // before starting.
    v.push(Action {
        id: "view.ui_zoom_in",
        label: "Zoom in (interface)",
        hint: "Enlarge the whole IDE — menus, panels and the canvas together",
        group: "View",
        place: Place::submenu(1, "Zoom", 1),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            ui_zoom: Some(Zoom::In),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.ui_zoom_out",
        label: "Zoom out (interface)",
        hint: "Shrink the whole IDE — menus, panels and the canvas together",
        group: "View",
        place: Place::submenu(1, "Zoom", 1),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            ui_zoom: Some(Zoom::Out),
            ..Effect::none()
        },
    });

    v.push(Action {
        id: "view.ui_zoom_reset",
        label: "Zoom 100% (interface)",
        hint: "Back to the system size — the IDE at the scale the desktop asked for",
        group: "View",
        place: Place::submenu(1, "Zoom", 1),
        shortcut: None,
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            ui_zoom: Some(Zoom::Reset),
            ..Effect::none()
        },
    });

    v.extend(component_actions());
    v.extend(alignment_actions());
    v.extend(order_actions());
    v.push(Action {
        id: "view.command_palette",
        label: "Command Palette",
        hint: "Search and run any command or capability across the IDE",
        group: "View",
        place: Place::section(3),
        shortcut: Some("Ctrl+Shift+P"),
        params: &[],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute: |_| Effect {
            view: Some(ViewEffect::CommandPalette),
            ..Default::default()
        },
    });

    v.push(Action {
        id: "designer.language",
        label: "Designer Language",
        hint: "Switch visual designer language preview (empty for source English)",
        group: "Designer",
        place: Place::TOP,
        shortcut: None,
        params: &[Param {
            name: "code",
            kind: ParamKind::String,
            description: "Language code to preview, or empty string for English source",
            list: false,
            required: false,
            domain: None,
        }],
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |c| {
            let code = c.args.as_text("code").unwrap_or_default().to_string();
            Effect {
                language: Some(code),
                ..Effect::none()
            }
        },
    });
    v.push(Action {
        id: "component.select",
        label: "Select Component",
        hint: "Select a component of the form by id (the Object Inspector picker)",
        group: "Designer",
        place: Place::TOP,
        shortcut: None,
        params: &[COMPONENT_ID_PARAM],
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |c| {
            let id = c.args.as_text("id").unwrap_or_default();
            let known = c.ir().is_some_and(|ir| ir.node(id).is_some());
            Effect {
                selection: known.then(|| vec![id.to_string()]),
                ..Effect::none()
            }
        },
    });
    // AFTER layout, and the order matters: `by_group` preserves declaration
    // order, and the `View` menu is drawn from it. These two are not
    // from the `View` group — see `explorer_actions` —, but the rule of never
    // pushing an existing group applies to whatever comes after.
    v.extend(explorer_actions());
    // After Explorer, and for the same reason written right above: none
    // of them is in the `View` group, and whoever comes after does not push any group.
    v.extend(inspector_actions());
    v
}

#[note("inspector-actions")]
fn inspector_actions() -> Vec<Action> {
    vec![
        Action {
            id: "inspector.open_editor",
            label: "Open property editor",
            hint: "Open whatever edits this property — the lines editor, the collection editor, the align picker",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "The component that owns the property",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "key",
                    kind: ParamKind::String,
                    description: "The catalogue key of the property — `items`, `lines`, `align`",
                    list: false,
                    required: true,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let (node, key) = match property_of(c) {
                    Ok(found) => found,
                    Err(why) => return Effect::none().with_status(why),
                };
                Effect {
                    inspector: Some(InspectorCommand::OpenEditor { node, key }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "inspector.open_collection",
            label: "Open collection editor",
            hint: "Open the dialog of a component's child collection — the Columns of a Grid, the Panels of a StatusBar",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[PARAM_COLLECTION_OWNER],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(node) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
                    return Effect::none()
                        .with_status("refused: `node` is required — which component?".to_string());
                };
                let Some(target) = c.ir().and_then(|ir| ir.node(node)) else {
                    return Effect::none().with_status(format!("no such component: `{node}`"));
                };
                if !action_list::is_flat_collection(c.registry, &target.class) {
                    return Effect::none()
                        .with_status(format!("{} has no item collection", target.class));
                }
                Effect {
                    inspector: Some(InspectorCommand::OpenCollection {
                        node: node.to_string(),
                    }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "inspector.close_editor",
            label: "Close property editor",
            hint: "Close the property editor without writing anything",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |_| Effect {
                inspector: Some(InspectorCommand::CloseEditor),
                ..Effect::none()
            },
        },
        Action {
            id: "inspector.show_events",
            label: "Edit events\u{2026}",
            hint: "Show the Events tab of the selected component",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |_| Effect {
                inspector: Some(InspectorCommand::ShowEvents),
                ..Effect::none()
            },
        },
        Action {
            id: "inspector.rename",
            label: "Rename",
            hint: "Type a new Name for the selected component in the Inspector",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.and(one_own_component),
            execute: |_| Effect {
                inspector: Some(InspectorCommand::EditName),
                ..Effect::none()
            },
        },
        Action {
            id: "form.open_base",
            label: "Go to base form",
            hint: "Open the form this one inherits from",
            group: "Go",
            place: Place::section(3),
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Form.and(|c| c.doc.is_some_and(|d| d.base().is_some())),
            execute: |_| Effect {
                file: Some(FileEffect::OpenBaseForm),
                ..Effect::none()
            },
        },
        Action {
            id: "inspector.toggle_group",
            label: "Toggle property group",
            hint: "Unfold the sub-rows of a property — `Rect`, `Margins`, `Anchors` — or fold them away",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "key",
                kind: ParamKind::String,
                description: "The catalogue key of the property that folds",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(key) = c.args.as_text("key").filter(|s| !s.is_empty()) else {
                    return Effect::none()
                        .with_status("refused: `key` is required — which property should fold?");
                };
                Effect {
                    inspector: Some(InspectorCommand::ToggleGroup {
                        key: key.to_string(),
                    }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "inspector.toggle_category",
            label: "Toggle property category",
            hint: "Fold a category of the Object Inspector — `Layout`, `Advanced` — or open it back",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "label",
                kind: ParamKind::String,
                description: "The label of the category that folds — `Identity`, `Appearance`, `Layout`, `Behavior`, `Advanced`",
                list: false,
                required: true,
                domain: None,
            }],
            // Without a document the panel says `No form open.` and there is no category
            // at all on screen — the same reason as `toggle_group` beside it.
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(label) = c.args.as_text("label").filter(|s| !s.is_empty()) else {
                    return Effect::none()
                        .with_status("refused: `label` is required — which category should fold?");
                };
                Effect {
                    inspector: Some(InspectorCommand::ToggleCategory {
                        label: label.to_string(),
                    }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "inspector.pick_picture",
            label: "Pick picture file",
            hint: "Choose the image file a `Picture` property shows — the file dialog, or `file` to skip it",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "The component that owns the picture",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "key",
                    kind: ParamKind::String,
                    description: "The catalogue key of the picture property — `picture`, `glyph`",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "file",
                    kind: ParamKind::String,
                    description: "The image file to read; without it the file dialog opens",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let (node, key) = match property_of(c) {
                    Ok(found) => found,
                    Err(why) => return Effect::none().with_status(why),
                };
                let file = c.args.as_text("file").filter(|s| !s.is_empty());
                Effect {
                    inspector: Some(InspectorCommand::PickPicture {
                        node,
                        key,
                        file: file.map(str::to_string),
                    }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "inspector.pick_font",
            label: "Pick font",
            hint: "Choose the font a `Font` property shows — the native font dialog, or `font` to skip it",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "The component that owns the font",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "key",
                    kind: ParamKind::String,
                    description: "The catalogue key of the font property — `font`",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "font",
                    kind: ParamKind::String,
                    description: "The font written formatted; without it the font dialog opens",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let (node, key) = match property_of(c) {
                    Ok(found) => found,
                    Err(why) => return Effect::none().with_status(why),
                };
                let font = c.args.as_text("font").filter(|s| !s.is_empty());
                Effect {
                    inspector: Some(InspectorCommand::PickFont {
                        node,
                        key,
                        font: font.map(str::to_string),
                    }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "inspector.set_name_width",
            label: "Set property name width",
            hint: "Move the splitter between the names and the values of the Object Inspector — the width of the name column, in pixels",
            group: "Inspector",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "width",
                kind: ParamKind::Number,
                description: "The width of the name column; the panel clamps it to what it can hold",
                list: false,
                required: true,
                domain: None,
            }],
            // The splitter is drawn over the rows, and without a document
            // there are no rows — the same reason as the folds beside it.
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(width) = c.args.as_number("width").filter(|w| w.is_finite()) else {
                    return Effect::none().with_status(
                        "refused: `width` is required — how wide should the names be?",
                    );
                };
                // Any finite number goes in; the panel clamps it, and this
                // bound only keeps the conversion honest.
                let width = width.round().clamp(-1e6, 1e6) as i32;
                Effect {
                    inspector: Some(InspectorCommand::SetNameWidth { width }),
                    ..Effect::none()
                }
            },
        },
    ]
}

#[note("property-of")]
fn property_of(c: &Context<'_>) -> Result<(String, String), String> {
    let Some(node) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
        return Err("refused: `node` is required — which component?".to_string());
    };
    let Some(key) = c.args.as_text("key").filter(|s| !s.is_empty()) else {
        return Err("refused: `key` is required — which property?".to_string());
    };
    let Some(target) = c.ir().and_then(|ir| ir.node(node)) else {
        return Err(format!("no such component: `{node}`"));
    };
    let known = c
        .registry
        .get(&target.class)
        .is_some_and(|d| d.props.iter().any(|p| p.key == key));
    if !known {
        return Err(format!("`{key}` is not a property of {}", target.class));
    }
    Ok((node.to_string(), key.to_string()))
}

#[note("explorer-actions")]
fn explorer_actions() -> Vec<Action> {
    vec![
        Action {
            id: "explorer.toggle_folder",
            label: "Toggle folder",
            hint: "Open the member node in the Explorer, or close it when it is open",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "The member folder, as the Explorer shows it",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |c| {
                // Empty is an EXPLICIT refusal, not a fold on the root: a
                // blank `path` would fold the empty `PathBuf`, which is not any
                // node and vanishes without saying anything.
                let Some(raw) = c.args.as_text("path").filter(|s| !s.is_empty()) else {
                    return Effect::none()
                        .with_status("refused: `path` is required — which folder should fold?");
                };
                Effect {
                    explorer: Some(ExplorerCommand::ToggleFolder(PathBuf::from(raw))),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.toggle_legend",
            label: "Toggle badge legend",
            hint: "Show or hide `What the badges mean` at the foot of the Explorer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::ToggleLegend),
                ..Effect::none()
            },
        },
        Action {
            id: "explorer.toggle_open_editors",
            label: "Toggle open editors",
            hint: "Show or hide open editors section in Explorer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::ToggleOpenEditors),
                ..Effect::none()
            },
        },
        Action {
            id: "explorer.toggle_outline",
            label: "Toggle outline",
            hint: "Show or hide outline section in Explorer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::ToggleOutline),
                ..Effect::none()
            },
        },
        Action {
            id: "explorer.toggle_timeline",
            label: "Toggle timeline",
            hint: "Show or hide timeline section in Explorer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::ToggleTimeline),
                ..Effect::none()
            },
        },
        Action {
            id: "explorer.open_timeline_entry",
            label: "Open timeline entry",
            hint: "Open local history view comparing the selected entry with current buffer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "index",
                kind: ParamKind::Integer,
                description: "the entry's position in the timeline, 0 for the newest",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |c| match c.args.as_number("index").filter(|n| *n >= 0.0) {
                Some(i) => Effect {
                    explorer: Some(ExplorerCommand::OpenTimelineEntry(i as usize)),
                    ..Effect::none()
                },
                None => Effect::none(),
            },
        },
        Action {
            id: "explorer.toggle_dependencies",
            label: "Toggle dependencies",
            hint: "Show or hide dependencies section in Explorer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::ToggleDependencies),
                ..Effect::none()
            },
        },
        Action {
            id: "explorer.new_file",
            label: "New File...",
            hint: "Create a new file in the selected folder",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Target folder path",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::NewFile(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.new_form",
            label: "New Form...",
            hint: "Create a new form (.crawform) in the selected folder",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Target folder path",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::NewForm(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.new_folder",
            label: "New Folder...",
            hint: "Create a new directory in the selected path",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Target folder path",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::NewFolder(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.open",
            label: "Open",
            hint: "Open selected file in editor or designer",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "File path to open",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    file: Some(FileEffect::OpenInEditor(None)),
                    status: Some(format!("Opening {}", p.display())),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.reveal",
            label: "Reveal in File Explorer",
            hint: "Open system file manager showing this file or folder",
            group: "Explorer",
            place: Place::TOP,
            shortcut: Some("Shift+Alt+R"),
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Path to reveal",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::Reveal(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.copy_path",
            label: "Copy Path",
            hint: "Copy absolute file path to clipboard",
            group: "Explorer",
            place: Place::TOP,
            shortcut: Some("Shift+Alt+C"),
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Path to copy",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::CopyPath(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.copy_rel_path",
            label: "Copy Relative Path",
            hint: "Copy relative file path to clipboard",
            group: "Explorer",
            place: Place::TOP,
            shortcut: Some("Ctrl+K Ctrl+Shift+C"),
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Relative path to copy",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::CopyRelativePath(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.rename",
            label: "Rename...",
            hint: "Rename selected file or folder",
            group: "Explorer",
            place: Place::TOP,
            shortcut: Some("F2"),
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Path to rename",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::Rename(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.delete",
            label: "Delete",
            hint: "Delete selected file or folder",
            group: "Explorer",
            place: Place::TOP,
            shortcut: Some("Del"),
            params: &[Param {
                name: "path",
                kind: ParamKind::String,
                description: "Path to delete",
                list: false,
                required: false,
                domain: None,
            }],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |ctx| {
                let p = path_arg(ctx);
                Effect {
                    explorer: Some(ExplorerCommand::Delete(p)),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "explorer.refresh",
            label: "Refresh Explorer",
            hint: "Refresh explorer file list",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::Refresh),
                ..Effect::none()
            },
        },
        Action {
            id: "explorer.collapse_all",
            label: "Collapse Folders in Explorer",
            hint: "Collapse all expanded folders in the explorer tree",
            group: "Explorer",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Project.only(),
            execute: |_| Effect {
                explorer: Some(ExplorerCommand::CollapseAll),
                ..Effect::none()
            },
        },
    ]
}

// ------------------------------------------------------------------ layout

#[note("panel-choices")]
fn panel_choices() -> Vec<Choice> {
    Panel::ALL
        .iter()
        .map(|p| Choice {
            id: p.id(),
            label: p.label(),
        })
        .collect()
}

#[note("side-choices")]
fn side_choices() -> Vec<Choice> {
    DockSide::ALL
        .iter()
        .map(|s| Choice {
            id: s.id(),
            label: s.label(),
        })
        .collect()
}

/// The `node` of `inspector.open_collection`.
const PARAM_COLLECTION_OWNER: Param = Param {
    name: "node",
    kind: ParamKind::ComponentId,
    description: "The component that owns the collection",
    list: false,
    required: true,
    domain: None,
};

#[note("param-panel")]
const PARAM_PANEL: Param = Param {
    name: "panel",
    kind: ParamKind::Enum(&[]),
    description: "Which panel to act on",
    list: false,
    required: true,
    domain: Some(Domain {
        values: panel_choices,
        // A menu that drew a row per panel for `show`, `hide` AND `float`
        // would say the same thing three times over — see
        // [`Domain::row_per_value`].
        row_per_value: false,
    }),
};

#[note("param-panel-row-per-value")]
const PARAM_PANEL_ROW_PER_VALUE: Param = Param {
    name: "panel",
    kind: ParamKind::Enum(&[]),
    description: "Which panel to act on",
    list: false,
    required: true,
    domain: Some(Domain {
        values: panel_choices,
        row_per_value: true,
    }),
};

#[note("panel-ids")]
fn panel_ids() -> String {
    Panel::ALL
        .iter()
        .map(|p| p.id())
        .collect::<Vec<_>>()
        .join(", ")
}

#[note("panel-from-arg")]
fn panel_from_arg(c: &Context, key: &str) -> Result<Panel, String> {
    let id = c
        .args
        .as_text(key)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("`{key}` is required: one of {}", panel_ids()))?;
    Panel::from_id(id).ok_or_else(|| format!("no such panel: `{id}` — try {}", panel_ids()))
}

#[note("panel-command")]
fn panel_command(c: &Context, make: fn(Panel) -> LayoutCommand) -> Effect {
    match panel_from_arg(c, "panel") {
        Ok(p) => Effect {
            layout: Some(make(p)),
            ..Effect::none()
        },
        Err(phrase) => Effect::none().with_status(phrase),
    }
}

#[note("layout-actions")]
fn layout_actions() -> Vec<Action> {
    vec![
        // TOGGLE COMES FIRST, and the order is the menu: `menu::rows` preserves
        // catalogue order inside a section on purpose, and this is where the
        // `View › Panels` submenu comes from — one row per panel, as the owner
        // designed and as modern editors show.
        Action {
            id: "layout.toggle_panel",
            label: "Toggle panel",
            hint: "Show the panel, or hide it when it is already the visible tab",
            group: "View",
            place: Place::submenu(1, "Panels", 0),
            shortcut: None,
            // The ONLY one with `row_per_value`: a menu draws one row per
            // panel, with the visible tick, and each row carries its own
            // argument. See [`Domain::row_per_value`].
            params: &[PARAM_PANEL_ROW_PER_VALUE],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |c| panel_command(c, LayoutCommand::Toggle),
        },
        Action {
            id: "layout.reset",
            label: "Reset Layout",
            hint: "Put every panel back where the IDE ships it",
            group: "View",
            place: Place::section(3),
            shortcut: None,
            params: &[],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |_| Effect {
                layout: Some(LayoutCommand::Reset),
                ..Effect::none()
            },
        },
        Action {
            id: "layout.show_panel",
            label: "Show panel",
            hint: "Open the panel if it was closed, and focus it",
            group: "View",
            place: Place::TOP,
            shortcut: None,
            params: &[PARAM_PANEL],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |c| panel_command(c, LayoutCommand::Show),
        },
        Action {
            id: "layout.hide_panel",
            label: "Hide panel",
            hint: "Close the panel's tab — the rail brings it back",
            group: "View",
            place: Place::TOP,
            shortcut: None,
            params: &[PARAM_PANEL],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |c| panel_command(c, LayoutCommand::Hide),
        },
        Action {
            id: "layout.float_panel",
            label: "Float panel",
            hint: "Detach the panel over the window — it does not leave the IDE",
            group: "View",
            place: Place::TOP,
            shortcut: None,
            params: &[PARAM_PANEL],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |c| panel_command(c, LayoutCommand::Float),
        },
        Action {
            id: "layout.move_panel",
            label: "Move panel to side",
            hint: "Dock the panel against the left, right or bottom edge",
            group: "View",
            place: Place::TOP,
            shortcut: None,
            params: &[
                PARAM_PANEL,
                Param {
                    name: "side",
                    kind: ParamKind::Enum(&[]),
                    description: "Which edge to dock against",
                    list: false,
                    required: true,
                    domain: Some(Domain {
                        values: side_choices,
                        // Two dimensions (panel x edge) are not ONE list of
                        // rows: whoever collects both is the panel's context
                        // menu, which already knows the panel.
                        row_per_value: false,
                    }),
                },
            ],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |c| {
                let panel = match panel_from_arg(c, "panel") {
                    Ok(p) => p,
                    Err(phrase) => return Effect::none().with_status(phrase),
                };
                let raw = c.args.as_text("side").map(str::trim).unwrap_or_default();
                let Some(side) = DockSide::from_id(raw) else {
                    return Effect::none()
                        .with_status(format!("no such side: `{raw}` — try left, right, bottom"));
                };
                Effect {
                    layout: Some(LayoutCommand::Move { panel, side }),
                    ..Effect::none()
                }
            },
        },
        Action {
            id: "layout.stack_panel",
            label: "Stack panel onto another",
            hint: "Put the panel in the same tab group as another panel",
            group: "View",
            place: Place::TOP,
            shortcut: None,
            params: &[
                PARAM_PANEL,
                Param {
                    name: "onto",
                    kind: ParamKind::Enum(&[]),
                    description: "The panel whose tab group receives it",
                    list: false,
                    required: true,
                    domain: Some(Domain {
                        values: panel_choices,
                        row_per_value: false,
                    }),
                },
            ],
            view: ViewAffinity::Any,
            enabled: Need::Nothing.only(),
            execute: |c| {
                let panel = match panel_from_arg(c, "panel") {
                    Ok(p) => p,
                    Err(phrase) => return Effect::none().with_status(phrase),
                };
                let onto = match panel_from_arg(c, "onto") {
                    Ok(p) => p,
                    Err(phrase) => return Effect::none().with_status(phrase),
                };
                if panel == onto {
                    return Effect::none()
                        .with_status(format!("`{}` cannot be stacked onto itself", panel.id()));
                }
                Effect {
                    layout: Some(LayoutCommand::Stack { panel, onto }),
                    ..Effect::none()
                }
            },
        },
        profile_action(
            "layout.apply_profile",
            "Apply Desktop Profile",
            "Put the window, docks, Inspector split, font and zoom as the profile has them",
            |c| profile_effect(c, ProfileCommand::Apply),
        ),
        profile_action(
            "layout.save_profile",
            "Save Desktop Profile",
            "Keep the window, docks, Inspector split, font and zoom under a name",
            |c| profile_effect(c, ProfileCommand::Save),
        ),
        profile_action(
            "layout.delete_profile",
            "Delete Desktop Profile",
            "Forget a saved desktop profile",
            |c| profile_effect(c, ProfileCommand::Delete),
        ),
    ]
}

#[note("param-profile")]
const PARAM_PROFILE: Param = Param {
    name: "name",
    kind: ParamKind::String,
    description: "The profile's name; `Default` is the IDE as it ships",
    list: false,
    required: true,
    domain: None,
};

#[note("profile-action")]
fn profile_action(
    id: &'static str,
    label: &'static str,
    hint: &'static str,
    execute: fn(&Context) -> Effect,
) -> Action {
    Action {
        id,
        label,
        hint,
        group: "View",
        place: Place::TOP,
        shortcut: None,
        params: &[PARAM_PROFILE],
        view: ViewAffinity::Any,
        enabled: Need::Nothing.only(),
        execute,
    }
}

fn profile_effect(c: &Context, make: fn(String) -> ProfileCommand) -> Effect {
    match c
        .args
        .as_text("name")
        .map(str::trim)
        .filter(|n| !n.is_empty())
    {
        Some(name) => Effect {
            profile: Some(make(name.to_string())),
            ..Effect::none()
        },
        None => Effect::none().with_status("a desktop profile needs a `name`"),
    }
}

#[note("component-actions")]
fn component_actions() -> Vec<Action> {
    vec![
        Action {
            id: "component.add",
            label: "Add component",
            hint: "Create a component of a class inside a parent, at a given rectangle",
            // Group of its own, and not `Edit`: none of the four is drawn by
            // any menu (all require an argument), and putting them in `Edit` would make the
            // menu group scan have to explain why four items
            // of the group do not appear there.
            group: "Component",
            place: Place::TOP,
            // No chord, by the rule of ADR 0011: the key is BORROWED from a
            // gesture that already exists, and creating a component is not a keyboard gesture in
            // any IDE — it is palette drag-and-drop, and dragging has no key.
            shortcut: None,
            params: &[
                Param {
                    name: "class",
                    kind: ParamKind::String,
                    description: "component class, e.g. Button — the palette lists them",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "parent",
                    kind: ParamKind::ComponentId,
                    description: "id of the container that receives it — absent puts it on the form",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "x",
                    kind: ParamKind::Number,
                    description: "left, relative to the parent — absent means 0",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "y",
                    kind: ParamKind::Number,
                    description: "top, relative to the parent — absent means 0",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "w",
                    kind: ParamKind::Number,
                    description: "width — absent uses the default size of the class",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "h",
                    kind: ParamKind::Number,
                    description: "height — absent uses the default size of the class",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(class) = c.args.as_text("class").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("add component: `class` is required");
                };
                // The phrase is the SAME that the shell already said when dropping an
                // unknown class from the palette. User text does not change owner
                // along with the code that produces it.
                let Some(desc) = c.registry.get(class) else {
                    return Effect::none().with_status(format!("unknown class: {class}"));
                };
                // The parent must exist BEFORE the transaction, and not out of
                // distrust of `crawrad-doc`: without it there is no way to know at what
                // index the child enters, and guessing zero would put the component at the
                // beginning of the order of an arbitrary container.
                let parent = match c.args.as_text("parent").filter(|s| !s.is_empty()) {
                    Some(p) if ir.node(p).is_some() => p.to_string(),
                    Some(p) => {
                        return Effect::none().with_status(format!("no such component: `{p}`"));
                    }
                    None => ir.root.clone(),
                };

                // **DOES THE PARENT ACCEPT THIS CLASS?**
                //
                // Until 09/18/2026 nobody asked: the class existed, the parent
                // existed, and that was it. A `Button` entered inside an
                // `ActionList` and the document accepted it, because
                // `Document::add_child` only checks duplicate id and existing parent.
                //
                // With the collection declaring the class of the items
                // (`Filhos::NaColecao { item }`, ADR 0037), the question gets
                // an answer — and it is HERE that it must be asked, because this
                // is the single mutation gate (ADR 0005). A rule without
                // a consumer is a guard that does not measure.
                if let Some(parent_desc) = ir.node(&parent).and_then(|n| c.registry.get(&n.class))
                    && !parent_desc.accepts_child(class)
                {
                    let reason = match parent_desc.item_class() {
                        Some(item) => {
                            format!("`{}` only takes `{item}` inside it", parent_desc.class)
                        }
                        None => format!("`{}` takes no children", parent_desc.class),
                    };
                    return Effect::none()
                        .with_status(format!("cannot add `{class}` there: {reason}"));
                }

                // `f64` from the argument to `f32` of the document, and the conversion is
                // here because it is here that it is seen: on the other side of the
                // protocol JSON sends `10` both for integer and for
                // real, and the document measures in `f32`.
                let (dw, dh) = desc.default_size;
                let rect = Rect::new(
                    c.args.as_number("x").unwrap_or(0.0) as f32,
                    c.args.as_number("y").unwrap_or(0.0) as f32,
                    c.args.as_number("w").unwrap_or(dw as f64) as f32,
                    c.args.as_number("h").unwrap_or(dh as f64) as f32,
                );
                let id = c.new_id(class);
                let Some(mut node) = c.registry.instantiate(class, id.clone(), rect) else {
                    return Effect::none().with_status(format!("unknown class: {class}"));
                };
                // Born at the end of the tab order sequence, as in the classic RAD IDE. This
                // lived in the shell, and therefore a component created outside the
                // window would be born with `tab_order` zero — tied with the
                // first button of the form.
                if node.props.contains_key("tab_order") {
                    node.props.insert(
                        "tab_order".into(),
                        Value::Int(arrangement::next_tab_order(ir)),
                    );
                }
                let index = ir.node(&parent).map(|n| n.children.len()).unwrap_or(0);
                Effect::tx(
                    Transaction::new(format!("Add {class}"), c.author.clone()).with(
                        Command::AddNode {
                            parent,
                            node,
                            index,
                        },
                    ),
                )
                .with_selection(vec![id])
            },
        },
        Action {
            id: "component.set_property",
            label: "Set property",
            hint: "Change a property on one or more components, in a single undo step",
            group: "Component",
            place: Place::TOP,
            shortcut: None,
            params: &[
                // **Plural, and singular is a list of one.** The alternative was
                // `node` with an outer loop, and it costs what `crawrad-doc`
                // gives for free: batch = one transaction = one Ctrl+Z. The Inspector
                // with N selected edits all N in a single transaction, and making
                // that the PRIMITIVE is what gives the same undo to an agent without
                // a second action to maintain.
                Param {
                    name: "nodes",
                    kind: ParamKind::ComponentId,
                    description: "ids of the components to change — all in one undo step",
                    list: true,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "key",
                    kind: ParamKind::String,
                    description: "property name, e.g. text, enabled, active_page",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "value",
                    kind: ParamKind::Any,
                    description: "the new value, in the type the property declares",
                    list: false,
                    required: true,
                    domain: None,
                },
                // The label of the undo step. Exists because it is USER
                // TEXT: the bar shows `<label> (rev N)` on every change, and
                // there are gestures whose good phrasing cannot be derived from the node/key pair —
                // switching tabs is `Change page`, not
                // `PageControl1.active_page`. In addition, an agent gets to
                // name what it did, which is what makes Ctrl+Z readable
                // after a conversation.
                Param {
                    name: "label",
                    kind: ParamKind::String,
                    description: "what the undo step is called — absent derives it from the component",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let targets = c.args.as_texts("nodes");
                if targets.is_empty() {
                    return Effect::none().with_status("set property: `nodes` is required");
                }
                let Some(key) = c.args.as_text("key").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("set property: `key` is required");
                };
                // Absence of `value` does NOT mean "delete the property",
                // even though `Command::SetProp` accepts `None`. An agent
                // that forgot the argument would delete the value silently, and
                // a destructive gesture must be requested explicitly — the
                // day deleting is worth having, it enters as an explicit
                // argument, not as forgetfulness.
                let Some(value) = c.args.value("value") else {
                    return Effect::none().with_status("set property: `value` is required");
                };

                // A key starting with "binding." on ONE node whose descriptor has
                // a bindable prop is a data binding edit.
                if key.starts_with("binding.") {
                    if targets.len() != 1 {
                        return Effect::none().with_status(
                            "set property: binding edit applies to one component at a time",
                        );
                    }
                    let target = targets[0];
                    let Some(node) = ir.node(target) else {
                        return Effect::none()
                            .with_status(format!("no such component: `{target}`"));
                    };
                    let Some(desc) = c.registry.get(&node.class) else {
                        return Effect::none()
                            .with_status(format!("unknown component class: `{}`", node.class));
                    };
                    let Some(bindable_prop) = desc.props.iter().find(|p| p.bindable) else {
                        return Effect::none().with_status(format!(
                            "component '{}' has no bindable property",
                            node.class
                        ));
                    };
                    let current_binding = node.binding(bindable_prop.key);
                    let raw_str = match value {
                        Value::Ref(s) | Value::Str(s) | Value::Enum(s) => s.trim(),
                        _ => "",
                    };
                    let new_binding = match key {
                        "binding.dataset" => {
                            let provides_dataset = ir
                                .node(raw_str)
                                .and_then(|n| c.registry.get(&n.class))
                                .is_some_and(|d| d.provides.contains(&"DbDataset"));
                            if raw_str.is_empty() {
                                None
                            } else if !provides_dataset {
                                return Effect::none().with_status(format!(
                                    "`{raw_str}` is not a dataset: pick a component that provides DbDataset"
                                ));
                            } else {
                                let (field, mode, converter) = match current_binding {
                                    Some(b) => (b.field.clone(), b.mode, b.converter.clone()),
                                    None => (None, crawrad_ir::BindingMode::TwoWay, None),
                                };
                                Some(crawrad_ir::BindingDef {
                                    source: raw_str.to_string(),
                                    field,
                                    mode,
                                    converter,
                                })
                            }
                        }
                        "binding.field" => {
                            let Some(current) = current_binding else {
                                return Effect::none().with_status("pick a Dataset first");
                            };
                            let mut b = current.clone();
                            b.field = (!raw_str.is_empty()).then(|| raw_str.to_string());
                            Some(b)
                        }
                        "binding.mode" => {
                            let Some(current) = current_binding else {
                                return Effect::none().with_status("pick a Dataset first");
                            };
                            let mut b = current.clone();
                            b.mode = parse_binding_mode(raw_str);
                            Some(b)
                        }
                        _ => {
                            return Effect::none()
                                .with_status(format!("unknown binding property: `{key}`"));
                        }
                    };

                    if current_binding == new_binding.as_ref() {
                        return Effect::none();
                    }
                    let tx_label = format!("{}.{} (binding)", node.name(), bindable_prop.key);
                    return Effect::tx(Transaction::new(tx_label, c.author.clone()).with(
                        Command::SetBinding {
                            node: target.to_string(),
                            prop: bindable_prop.key.to_string(),
                            binding: new_binding,
                        },
                    ));
                }

                // **THE KEY MUST BELONG TO THE CLASS OF THE TARGET** — [ADR 0031,
                // Amendment 1].
                //
                // # Why here, and why only now
                //
                // This is the single mutation surface (ADR 0005) and it has the
                // catalogue in hand — `Context::registry`. It is the only point in the
                // system where the two things meet before the document
                // changes.
                //
                // Until 09/17/2026 what refused a key was a global list of
                // forty names in `crawrad-ir`, and it did not know WHOSE
                // key it was: `component.set_property` with `interval` on a `Button`
                // passed, because `interval` is a valid key — of `Timer`. The
                // Inspector never offered that, but this action is published for
                // agents, and an agent asks by name.
                //
                // A key with a target prefix (`makepad.x`) passes without the class
                // declaring it: the contract promises to preserve it, and what judges the
                // prefix is `Document::validate`.
                if !key.contains('.') {
                    for target in &targets {
                        let Some(node) = ir.node(target) else {
                            continue;
                        };
                        let Some(desc) = c.registry.get(&node.class) else {
                            continue;
                        };
                        if desc.prop(key).is_none() {
                            return Effect::none().with_status(format!(
                                "set property: the class '{}' has no property '{key}'",
                                node.class
                            ));
                        }
                    }
                }

                // The value is coerced to the TYPE the property declares
                // (an enum name, a font object, a component name for a
                // reference...), one table for every class; what cannot be
                // coerced is refused with the shape it expects
                let mut coerced_values = Vec::with_capacity(targets.len());
                let target_prefixed = matches!(
                    crawrad_ir::PropKey::parse(key),
                    Ok(crawrad_ir::PropKey::Target { .. })
                );
                for target in &targets {
                    let prop = ir
                        .node(target)
                        // a declared `makepad.*` key is typed too
                        .filter(|_| !key.contains('.') || target_prefixed)
                        .and_then(|node| c.registry.get(&node.class))
                        .and_then(|desc| desc.prop(key));
                    let v = match prop {
                        Some(prop) => match coerce_to_property(value, prop, ir, c.registry) {
                            Ok(v) => v,
                            Err(msg) => {
                                return Effect::none().with_status(format!("set property: {msg}"));
                            }
                        },
                        None => value.clone(),
                    };
                    coerced_values.push((*target, v));
                }

                let coerced_first = coerced_values
                    .first()
                    .map(|(_, v)| v.clone())
                    .unwrap_or_else(|| value.clone());
                let value = &coerced_first;

                let first = ir.node(targets[0]);
                let label = match c.args.as_text("label").filter(|s| !s.is_empty()) {
                    Some(l) => l.to_string(),
                    // The two phrases that Inspector wrote by hand, word
                    // for word: with one selected the NAME, with several the
                    // class of the first and the count.
                    None if targets.len() > 1 => format!(
                        "{}.{key} ({} items)",
                        first.map(|n| n.class.as_str()).unwrap_or_default(),
                        targets.len()
                    ),
                    None => format!("{}.{key}", first.map(|n| n.name()).unwrap_or_default()),
                };

                let mut tx = Transaction::new(label, c.author.clone());
                for (target, v) in &coerced_values {
                    tx.push(Command::SetProp {
                        node: target.to_string(),
                        key: key.to_string(),
                        value: Some(v.clone()),
                    });
                }
                // A font or a colour of its own is no longer the
                // parent's, as in the classic RAD IDE: choosing one clears
                // `ParentFont`/`ParentColor` in the same step, or the form
                // would keep drawing the parent's and the choice would vanish
                let parent = match key {
                    "font" => Some("parent_font"),
                    "color" => Some("parent_color"),
                    _ => None,
                };
                for target in targets.iter().filter(|_| parent.is_some()) {
                    let class = ir.node(target).and_then(|n| c.registry.get(&n.class));
                    if let Some(parent) =
                        parent.filter(|p| class.is_some_and(|d| d.prop(p).is_some()))
                    {
                        tx.push(Command::SetProp {
                            node: target.to_string(),
                            key: parent.to_string(),
                            value: Some(Value::Bool(false)),
                        });
                    }
                }
                if key == "checked" || key == "state" {
                    // A CheckBox's Checked and State say the same thing
                    // twice, as in the classic RAD IDE; setting one sets the
                    // other in the same step, so the file never holds a
                    // checked box whose state says unchecked
                    let other = match (key, value) {
                        ("checked", Value::Bool(b)) => Some((
                            "state",
                            Value::Enum(if *b { "cbChecked" } else { "cbUnchecked" }.into()),
                        )),
                        ("state", Value::Enum(s)) => {
                            Some(("checked", Value::Bool(s == "cbChecked")))
                        }
                        _ => None,
                    };
                    if let Some((other_key, other_value)) = other {
                        for target in &targets {
                            if ir.node(target).is_some_and(|n| n.class == "CheckBox") {
                                tx.push(Command::SetProp {
                                    node: target.to_string(),
                                    key: other_key.to_string(),
                                    value: Some(other_value.clone()),
                                });
                            }
                        }
                    }
                }
                if key == "kind" {
                    // A ScrollBar's Kind IS its orientation, which the
                    // canvas and both targets read from the box's shape — so,
                    // as in the classic RAD IDE, turning it swaps the width
                    // and the height, and the two can never disagree
                    let kind = match value {
                        Value::Enum(k) => k.as_str(),
                        _ => "",
                    };
                    for target in &targets {
                        let Some(n) = ir.node(target).filter(|n| n.class == "ScrollBar") else {
                            continue;
                        };
                        if let Some(r) = n
                            .rect()
                            .and_then(|r| crawrad_ir::scrolling::scroll_bar_box_for_kind(r, kind))
                        {
                            tx.push(Command::SetProp {
                                node: target.to_string(),
                                key: "rect".to_string(),
                                value: Some(Value::Rect(r)),
                            });
                        }
                    }
                }
                if key == "align" || key == "margins" || key == "align_with_margins" {
                    let mut doc_temp = crawrad_doc::Doc::new(ir.clone());
                    if doc_temp.apply(tx.clone()).is_ok()
                        && let Some(layout_tx) = crawrad_designer::layout::apply(doc_temp.ir())
                    {
                        for cmd in layout_tx.commands {
                            tx.push(cmd);
                        }
                    }
                } else if key == "rect"
                    && targets.contains(&ir.root.as_str())
                    && let Value::Rect(r_new) = value
                    && let Some(r_old) = ir.node(&ir.root).and_then(|n| n.rect())
                {
                    let (dw, dh) = (r_new.w - r_old.w, r_new.h - r_old.h);
                    let mut doc_temp = crawrad_doc::Doc::new(ir.clone());
                    if doc_temp.apply(tx.clone()).is_ok() {
                        if let Some(anchors_tx) =
                            crawrad_designer::layout::apply_anchors(doc_temp.ir(), dw, dh)
                        {
                            let _ = doc_temp.apply(anchors_tx.clone());
                            for cmd in anchors_tx.commands {
                                tx.push(cmd);
                            }
                        }
                        if let Some(layout_tx) = crawrad_designer::layout::apply(doc_temp.ir()) {
                            for cmd in layout_tx.commands {
                                tx.push(cmd);
                            }
                        }
                    }
                }
                Effect::tx(tx)
            },
        },
        Action {
            id: "component.add_item",
            label: "Add item",
            hint: "Create an item inside a collection, of the class it declares",
            group: "Component",
            place: Place::TOP,
            // No chord: the gesture is the structure tree button, and the key
            // is borrowed from a gesture that already exists (ADR 0011).
            shortcut: None,
            params: &[Param {
                name: "parent",
                kind: ParamKind::ComponentId,
                description: "id of the collection that receives the item",
                list: false,
                required: true,
                domain: None,
            }],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            // **The missing gesture, and without it the collection is an empty declaration.**
            //
            // `Action` has `palette: ""` on purpose — *"an action is born inside
            // a list, not loose on the form"* —, and the only path that
            // created a node was dropping from the palette. Measured outcome on 09/18/2026:
            // **there was no way for the user to create an `Action`.** The class
            // existed, the collection existed, and nobody could put an item inside.
            //
            // It does not know the name of any class: it asks the descriptor of the
            // parent what the class of its items is. A third-party grid that
            // declares `item_class = "Column"` in `manifest.json` earns this
            // gesture without a single line written for it — which is all of ADR 0037.
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(parent) = c.args.as_text("parent").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("add item: `parent` is required");
                };
                let Some(node) = ir.node(parent) else {
                    return Effect::none().with_status(format!("no such component: `{parent}`"));
                };
                let Some(desc) = c.registry.get(&node.class) else {
                    return Effect::none().with_status(format!("unknown class: {}", node.class));
                };
                let Some(item) = desc.item_class() else {
                    return Effect::none()
                        .with_status(format!("`{}` holds no collection", node.class));
                };
                let id = c.new_id(item);
                let (w, h) = c
                    .registry
                    .get(item)
                    .map(|d| d.default_size)
                    .unwrap_or((28.0, 28.0));
                let Some(new_node) =
                    c.registry
                        .instantiate(item, id.clone(), Rect::new(0.0, 0.0, w, h))
                else {
                    return Effect::none().with_status(format!("unknown class: {item}"));
                };
                Effect::tx(
                    Transaction::new(format!("Add {item}"), c.author.clone()).with(
                        Command::AddNode {
                            parent: parent.to_string(),
                            node: new_node,
                            // At the end of the collection: new item enters after
                            // what is already there, which is where the eye
                            // looks for it.
                            index: node.children.len(),
                        },
                    ),
                )
                .with_selection(vec![id])
            },
        },
        Action {
            id: "component.reorder",
            label: "Reorder",
            hint: "Move a component up or down among its siblings",
            group: "Component",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "id of the component to move",
                    list: false,
                    required: true,
                    domain: None,
                },
                // A STEP, and not a destination index: whoever calls is a
                // `Move Up` button, and calculating "where it is today" is the same calculation that
                // this action already has to do to refuse the first and the last.
                // An absolute index would force the caller to recalculate it, and the
                // two calculations would disagree the day the list changed between
                // one and the other.
                Param {
                    name: "step",
                    kind: ParamKind::Integer,
                    description: "-1 moves it up, 1 moves it down",
                    list: false,
                    required: true,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(target) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("reorder: `node` is required");
                };
                // `as_number` because it is the accessor that `Args` has, and the
                // argument arrives as `Value::Int` in any case — the
                // truncation here is what turns a `1.0` coming from the
                // protocol into the integer step that `Reorder` asks for.
                let Some(step) = c
                    .args
                    .as_number("step")
                    .map(|p| p as i64)
                    .filter(|p| *p != 0)
                else {
                    return Effect::none().with_status("reorder: `step` must be -1 or 1");
                };
                let Some(parent) = ir.parent_of(target).cloned() else {
                    return Effect::none()
                        .with_status(format!("`{target}` has no parent to be reordered in"));
                };
                let siblings = ir
                    .node(&parent)
                    .map(|n| n.children.clone())
                    .unwrap_or_default();
                let Some(from) = siblings.iter().position(|c| c == target) else {
                    return Effect::none();
                };
                let to = from as i64 + step;
                // THE BOUNDARY IS NOT AN ERROR. The first in the list with `Move Up` is the
                // gesture that does nothing, and that is how the classic RAD IDE behaves:
                // the button is dimmed, and whoever calls anyway — an agent,
                // over the protocol — receives silence instead of a transaction that
                // reorders to the same place.
                if to < 0 || to as usize >= siblings.len() {
                    return Effect::none();
                }
                let label = if step < 0 { "Move up" } else { "Move down" };
                Effect::tx(Transaction::new(label.to_string(), c.author.clone()).with(
                    Command::Reorder {
                        parent,
                        from,
                        to: to as usize,
                    },
                ))
            },
        },
        Action {
            id: "component.rename",
            label: "Rename",
            hint: "Change the Name of a component — it is unique in the form",
            group: "Component",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "id of the component to rename",
                    list: false,
                    required: true,
                    domain: None,
                },
                // Singular, unlike `component.set_property`: `Name`
                // is unique in the form since ADR 0008, so renaming N nodes
                // to the same name is a request that does not exist.
                Param {
                    name: "name",
                    kind: ParamKind::String,
                    description: "the new Name — unique in the form",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "everywhere",
                    kind: ParamKind::Boolean,
                    description: "true (default): also rename the event handlers that carry the component's default name; false: only the component",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(target) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("rename: `node` is required");
                };
                let Some(node) = ir.node(target) else {
                    return Effect::none().with_status(format!("no such component: `{target}`"));
                };
                // A missing ARGUMENT is not the same as an invalid name: the former
                // is the caller that said nothing, and the latter is
                // `Command::Rename` refusing with its phrase — empty name,
                // name with neither letter nor digit, repeated name. Repeating the second
                // check here would give two messages for the same defect, and
                // one of them would age silently.
                let Some(new_name) = c.args.as_text("name").map(str::trim) else {
                    return Effect::none().with_status("rename: `name` is required");
                };
                // Renaming to what it already is is not a change: a transaction here
                // would stack a Ctrl+Z that undoes nothing visible.
                let prev_name = node.name().to_string();
                if new_name == prev_name {
                    return Effect::none();
                }
                let handlers = c.args.as_bool("everywhere").unwrap_or(true);
                Effect::tx(
                    Transaction::new(
                        format!("Rename {prev_name} to {new_name}"),
                        c.author.clone(),
                    )
                    .with(Command::Rename {
                        node: target.to_string(),
                        name: new_name.to_string(),
                        handlers,
                    }),
                )
            },
        },
        Action {
            // `event.`, and not `component.`: the id is the surface of the MCP
            // tool, and the day there is an `event.list` or an
            // `event.unbind_all` they will live alongside this one. The GROUP continues
            // to be `Component` because group is where the interface would draw it, and
            // the interface draws event in the same panel as property.
            id: "event.bind",
            label: "Bind event",
            hint: "Point an event of a component at a handler function — empty unbinds it",
            group: "Component",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "id of the component that owns the event",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "event",
                    kind: ParamKind::String,
                    description: "event name as the catalogue declares it, e.g. on_click",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "handler",
                    kind: ParamKind::String,
                    description: "name of the handler function — absent or empty unbinds the event",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(target) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("bind event: `node` is required");
                };
                let Some(node) = ir.node(target) else {
                    return Effect::none().with_status(format!("no such component: `{target}`"));
                };
                let Some(event) = c.args.as_text("event").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("bind event: `event` is required");
                };
                // The REGISTRY says what events the class has, and the answer is
                // free: `ComponentDesc` is already in context and the list has a
                // handful of entries.
                //
                // THIS IS NOT A GUARD, and the difference matters.
                // `Document::validate` **does not look at `events`**: an invented
                // event entering by another path still passes and
                // still earns a stub in the generated code. The real gate is
                // in `crawrad-doc`, which is the single write gate — what is
                // gained here is the caller receiving `` `on_clik` is not an event
                // of Button `` instead of a handler that never fires. Closing the
                // hole belongs to the events module, not to this turn.
                let known = c
                    .registry
                    .get(&node.class)
                    .is_some_and(|d| d.events.iter().any(|e| e.name == event));
                if !known {
                    return Effect::none()
                        .with_status(format!("`{event}` is not an event of {}", node.class));
                }

                // Absent and empty say the SAME thing, and that is what the Inspector
                // field has always done: clearing the handler text unbinds the
                // event. That is why this is the only optional of the three — the
                // gesture of unbinding has no argument to give.
                let handler = c
                    .args
                    .as_text("handler")
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                let current = node.events.get(event).cloned().unwrap_or_default();
                if handler == current {
                    return Effect::none();
                }
                Effect::tx(
                    Transaction::new(format!("{}.{event}", node.name()), c.author.clone()).with(
                        Command::SetEvent {
                            node: target.to_string(),
                            event: event.to_string(),
                            handler: (!handler.is_empty()).then_some(handler),
                        },
                    ),
                )
            },
        },
        inherited_event_action(
            "event.override",
            "Override inherited event",
            "Give an inherited component's event a handler of this form that calls the base's first",
            |c| {
                let Some(e) = inherited_event_of(c) else {
                    return Effect::none().with_status(NOT_INHERITED_EVENT);
                };
                let name = inspector_model::override_handler(&e.base);
                let mut effect = Effect::none();
                if e.current != name {
                    effect = Effect::tx(
                        Transaction::new(format!("override {}", e.event), c.author.clone()).with(
                            Command::SetEvent {
                                node: e.node,
                                event: e.event,
                                handler: Some(name.clone()),
                            },
                        ),
                    );
                }
                effect.handler = Some(HandlerJump {
                    name,
                    inherited: false,
                });
                effect
            },
        ),
        inherited_event_action(
            "event.go_to_handler",
            "Go to handler",
            "Open this form's handler of the event in the code",
            |c| {
                let Some(e) = inherited_event_of(c).filter(|e| !e.current.is_empty()) else {
                    return Effect::none().with_status(NOT_INHERITED_EVENT);
                };
                Effect {
                    handler: Some(HandlerJump {
                        name: e.current,
                        inherited: false,
                    }),
                    ..Effect::none()
                }
            },
        ),
        inherited_event_action(
            "event.go_to_inherited",
            "Go to inherited handler",
            "Open the base form's handler of the event in the code",
            |c| {
                let Some(e) = inherited_event_of(c) else {
                    return Effect::none().with_status(NOT_INHERITED_EVENT);
                };
                Effect {
                    handler: Some(HandlerJump {
                        name: e.base,
                        inherited: true,
                    }),
                    ..Effect::none()
                }
            },
        ),
        inherited_event_action(
            "event.revert_inherited",
            "Revert to inherited",
            "Point the event back at the base's handler; the method you wrote stays in the code",
            |c| {
                let Some(e) = inherited_event_of(c).filter(|e| e.current != e.base) else {
                    return Effect::none().with_status(NOT_INHERITED_EVENT);
                };
                Effect::tx(
                    Transaction::new(format!("revert {}", e.event), c.author.clone()).with(
                        Command::SetEvent {
                            node: e.node,
                            event: e.event,
                            handler: Some(e.base),
                        },
                    ),
                )
            },
        ),
        Action {
            id: "event.rename",
            label: "Rename event handler",
            hint: "Give the handler of an event a new name, keeping its code — every event bound to it follows",
            group: "Component",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "id of the component that owns the event",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "event",
                    kind: ParamKind::String,
                    description: "event name as the catalogue declares it, e.g. on_click",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "name",
                    kind: ParamKind::String,
                    description: "the new handler name — a Rust identifier not used by another handler of the form",
                    list: false,
                    required: true,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(target) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("rename handler: `node` is required");
                };
                let Some(node) = ir.node(target) else {
                    return Effect::none().with_status(format!("no such component: `{target}`"));
                };
                let Some(event) = c.args.as_text("event").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("rename handler: `event` is required");
                };
                let Some(name) = c.args.as_text("name") else {
                    return Effect::none().with_status("rename handler: `name` is required");
                };
                let Some(old) = node.events.get(event).cloned() else {
                    return Effect::none().with_status(format!(
                        "`{}.{event}` has no handler to rename",
                        node.name()
                    ));
                };
                let new = name.trim();
                if new == old {
                    return Effect::none();
                }
                if !crawrad_ir::is_handler_ident(new) {
                    return Effect::none()
                        .with_status(format!("`{new}` is not a valid handler name"));
                }
                if handler_in_use(ir, new) {
                    return Effect::none()
                        .with_status(format!("`{new}` is already a handler of this form"));
                }
                let mut tx =
                    Transaction::new(format!("Rename handler {old} to {new}"), c.author.clone());
                for (node_id, n) in &ir.nodes {
                    for (ev, h) in &n.events {
                        if h == &old {
                            tx.push(Command::SetEvent {
                                node: node_id.clone(),
                                event: ev.clone(),
                                handler: Some(new.to_string()),
                            });
                        }
                    }
                }
                Effect::tx(tx)
            },
        },
        Action {
            id: "binding.set",
            label: "Bind property",
            hint: "Bind a component property to a data source — empty source unbinds it",
            group: "Component",
            place: Place::TOP,
            shortcut: None,
            params: &[
                Param {
                    name: "node",
                    kind: ParamKind::ComponentId,
                    description: "id of the component that owns the property",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "prop",
                    kind: ParamKind::String,
                    description: "property name to bind, e.g. text, checked, value",
                    list: false,
                    required: true,
                    domain: None,
                },
                Param {
                    name: "source",
                    kind: ParamKind::String,
                    description: "data source path (e.g. ds_clients.NAME) — absent or empty unbinds it",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "field",
                    kind: ParamKind::String,
                    description: "optional field or column name if not embedded in source",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "mode",
                    kind: ParamKind::Enum(&["TwoWay", "OneWay", "OneWayToSource", "OneTime"]),
                    description: "binding mode: TwoWay (default), OneWay, OneWayToSource, OneTime",
                    list: false,
                    required: false,
                    domain: None,
                },
                Param {
                    name: "converter",
                    kind: ParamKind::String,
                    description: "optional converter name",
                    list: false,
                    required: false,
                    domain: None,
                },
            ],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| {
                let Some(ir) = c.ir() else {
                    return Effect::none();
                };
                let Some(target) = c.args.as_text("node").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("bind property: `node` is required");
                };
                let Some(node) = ir.node(target) else {
                    return Effect::none().with_status(format!("no such component: `{target}`"));
                };
                let Some(prop) = c.args.as_text("prop").filter(|s| !s.is_empty()) else {
                    return Effect::none().with_status("bind property: `prop` is required");
                };
                let known = c
                    .registry
                    .get(&node.class)
                    .is_some_and(|d| d.props.iter().any(|p| p.key == prop));
                if !known {
                    return Effect::none()
                        .with_status(format!("`{prop}` is not a property of {}", node.class));
                }
                let is_bindable = c
                    .registry
                    .get(&node.class)
                    .and_then(|d| d.prop(prop))
                    .is_some_and(|p| p.bindable);
                let source_raw = c
                    .args
                    .as_text("source")
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                let binding = if source_raw.is_empty() {
                    None
                } else {
                    let (source, field) = if let Some(explicit_field) = c.args.as_text("field") {
                        (source_raw, Some(explicit_field.trim().to_string()))
                    } else if let Some((src, fld)) = source_raw.split_once('.') {
                        (src.trim().to_string(), Some(fld.trim().to_string()))
                    } else {
                        (source_raw, None)
                    };

                    let mode_str = c.args.as_text("mode").unwrap_or("TwoWay");
                    let mode = parse_binding_mode(mode_str);
                    let converter = c
                        .args
                        .as_text("converter")
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.trim().to_string());
                    Some(crawrad_ir::binding::BindingDef {
                        source,
                        field,
                        mode,
                        converter,
                    })
                };
                if node.binding(prop) == binding.as_ref() {
                    return Effect::none();
                }
                let mut effect = Effect::tx(
                    Transaction::new(
                        format!("{}.{prop} (binding)", node.name()),
                        c.author.clone(),
                    )
                    .with(Command::SetBinding {
                        node: target.to_string(),
                        prop: prop.to_string(),
                        binding: binding.clone(),
                    }),
                );
                if binding.is_some() && !is_bindable {
                    effect = effect
                        .with_status("data binding is not supported by the generated app yet");
                }
                effect
            },
        },
    ]
}

#[note("create-standard-actions")]
pub fn create_standard_actions(c: &Context, names: &[&str]) -> Effect {
    let Some(list) = action_list_node(c) else {
        return Effect::none();
    };
    if names.is_empty() {
        return Effect::none();
    }
    let templates = crawrad_registry::standard_actions();
    let label = if names.len() == 1 {
        "New action".to_string()
    } else {
        format!("Add {} actions", names.len())
    };
    let mut tx = Transaction::new(label, c.author.clone());
    let mut created = Vec::new();
    let mut offset = c
        .ir()
        .and_then(|ir| ir.node(&list))
        .map(|l| l.children.len())
        .unwrap_or(0);

    for name in names {
        let tpl = templates.iter().find(|t| &t.name == name);
        // The name comes from TEMPLATE and not a generic `Action5` — it is what
        // classic RAD IDE does, and what makes the list readable: `edit_copy1` says what the
        // action is, `action5` says nothing.
        //
        // **In snake_case**, and that was the defect: `suggest_id_base`
        // receives the RAW base, and the template name is `EditCopy`. The result on
        // screen was `EditCut1` in the middle of `edit_copy1`, `edit_cut1` — the owner noticed
        // first, 17/09/2026: *"the IDE is creating `Edit` instead of `edit`"*.
        //
        // `suggest_id` (by CLASS) had already done this all along
        // (`crawrad-registry/src/lib.rs:621-625`); this path did not go through
        // it. The house rule is the memory `classic RAD IDE is ref for FEATURE, not
        // syntax`: WHAT the IDE creates is from classic RAD IDE, HOW it is written is from Rust.
        let base = tpl.map(|t| t.base_name).unwrap_or("action");
        let id = c.registry.suggest_id_base(base, |i| {
            c.ir()
                .is_some_and(|ir| ir.node(i).is_some() || ir.node_by_name(i).is_some())
                || created.contains(&i.to_string())
        });
        let Some(mut n) = c.registry.instantiate(
            "Action",
            id.clone(),
            Rect::new(
                0.0,
                0.0,
                crawrad_registry::ICON_SIZE.0,
                crawrad_registry::ICON_SIZE.1,
            ),
        ) else {
            continue;
        };

        if let Some(t) = tpl {
            n.props
                .insert("text".into(), crawrad_ir::Value::Str(t.text.into()));
            n.props
                .insert("hint".into(), crawrad_ir::Value::Str(t.hint.into()));
            if !t.shortcut.is_empty() {
                n.props
                    .insert("shortcut".into(), crawrad_ir::Value::Str(t.shortcut.into()));
            }
            n.props
                .insert("image_index".into(), crawrad_ir::Value::Int(t.image_index));
            // "(No Category)" is presentation label, not value: in document
            // it is the empty string, like the empty Category of an action in the classic RAD IDE.
            let cat = if t.category == "(No Category)" {
                ""
            } else {
                t.category
            };
            n.props
                .insert("category".into(), crawrad_ir::Value::Str(cat.into()));
            n.props.insert(
                "dataset_operation".into(),
                crawrad_ir::Value::Enum(t.dataset_operation.into()),
            );
        } else {
            // Generic base action
            n.props
                .insert("text".into(), crawrad_ir::Value::Str(id.clone()));
        }

        tx.push(Command::AddNode {
            parent: list.clone(),
            node: n,
            index: offset,
        });
        created.push(id);
        offset += 1;
    }

    if created.is_empty() {
        return Effect::none();
    }

    Effect::tx(tx).with_selection(created)
}

fn alignment_actions() -> Vec<Action> {
    macro_rules! align {
        ($id:literal, $label:literal, $how:expr) => {
            Action {
                id: $id,
                label: $label,
                hint: $label,
                group: "Align",
                place: Place::TOP,
                shortcut: None,
                params: &[],
                view: ViewAffinity::Design,
                enabled: Need::Selection.and(two_or_more),
                execute: |c| match c
                    .ir()
                    .and_then(|ir| arrangement::align(ir, &c.sel(), $how, Reference::First))
                {
                    Some(tx) => Effect::tx(tx),
                    None => Effect::none(),
                },
            }
        };
    }

    vec![
        align!("align.left", "Align left", AlignMode::Left),
        align!(
            "align.centers",
            "Align centers",
            AlignMode::CenterHorizontal
        ),
        align!("align.right", "Align right", AlignMode::Right),
        align!("align.top", "Align top", AlignMode::Top),
        align!("align.middles", "Align middles", AlignMode::CenterVertical),
        align!("align.bottom", "Align bottom", AlignMode::Bottom),
        Action {
            id: "align.space_horizontally",
            label: "Space equally, horizontally",
            hint: "Same gap between components, horizontally",
            group: "Align",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.and(three_or_more),
            execute: |c| match c
                .ir()
                .and_then(|ir| arrangement::distribute(ir, &c.sel(), Distribute::Horizontal))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "align.space_vertically",
            label: "Space equally, vertically",
            hint: "Same gap between components, vertically",
            group: "Align",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.and(three_or_more),
            execute: |c| match c
                .ir()
                .and_then(|ir| arrangement::distribute(ir, &c.sel(), Distribute::Vertical))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "align.same_width",
            label: "Same width",
            hint: "Match the width of the first selected",
            group: "Size",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.and(two_or_more),
            execute: |c| match c
                .ir()
                .and_then(|ir| arrangement::equalize(ir, &c.sel(), Equalize::Width))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "align.same_height",
            label: "Same height",
            hint: "Match the height of the first selected",
            group: "Size",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.and(two_or_more),
            execute: |c| match c
                .ir()
                .and_then(|ir| arrangement::equalize(ir, &c.sel(), Equalize::Height))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "align.same_size",
            label: "Same size",
            hint: "Match the size of the first selected",
            group: "Size",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.and(two_or_more),
            execute: |c| match c
                .ir()
                .and_then(|ir| arrangement::equalize(ir, &c.sel(), Equalize::Both))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "align.center_in_form_h",
            label: "Center horizontally in form",
            hint: "Center the selection in the form, horizontally",
            group: "In form",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| match c.ir().and_then(|ir| {
                arrangement::center_in_form(ir, &c.sel(), AlignMode::CenterHorizontal)
            }) {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "align.center_in_form_v",
            label: "Center vertically in form",
            hint: "Center the selection in the form, vertically",
            group: "In form",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| match c
                .ir()
                .and_then(|ir| arrangement::center_in_form(ir, &c.sel(), AlignMode::CenterVertical))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
    ]
}

fn order_actions() -> Vec<Action> {
    macro_rules! order {
        ($id:literal, $label:literal, $how:expr) => {
            Action {
                id: $id,
                label: $label,
                hint: $label,
                group: "Order",
                place: Place::TOP,
                shortcut: None,
                params: &[],
                view: ViewAffinity::Design,
                enabled: Need::Selection.only(),
                execute: |c| match c
                    .ir()
                    .and_then(|ir| arrangement::z_order(ir, &c.sel(), $how))
                {
                    Some(tx) => Effect::tx(tx),
                    None => Effect::none(),
                },
            }
        };
    }

    vec![
        order!("order.bring_to_front", "Bring to front", ZOrder::Front),
        order!("order.bring_forward", "Bring forward", ZOrder::Forward),
        order!("order.send_backward", "Send backward", ZOrder::Backward),
        order!("order.send_to_back", "Send to back", ZOrder::Back),
        Action {
            id: "order.renumber_tab_order",
            label: "Renumber tab order",
            hint: "Renumber the tab sequence from 0, removing gaps",
            group: "Order",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Form.only(),
            execute: |c| match c.ir().and_then(arrangement::normalize_tab_order) {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "order.tab_earlier",
            label: "Earlier in tab order",
            hint: "Move the selected component earlier in the tab sequence",
            group: "Order",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| match c
                .sel()
                .first()
                .zip(c.ir())
                .and_then(|(id, ir)| arrangement::move_tab_order(ir, id, true))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
        Action {
            id: "order.tab_later",
            label: "Later in tab order",
            hint: "Move the selected component later in the tab sequence",
            group: "Order",
            place: Place::TOP,
            shortcut: None,
            params: &[],
            view: ViewAffinity::Design,
            enabled: Need::Selection.only(),
            execute: |c| match c
                .sel()
                .first()
                .zip(c.ir())
                .and_then(|(id, ir)| arrangement::move_tab_order(ir, id, false))
            {
                Some(tx) => Effect::tx(tx),
                None => Effect::none(),
            },
        },
    ]
}

#[note("preferred-shortcuts")]
pub const PREFERRED_SHORTCUTS: &[(&str, &[&str])] = &[
    // THEY POINT TO INTERFACE ZOOM, not canvas zoom.
    //
    // It was the error the owner corrected: *"WRONG, zoom is not only in the
    // designer area, it is in ALL OF THE IDE, WITHOUT EXCEPTION"*. `Ctrl +` always meant
    // *enlarge the IDE* — and the egui shell always did that, through the host.
    // The canvas has its own magnifying glass, with its own key.
    (
        "view.ui_zoom_in",
        &["Ctrl+=", "Ctrl+Shift+=", "Ctrl+NumpadAdd"],
    ),
    ("view.ui_zoom_out", &["Ctrl+-", "Ctrl+NumpadSubtract"]),
    // `Ctrl+0` is the *reset* of browsers and modern editors.
    ("view.ui_zoom_reset", &["Ctrl+0", "Ctrl+Numpad0"]),
];

#[note("preferred-shortcut")]
pub fn preferred_shortcut(id: &str) -> Option<&'static str> {
    preferred_shortcuts(id).first().copied()
}

#[note("preferred-shortcuts-2")]
pub fn preferred_shortcuts(id: &str) -> &'static [&'static str] {
    PREFERRED_SHORTCUTS
        .iter()
        .find(|(a, _)| *a == id)
        .map(|(_, k)| *k)
        .unwrap_or(&[])
}

#[note("by-id")]
pub fn by_id(id: &str) -> Option<Action> {
    by_id_with_scheme(id, KeymapScheme::Modern)
}

#[note("by-id-with-scheme")]
pub fn by_id_with_scheme(id: &str, scheme: KeymapScheme) -> Option<Action> {
    catalogue_with_scheme(scheme)
        .into_iter()
        .find(|a| a.id == id)
}

#[note("by-group")]
pub fn by_group(group: &str) -> Vec<Action> {
    by_group_with_scheme(group, KeymapScheme::Modern)
}

#[note("by-group-with-scheme")]
pub fn by_group_with_scheme(group: &str, scheme: KeymapScheme) -> Vec<Action> {
    catalogue_with_scheme(scheme)
        .into_iter()
        .filter(|a| a.group == group)
        .collect()
}

#[note("args-from-json")]
pub fn args_from_json(args: &serde_json::Value) -> Args {
    let mut map = std::collections::BTreeMap::new();
    if let Some(obj) = args.as_object() {
        for (k, v) in obj {
            let crawrad_val = value_from_json(v);
            map.insert(k.clone(), crawrad_val);
        }
    }
    Args::from_map(map)
}

#[note("value-from-json")]
pub fn value_from_json(v: &serde_json::Value) -> crawrad_ir::Value {
    match v {
        serde_json::Value::Bool(b) => crawrad_ir::Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                crawrad_ir::Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                crawrad_ir::Value::Float(f)
            } else {
                crawrad_ir::Value::Str(n.to_string())
            }
        }
        serde_json::Value::String(s) => crawrad_ir::Value::Str(s.clone()),
        serde_json::Value::Array(arr) => {
            let list = arr.iter().map(value_from_json).collect();
            crawrad_ir::Value::List(list)
        }
        serde_json::Value::Object(map) => {
            let mut obj_map = std::collections::BTreeMap::new();
            for (k, val) in map {
                obj_map.insert(k.clone(), value_from_json(val));
            }
            crawrad_ir::Value::Map(obj_map)
        }
        serde_json::Value::Null => crawrad_ir::Value::Null,
    }
}

#[cfg(test)]
mod effect_coverage_gate;

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "test/capability_matrix/test.rs"]
mod capability_matrix_test;

#[cfg(test)]
#[path = "test/collection/test.rs"]
mod collection_test;

#[cfg(test)]
#[path = "test/coerce/test.rs"]
mod coerce_test;

#[cfg(test)]
#[path = "test/style_rule_check/test.rs"]
mod style_rule_check_test;

#[cfg(test)]
#[path = "test/editor_text/test.rs"]
mod editor_text_test;

#[cfg(test)]
#[path = "test/key_routing/test.rs"]
mod key_routing_test;
