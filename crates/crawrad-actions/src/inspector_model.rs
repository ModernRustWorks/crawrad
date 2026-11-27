//! The Object Inspector of the Makepad shell — its rows, and where they come
//! from.
//!
//! # Why it is the instrument, and not one more feature
//!
//! The visual parity of the canvas components was finished and **could not be
//! judged**: you can see that a border exists, and you cannot tick a
//! `CheckBox`, drag the thumb of a `ScrollBar`, fill a `ListBox` or disable a
//! `Button` and watch the drawing answer. Without this panel, the only way to
//! check the drawing of a property is to edit the `.crawform` in a text editor
//! and restart the IDE.
//!
//! # Composition, and the test that charges it
//!
//! There is no hand-written list of properties here, and no `match` by class.
//! The rows come out of the **catalogue** (`crawrad-registry`): the order is the
//! order of the categories, the labels are the `display` of the descriptors and
//! the editor is chosen by the [`crawrad_registry::PropKind`]. A property new in
//! the catalogue shows up in the Inspector **without a line edited here** —
//! which is what `toda_propriedade_do_catalogo_vira_linha_sem_lista_a_mao`
//! charges.
//!
//! And the editors themselves are **one file each**
//! ([`editors`]): adding a kind of editor costs creating a file, not editing a
//! `match` that lives somewhere else. What charges *that* is
//! `todo_propkind_do_catalogo_tem_editor_na_casca`, which reads the variants of
//! `PropKind` out of the registry's own source text — so a kind that lands
//! tomorrow is born charged.
//!
//! # The three shapes of a row
//!
//! A row of this panel is **not** "label plus field". It has three shapes, and
//! which one a property gets is derived from its kind, never from its name:
//!
//! 1. **a field, right here** — `String`, `Bool`, `Int`, `Float`, `Enum`,
//!    `Color`, `Rect`, `Reference`;
//! 2. **a summary and a button that opens** — `Multiline`, `Collection`, and
//!    the drawn picker of `Align`;
//! 3. **a summary that unfolds in place** — `Edges`, over its two shapes
//!    (`Margins` and `Anchors`).
//!
//! An Inspector that only knew the first shape would be poor with all eleven
//! kinds implemented.
//!
//! # The two tabs, and why the events one is not an extra
//!
//! The panel has `Properties` and `Events`, like the classic RAD one and like the egui
//! shell's (`crates/crawrad-ide/src/inspector.rs:18-23`). The owner was
//! explicit: without the events the Inspector does not validate, because an
//! Object Inspector that shows half the component hides exactly the half that
//! ties the form to the code.
//!
//! The events come from the **same place** as the properties — the catalogue
//! (`crawrad_registry::EventDesc`). No `on_click` is written in this shell, and
//! whoever binds the handler is the `event.bind` action
//! (`crawrad-actions/src/lib.rs`), never a `Transaction` written here. Since
//! `event.bind` is a catalogue action, the same gesture is available to an
//! agent (ADR 0029).
//!
//! # This module does not draw
//!
//! It is a pure function over the catalogue and the document, and that is why
//! the order, the value and the cells of every row can be measured without
//! opening a window. Whoever paints is the `handle_draw_2d` of the shell;
//! whoever writes is the catalogue of actions
//! (`component.set_property` / `component.rename` / `inspector.*`), never this
//! file.

use std::collections::BTreeSet;

use crawrad_ir::{Document, NodeId, Value};
use crawrad_registry::{Category, ComponentDesc, PropDesc, Registry};

/// The editor contract — `#[path]` for the same reason as in `ide_app.rs`:
/// a module loaded by `#[path]` looks for children in the file's own
/// directory, not in a directory named after it.
#[path = "inspector_model/editor.rs"]
pub mod editor;

/// One file per editor type.
#[path = "inspector_model/editors.rs"]
pub mod editors;

/// The drawings that a row carries.
pub use crawrad_designer::inspector_icon as icon;

use crawrad_note::note;
pub use editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
pub use editors::font::{font_from_choice, font_to_choice, pick_font};
use editors::{Subject, editor_for, mute::Mute};

#[note("inspector-model-tab")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Properties,
    Events,
}

impl Tab {
    #[note("inspector-model-tab-all")]
    pub const ALL: &'static [Self] = &[Self::Properties, Self::Events];

    #[note("inspector-model-tab-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Properties => "Properties",
            Self::Events => "Events",
        }
    }
}

#[note("inspector-model-action-open-editor")]
pub const ACTION_OPEN_EDITOR: &str = "inspector.open_editor";
/// The key of the synthesized child-collection row — not a catalogue property.
pub const CHILD_COLLECTION_KEY: &str = "child_collection";
pub use editors::child_collection::ACTION_OPEN_COLLECTION;

#[note("inspector-model-action-close-editor")]
pub const ACTION_CLOSE_EDITOR: &str = "inspector.close_editor";

#[note("inspector-model-action-toggle-group")]
pub const ACTION_TOGGLE_GROUP: &str = "inspector.toggle_group";

#[note("inspector-model-action-pick-picture")]
pub const ACTION_PICK_PICTURE: &str = "inspector.pick_picture";

#[note("inspector-model-action-pick-font")]
pub const ACTION_PICK_FONT: &str = "inspector.pick_font";

#[note("inspector-model-action-toggle-category")]
pub const ACTION_TOGGLE_CATEGORY: &str = "inspector.toggle_category";

#[note("inspector-model-no-form")]
pub const NO_FORM: &str = "No form open.";

#[note("inspector-model-nothing-selected")]
pub const NOTHING_SELECTED: &str = "Nothing selected.";

#[note("inspector-model-how-to-select")]
pub const HOW_TO_SELECT: &str = "Click a component on the form.";

#[note("inspector-model-no-descriptor")]
pub const NO_DESCRIPTOR: &str = "component has no descriptor in the registry";

#[note("inspector-model-no-events")]
pub const NO_EVENTS: &str = "No events available for this component.";

#[note("inspector-model-events-single-only")]
pub const EVENTS_SINGLE_ONLY: &str = "Events are not available for multiple components.";

#[note("inspector-model-data-category")]
pub const DATA_CATEGORY: &str = "Data";

#[note("inspector-model-bound-to-dataset")]
pub const BOUND_TO_DATASET: &str = "bound";

#[note("inspector-model-linked-to-action")]
pub const LINKED_TO_ACTION: &str = "action";

/// What the shell knows of a dataset's columns: not yet (`None`), the list,
/// or why they could not be read.
#[note("inspector-model-column-answer")]
pub type ColumnAnswer = Option<Result<Vec<String>, String>>;

#[note("inspector-model-columns-unknown")]
pub const COLUMNS_UNKNOWN: &str = "Columns not listed";

#[note("inspector-model-parse-binding-mode")]
pub fn parse_binding_mode(s: &str) -> crawrad_ir::BindingMode {
    match s.trim().to_ascii_lowercase().as_str() {
        "oneway" | "one_way" => crawrad_ir::BindingMode::OneWay,
        "onewaytosource" | "one_way_to_source" => crawrad_ir::BindingMode::OneWayToSource,
        "onetime" | "one_time" => crawrad_ir::BindingMode::OneTime,
        _ => crawrad_ir::BindingMode::TwoWay,
    }
}

#[note("inspector-model-binding-mode-name")]
pub fn binding_mode_name(mode: crawrad_ir::BindingMode) -> &'static str {
    match mode {
        crawrad_ir::BindingMode::OneWay => "OneWay",
        crawrad_ir::BindingMode::TwoWay => "TwoWay",
        crawrad_ir::BindingMode::OneWayToSource => "OneWayToSource",
        crawrad_ir::BindingMode::OneTime => "OneTime",
    }
}

#[note("inspector-model-separator")]
const SEPARATOR: &str = "  ·  ";

#[note("inspector-model-order")]
const ORDER: [Category; 6] = [
    Category::Identity,
    Category::Appearance,
    Category::Data,
    Category::Layout,
    Category::Behavior,
    Category::Advanced,
];

#[note("inspector-model-row")]
#[derive(Debug)]
pub enum Row {
    /// The label of a category — `Appearance`, `Layout` — and whether what is
    /// under it is on screen.
    ///
    /// The heading FOLDS, like the egui shell's (`inspector.rs`, an
    /// `egui::CollapsingHeader` per category) and like the classic RAD one. `open` is
    /// carried on the row rather than read again at draw time because the row
    /// is what the `PortalList` hands back on a click, and re-reading the state
    /// there would let the chevron and the rows disagree for one frame.
    Category { label: &'static str, open: bool },
    /// A property, with the editor its `PropKind` asks for.
    Prop(PropRow),
    /// An event of the `Events` tab, with the handler it points at today.
    Event(EventRow),
    /// The panel has nothing to show, and says why.
    Notice(&'static str),
    /// The selected component comes from this base form.
    InheritedFrom(String),
}

#[note("inspector-model-rows-json")]
pub fn rows_json(rows: &[Row]) -> serde_json::Value {
    rows_json_with_rects(rows, &[])
}

/// Serializes inspector rows with measured on-screen rectangles.
pub fn rows_json_with_rects(rows: &[Row], rects: &[(usize, [f64; 4])]) -> serde_json::Value {
    use serde_json::json;
    let cell = |c: &Cell| {
        let slot = format!("{:?}", c.slot());
        match c {
            Cell::Field { text, .. } => json!({ "slot": slot, "kind": "field", "text": text }),
            Cell::Check { on, .. } => json!({ "slot": slot, "kind": "check", "on": on }),
            Cell::Choice {
                options, current, ..
            } => {
                json!({ "slot": slot, "kind": "choice", "options": options, "current": current })
            }
            Cell::Caption { text, .. } => json!({ "slot": slot, "kind": "caption", "text": text }),
            Cell::Press { text, .. } => json!({ "slot": slot, "kind": "button", "text": text }),
            Cell::Swatch { rgba, .. } => json!({ "slot": slot, "kind": "swatch", "rgba": rgba }),
        }
    };
    let row = |(i, r): (usize, &Row)| {
        let rect = rects.iter().find(|(n, _)| *n == i).map(|(_, r)| *r);
        match r {
            Row::Category { label, open } => {
                json!({ "category": label, "open": open, "rect": rect })
            }
            Row::Prop(p) => json!({
                "key": p.key,
                "label": p.label,
                "depth": p.depth,
                "template": p.editor.template(),
                "cells": p.editor.cells().iter().map(cell).collect::<Vec<_>>(),
                "rect": rect,
            }),
            Row::Event(e) => json!({
                "event": e.event,
                "label": e.label,
                "handler": e.handler,
                "base_handler": e.base_handler,
                "template": e.template(),
                "rect": rect,
            }),
            Row::Notice(text) => json!({ "notice": text, "rect": rect }),
            Row::InheritedFrom(form) => json!({ "inherited_from": form, "rect": rect }),
        }
    };
    json!({ "rows": rows.iter().enumerate().map(row).collect::<Vec<_>>() })
}

#[note("inspector-model-eventrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRow {
    /// The name as the catalogue declares it — `on_click`. It is what goes in
    /// the `event` argument of the `event.bind` action.
    pub event: &'static str,
    /// The name the row shows, the `display` of the descriptor — `OnClick`.
    pub label: &'static str,
    /// The function the event points at today, or empty when it is unbound.
    pub handler: String,
    /// The signature the catalogue declares, for the hint of the row.
    pub signature: &'static str,
    /// The base form's handler of this event, when the component is
    /// inherited and the base handles it.
    pub base_handler: Option<String>,
    /// The base form that handler is in.
    pub base_form: Option<String>,
}

impl EventRow {
    /// Only inherited: the event still runs the base's handler.
    pub fn is_inherited(&self) -> bool {
        self.base_handler.as_deref() == Some(self.handler.trim())
    }

    /// This form gave the inherited event a handler of its own.
    pub fn is_override(&self) -> bool {
        self.base_handler.is_some() && !self.is_inherited() && !self.handler.trim().is_empty()
    }

    /// The template the row is drawn with.
    pub fn template(&self) -> &'static str {
        if self.is_inherited() {
            "RowEventInherited"
        } else if self.is_override() {
            "RowEventOverride"
        } else {
            "RowEvent"
        }
    }
}

#[note("inspector-model-override-handler")]
pub fn override_handler(base: &str) -> String {
    format!("{base}_override")
}

#[note("inspector-model-mark-inherited")]
pub fn mark_inherited(rows: &mut Vec<Row>, doc: &crawrad_doc::Doc, selection: &[NodeId]) {
    let ([id], Some(base)) = (selection, doc.base()) else {
        return;
    };
    let Some(base_node) = base.node(id).filter(|_| doc.is_inherited(id)) else {
        return;
    };
    let mut events = false;
    for row in rows.iter_mut() {
        if let Row::Event(e) = row {
            let h = base_node.events.get(e.event).map(|h| h.trim());
            e.base_handler = h.filter(|h| !h.is_empty()).map(str::to_string);
            e.base_form = doc.ir().inherits().map(str::to_string);
            events = true;
        }
    }
    if events && let Some(form) = doc.ir().inherits() {
        rows.insert(0, Row::InheritedFrom(form.to_string()));
    }
}

#[note("inspector-model-proprow")]
#[derive(Debug)]
pub struct PropRow {
    /// The catalogue key — it is what goes in the `component.set_property`.
    pub key: &'static str,
    /// The name the row shows. `String` and not `&'static str` because a
    /// sub-row of a folded property is named by the editor above it (`Left`,
    /// `akTop`), and those names are not in the catalogue.
    pub label: String,
    /// How deep it sits: zero for a property, one for a sub-row of one that
    /// folded open.
    pub depth: usize,
    /// How this property is edited (or why it is not).
    pub editor: Editor,
    /// Does this row get the STRIPE?
    ///
    /// # The stripe resets on each category, and that was measured in the screenshot
    ///
    /// In the reference screenshot — the egui shell, which the owner approved — `Items` comes out
    /// dark and `Images` light; then `Rect` dark, `Anchors` light, `Align`
    /// dark and `Margins` light. Counted from the start of the list the alternation does not
    /// match; counted from the start of EACH CATEGORY, it matches exactly.
    ///
    /// And the reason is structural, not aesthetic: there, each category is its own
    /// `egui::Grid` inside its `CollapsingHeader`
    /// (`crates/crawrad-ide/src/inspector.rs`), and egui's `striped(true)`
    /// counts per grid. Folding a category cannot reshuffle the stripes
    /// of the others — and does not reshuffle, because each one counts its own.
    ///
    /// The SUB-ROW of a folded property inherits the stripe of the row that
    /// opened it: they are a single unit on the screen, and alternating in the middle would split the
    /// block in half.
    pub striped: bool,
}

#[note("inspector-model-header")]
pub fn header(doc: Option<&Document>, selection: &[NodeId]) -> String {
    let Some(doc) = doc else {
        return "—".to_string();
    };
    match selection {
        [] => "—".to_string(),
        [one] => match doc.node(one) {
            Some(n) if !n.class.is_empty() => format!("{}{SEPARATOR}{}", n.name(), n.class),
            Some(n) => n.name().to_string(),
            None => "—".to_string(),
        },
        many => format!("{} items selected", many.len()),
    }
}

/// Every component of the form, the form itself included, for the Object Inspector's
/// component picker: `(id, name, class)` sorted by name, case-insensitive.
pub fn component_choices(doc: Option<&Document>) -> Vec<(NodeId, String, String)> {
    let Some(doc) = doc else {
        return Vec::new();
    };
    let mut out: Vec<_> = (doc.walk().into_iter())
        .map(|n| (n.id.clone(), n.name().to_string(), n.class.clone()))
        .collect();
    out.sort_by_key(|(_, name, _)| name.to_lowercase());
    out
}

/// Whether `p` is hidden because a sibling reference points at a component
/// answering to the capability it names (`PropDesc::hidden_when`) — by TYPE.
fn hidden_by_reference(
    registry: &Registry,
    doc: &Document,
    node: &crawrad_ir::Node,
    p: &PropDesc,
) -> bool {
    let Some((reference, capability)) = p.hidden_when else {
        return false;
    };
    let Some(Value::Ref(id)) = node.props.get(reference) else {
        return false;
    };
    doc.node(id)
        .and_then(|target| registry.get(&target.class))
        .is_some_and(|desc| desc.answers_to(capability))
}

/// A non-visual component's `rect` only places its icon on the design
/// surface: the running app never reads it and its size is fixed. It is
/// still stored, and moved by dragging the icon — never shown as a row.
fn icon_rect(desc: &ComponentDesc, p: &PropDesc) -> bool {
    !desc.visual && p.key == "rect"
}

#[note("inspector-model-rows")]
pub fn rows(
    registry: &Registry,
    doc: Option<&Document>,
    selection: &[NodeId],
    target: &str,
    tab: Tab,
    open: &BTreeSet<String>,
    folded: &BTreeSet<String>,
) -> Vec<Row> {
    rows_with_columns(registry, doc, selection, target, tab, open, folded, &|_| {
        None
    })
}

#[note("inspector-model-rows-with-columns")]
pub fn rows_with_columns(
    registry: &Registry,
    doc: Option<&Document>,
    selection: &[NodeId],
    target: &str,
    tab: Tab,
    open: &BTreeSet<String>,
    folded: &BTreeSet<String>,
    columns: &dyn Fn(&str) -> ColumnAnswer,
) -> Vec<Row> {
    let Some(doc) = doc else {
        return vec![Row::Notice(NO_FORM)];
    };
    let Some(first) = selection.first() else {
        return vec![Row::Notice(NOTHING_SELECTED), Row::Notice(HOW_TO_SELECT)];
    };
    let Some(node) = doc.node(first) else {
        return vec![Row::Notice(NOTHING_SELECTED)];
    };
    let Some(desc) = registry.get(&node.class) else {
        return vec![Row::Notice(NO_DESCRIPTOR)];
    };

    let multi = selection.len() > 1;

    if let Tab::Events = tab {
        if multi {
            return vec![Row::Notice(EVENTS_SINGLE_ONLY)];
        }
        if desc.events.is_empty() {
            return vec![Row::Notice(NO_EVENTS)];
        }
        return desc
            .events
            .iter()
            .map(|e| {
                Row::Event(EventRow {
                    event: e.name,
                    label: e.display,
                    // The handler COMES FROM THE DOCUMENT, and the absent one
                    // is empty — never the name it would have if it were
                    // bound. The christening (`Button1Click`) belongs to the
                    // gesture that binds, not to the panel that shows: written
                    // here, the row would say the event is bound when it is
                    // not.
                    handler: node.events.get(e.name).cloned().unwrap_or_default(),
                    signature: e.signature,
                    base_handler: None,
                    base_form: None,
                })
            })
            .collect();
    }

    let shared = multi.then(|| shared_keys(registry, doc, selection));
    let bindable_prop = if !multi {
        desc.props.iter().find(|p| p.bindable)
    } else {
        None
    };

    let mut out = Vec::new();
    for category in ORDER {
        let aberta = !folded.contains(category.label());
        let mut opened = false;
        let mut na_categoria = 0usize;
        for p in desc.props_for(target).filter(|p| p.category == category) {
            if shared.as_ref().is_some_and(|c| !c.contains(&p.key)) {
                continue;
            }
            if hidden_by_reference(registry, doc, node, p) || icon_rect(desc, p) {
                continue;
            }
            // THE HEADER APPEARS EVEN WHEN CLOSED, while its rows do not. That is what
            // makes folding reversible: a header that disappeared along with the
            // content would leave the category with nothing on screen to reopen.
            //
            // And it only appears when there is at least one property to show —
            // the same rule as before. A category that this target lacks continues
            // not appearing, closed or open.
            if !opened {
                out.push(Row::Category {
                    label: category.label(),
                    open: aberta,
                });
                opened = true;
            }
            if !aberta {
                continue;
            }
            // The stripe counts WITHIN the category — see `PropRow::striped`.
            let listrada = na_categoria % 2 == 1;
            na_categoria += 1;
            let editor: Box<dyn PropEditor> = if let (
                crawrad_registry::PropKind::FieldOf(source_key),
                false,
            ) = (&p.kind, multi)
            {
                let lookup_src = match node.props.get(*source_key) {
                    Some(Value::Ref(s)) if !s.is_empty() => Some(s.as_str()),
                    Some(Value::Str(s)) if !s.is_empty() => Some(s.as_str()),
                    _ => None,
                };
                let known_columns = lookup_src
                    .and_then(columns)
                    .and_then(Result::ok)
                    .filter(|cols| !cols.is_empty());
                match known_columns {
                    Some(mut cols) => {
                        let cur_str = match node.props.get(p.key) {
                            Some(Value::Str(s) | Value::Enum(s)) => s.clone(),
                            _ => String::new(),
                        };
                        if !cur_str.is_empty() && !cols.contains(&cur_str) {
                            cols.insert(0, cur_str.clone());
                        }
                        Box::new(editors::choice::Choice::new(&cols, &cur_str))
                    }
                    None => editor_of(registry, p, doc, selection, first, multi, open),
                }
            } else {
                editor_of(registry, p, doc, selection, first, multi, open)
            };
            // The sub-rows of a property that folded open come from the editor
            // itself, which is what keeps the assembler from knowing that
            // `Margins` has four sides.
            let children = editor.children();
            out.push(Row::Prop(PropRow {
                key: p.key,
                label: p.display.to_string(),
                depth: 0,
                editor,
                striped: listrada,
            }));
            for (label, child) in children {
                out.push(Row::Prop(PropRow {
                    key: p.key,
                    label,
                    depth: 1,
                    editor: child,
                    // THE SAME stripe as the row that opened it — see `PropRow::striped`.
                    striped: listrada,
                }));
            }
        }
        // The child collection (`Columns (3) …`) closes the Identity
        // category: always in sight, and it is never folded away.
        if category == Category::Identity && !multi && aberta {
            // flat only: a menu's items nest, and their editor is the tree
            if let Some(label) = desc
                .collection_label()
                .filter(|_| crate::action_list::is_flat_collection(registry, &node.class))
            {
                out.push(Row::Prop(PropRow {
                    key: CHILD_COLLECTION_KEY,
                    label,
                    depth: 0,
                    editor: Box::new(editors::child_collection::ChildCollection::new(
                        first.clone(),
                        node.children.len(),
                    )),
                    striped: na_categoria % 2 == 1,
                }));
            }
        }
        // the Data category comes right after Appearance
        if let Some(b_prop) = bindable_prop.filter(|_| category == Category::Appearance) {
            let data_open = !folded.contains(DATA_CATEGORY);
            out.push(Row::Category {
                label: DATA_CATEGORY,
                open: data_open,
            });
            if data_open {
                let binding = node.binding(b_prop.key);
                let current_field = binding
                    .and_then(|b| b.field.as_ref())
                    .cloned()
                    .unwrap_or_default();
                let synth_props = [
                    (
                        crawrad_registry::Props::binding_dataset(),
                        Value::Ref(binding.map(|b| b.source.clone()).unwrap_or_default()),
                    ),
                    (
                        crawrad_registry::Props::binding_field(),
                        Value::Str(current_field.clone()),
                    ),
                    (
                        crawrad_registry::Props::binding_mode(),
                        Value::Enum(
                            binding
                                .map(|b| binding_mode_name(b.mode))
                                .unwrap_or("TwoWay")
                                .to_string(),
                        ),
                    ),
                ];
                for (i, (synth_desc, val)) in synth_props.into_iter().enumerate() {
                    // Field becomes a drop-down over the dataset's columns
                    // once they are known; until then it stays free text, and
                    // when they cannot be read (no `crawrad-dbprobe` next to the
                    // IDE, a bad url) the row says why instead of staying
                    // silently a text box.
                    let answer = binding
                        .filter(|_| synth_desc.key == crawrad_registry::Props::binding_field().key)
                        .filter(|b| !b.source.is_empty())
                        .and_then(|b| columns(&b.source));
                    let (known_columns, problem) = match answer {
                        Some(Ok(cols)) if !cols.is_empty() => (Some(cols), None),
                        Some(Err(reason)) => (None, Some(reason)),
                        _ => (None, None),
                    };
                    let editor: Box<dyn PropEditor> = match known_columns {
                        // a field the table does not have (or none yet) heads
                        // the list: the drop-down must show what the form says
                        Some(mut cols) => {
                            if !cols.contains(&current_field) {
                                cols.insert(0, current_field.clone());
                            }
                            Box::new(editors::choice::Choice::new(&cols, &current_field))
                        }
                        None if problem.is_some() => Box::new(editors::text::Unlisted::new(
                            current_field.clone(),
                            format!("{COLUMNS_UNKNOWN}: {}", problem.unwrap_or_default()),
                        )),
                        None => {
                            let subject = Subject {
                                desc: &synth_desc,
                                registry,
                                value: &val,
                                node: first,
                                doc,
                                open: false,
                            };
                            editor_for(&subject).unwrap_or_else(|| {
                                Box::new(Mute::new(val.describe(), editors::mute::BAD_TYPE))
                            })
                        }
                    };
                    out.push(Row::Prop(PropRow {
                        key: synth_desc.key,
                        label: synth_desc.display.to_string(),
                        depth: 0,
                        editor,
                        striped: i % 2 == 1,
                    }));
                }
            }
        }
    }
    out
}

#[note("inspector-model-shared-keys")]
fn shared_keys(
    registry: &Registry,
    doc: &Document,
    selection: &[NodeId],
) -> BTreeSet<&'static str> {
    let mut shared: Option<BTreeSet<&'static str>> = None;
    for id in selection {
        let Some(d) = doc.node(id).and_then(|n| registry.get(&n.class)) else {
            continue;
        };
        let his: BTreeSet<&'static str> = d.props.iter().map(|p| p.key).collect();
        shared = Some(match shared {
            None => his,
            Some(a) => a.intersection(&his).copied().collect(),
        });
    }
    shared.unwrap_or_default()
}

#[note("inspector-model-shared-value")]
fn shared_value(doc: &Document, selection: &[NodeId], p: &PropDesc) -> Option<Value> {
    let mut seen: Option<Value> = None;
    for id in selection {
        let v = doc
            .node(id)?
            .props
            .get(p.key)
            .cloned()
            .unwrap_or_else(|| p.default.clone());
        match &seen {
            None => seen = Some(v),
            Some(a) if *a == v => {}
            Some(_) => return None,
        }
    }
    seen
}

#[note("inspector-model-editor-of")]
fn editor_of(
    registry: &Registry,
    p: &PropDesc,
    doc: &Document,
    selection: &[NodeId],
    node: &NodeId,
    multi: bool,
    open: &BTreeSet<String>,
) -> Editor {
    // A bound property belongs to its binding: it shows where the value comes
    // from and is changed under Data. With SEVERAL selected, one bound
    // is enough to mute the row — typing there would write a value the binding
    // overwrites at run time, on the one component nobody was looking at.
    let mut bound = selection
        .iter()
        .filter_map(|id| doc.node(id).and_then(|n| n.binding(p.key)));
    if let Some(b) = bound.next() {
        let text = match &b.field {
            _ if multi => "(bound)".to_string(),
            Some(f) if !f.is_empty() => format!("from {}.{}", b.source, f),
            _ => format!("from {}", b.source),
        };
        return Box::new(Mute::new(text, BOUND_TO_DATASET));
    }
    // A caption linked to an Action belongs to the action: the canvas and the
    // app show the action's text (`Document::linked_action_text`), so the row
    // shows that same text and does not edit — typing there would change a
    // value nobody sees.
    if p.key == "text"
        && let Some(t) = selection.iter().find_map(|id| doc.linked_action_text(id))
    {
        let text = if multi { "(linked)" } else { t };
        return Box::new(Mute::new(text, LINKED_TO_ACTION));
    }
    let Some(current) = shared_value(doc, selection, p) else {
        return Mute::differing();
    };
    // The `Name` of several is the one case where the divergence does not show
    // up on its own: two components may have different names and the line
    // above already covers it, but two with the SAME name do not exist (the
    // `Command::Rename` refuses). What is left is the case of one, which falls
    // through to the text editor.
    if multi && p.key == "name" {
        return Mute::differing();
    }
    let subject = Subject {
        desc: p,
        registry,
        value: &current,
        node,
        doc,
        open: open.contains(p.key),
    };
    // `None` is not a hole in the coverage — it is the document holding a value
    // of a kind the catalogue does not declare. Showing the value and refusing
    // the gesture is the only honest road: without it, the text editor would
    // show `0` for a `Value::List` and write over what was there.
    editor_for(&subject)
        .unwrap_or_else(|| Box::new(Mute::new(current.describe(), editors::mute::BAD_TYPE)))
}

#[note("inspector-model-categories-closed-on-open")]
pub fn categories_closed_on_open() -> BTreeSet<String> {
    ORDER
        .iter()
        .filter(|c| **c == Category::Advanced)
        .map(|c| c.label().to_string())
        .collect()
}

#[note("inspector-model-name-width-default")]
pub const NAME_WIDTH_DEFAULT: f64 = 136.0;

#[note("inspector-model-name-width-min")]
pub const NAME_WIDTH_MIN: f64 = 48.0;

#[note("inspector-model-value-width-min")]
pub const VALUE_WIDTH_MIN: f64 = 64.0;

#[note("inspector-model-name-width")]
pub fn name_width(requested: f64, room: f64) -> f64 {
    let widest = (room - VALUE_WIDTH_MIN).max(NAME_WIDTH_MIN);
    requested.clamp(NAME_WIDTH_MIN, widest)
}

#[note("inspector-model-row-chrome")]
pub const ROW_CHROME: f64 = 24.0;

#[note("inspector-model-splitter-offset")]
pub const SPLITTER_OFFSET: f64 = 4.0 + 2.0 - 3.5;

#[note("inspector-model-namesplitter")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NameSplitter {
    /// The width asked, by the drag or by `inspector.set_name_width`.
    pub requested: f64,
    /// The width last drawn.
    shown: f64,
}

impl Default for NameSplitter {
    fn default() -> Self {
        Self {
            requested: NAME_WIDTH_DEFAULT,
            shown: NAME_WIDTH_DEFAULT,
        }
    }
}

impl NameSplitter {
    #[note("inspector-model-namesplitter-layout")]
    pub fn layout(&mut self, list_width: f64) -> (f64, f64) {
        let room = list_width - ROW_CHROME;
        self.shown = if room > 0.0 {
            name_width(self.requested, room)
        } else {
            self.requested
        };
        (self.shown, self.shown + SPLITTER_OFFSET)
    }

    #[note("inspector-model-namesplitter-ask")]
    pub fn ask(&mut self, width: i32) {
        self.requested = f64::from(width);
    }
}

#[note("inspector-model-window")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    /// The Items Editor of a collection, the classic RAD IDE's own.
    Items,
    /// The String List Editor of a multi-line text (`Memo.Lines`).
    Lines,
    /// The Color dialog.
    Color,
    /// The pad drawn over the form — the `Align` picker.
    Pad,
}

#[note("inspector-model-window-for")]
pub fn window_for(rows: &[Row], key: &str) -> Window {
    let editor = rows.iter().find_map(|row| match row {
        Row::Prop(p) if p.key == key => Some(&p.editor),
        _ => None,
    });
    match editor {
        Some(e) if e.window_color().is_some() => Window::Color,
        Some(e) if e.window_items().is_some() => Window::Items,
        Some(e) if e.window_text().is_some() => Window::Lines,
        _ => Window::Pad,
    }
}

#[cfg(test)]
#[path = "test/inspector_model/test.rs"]
mod tests;
