//! The kinds of editor the Object Inspector draws — **one file each**.
//!
//! That is the whole rule of this folder, and it is the one the house asks
//! for: adding a kind of editor costs *creating a file*, not editing a `match`
//! that lives somewhere else. Each file below owns everything about its own
//! editor — the template, what goes in each slot, what a gesture means, the
//! sub-rows it unfolds into, and the test data the gate draws it with — and
//! nothing outside it has to learn the new kind exists.
//!
//! The one place that still has to hear about a new editor is `KINDS`, and
//! that is deliberate rather than forgotten, for two reasons:
//!
//! 1. the choice of editor is **derived from data** — the
//!    [`crawrad_registry::PropKind`] of the property — so some ordered chain has
//!    to exist; what the chain must not be is a `match` that also knows how to
//!    draw and how to parse;
//! 2. the DSL gate covers **exactly** what the sample instantiates, so an
//!    editor nobody samples is a template nobody checks — and an unchecked
//!    template is one that draws nothing at all, in silence.
//!
//! # The order of `KINDS` is load-bearing
//!
//! The first maker that accepts wins, and three pairs overlap on purpose:
//!
//! - `collection` and `lines` come **before** `mute`, because a property
//!   with a WINDOW behind it is not read-only in any sense the user can feel.
//!   `readonly` in the catalogue means *no field to type into on this row* —
//!   and a window is not a field on the row. It is the egui shell's rule, word
//!   for word: `if p.readonly || janela.is_some()` (`inspector.rs`), where the
//!   `…` lives inside the branch.
//!
//!   This was a defect the owner saw: the `Items` of a `TreeView` showed
//!   `4 items` and a grey `read-only`, and **no `…` at all** — the collection
//!   editor exists, it is wired, and nothing on screen led to it. The four
//!   `so_leitura()` of the catalogue are the `Items`, and all four have a
//!   window (`crawrad-registry/src/props.rs`);
//! - `mute` comes next, because after those two `readonly` in the catalogue
//!   does beat every type — the value still shows, and the row says why it
//!   does not edit;
//! - `anchors` comes before `edges`, because `Anchors` is declared
//!   `PropKind::Edges` and **holds a `Value::Map`** (`crawrad-registry/src/props.rs`,
//!   `Props::anchors`) — the same kind, two shapes, and the four check boxes
//!   are not four numbers;
//! - `align` comes before `choice`, because `Align` is a `PropKind::Enum`
//!   like any other and gets the drawn picker of the classic RAD instead of a list
//!   of names.

use crawrad_ir::{Document, NodeId, Value};
use crawrad_registry::{PropDesc, Registry};

use super::editor::Editor;
// The editors below reach `note` through `super`, on their `use super::{..}` line.
pub(crate) use crawrad_note::note;

// The `#[path]` attributes for the same reason as in `ide_app/explorer/rows.rs`:
// a module loaded by `#[path]` looks for children in the file's own directory,
// not in a directory named after it.
#[path = "editors/align.rs"]
pub mod align;
#[path = "editors/anchors.rs"]
pub mod anchors;
#[path = "editors/check.rs"]
pub mod check;
#[path = "editors/child_collection.rs"]
pub mod child_collection;
#[path = "editors/choice.rs"]
pub mod choice;
#[path = "editors/collection.rs"]
pub mod collection;
#[path = "editors/color.rs"]
pub mod color;
#[path = "editors/cursor.rs"]
pub mod cursor;
#[path = "editors/date_time.rs"]
pub mod date_time;
#[path = "editors/decimal.rs"]
pub mod decimal;
#[path = "editors/edges.rs"]
pub mod edges;
#[path = "editors/file_path.rs"]
pub mod file_path;
#[path = "editors/font.rs"]
pub mod font;
#[path = "editors/integer.rs"]
pub mod integer;
#[path = "editors/lines.rs"]
pub mod lines;
#[path = "editors/modal_result.rs"]
pub mod modal_result;
#[path = "editors/mute.rs"]
pub mod mute;
#[path = "editors/picture.rs"]
pub mod picture;
#[path = "editors/rect.rs"]
pub mod rect;
#[path = "editors/reference.rs"]
pub mod reference;
#[path = "editors/shortcut.rs"]
pub mod shortcut;
#[path = "editors/text.rs"]
pub mod text;
#[path = "editors/when.rs"]
pub mod when;

#[note("inspector-model-editors-subject")]
pub struct Subject<'a> {
    /// What the catalogue says about the property.
    pub desc: &'a PropDesc,
    /// **The catalogue itself**, for the editor that has to ask about OTHER
    /// components.
    ///
    /// Only the reference editor needs it today, and it needs it for a reason
    /// the descriptor cannot cover: a reference declares the name it accepts,
    /// and who answers to that name is a question about every class in the
    /// catalogue, not about this property.
    pub registry: &'a Registry,
    /// What the document holds, already defaulted from the catalogue.
    pub value: &'a Value,
    /// The component that owns it — where a gesture on the row writes.
    pub node: &'a NodeId,
    /// The open form.
    pub doc: &'a Document,
    /// Are the sub-rows of this property unfolded?
    ///
    /// Meaningless to every editor that does not fold, and the folding ones
    /// take it as *"hand back children, or do not"*.
    pub open: bool,
}

#[note("inspector-model-editors-kind")]
pub struct Kind {
    /// Does this file edit that property? `None` means *ask the next one*.
    pub make: fn(&Subject<'_>) -> Option<Editor>,
    /// One of these, built out of nothing, for the DSL gate to draw.
    ///
    /// It exists so the gate charges the template **and** the templates of the
    /// children: a folding editor samples itself open.
    ///
    /// Not `#[cfg(test)]`: the shell's DSL gate reads it from another crate.
    pub sample: fn() -> Editor,
}

#[note("inspector-model-editors-kinds")]
pub const KINDS: &[Kind] = &[
    // THE TWO WITH A WINDOW BEHIND THEM come before `mute` — see the module note.
    // They only accept their own `PropKind`, so being in front gives them
    // nothing that was not already theirs.
    collection::KIND,
    lines::KIND,
    mute::KIND,
    anchors::KIND,
    edges::KIND,
    align::KIND,
    choice::KIND,
    // before `text`: a StyleRule's `When` is a String that carries a verdict
    when::KIND,
    text::KIND,
    check::KIND,
    integer::KIND,
    decimal::KIND,
    color::KIND,
    rect::KIND,
    reference::KIND,
    date_time::KIND,
    file_path::KIND,
    cursor::KIND,
    modal_result::KIND,
    shortcut::KIND,
    font::KIND,
    picture::KIND,
];

#[note("inspector-model-editors-editor-for")]
pub fn editor_for(subject: &Subject<'_>) -> Option<Editor> {
    KINDS.iter().find_map(|k| (k.make)(subject))
}

#[note("inspector-model-editors-samples")]
pub fn samples() -> Vec<Editor> {
    let mut out = Vec::new();
    for kind in KINDS {
        let editor = (kind.sample)();
        for (_, child) in editor.children() {
            out.push(child);
        }
        out.push(editor);
    }
    out
}
