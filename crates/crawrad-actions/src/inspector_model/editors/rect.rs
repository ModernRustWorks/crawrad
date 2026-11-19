//! Geometry — `Rect`, a summary row that folds open into `Left`, `Top`,
//! `Width` and `Height`.
//!
//! # It folds, like `Margins`
//!
//! It was four fields on one row, `[x 536][y 400][w 140][h 160]`. The owner
//! asked on 13/10/2026 for the model `Margins` already has: shut, the row reads
//! the rectangle (`536, 400, 140 × 160`); open, it is four rows, one per
//! number, each typed over. One row model for every compound property, instead
//! of an editor of its own just for geometry — and the four fields no longer
//! fight a narrow dock for room.
//!
//! The fold is the same as [`super::edges`]: the chevron runs
//! `inspector.toggle_group` with the key, so an agent unfolds `Rect` with a
//! tool, and the open set lives in the shell.
//!
//! # The floor of 8 on `Width` and `Height` is a limit of GESTURE
//!
//! A typed size below it goes in clamped, and **nothing rewrites the document
//! on its own**. In the egui shell that distinction cost a real defect: with
//! the passive clamp of `DragValue` on, merely *selecting* a collapsed
//! component with the `Layout` section open wrote `w`/`h` back as 8 — the owner
//! shrank the form, clicked the component, and the size never grew back. The
//! number 8 is the owner's decision and is not touched here.
//!
//! `Left` and `Top` have no floor, and that is deliberate: a component parked
//! at a negative coordinate is a component being dragged in from the edge,
//! which is a thing people do.
//!
//! # Every quarter writes the WHOLE rectangle
//!
//! There is no partial-field command in `crawrad-doc`, and there should not be —
//! one batch is one `Ctrl+Z`, and `component.set_property` already takes the
//! selection.

use crate::Args;
use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::decimal::Decimal;
use super::{Kind, Subject, note};

#[note("inspector-model-editors-rect-floor")]
const FLOOR: f64 = 8.0;

#[note("inspector-model-editors-rect-ceiling")]
const CEILING: f64 = 4000.0;

#[note("inspector-model-editors-rect-quarters")]
const QUARTERS: [(&str, Quarter); 4] = [
    ("Left", Quarter::Left),
    ("Top", Quarter::Top),
    ("Width", Quarter::Width),
    ("Height", Quarter::Height),
];

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-rect-quarter")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Quarter {
    Left,
    Top,
    Width,
    Height,
}

#[note("inspector-model-editors-rect-rectrow")]
#[derive(Debug, Clone, PartialEq)]
pub struct RectRow {
    key: &'static str,
    rect: crawrad_ir::Rect,
    open: bool,
}

impl RectRow {
    pub fn new(key: &'static str, rect: crawrad_ir::Rect, open: bool) -> Self {
        Self { key, rect, open }
    }
}

impl PropEditor for RectRow {
    fn template(&self) -> &'static str {
        "RowFold"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Press {
                slot: Slot::Fold,
                text: if self.open {
                    icon::FOLD_OPEN.to_string()
                } else {
                    icon::FOLD_CLOSED.to_string()
                },
            },
            Cell::Caption {
                slot: Slot::Value,
                text: Value::Rect(self.rect).describe(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Fold, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_TOGGLE_GROUP,
                Args::text("key", self.key.to_string()),
            ),
            _ => Outcome::Ignore,
        }
    }

    fn children(&self) -> Vec<(String, Editor)> {
        if !self.open {
            return Vec::new();
        }
        QUARTERS
            .into_iter()
            .map(|(label, quarter)| {
                (
                    label.to_string(),
                    Box::new(RectQuarter {
                        whole: self.rect,
                        quarter,
                    }) as Editor,
                )
            })
            .collect()
    }
}

#[note("inspector-model-editors-rect-rectquarter")]
#[derive(Debug, Clone, Copy, PartialEq)]
struct RectQuarter {
    whole: crawrad_ir::Rect,
    quarter: Quarter,
}

impl RectQuarter {
    fn current(&self) -> f32 {
        match self.quarter {
            Quarter::Left => self.whole.x,
            Quarter::Top => self.whole.y,
            Quarter::Width => self.whole.w,
            Quarter::Height => self.whole.h,
        }
    }
}

impl PropEditor for RectQuarter {
    fn template(&self) -> &'static str {
        "SubText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: Decimal::show(f64::from(self.current())),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        let (Slot::Value, Gesture::Typed(text)) = (slot, gesture) else {
            return Outcome::Ignore;
        };
        // The range belongs to the quarter, not to the row: a position may be
        // negative and a size may not be zero.
        let (min, max) = match self.quarter {
            Quarter::Width | Quarter::Height => (FLOOR, CEILING),
            Quarter::Left | Quarter::Top => (-CEILING, CEILING),
        };
        let parsed = match Decimal::parse(text, min, max) {
            Ok(v) => v as f32,
            Err(why) => return Outcome::Refuse(why),
        };
        let mut r = self.whole;
        match self.quarter {
            Quarter::Left => r.x = parsed,
            Quarter::Top => r.y = parsed,
            Quarter::Width => r.w = parsed,
            Quarter::Height => r.h = parsed,
        }
        Outcome::Set(Value::Rect(r))
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Rect, Value::Rect(r)) => Some(Box::new(RectRow::new(s.desc.key, *r, s.open))),
        _ => None,
    }
}

#[note("inspector-model-editors-rect-sample")]
fn sample() -> Editor {
    Box::new(RectRow::new(
        "rect",
        crawrad_ir::Rect::new(536.0, 400.0, 140.0, 160.0),
        true,
    ))
}
