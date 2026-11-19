//! Graphic picture editor — `Picture`.
//!
//! The row reads the picture (`(None)`, `(Picture: logo.png)`) with a `…`
//! that opens the file dialog filtered to images, as the classic RAD IDE does,
//! and a `×` that puts it back to `(None)`. It used to be a text field, and
//! the owner, clicking `Picture` on an `Image`, had nothing to choose a file
//! with.
//!
//! The `…` runs `inspector.pick_picture` with `{node, key}`, so every
//! component with a property of this TYPE gets the dialog, and an agent runs
//! the same action with a `file`. The `×` writes the empty picture
//! straight away: there is nothing to choose.

use crate::Args;
use crawrad_ir::{NodeId, Picture, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-picture-pictureeditor")]
#[derive(Debug, Clone, PartialEq)]
pub struct PictureEditor {
    node: NodeId,
    key: &'static str,
    picture: Picture,
}

impl PictureEditor {
    pub fn new(node: NodeId, key: &'static str, picture: Picture) -> Self {
        Self { node, key, picture }
    }
}

impl PropEditor for PictureEditor {
    fn template(&self) -> &'static str {
        "RowOpen"
    }

    fn cells(&self) -> Vec<Cell> {
        // An empty picture has nothing to clear, and the `×` says so by not
        // being drawn: a button that does nothing is the dead control this
        // panel does not keep.
        let clear = if self.picture.is_empty() {
            ""
        } else {
            icon::CLEAR
        };
        let mut cells = super::collection::open_cells(self.picture.to_string());
        cells.push(Cell::Press {
            slot: Slot::Clear,
            text: clear.to_string(),
        });
        cells
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Button, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_PICK_PICTURE,
                Args::text("node", self.node.clone()).and("key", Value::Str(self.key.to_string())),
            ),
            (Slot::Clear, Gesture::Pressed) if !self.picture.is_empty() => {
                Outcome::Set(Value::Picture(Picture::empty()))
            }
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Picture, Value::Picture(p)) => Some(Box::new(PictureEditor::new(
            s.node.clone(),
            s.desc.key,
            p.clone(),
        ))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(PictureEditor::new(
        "Image1".to_string(),
        "picture",
        Picture::empty().with_source("logo.png"),
    ))
}
