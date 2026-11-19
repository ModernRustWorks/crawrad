//! The classic `Align`: automatic alignment inside the form.
//!
//! Unlike the Alignment Palette (which is an operation on several), this is a
//! **property of the component**: `alTop` sticks to the top occupying the whole
//! width, `alClient` takes what remains, and so on.
//!
//! The rule is that of the classic IDE: components are processed **in Z-order**, and each
//! one takes a slice of the area that still remains. The result is written to the
//! document as `SetRect`, so the `.crawform` continues to be the source of
//! truth and undo works the same.

use crawrad_doc::{Command, Transaction};
use crawrad_ir::{Document, Edges, Node, Rect, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    None,
    Top,
    Bottom,
    Left,
    Right,
    Client,
}

impl Align {
    pub fn from_value(v: Option<&Value>) -> Self {
        match v {
            Some(Value::Enum(s)) => match s.as_str() {
                "Top" => Self::Top,
                "Bottom" => Self::Bottom,
                "Left" => Self::Left,
                "Right" => Self::Right,
                "Client" => Self::Client,
                _ => Self::None,
            },
            _ => Self::None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Top => "Top",
            Self::Bottom => "Bottom",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Client => "Client",
        }
    }

    #[note("layout-align-is-fixed")]
    pub fn is_fixed(self) -> bool {
        self != Self::None
    }
}

#[note("layout-margins")]
pub struct Margins;

impl Margins {
    #[note("layout-margins-default")]
    pub const DEFAULT: Edges = Edges::uniform(3.0);

    #[note("layout-margins-from-value")]
    pub fn from_value(v: Option<&Value>) -> Edges {
        match v {
            Some(Value::Edges(e)) => *e,
            _ => Self::DEFAULT,
        }
    }

    #[note("layout-margins-effective")]
    pub fn effective(n: &Node) -> Edges {
        if matches!(n.props.get("align_with_margins"), Some(Value::Bool(true))) {
            Self::from_value(n.props.get("margins"))
        } else {
            Edges::uniform(0.0)
        }
    }
}

#[note("layout-pass")]
struct Pass<'a> {
    doc: &'a Document,
    /// Decided in this pass, not yet applied to the document.
    decided: std::collections::BTreeMap<String, Rect>,
    tx: Transaction,
}

impl<'a> Pass<'a> {
    fn new(doc: &'a Document) -> Self {
        Self {
            doc,
            decided: std::collections::BTreeMap::new(),
            tx: Transaction::user("Apply Align"),
        }
    }

    #[note("layout-pass-rect")]
    fn rect(&self, id: &str) -> Option<Rect> {
        self.decided
            .get(id)
            .copied()
            .or_else(|| self.doc.node(id)?.rect())
    }

    #[note("layout-pass-set")]
    fn set(&mut self, id: &str, new_rect: Rect) {
        if self.rect(id) == Some(new_rect) {
            return;
        }
        self.decided.insert(id.to_string(), new_rect);
        self.tx.push(Command::SetRect {
            node: id.to_string(),
            rect: new_rect,
        });
    }
}

#[note("layout-normalize-pages")]
fn normalize_pages(pass: &mut Pass, id: &str) {
    let doc = pass.doc;
    let Some(pc) = doc.node(id) else { return };
    if pc.class != "PageControl" {
        return;
    }
    let Some(r) = pass.rect(id) else { return };
    // The pages sit below the strip, or above it when the tabs are at
    // the bottom
    let target = Rect::new(
        0.0,
        crawrad_registry::pages_top(crawrad_registry::tabs_at_bottom(pc)),
        r.w,
        (r.h - crawrad_registry::TAB_STRIP_HEIGHT).max(0.0),
    );
    for child in &pc.children {
        if doc.node(child).is_some_and(|n| n.class == "TabSheet") {
            pass.set(child, target);
        }
    }
}

#[note("layout-normalize-bar")]
fn normalize_bar(pass: &mut Pass, id: &str) {
    let doc = pass.doc;
    let Some(bar) = doc.node(id) else { return };
    if bar.class != "ToolBar" {
        return;
    }
    let Some(area) = pass.rect(id) else { return };
    let int = |key: &str, default: f32| match bar.props.get(key) {
        Some(crawrad_ir::Value::Int(i)) => *i as f32,
        _ => default,
    };
    let (bw, bh) = (int("button_width", 24.0), int("button_height", 24.0));
    let mut x = 0.0f32;
    for child in &bar.children {
        let Some(n) = doc.node(child) else { continue };
        let Some(r) = pass.rect(child) else { continue };
        let (w, h) = if matches!(n.class.as_str(), "Button" | "SpeedButton" | "BitBtn") {
            (bw, bh)
        } else {
            (r.w, r.h)
        };
        let y = ((area.h - h) / 2.0).max(0.0);
        pass.set(child, Rect::new(x, y, w, h));
        x += w;
    }
}

#[note("layout-apply")]
pub fn apply(doc: &Document) -> Option<Transaction> {
    let mut pass = Pass::new(doc);
    // Each container aligns its OWN children over its OWN area
    // (ADR 0012).
    // Was read in the source of the classic component library: `AlignControls` runs on the `ClientRect` of each
    // `TWinControl`, and there is nothing global. An `alClient` inside a panel
    // takes the client OF THE PANEL.
    //
    // The root enters first: the form is the top container. After it,
    // `in_paint_order` returns parent before children, and that order is what
    // makes ONE turn suffice — when a container's turn arrives, its
    // rectangle has already been decided by what contains it, and `Pass` knows that.
    align_children(&mut pass, &doc.root);
    for id in doc.in_paint_order() {
        normalize_pages(&mut pass, &id);
        normalize_bar(&mut pass, &id);
        align_children(&mut pass, &id);
    }
    (!pass.tx.is_empty()).then_some(pass.tx)
}

#[note("layout-align-children")]
fn align_children(pass: &mut Pass, parent: &str) {
    let doc = pass.doc;
    let Some(node) = doc.node(parent) else { return };
    if node.children.is_empty() {
        return;
    }
    let Some(area) = pass.rect(parent) else {
        return;
    };

    // the area that still remains, consumed in Z-order. In PARENT coordinates:
    // the child stores the `rect` relative to it.
    let (mut left, mut top) = (0.0f32, 0.0f32);
    let (mut right, mut bottom) = (area.w, area.h);
    let mut client: Option<(String, Edges)> = None;

    for id in &node.children {
        let Some(n) = doc.node(id) else { continue };
        let Some(r) = pass.rect(id) else { continue };
        // The switch already comes folded inside the read: without
        // `AlignWithMargins` this is zero and the calculation below is the usual one, byte
        // for byte. An `if` here would be the same rule written twice.
        let m = Margins::effective(n);

        // `.max(0.0)` in all four: margin may be larger than the remaining
        // area, and negative width is not "small" — it is a rectangle that
        // `Rect::intersects` of marquee selection reads backwards.
        //
        // **Reading backwards makes the component DISAPPEAR from marquee, not appear
        // too much.** Measured on 2026-09-09, scanning 6912 marquees against an
        // inverted rectangle: NONE starts hitting because of inversion, and
        // 5472 cease to hit. The inverted condition is more restrictive, not more
        // loose — the marquee would have to cover the entire inverted strip.
        //
        // The swapped reason is corrected where it resided: in the comment of
        // `margin_larger_than_area_does_not_produce_negative_rectangle`, in
        // `layout/tests.rs`, which is what tests this line.
        let new_rect = match Align::from_value(n.props.get("align")) {
            Align::None => continue,
            Align::Top => {
                let nr = Rect::new(
                    left + m.left,
                    top + m.top,
                    (right - left - m.left - m.right).max(0.0),
                    r.h,
                );
                top += m.top + r.h + m.bottom;
                nr
            }
            Align::Bottom => {
                let nr = Rect::new(
                    left + m.left,
                    bottom - m.bottom - r.h,
                    (right - left - m.left - m.right).max(0.0),
                    r.h,
                );
                bottom -= m.top + r.h + m.bottom;
                nr
            }
            Align::Left => {
                let nr = Rect::new(
                    left + m.left,
                    top + m.top,
                    r.w,
                    (bottom - top - m.top - m.bottom).max(0.0),
                );
                left += m.left + r.w + m.right;
                nr
            }
            Align::Right => {
                let nr = Rect::new(
                    right - m.right - r.w,
                    top + m.top,
                    r.w,
                    (bottom - top - m.top - m.bottom).max(0.0),
                );
                right -= m.left + r.w + m.right;
                nr
            }
            Align::Client => {
                // client is saved for the end: it takes what remains of all
                client = Some((id.clone(), m));
                continue;
            }
        };

        pass.set(id, new_rect);
    }

    if let Some((id, m)) = client {
        let new_rect = Rect::new(
            left + m.left,
            top + m.top,
            (right - left - m.left - m.right).max(0.0),
            (bottom - top - m.top - m.bottom).max(0.0),
        );
        pass.set(&id, new_rect);
    }
}

#[note("layout-is-fixed")]
pub fn is_fixed(doc: &Document, id: &str) -> bool {
    doc.node(id)
        .map(|n| Align::from_value(n.props.get("align")).is_fixed())
        .unwrap_or(false)
}

// ---------------------------------------------------------------- anchors

// **THE TYPE LIVES IN THE BOX**, and this here is only what the IDE adds.
//
// Until 2026-09-18 there were TWO `Anchors`: this one, with fields in Portuguese
// (`esquerda`, `topo`, `direita`, `base`), and the one that the generator wrote inside the
// user's project, with fields in English. Same concept, two types, two
// languages — the scar that the house rule calls "one concept with two
// names".
//
// And the CALCULATION was written twice as well. The two were equalized by hand on
// 2026-09-09, after diverging: the generator's branch added `dw / 2.0` and
// threw the component 50 pt to the side. Today it is one, in the box, and both
// sides call it.
pub use crawrad_component::{Anchors, anchored};
use crawrad_note::note;

#[note("layout-anchoring")]
pub trait Anchoring: Sized {
    #[note("layout-anchoring-read")]
    fn read(v: Option<&Value>) -> Self;

    #[note("layout-anchoring-write")]
    fn write(self) -> Value;

    #[note("layout-anchoring-adjust")]
    fn adjust(self, r: Rect, dw: f32, dh: f32) -> Rect;
}

impl Anchoring for Anchors {
    fn read(v: Option<&Value>) -> Self {
        let Some(Value::Map(m)) = v else {
            return Self::default();
        };
        let b = |k: &str, default_val: bool| match m.get(k) {
            Some(Value::Bool(x)) => *x,
            _ => default_val,
        };
        Self {
            left: b("left", true),
            top: b("top", true),
            right: b("right", false),
            bottom: b("bottom", false),
        }
    }

    fn write(self) -> Value {
        Value::Map(
            [
                ("left".to_string(), Value::Bool(self.left)),
                ("top".to_string(), Value::Bool(self.top)),
                ("right".to_string(), Value::Bool(self.right)),
                ("bottom".to_string(), Value::Bool(self.bottom)),
            ]
            .into_iter()
            .collect(),
        )
    }

    fn adjust(self, r: Rect, dw: f32, dh: f32) -> Rect {
        let (x, y, w, h) = anchored(r.x, r.y, r.w, r.h, self, dw, dh);
        Rect::new(x, y, w, h)
    }
}

#[note("layout-apply-anchors")]
pub fn apply_anchors(doc: &Document, dw: f32, dh: f32) -> Option<Transaction> {
    if dw == 0.0 && dh == 0.0 {
        return None;
    }
    let root = doc.node(&doc.root)?;
    let mut tx = Transaction::user("Anchors");
    for id in &root.children {
        let Some(n) = doc.node(id) else { continue };
        if Align::from_value(n.props.get("align")).is_fixed() {
            continue;
        }
        let Some(r) = n.rect() else { continue };
        let new_rect = Anchors::read(n.props.get("anchors")).adjust(r, dw, dh);
        if new_rect != r {
            tx.push(Command::SetRect {
                node: id.clone(),
                rect: new_rect,
            });
        }
    }
    (!tx.is_empty()).then_some(tx)
}

#[cfg(test)]
#[path = "test/layout/test.rs"]
mod tests;
