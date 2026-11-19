//! Align, distribute and equalize size — the Alignment Palette of the classic IDE.
//!
//! Everything here is a pure function over the document: takes the selection, returns a
//! [`Transaction`]. None of these operations knows what a screen is, and all of them fit
//! in one Ctrl+Z because they come out as a single batch.

use crawrad_doc::{Command, Transaction};
use crawrad_ir::{Document, NodeId, Rect};
use crawrad_note::note;
use std::collections::BTreeMap;

#[note("arrangement-alignmode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignMode {
    Left,
    CenterHorizontal,
    Right,
    Top,
    CenterVertical,
    Bottom,
}

impl AlignMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Left => "Align left",
            Self::CenterHorizontal => "Align centers",
            Self::Right => "Align right",
            Self::Top => "Align top",
            Self::CenterVertical => "Align middles",
            Self::Bottom => "Align bottom",
        }
    }

    fn horizontal(self) -> bool {
        matches!(self, Self::Left | Self::CenterHorizontal | Self::Right)
    }
}

#[note("arrangement-reference")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reference {
    /// The component that was selected first — that is what the classic IDE does.
    First,
    /// The boundaries of the form.
    Form,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distribute {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Equalize {
    Width,
    Height,
    Both,
}

fn rects(doc: &Document, sel: &[NodeId]) -> Vec<(NodeId, Rect)> {
    sel.iter()
        .filter_map(|id| doc.node(id).and_then(|n| n.rect()).map(|r| (id.clone(), r)))
        .collect()
}

#[note("arrangement-align")]
pub fn align(
    doc: &Document,
    sel: &[NodeId],
    mode: AlignMode,
    reference: Reference,
) -> Option<Transaction> {
    let items = rects(doc, sel);
    if items.is_empty() {
        return None;
    }

    let base = match reference {
        Reference::First => {
            if items.len() < 2 {
                return None;
            }
            items[0].1
        }
        Reference::Form => doc.node(&doc.root)?.rect()?,
    };

    let mut tx = Transaction::user(mode.label());
    let skip_first = reference == Reference::First;
    for (i, (id, r)) in items.iter().enumerate() {
        if skip_first && i == 0 {
            continue;
        }
        let new_rect = match mode {
            AlignMode::Left => Rect::new(base.x, r.y, r.w, r.h),
            AlignMode::Right => Rect::new(base.x + base.w - r.w, r.y, r.w, r.h),
            AlignMode::CenterHorizontal => Rect::new(base.x + (base.w - r.w) / 2.0, r.y, r.w, r.h),
            AlignMode::Top => Rect::new(r.x, base.y, r.w, r.h),
            AlignMode::Bottom => Rect::new(r.x, base.y + base.h - r.h, r.w, r.h),
            AlignMode::CenterVertical => Rect::new(r.x, base.y + (base.h - r.h) / 2.0, r.w, r.h),
        };
        if new_rect != *r {
            tx.push(Command::SetRect {
                node: id.clone(),
                rect: new_rect,
            });
        }
    }
    (!tx.is_empty()).then_some(tx)
}

#[note("arrangement-distribute")]
pub fn distribute(doc: &Document, sel: &[NodeId], mode: Distribute) -> Option<Transaction> {
    let mut items = rects(doc, sel);
    if items.len() < 3 {
        return None;
    }
    let horizontal = mode == Distribute::Horizontal;
    items.sort_by(|a, b| {
        let (x, y) = if horizontal {
            (a.1.x, b.1.x)
        } else {
            (a.1.y, b.1.y)
        };
        x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal)
    });

    // the total gap is what remains between the first and last endpoints
    let first = items.first()?.1;
    let last = items.last()?.1;
    let (start, end, sum) = if horizontal {
        (
            first.x + first.w,
            last.x,
            items[1..items.len() - 1]
                .iter()
                .map(|(_, r)| r.w)
                .sum::<f32>(),
        )
    } else {
        (
            first.y + first.h,
            last.y,
            items[1..items.len() - 1]
                .iter()
                .map(|(_, r)| r.h)
                .sum::<f32>(),
        )
    };
    let gap = (end - start - sum) / (items.len() - 1) as f32;

    let mut tx = Transaction::user(if horizontal {
        "Space equally, horizontally"
    } else {
        "Space equally, vertically"
    });
    let mut cursor = start;
    for (id, r) in &items[1..items.len() - 1] {
        let new_rect = if horizontal {
            Rect::new(cursor + gap, r.y, r.w, r.h)
        } else {
            Rect::new(r.x, cursor + gap, r.w, r.h)
        };
        cursor += gap + if horizontal { r.w } else { r.h };
        if new_rect != *r {
            tx.push(Command::SetRect {
                node: id.clone(),
                rect: new_rect,
            });
        }
    }
    (!tx.is_empty()).then_some(tx)
}

#[note("arrangement-equalize")]
pub fn equalize(doc: &Document, sel: &[NodeId], mode: Equalize) -> Option<Transaction> {
    let items = rects(doc, sel);
    if items.len() < 2 {
        return None;
    }
    let model = items[0].1;

    let mut tx = Transaction::user(match mode {
        Equalize::Width => "Same width",
        Equalize::Height => "Same height",
        Equalize::Both => "Same size",
    });
    for (id, r) in items.iter().skip(1) {
        let new_rect = match mode {
            Equalize::Width => Rect::new(r.x, r.y, model.w, r.h),
            Equalize::Height => Rect::new(r.x, r.y, r.w, model.h),
            Equalize::Both => Rect::new(r.x, r.y, model.w, model.h),
        };
        if new_rect != *r {
            tx.push(Command::SetRect {
                node: id.clone(),
                rect: new_rect,
            });
        }
    }
    (!tx.is_empty()).then_some(tx)
}

#[note("arrangement-center-in-form")]
pub fn center_in_form(doc: &Document, sel: &[NodeId], mode: AlignMode) -> Option<Transaction> {
    let items = rects(doc, sel);
    if items.is_empty() {
        return None;
    }
    let form = doc.node(&doc.root)?.rect()?;

    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for (_, r) in &items {
        x0 = x0.min(r.x);
        y0 = y0.min(r.y);
        x1 = x1.max(r.x + r.w);
        y1 = y1.max(r.y + r.h);
    }

    let (dx, dy) = if mode.horizontal() {
        ((form.w - (x1 - x0)) / 2.0 - x0, 0.0)
    } else {
        (0.0, (form.h - (y1 - y0)) / 2.0 - y0)
    };

    let mut tx = Transaction::user(if mode.horizontal() {
        "Center horizontally in form"
    } else {
        "Center vertically in form"
    });
    for (id, r) in items {
        tx.push(Command::SetRect {
            node: id,
            rect: Rect::new(r.x + dx, r.y + dy, r.w, r.h),
        });
    }
    (!tx.is_empty()).then_some(tx)
}
// ------------------------------------------------------- Z-order and tab order

#[note("arrangement-zorder")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZOrder {
    Front,
    Back,
    Forward,
    Backward,
}

impl ZOrder {
    pub fn label(self) -> &'static str {
        match self {
            Self::Front => "Bring to front",
            Self::Back => "Send to back",
            Self::Forward => "Bring forward",
            Self::Backward => "Send backward",
        }
    }
}

#[note("arrangement-z-order")]
pub fn z_order(doc: &Document, sel: &[NodeId], mode: ZOrder) -> Option<Transaction> {
    // Groups the selection by parent, preserving the order of each list.
    let mut per_parent: BTreeMap<NodeId, Vec<usize>> = BTreeMap::new();
    for id in sel {
        let Some(parent) = doc.parent_of(id) else {
            continue;
        };
        let Some(children) = doc.node(parent).map(|n| &n.children) else {
            continue;
        };
        if children.len() < 2 {
            continue;
        }
        if let Some(i) = children.iter().position(|c| c == id) {
            per_parent.entry(parent.clone()).or_default().push(i);
        }
    }
    if per_parent.is_empty() {
        return None;
    }

    let mut tx = Transaction::user(mode.label());
    for (parent, mut targets) in per_parent {
        targets.sort_unstable();
        let children = doc.node(&parent).map(|n| n.children.len()).unwrap_or(0);
        reorder_one_parent(&mut tx, &parent, &targets, children, mode);
    }
    (!tx.is_empty()).then_some(tx)
}

#[note("arrangement-reorder-one-parent")]
fn reorder_one_parent(
    tx: &mut Transaction,
    parent: &NodeId,
    targets: &[usize],
    count: usize,
    mode: ZOrder,
) {
    let last = count - 1;

    match mode {
        ZOrder::Front => {
            // from back to front, accounting for those that already left the queue
            for (n, from) in targets.iter().enumerate() {
                let from = from - n;
                if from != last {
                    tx.push(Command::Reorder {
                        parent: parent.clone(),
                        from,
                        to: last,
                    });
                }
            }
        }
        ZOrder::Back => {
            for (n, from) in targets.iter().rev().enumerate() {
                let from = from + n;
                if from != 0 {
                    tx.push(Command::Reorder {
                        parent: parent.clone(),
                        from,
                        to: 0,
                    });
                }
            }
        }
        ZOrder::Forward => {
            for from in targets.iter().rev() {
                if *from < last {
                    tx.push(Command::Reorder {
                        parent: parent.clone(),
                        from: *from,
                        to: from + 1,
                    });
                }
            }
        }
        ZOrder::Backward => {
            for from in targets {
                if *from > 0 {
                    tx.push(Command::Reorder {
                        parent: parent.clone(),
                        from: *from,
                        to: from - 1,
                    });
                }
            }
        }
    }
}

#[note("arrangement-tab-order")]
pub fn tab_order(doc: &Document) -> Vec<(NodeId, i64)> {
    let Some(root) = doc.node(&doc.root) else {
        return Vec::new();
    };
    let mut list: Vec<(NodeId, i64)> = root
        .children
        .iter()
        .filter_map(|id| {
            let n = doc.node(id)?;
            n.props.get("tab_order").map(|v| {
                (
                    id.clone(),
                    match v {
                        crawrad_ir::Value::Int(i) => *i,
                        _ => 0,
                    },
                )
            })
        })
        .collect();
    // ties are resolved by Z-order, which is what the classic IDE does
    list.sort_by_key(|(_, t)| *t);
    list
}

#[note("arrangement-next-tab-order")]
pub fn next_tab_order(doc: &Document) -> i64 {
    tab_order(doc)
        .iter()
        .map(|(_, t)| t + 1)
        .max()
        .unwrap_or(0)
        .max(0)
}

#[note("arrangement-normalize-tab-order")]
pub fn normalize_tab_order(doc: &Document) -> Option<Transaction> {
    let list = tab_order(doc);
    let mut tx = Transaction::user("Renumber tab order");
    for (i, (id, current)) in list.iter().enumerate() {
        let new_val = i as i64;
        if *current != new_val {
            tx.push(Command::SetProp {
                node: id.clone(),
                key: "tab_order".into(),
                value: Some(crawrad_ir::Value::Int(new_val)),
            });
        }
    }
    (!tx.is_empty()).then_some(tx)
}

#[note("arrangement-move-tab-order")]
pub fn move_tab_order(doc: &Document, id: &NodeId, up: bool) -> Option<Transaction> {
    let list = tab_order(doc);
    let pos = list.iter().position(|(i, _)| i == id)?;
    let other = if up {
        pos.checked_sub(1)?
    } else {
        (pos + 1 < list.len()).then_some(pos + 1)?
    };

    let mut tx = Transaction::user("Change tab order");
    tx.push(Command::SetProp {
        node: list[pos].0.clone(),
        key: "tab_order".into(),
        value: Some(crawrad_ir::Value::Int(other as i64)),
    });
    tx.push(Command::SetProp {
        node: list[other].0.clone(),
        key: "tab_order".into(),
        value: Some(crawrad_ir::Value::Int(pos as i64)),
    });
    Some(tx)
}

#[cfg(test)]
#[path = "test/arrangement/test.rs"]
mod tests;
