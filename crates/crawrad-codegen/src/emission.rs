//! **The four verbs** — what a target needs to know to emit ANY
//! component, including one loaded from a file.
//!
//! Decided in ADR 0036.
//!
//! # Why four, and why THESE
//!
//! The unit of the format is the OBJECT, not the widget — it is this choice,
//! not whether to generate or interpret, that buys uniformity. The proof lies in
//! two opposing precedents: WinForms **generates code** and has uniform
//! non-visuals, containers, and roots; Qt also generates code and has **no**
//! non-visuals, because `QDesignerCustomWidgetInterface::createWidget` returns
//! `QWidget*` and nothing that is not a widget fits into `.ui`.
//!
//! About an object there are only four things to say: that it **exists**, what
//! it **is worth**, whose **child** it is, and what it **responds to**.
//!
//! | verb | the Makepad target translates |
//! |---|---|
//! | [`Emission::Create`] | `let mut id = Class::default();` |
//! | [`Emission::Set`] | `id.key = v;` — field name by CONVENTION |
//! | [`Emission::Child`] | nesting in `live_design!` |
//! | [`Emission::Bind`] | the entry in the event dispatch table |
//!
//! # They speak SEMANTICS, not text
//!
//! This is what separates this from the alternative rejected by ADR 0036: a
//! third-party component **does not embed Makepad code strings**. Of the eight
//! measured precedents, zero ask the component author to write target-language
//! text — Qt's `domXml()` returns XML of the `.ui` itself, that is, data in the
//! TOOL'S vocabulary. A new target tomorrow must not invalidate an already
//! published component.
//!
//! # A reference is NOT a verb
//!
//! `Value::Ref` — the `Action` a button points to — is **emission ordering**: create
//! everything, then bind. A classic RAD IDE resolves by name on load,
//! because it interprets a file at runtime; we
//! resolve by topology during generation, which is cheaper.

use crawrad_ir::{Document, Node, Value};
use crawrad_note::note;
use crawrad_registry::{Category, ComponentDesc};

#[note("emission-emission")]
#[derive(Debug, Clone, PartialEq)]
pub enum Emission {
    /// The object exists, and is of this class.
    Create {
        /// The node identifier in the document.
        id: String,
        /// The class name in the registry — `"Button"`, `"Connection"`.
        class: String,
    },
    /// A property of the object has this value.
    ///
    /// The key is that of the REGISTRY, never the field name in the target: translating
    /// is the responsibility of whoever honors it, which is why the same `Set` serves
    /// Makepad and egui, which store `item_index` under different names.
    Set {
        id: String,
        key: String,
        value: Value,
    },
    /// This object is a child of that one, at this position.
    ///
    /// Serves containers, collection items, actions in an `ActionList`, and
    /// grid columns — **collections cease to be a category** when the item
    /// is an object with a class (a classic RAD IDE gives each collection an
    /// item class for the same reason).
    Child {
        parent: String,
        id: String,
        index: usize,
    },
    /// This event of the object calls this user handler.
    ///
    /// It is the only verb touching the **irreducible floor**: in Rust there is no
    /// lookup of a method by its name at runtime, as a classic RAD IDE has, so the handler
    /// name must become a dispatch table at generation time.
    Bind {
        id: String,
        event: String,
        handler: String,
    },
}

#[note("emission-notset")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotSet {
    /// `name` is the object's IDENTITY: it becomes the field name, and an object
    /// does not carry its own name as a value.
    Identity,
    /// `rect` is GEOMETRY: consumed by the target's layout pass, which
    /// positions the object, not its state.
    Geometry,
    /// `anchors`, `align`, `margins`, `align_with_margins` — the entire
    /// [`Category::Layout`] category is resolved by layout, not stored.
    Layout,
    /// `tab_order` and `tab_stop` are resolved at generation into the
    /// form's Tab sequence (`targets::tab_places`): the sequence is the
    /// form's, not a value either control keeps.
    TabSequence,
    /// `makepad.<name>` / `egui.<name>`: a property only one framework
    /// can draw. The state both targets share never carries it; the target's
    /// own component module reads it from the node, and the other target never
    /// sees it.
    TargetOnly,
    /// `design_rows`: how many real rows the Designer shows in a bound
    /// grid. Only the canvas reads it; the app reads its own data at run time.
    DesignTime,
}

#[note("emission-why-not-set")]
pub fn why_not_set(key: &str, category: Category) -> Option<NotSet> {
    if matches!(
        crawrad_ir::PropKey::parse(key),
        Ok(crawrad_ir::PropKey::Target { .. })
    ) {
        return Some(NotSet::TargetOnly);
    }
    match key {
        "name" => Some(NotSet::Identity),
        "rect" => Some(NotSet::Geometry),
        "tab_order" | "tab_stop" => Some(NotSet::TabSequence),
        crawrad_ir::design_rows::DESIGN_ROWS_KEY => Some(NotSet::DesignTime),
        _ if category == Category::Layout => Some(NotSet::Layout),
        _ => None,
    }
}

#[note("emission-emissions-of")]
pub fn emissions_of(doc: &Document, node: &Node, desc: &ComponentDesc) -> Vec<Emission> {
    let mut out = vec![Emission::Create {
        id: node.id.clone(),
        class: node.class.clone(),
    }];

    for prop in &desc.props {
        if why_not_set(prop.key, prop.category).is_some() {
            continue;
        }
        let value = node
            .props
            .get(prop.key)
            .cloned()
            .unwrap_or_else(|| prop.default.clone());
        out.push(Emission::Set {
            id: node.id.clone(),
            key: prop.key.to_string(),
            value,
        });
    }

    for (index, filho) in node.children.iter().enumerate() {
        out.push(Emission::Child {
            parent: node.id.clone(),
            id: filho.clone(),
            index,
        });
    }

    // Events come from the DESCRIPTOR and not the node: thus the order is what the class
    // declared, not what the document's `BTreeMap` returned. Two documents
    // with the same handlers emit the same output.
    for ev in &desc.events {
        if let Some(handler) = node.events.get(ev.name) {
            out.push(Emission::Bind {
                id: node.id.clone(),
                event: ev.name.to_string(),
                handler: handler.clone(),
            });
        }
    }

    let _ = doc;
    out
}
