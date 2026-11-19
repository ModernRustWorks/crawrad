//! **What a Splitter moves**, the same for both targets.
//!
//! The generated geometry is literal, so a Splitter keeps an `offset` (how
//! far it was dragged) and the controls it moves ADD it to their literals:
//! the neighbour before it grows, the Splitter itself moves, and the
//! neighbour after it shrinks — the classic RAD IDE's splitter between
//! aligned controls. The neighbours are found once, at generation, as the
//! siblings that touch the Splitter's two long edges.

use crawrad_ir::{Document, Node, Rect};
use crawrad_note::note;

#[note("targets-splitters-split")]
pub struct Split {
    /// The Splitter's field name.
    pub field: String,
    /// A vertical bar (taller than wide) moves left and right.
    pub vertical: bool,
    /// The neighbour before it (left / above) and its length along the drag.
    pub before: Option<(String, f32)>,
    /// The neighbour after it (right / below) and its length along the drag.
    pub after: Option<(String, f32)>,
}

const TOUCH: f32 = 1.5;

fn overlaps(a0: f32, a1: f32, b0: f32, b1: f32) -> bool {
    a0 < b1 && b0 < a1
}

#[note("targets-splitters-splits")]
pub fn splits(doc: &Document) -> Vec<Split> {
    let mut out = Vec::new();
    for s in doc.nodes.values().filter(|n| n.class == "Splitter") {
        let (Some(r), Ok(field)) = (s.rect(), crate::identificador(s.name())) else {
            continue;
        };
        let vertical = r.h >= r.w;
        let parent = doc.parent_of(&s.id);
        let siblings: Vec<(&Node, Rect)> = doc
            .nodes
            .values()
            .filter(|n| n.id != s.id && doc.parent_of(&n.id) == parent)
            .filter_map(|n| n.rect().map(|nr| (n, nr)))
            .collect();
        let find = |before: bool| {
            siblings.iter().find_map(|(n, nr)| {
                let touches = if vertical {
                    overlaps(nr.y, nr.y + nr.h, r.y, r.y + r.h)
                        && if before {
                            (nr.x + nr.w - r.x).abs() <= TOUCH
                        } else {
                            (nr.x - (r.x + r.w)).abs() <= TOUCH
                        }
                } else {
                    overlaps(nr.x, nr.x + nr.w, r.x, r.x + r.w)
                        && if before {
                            (nr.y + nr.h - r.y).abs() <= TOUCH
                        } else {
                            (nr.y - (r.y + r.h)).abs() <= TOUCH
                        }
                };
                let len = if vertical { nr.w } else { nr.h };
                touches
                    .then(|| crate::identificador(n.name()).ok().map(|f| (f, len)))
                    .flatten()
            })
        };
        out.push(Split {
            field,
            vertical,
            before: find(true),
            after: find(false),
        });
    }
    out
}

#[note("targets-splitters-terms")]
pub fn terms(doc: &Document, node: &Node, prefix: &str) -> [String; 4] {
    let mut t: [String; 4] = Default::default();
    let Ok(me) = crate::identificador(node.name()) else {
        return t;
    };
    for s in splits(doc) {
        let off = format!(" + {prefix}{}.offset", s.field);
        let neg = format!(" - {prefix}{}.offset", s.field);
        let (pos, len) = if s.vertical { (0, 2) } else { (1, 3) };
        if s.field == me {
            t[pos].push_str(&off);
        }
        if s.before.as_ref().is_some_and(|(f, _)| *f == me) {
            t[len].push_str(&off);
        }
        if s.after.as_ref().is_some_and(|(f, _)| *f == me) {
            t[pos].push_str(&off);
            t[len].push_str(&neg);
        }
    }
    t
}

#[note("targets-splitters-split-of")]
pub fn split_of(doc: &Document, field: &str) -> Option<Split> {
    splits(doc).into_iter().find(|s| s.field == field)
}
