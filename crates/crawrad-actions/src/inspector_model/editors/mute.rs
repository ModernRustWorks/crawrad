//! The row that **shows the value and says why it does not edit**.
//!
//! It is the opposite of a dead field. A dead field is one that takes the
//! gesture and changes nothing, with nothing on screen saying so — the family
//! of defect this house took out of the status bar and out of the AI panel,
//! and it does not come back through the Inspector.
//!
//! # The three sentences, and who writes each one
//!
//! - `read-only` — the **catalogue** declared it
//!   ([`crawrad_registry::PropDesc::readonly`]). This is the only one this file
//!   decides by itself, and it is why [`KIND`] comes first in
//!   `super::KINDS`: read-only beats every type.
//! - `differs` — the **selection** decided it. Several components are selected
//!   and they do not agree on the value. The egui shell leaves a divergent text
//!   field editable (`crates/crawrad-ide/src/inspector.rs:213`) and this one
//!   does not: writing over two different values from a field that was showing
//!   one of them is the change nobody asked for.
//! - `bad type` — the **document** caused it. The value stored is not of the
//!   kind the catalogue declares, so no editor accepted it. Not a gaveta
//!   hypothesis: a `.crawform` written by hand, or a catalogue that changed the
//!   type of a key already saved, arrive exactly like this. Without the row,
//!   the text editor would show `0` for a `Value::List` and write over what was
//!   there.
//!
//! What is **not** here any more is `no editor`. It was the sentence of six of
//! the eleven property kinds, and the whole point of this folder is that no
//! property the egui shell can edit says it.

use super::super::editor::{Cell, Editor, PropEditor, Slot};
use super::{Kind, Subject, note};

#[note("inspector-model-editors-mute-read-only")]
pub const READ_ONLY: &str = "read-only";

#[note("inspector-model-editors-mute-differs")]
pub const DIFFERS: &str = "differs";

#[note("inspector-model-editors-mute-bad-type")]
pub const BAD_TYPE: &str = "bad type";

#[note("inspector-model-editors-mute-various")]
pub const VARIOUS: &str = "(various)";

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-mute-mute")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mute {
    value: String,
    note: &'static str,
}

impl Mute {
    pub fn new(value: impl Into<String>, note: &'static str) -> Self {
        Self {
            value: value.into(),
            note,
        }
    }

    #[note("inspector-model-editors-mute-mute-differing")]
    pub fn differing() -> Editor {
        Box::new(Self::new(VARIOUS, DIFFERS))
    }
}

impl PropEditor for Mute {
    fn template(&self) -> &'static str {
        "RowMute"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Caption {
                slot: Slot::Value,
                text: self.value.clone(),
            },
            Cell::Caption {
                slot: Slot::Note,
                text: self.note.to_string(),
            },
        ]
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    s.desc
        .readonly
        .then(|| Box::new(Mute::new(s.value.describe(), READ_ONLY)) as Editor)
}

fn sample() -> Editor {
    Box::new(Mute::new("(empty)", READ_ONLY))
}
