//! # crawrad-runtime — the boundary between the document and what renders it
//!
//! **The single boundary.** Everything that knows how to render enters through here: RCL (Win32),
//! the future RXP (egui), and the test mock backend. Nothing above this trait
//! knows HWND, widget, or device pixel.
//!
//! This is what keeps the decision of ADR 0002 alive:
//! swapping or adding a runtime does not touch the IR, the document, the designer, or
//! the future RTA.

// This crate has no `unsafe`, and the compiler now guarantees it. CrawRAD
// does not have A SINGLE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and this boundary is just as
// important as the one in ADR 0002.
#![forbid(unsafe_code)]

pub mod base;
pub mod db_url;
pub mod dialogs;
pub mod file_dialog;
pub mod form;
pub mod grid;
pub mod link;
pub mod list_selection;
// The EditMask engine and the DateTimePicker values live in
// `crawrad-ir`, where the generators read them too; the generated apps keep
// their `crawrad_runtime::` paths.
pub use crawrad_ir::{date_time, mask};
pub mod pieces;
pub mod shell;
pub mod startup;
pub mod string_grid;
pub mod tree;
pub use base::{ComponentBase, VisualControlBase, WindowControlBase};
pub use dialogs::*;
pub use form::{Form, FormBase};
pub use startup::report_startup_failure;

/// Idiomatic `crawrad::...` alias for runtime routine calls.
pub mod crawrad {
    pub use crate::dialogs::*;
}

pub mod prelude {
    pub use crate::SpinEditHooks;
    pub use crate::base::{ComponentBase, VisualControlBase, WindowControlBase};
    pub use crate::dialogs::{
        DialogButtons, DialogKind, PendingAnswer, active_dialog, is_dialog_open, message_dlg,
        resolve_dialog, show_confirm, show_confirm_sync, show_error, show_message, show_warning,
    };
    pub use crate::form::{Form, FormBase};
    pub use crawrad_component::pieces::{
        Anchors, Collection, Color, Cursor, DateTime, Edges, FilePath, Font, Key, KeyCode,
        ModalResult, Picture, Shortcut, Strings, anchored,
    };
}

/// Behaviour hooks for an external component that inherits from `SpinEdit`.
pub trait SpinEditHooks {
    /// Formats the integer `value` for display in the editor.
    fn display(&self, value: i64) -> String {
        value.to_string()
    }

    /// Parses the typed `text` back into an integer value, or `None` to keep the current value.
    fn parse(&self, text: &str) -> Option<i64> {
        text.trim().parse().ok()
    }

    /// Called after the value changes, before the user's `on_change` handler.
    fn changed(&mut self, value: i64) {
        let _ = value;
    }
}

use crawrad_ir::{Document, NodeId, Value};
use crawrad_note::note;
use std::collections::BTreeSet;

#[note("viewportpx")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewportPx {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[note("scale")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale(pub f32);

impl Scale {
    pub fn to_px(self, v: f32) -> i32 {
        (v * self.0).round() as i32
    }
    pub fn to_design(self, px: i32) -> f32 {
        px as f32 / self.0
    }
}

#[note("mode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Design,
    Live,
}

impl Mode {
    pub fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }
}

#[note("guidekind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideKind {
    /// Border or center of the component (amber).
    Component,
    /// Text line inside the component (pink). This is what makes a Label stay
    /// typographically aligned with an Edit that is taller than it.
    Text,
    /// The gap between two components hit the default margin (hatched blue).
    Spacing,
}

/// What a form-check mark on the canvas says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinKind {
    Error,
    Warning,
    /// No pin: the control's Tab-order badge turns orange (a tied TabOrder).
    TabClash,
}

/// A form-check mark: the finding's number on its control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingPin {
    pub node: NodeId,
    pub number: usize,
    pub kind: PinKind,
}

#[note("guideline")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GuideLine {
    pub vertical: bool,
    pub pos: f32,
    pub from: f32,
    pub to: f32,
    pub kind: GuideKind,
    /// Zero for a line. Greater than zero renders a rectangle — this is how the
    /// spacing gap is hatched.
    pub thickness: f32,
}

#[note("runtimeevent")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuntimeEvent {
    PointerDown {
        x: f32,
        y: f32,
        ctrl: bool,
    },
    PointerMove {
        x: f32,
        y: f32,
        /// `Alt` held: place freely, off the grid.
        free: bool,
    },
    PointerUp {
        x: f32,
        y: f32,
    },
    PointerDblClick {
        x: f32,
        y: f32,
    },
    /// The user requested to cancel the drag (Esc).
    Cancel,
}

#[note("runtimeerror")]
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("runtime not started")]
    NotStarted,
    #[error("component '{0}' is not supported by this runtime")]
    Unsupported(String),
    #[error("{0}")]
    Backend(String),
}

#[note("axis")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// ↘↖ — top-left and bottom-right corners.
    Nwse,
    /// ↙↗ — top-right and bottom-left corners.
    Nesw,
    /// ↔ — left and right sides.
    We,
    /// ↕ — top and bottom.
    Ns,
}
#[note("runtime")]
pub trait Runtime {
    #[note("runtime-target")]
    fn target(&self) -> &'static str;

    #[note("runtime-classes")]
    fn classes(&self) -> &'static [&'static str];

    #[note("runtime-open")]
    fn open(&mut self, doc: &Document) -> Result<(), RuntimeError>;

    #[note("runtime-update")]
    fn update(&mut self, doc: &Document, touched: &BTreeSet<NodeId>) -> Result<(), RuntimeError>;

    #[note("runtime-set-viewport")]
    fn set_viewport(&mut self, vp: ViewportPx, scale: Scale);

    #[note("runtime-set-selection")]
    fn set_selection(&mut self, sel: &[NodeId]);

    #[note("runtime-set-marquee")]
    fn set_marquee(&mut self, rect: Option<crawrad_ir::Rect>);

    #[note("runtime-set-badges")]
    fn set_badges(&mut self, badges: &[(NodeId, String)]) {
        let _ = badges;
    }

    /// The form checker's marks.
    fn set_findings(&mut self, pins: &[FindingPin]) {
        let _ = pins;
    }

    #[note("runtime-set-inherited")]
    fn set_inherited(&mut self, inherited: &[NodeId]) {
        let _ = inherited;
    }

    #[note("runtime-set-grid")]
    fn set_grid(&mut self, grid: f32) {
        let _ = grid;
    }

    #[note("runtime-set-zoom")]
    fn set_zoom(&mut self, zoom: f32) {
        let _ = zoom;
    }

    #[note("runtime-set-mode")]
    fn set_mode(&mut self, mode: Mode) {
        let _ = mode;
    }

    #[note("runtime-set-guides")]
    fn set_guides(&mut self, guides: &[GuideLine]);

    #[note("runtime-busy")]
    fn busy(&self) -> bool {
        false
    }

    #[note("runtime-frame")]
    fn frame(&mut self) {}

    #[note("runtime-drain-events")]
    fn drain_events(&mut self) -> Vec<RuntimeEvent>;

    #[note("runtime-close")]
    fn close(&mut self);
}

// ---------------------------------------------------------------------------
// What a surface displays
// ---------------------------------------------------------------------------

#[note("on-surface")]
pub fn on_surface(doc: &Document, id: &str, is_visual: impl Fn(&str) -> bool) -> bool {
    let inside_non_visual = doc
        .ancestors(id)
        .iter()
        .any(|a| doc.node(a).is_some_and(|n| !is_visual(&n.class)));
    !inside_non_visual && !in_hidden_page(doc, id) && !in_collapsed_category(doc, id)
}

#[note("in-collapsed-category")]
pub fn in_collapsed_category(doc: &Document, id: &str) -> bool {
    doc.ancestors(id).iter().any(|a| {
        doc.node(a).is_some_and(|n| {
            n.class == "CategoryPanel"
                && matches!(
                    n.props.get("collapsed"),
                    Some(crawrad_ir::Value::Bool(true))
                )
        })
    })
}

#[note("derived-geometry")]
pub const DERIVED_GEOMETRY: &[&str] = &["TabSheet"];

#[note("in-hidden-page")]
pub fn in_hidden_page(doc: &Document, id: &str) -> bool {
    for node in std::iter::once(id).chain(doc.ancestors(id).iter().map(|s| s.as_str())) {
        if doc.node(node).is_some_and(|n| n.class == "TabSheet") && !is_active_page(doc, node) {
            return true;
        }
    }
    false
}

#[note("is-active-page")]
fn is_active_page(doc: &Document, page: &str) -> bool {
    let Some(parent) = doc.parent_of(page) else {
        return true;
    };
    let Some(pc) = doc.node(parent) else {
        return true;
    };
    if pc.class != "PageControl" {
        return true;
    }
    // The count is only among PAGES: `ActivePageIndex` counts pages, not
    // children, and a malformed document might have something else in there.
    let pages: Vec<&NodeId> = pc
        .children
        .iter()
        .filter(|c| doc.node(c).is_some_and(|n| n.class == "TabSheet"))
        .collect();
    let active = match pc.props.get("active_page") {
        Some(Value::Int(i)) if *i >= 0 => *i as usize,
        _ => 0,
    };
    match pages.get(active) {
        Some(c) => **c == page,
        // Out-of-bounds index: the classic RAD IDE displays the first. Better an open
        // page than an empty control that looks broken.
        None => pages.first().is_some_and(|c| **c == page),
    }
}

// ---------------------------------------------------------------------------
// Selection handles
// ---------------------------------------------------------------------------

#[note("handle-side")]
pub const HANDLE_SIDE: f32 = 7.0;

#[note("handle-target")]
pub const HANDLE_TARGET: f32 = HANDLE_SIDE * 2.0;

#[note("handle-outset")]
pub const HANDLE_OUTSET: f32 = HANDLE_SIDE / 2.0;

#[note("handles")]
pub fn handles(x: f32, y: f32, w: f32, h: f32) -> [(f32, f32); 8] {
    let f = HANDLE_OUTSET;
    let (cx, cy) = (x + w / 2.0, y + h / 2.0);
    let (l, r) = (x - f, x + w + f);
    let (t, b) = (y - f, y + h + f);
    [
        (l, t),  // NW
        (cx, t), // N
        (r, t),  // NE
        (r, cy), // E
        (r, b),  // SE
        (cx, b), // S
        (l, b),  // SW
        (l, cy), // W
    ]
}

// ---------------------------------------------------------------------------
// Canvas zoom
// ---------------------------------------------------------------------------

#[note("zooms")]
pub const ZOOMS: [f32; 8] = [0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0];

#[note("zoom-one")]
pub const ZOOM_ONE: f32 = 1.0;

#[note("adjacent-zoom")]
pub fn adjacent_zoom(current: f32, zoom_in: bool) -> f32 {
    if zoom_in {
        ZOOMS
            .iter()
            .copied()
            .find(|z| *z > current + f32::EPSILON)
            .unwrap_or(ZOOMS[ZOOMS.len() - 1])
    } else {
        ZOOMS
            .iter()
            .copied()
            .rev()
            .find(|z| *z < current - f32::EPSILON)
            .unwrap_or(ZOOMS[0])
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "test/dialogs/test.rs"]
mod test_dialogs;

#[cfg(test)]
#[path = "test/startup/test.rs"]
mod test_startup;
