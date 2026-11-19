//! **The four-verb `Bind`** — how this target asks the framework whether an
//! event happened.
//!
//! It is the half that touches the **irreducible floor** of
//! ADR 0036:
//! in Rust there is no lookup of a method by its name at runtime, as a
//! classic RAD IDE has, so the handler name must become a dispatch
//! table at generation time. What can be done is to keep the table small
//! and declared, rather than scattered across sixteen functions.
//!
//! # The vocabulary is closed, and was MEASURED not chosen
//!
//! Scanning the sixteen handwritten `emit_actions`, what varies among them
//! comes down to three things:
//!
//! | | count | which |
//! |---|---|---|
//! | widget accessor | **6** | `button`, `check_box`, `radio_button`, `drop_down`, `text_input`, `view` |
//! | query | **4** | `clicked`, `changed`, `selected`, `finger_up` |
//! | shape | **3** | see [`Shape`] |
//!
//! Comparison: `Brush`, the closed vocabulary already running in
//! production here for DRAWING, has twelve verbs. This has fewer.
//!
//! # What the measurement found wrong, in BOTH directions
//!
//! - **Event declared and not wired.** `Button` declares **twelve** events in the
//!   catalog and the generator wires **one**. The other eleven appear in the Object
//!   Inspector, the user writes the handler, and the application never calls it.
//! - **Event wired and not declared.** `Panel` and `GroupBox` wire
//!   `on_click`, and their descriptors declare **zero** events — the
//!   Inspector never offers this event, so that code never runs. It is
//!   dead code disguised as a feature.
//!
//! Both debts are recorded in the gate, which enforces the exact set.

use crawrad_note::note;
#[note("targets-makepad-binding-shape")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `if X.clicked(actions) { … }` — the query already returns `bool`.
    Plain,
    /// `if let Some(_val) = X.changed(actions) { … }` — the query returns the
    /// NEW value, and the target writes it back to state before calling the handler.
    Value,
    /// `if X.finger_up(actions).is_some() { … }` — returns `Option` of something
    /// not needed.
    Option,
    /// `if let <query> = actions.find_widget_action_cast(X.widget_uid()) { … }`
    ///
    /// **The fourth shape, and it has no `.query(actions)`.** It is what the
    /// framework's `RadioButton` requires: the radio group does not respond with
    /// `clicked`; it publishes a `RadioButtonAction`. Here [`Binding::query`]
    /// ceases to be a method and becomes the **action type**.
    ///
    /// It was added because the measurement found it, not because I predicted four:
    /// the first three came from the scan and this one appeared when the fidelity
    /// gate failed. That is the gate doing its job.
    ActionCast,
}

#[note("targets-makepad-binding-binding")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binding {
    /// The accessor on `WidgetRef`: `self.ui.<accessor>(id!(campo))`.
    pub accessor: &'static str,
    /// Where under the control's field the widget that reports the event
    /// lives: `""` is the field itself, `"_box"` is `<field>_box` (a
    /// CheckBox is a row, and the box in it reports the click).
    pub part: &'static str,
    /// The query: `.<query>(actions)`.
    pub query: &'static str,
    pub shape: Shape,
    /// What the target writes back to state BEFORE calling the user handler.
    ///
    /// `None` when the event does not carry a value. When it does, writing back
    /// is required: a `CheckBox` whose `checked` does not follow the click is a
    /// component that lies to its own handler.
    pub writeback: Option<&'static str>,
}

#[note("targets-makepad-binding-text-box-changed")]
const TEXT_BOX_CHANGED: Binding = Binding {
    accessor: "text_input",
    part: "",
    query: "changed",
    shape: Shape::Value,
    writeback: Some("text = _val"),
};

#[note("targets-makepad-binding-text-box-returned")]
const TEXT_BOX_RETURNED: Binding = Binding {
    accessor: "text_input",
    part: "",
    query: "returned",
    shape: Shape::Option,
    writeback: None,
};

#[note("targets-makepad-binding-bindings")]
pub const BINDINGS: &[(&str, &str, Binding)] = &[
    (
        "Button",
        "on_click",
        Binding {
            accessor: "button",
            part: "",
            query: "clicked",
            shape: Shape::Plain,
            writeback: None,
        },
    ),
    (
        "MenuItem",
        "on_click",
        Binding {
            accessor: "button",
            part: "",
            query: "clicked",
            shape: Shape::Plain,
            writeback: None,
        },
    ),
    (
        "Toggle",
        "on_click",
        Binding {
            accessor: "check_box",
            part: "",
            query: "changed",
            shape: Shape::Value,
            writeback: Some("checked = _val"),
        },
    ),
    (
        "Toggle",
        "on_change",
        Binding {
            accessor: "check_box",
            part: "",
            query: "changed",
            shape: Shape::Value,
            writeback: Some("checked = _val"),
        },
    ),
    (
        "BitBtn",
        "on_click",
        Binding {
            accessor: "button",
            part: "",
            query: "clicked",
            shape: Shape::Plain,
            writeback: None,
        },
    ),
    (
        "SpeedButton",
        "on_click",
        Binding {
            accessor: "button",
            part: "",
            query: "clicked",
            shape: Shape::Plain,
            writeback: None,
        },
    ),
    (
        "CheckBox",
        "on_click",
        Binding {
            accessor: "check_box",
            part: "_box",
            query: "changed",
            shape: Shape::Value,
            writeback: Some("checked = next == \"cbChecked\""),
        },
    ),
    (
        "RadioButton",
        "on_click",
        Binding {
            accessor: "radio_button",
            part: "",
            query: "RadioButtonAction::Clicked",
            shape: Shape::ActionCast,
            writeback: Some("checked = true"),
        },
    ),
    // one question for any row of the CrawradListBox (crawrad_runtime_makepad::list)
    (
        "ListBox",
        "on_change",
        Binding {
            accessor: "crawrad_list_box",
            part: "",
            query: "selected",
            shape: Shape::Value,
            writeback: Some("selected = Some(_val)"),
        },
    ),
    (
        "ListBox",
        "on_click",
        Binding {
            accessor: "crawrad_list_box",
            part: "",
            query: "selected",
            shape: Shape::Value,
            writeback: Some("selected = Some(_val)"),
        },
    ),
    // the TrackBar is a makepad Slider
    (
        "TrackBar",
        "on_change",
        Binding {
            accessor: "slider",
            part: "",
            query: "slided",
            shape: Shape::Value,
            writeback: Some("position = _val.round() as i64"),
        },
    ),
    (
        "ComboBox",
        "on_change",
        Binding {
            accessor: "crawrad_combo_box",
            part: "",
            query: "selected",
            shape: Shape::Value,
            writeback: Some("selected = Some(_val)"),
        },
    ),
    ("Edit", "on_change", TEXT_BOX_CHANGED),
    ("Edit", "on_return", TEXT_BOX_RETURNED),
    // the MaskEdit is the Edit's text box, its mask shaping `_val`
    ("MaskEdit", "on_change", TEXT_BOX_CHANGED),
    ("MaskEdit", "on_return", TEXT_BOX_RETURNED),
    (
        "Memo",
        "on_change",
        Binding {
            accessor: "text_input",
            part: "",
            query: "changed",
            shape: Shape::Value,
            writeback: Some("lines = Strings::from_text(&_val)"),
        },
    ),
];

#[note("targets-makepad-binding-binding-of")]
pub fn binding_of(class: &str, event: &str) -> Option<Binding> {
    BINDINGS
        .iter()
        .find(|(c, e, _)| *c == class && *e == event)
        .map(|(_, _, b)| *b)
}

#[note("targets-makepad-binding-accessors")]
pub fn accessors() -> Vec<&'static str> {
    let mut v: Vec<&str> = BINDINGS.iter().map(|(_, _, b)| b.accessor).collect();
    v.sort_unstable();
    v.dedup();
    v
}

#[note("targets-makepad-binding-queries")]
pub fn queries() -> Vec<&'static str> {
    let mut v: Vec<&str> = BINDINGS.iter().map(|(_, _, b)| b.query).collect();
    v.sort_unstable();
    v.dedup();
    v
}

#[note("targets-makepad-binding-bindings-declarados")]
pub const BINDINGS_DECLARADOS: &[(&str, Binding)] = &[(
    "on_click",
    Binding {
        accessor: "view",
        part: "",
        query: "finger_up",
        shape: Shape::Option,
        writeback: None,
    },
)];

#[note("targets-makepad-binding-binding-declarado")]
pub fn binding_declarado(event: &str) -> Option<Binding> {
    BINDINGS_DECLARADOS
        .iter()
        .find(|(e, _)| *e == event)
        .map(|(_, b)| *b)
}

#[note("targets-makepad-binding-emitir")]
pub fn emitir(b: &Binding, field: &str, handler: &str) -> String {
    emitir_com_corpo(b, field, &format!("            self.form.{handler}();\n"))
}

#[note("targets-makepad-binding-emitir-com-corpo")]
pub fn emitir_com_corpo(b: &Binding, field: &str, body: &str) -> String {
    let dentro = match b.writeback {
        Some(w) => format!("            self.form.{field}.{w};\n{body}"),
        None => body.to_string(),
    };
    let alvo = format!("self.ui.{}(id!({field}{}))", b.accessor, b.part);
    match b.shape {
        Shape::Plain => format!(
            "        if {alvo}.{}(actions) {{\n{dentro}        }}\n",
            b.query
        ),
        Shape::Value => format!(
            "        if let Some(_val) = {alvo}.{}(actions) {{\n{dentro}        }}\n",
            b.query
        ),
        Shape::Option => format!(
            "        if {alvo}.{}(actions).is_some() {{\n{dentro}        }}\n",
            b.query
        ),
        Shape::ActionCast => format!(
            "        if let {} = actions.find_widget_action_cast({alvo}.widget_uid()) {{\n\
             {dentro}        }}\n",
            b.query
        ),
    }
}
