//! **A class's state, derived from its descriptor** — without one line per
//! class.
//!
//! It is the declarative half of the four verbs in
//! ADR 0036:
//! `Create` becomes the `struct`, `Set` becomes the field and initializer.
//!
//! # What it replaces
//!
//! Sixteen copies of the SAME loop over `desc.props`, one per component
//! file. Normalizing the class name and the descriptor call, the
//! target's fifteen `struct_definition`s collapsed into **ten distinct forms** — and
//! what varied between them was the handwritten list of names to skip, which is
//! a manual copy of `crawrad_registry::componentes::base`.
//!
//! The copy had already diverged: `Button`, `ScrollBar`, and `ScrollBox` emitted a
//! `tab_order` field that their eight siblings sharing the same base did not emit, because the
//! three did not write the name in the list and `tab_order` is `Comportamento`, not
//! `Layout`.
//!
//! # Convention, and an escape hatch — because no precedent lived without one
//!
//! The general rule is that of Qt's `uic`, measured in `cppwriteinitialization.cpp`: the
//! field name is **manufactured** from the property name, and the compiler is what
//! checks it. The escape hatch is [`FIELD_MAP`], and it is DATA — not a handler per
//! class.
//!
//! The price is declared in ADR 0036 and accepted: the naming convention becomes
//! a contract, and the error appears in the generated code rather than in the designer. That is what
//! Qt, WinForms, and IntelliJ pay, and it is less than the other three.

use crawrad_ir::{Node, Value};
use crawrad_registry::ComponentDesc;

use crate::emission::why_not_set;

#[note("targets-makepad-state-field")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    /// The field name, when it is not the property key.
    pub name: &'static str,
    /// The Rust type, when it is not what the default value implies.
    pub ty: &'static str,
}

#[note("targets-makepad-state-field-map")]
pub const FIELD_MAP: &[(&str, &str, Field)] = &[
    // `interval` is `Int` in the catalog and Makepad needs unsigned
    // milliseconds for the framework's `Timer`.
    (
        "Timer",
        "interval",
        Field {
            name: "interval_ms",
            ty: "u64",
        },
    ),
    // `images_data` is a collection in the catalog (`Images`) and the target stores it as `images: Vec<ImageItem>`.
    (
        "ImageList",
        "images_data",
        Field {
            name: "images",
            ty: "Vec<ImageItem>",
        },
    ),
    // `items` is a hierarchical collection for `ListView`.
    (
        "ListView",
        "items",
        Field {
            name: "items",
            ty: "Vec<ListViewItem>",
        },
    ),
    // The TreeView holds its nodes as a run-time tree.
    (
        "TreeView",
        "items",
        Field {
            name: "items",
            ty: "Vec<crawrad_runtime::tree::TreeNode>",
        },
    ),
    // The catalog specifies `item_index: Int(-1)` — the `-1` is "none" in a classic RAD IDE.
    // In Rust "none" has a type, and that is why the target does not keep the
    // sentinel.
    (
        "ComboBox",
        "item_index",
        Field {
            name: "selected",
            ty: "Option<usize>",
        },
    ),
    (
        "ListBox",
        "item_index",
        Field {
            name: "selected",
            ty: "Option<usize>",
        },
    ),
    (
        "TreeView",
        "item_index",
        Field {
            name: "selected",
            ty: "Option<usize>",
        },
    ),
    // The active page is an index, and an index in Rust is `usize`.
    (
        "PageControl",
        "active_page",
        Field {
            name: "active_page",
            ty: "usize",
        },
    ),
    // `ScrollBar` stores the five measurements in `f32`, and the catalog declares them
    // as `Int`. The reason is thumb arithmetic: the ratio `(pos - min) /
    // (max - min)` is fractional, and rounding it every frame would cause the thumb
    // to jump on a long bar. The egui target stores the same five in
    // `i64` — exactly the kind of divergence a convention without an escape hatch
    // could not express.
    (
        "ScrollBar",
        "min",
        Field {
            name: "min",
            ty: "f32",
        },
    ),
    (
        "ScrollBar",
        "max",
        Field {
            name: "max",
            ty: "f32",
        },
    ),
    (
        "ScrollBar",
        "position",
        Field {
            name: "position",
            ty: "f32",
        },
    ),
    (
        "ScrollBar",
        "small_change",
        Field {
            name: "small_change",
            ty: "f32",
        },
    ),
    (
        "ScrollBar",
        "large_change",
        Field {
            name: "large_change",
            ty: "f32",
        },
    ),
];

#[note("targets-makepad-state-field-of")]
pub fn field_of(class: &str, key: &str, default: &Value) -> Field {
    for (c, k, f) in FIELD_MAP {
        if *c == class && *k == key {
            return *f;
        }
    }
    Field {
        name: "",
        ty: rust_type_of(default),
    }
}

/// The Rust type implied by a default value.
pub use crate::targets::shared::rust_type_of;
use crawrad_note::note;

#[note("targets-makepad-state-extra-fields")]
pub const EXTRA_FIELDS: &[(&str, &str, &str)] = &[
    ("DateTimePicker", "shown", "String"),
    // The tab in focus is NOT the active page: the user may be navigating the
    // strip with the keyboard without having switched pages yet.
    ("PageControl", "active_tab", "usize"),
    // `Memo` stores the text AND the lines: the catalog declares one property
    // (`multiline_text`), and the target needs both forms — the framework's `TextInput`
    // wants the `String`, the drawing wants the lines.
    ("Memo", "lines", "Strings"),
    // `TreeView` does not have `item_index` in the catalog (unlike `ComboBox`
    // and `ListBox`): its selection is target state.
    ("TreeView", "selected", "Option<usize>"),
    // Which nodes are collapsed belongs to drawing, not the document — `.crawform`
    // stores the tree, never what is expanded on screen.
    ("TreeView", "collapsed", "std::collections::BTreeSet<usize>"),
    // How far a Splitter was dragged, and where the drag started from
    // its neighbours add the offset to their literal boxes.
    ("Splitter", "offset", "f32"),
    ("Splitter", "press", "f32"),
    ("MessageDialog", "modal_result", "ModalResult"),
    ("MessageDialog", "answer", "PendingAnswer"),
    ("OpenDialog", "files", "Vec<String>"),
    // Every row a ListBox marks; `item_index` is only the row in focus
    ("ListBox", "selection", "Vec<usize>"),
    // The string grid stores its cell contents and selection in the form state.
    ("Grid", "cells", "crawrad_runtime::string_grid::Cells"),
    ("Grid", "selected", "Option<(usize, usize)>"),
];

#[note("targets-makepad-state-fields-of")]
pub fn fields_of(desc: &ComponentDesc) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for prop in &desc.props {
        if why_not_set(prop.key, prop.category).is_some() {
            continue;
        }
        let f = field_of(desc.class, prop.key, &prop.default);
        let nome = if f.name.is_empty() {
            prop.key.to_string()
        } else {
            f.name.to_string()
        };
        out.push((nome, f.ty.to_string()));
    }
    for (class, nome, ty) in EXTRA_FIELDS {
        if *class == desc.class {
            out.push((nome.to_string(), ty.to_string()));
        }
    }
    out
}

#[note("targets-makepad-state-inits-of")]
pub fn inits_of(node: &Node, desc: &ComponentDesc) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for prop in &desc.props {
        if why_not_set(prop.key, prop.category).is_some() {
            continue;
        }
        let f = field_of(desc.class, prop.key, &prop.default);
        let nome = if f.name.is_empty() {
            prop.key.to_string()
        } else {
            f.name.to_string()
        };
        let valor = node.props.get(prop.key).unwrap_or(&prop.default);
        out.push((nome, super::components::util::value_to_rust_literal(valor)));
    }
    out
}
