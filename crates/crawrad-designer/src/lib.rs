//! # crawrad-designer — the designer logic, without any window
//!
//! Hit-testing, selection, dragging, handle resizing, grid, and snap. Receives
//! pointer in **design units**, inspects the [`Document`] and returns
//! [`Transaction`] for [`crawrad_doc::Doc`] to apply.
//!
//! Knows nothing about HWND, egui, pixels or the operating system — which is why
//! it is testable in a standard unit test, and that is what is guaranteed down below.

// This crate has no `unsafe`, and the compiler now guarantees this. CrawRAD
// has not ONE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and this boundary is as
// important as that of ADR 0002.
#![forbid(unsafe_code)]

pub mod arrangement;
pub mod clipboard;
pub mod form_check;
pub mod form_check_panel;
pub mod image_list_model;
pub mod inspector_icon;
pub mod layout;
pub mod lines_editor_model;
pub mod palette;
pub mod size_preview;
pub mod translations;

use crawrad_doc::{Command, Transaction};
use crawrad_ir::{Document, NodeId, Rect};
use crawrad_registry::Registry;
pub use crawrad_runtime::GuideKind;

#[note("marquee-threshold")]
const MARQUEE_THRESHOLD: f32 = 4.0;

#[note("guide-tolerance")]
const GUIDE_TOLERANCE: f32 = 5.0;

#[note("locked-warning")]
pub const LOCKED_WARNING: &str = "controls are locked — Edit ▸ Lock controls to unlock";

/// A guideline during dragging — the stroke that the classic IDE shows
/// when one component aligns with another. It is the same type that the runtime
/// trait receives, so that there is no conversion along the way.
pub use crawrad_runtime::GuideLine as Guide;

use crawrad_note::note;
/// The axis of a handle — from `crawrad-runtime`, so that the adapter can consume it
/// without depending on the designer.
pub use crawrad_runtime::Axis;

#[note("handle")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handle {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

impl Handle {
    pub const ALL: [Handle; 8] = [
        Handle::NW,
        Handle::N,
        Handle::NE,
        Handle::E,
        Handle::SE,
        Handle::S,
        Handle::SW,
        Handle::W,
    ];

    fn moves_west(self) -> bool {
        matches!(self, Self::NW | Self::W | Self::SW)
    }
    fn moves_east(self) -> bool {
        matches!(self, Self::NE | Self::E | Self::SE)
    }
    fn moves_north(self) -> bool {
        matches!(self, Self::NW | Self::N | Self::NE)
    }
    fn moves_south(self) -> bool {
        matches!(self, Self::SW | Self::S | Self::SE)
    }

    #[note("handle-axis")]
    pub fn axis(self) -> Axis {
        match self {
            Self::NW | Self::SE => Axis::Nwse,
            Self::NE | Self::SW => Axis::Nesw,
            Self::W | Self::E => Axis::We,
            Self::N | Self::S => Axis::Ns,
        }
    }

    #[note("handle-anchor")]
    pub fn anchor(self, r: Rect) -> (f32, f32) {
        // The geometry comes from the BOUNDARY, not from here. The adapter draws
        // the handle and this file looks for the click: two places, one single
        // number. They used to be two numbers that matched by luck.
        crawrad_runtime::handles(r.x, r.y, r.w, r.h)[self.index()]
    }

    #[note("handle-index")]
    fn index(self) -> usize {
        match self {
            Self::NW => 0,
            Self::N => 1,
            Self::NE => 2,
            Self::E => 3,
            Self::SE => 4,
            Self::S => 5,
            Self::SW => 6,
            Self::W => 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Drag {
    Move {
        origin: (f32, f32),
        start: Vec<(NodeId, Rect)>,
    },
    Resize {
        handle: Handle,
        origin: (f32, f32),
        node: NodeId,
        start: Rect,
    },
    /// Marquee selection over the form background.
    Marquee {
        origin: (f32, f32),
        current: (f32, f32),
        /// Ctrl pressed: adds to selection instead of replacing.
        add: bool,
        base: Vec<NodeId>,
    },
}

#[note("arrownudge")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowNudge {
    /// Plain arrow: moves one grid step.
    Move,
    /// `Ctrl`: moves ONE unit — the fine adjustment, and the only way to leave the
    /// grid using the keyboard.
    MoveFine,
    /// `Ctrl+Shift`: moves [`FAST_STEP`] grid steps.
    MoveFast,
    /// `Shift`: resizes one grid step, via the bottom-right corner.
    Resize,
}

#[note("fast-step")]
pub const FAST_STEP: f32 = 3.0;

#[note("grid-steps")]
pub const GRID_STEPS: [f32; 6] = [2.0, 4.0, 5.0, 8.0, 10.0, 16.0];

#[note("snap-hint")]
pub const SNAP_HINT: &str = "Snap to grid (hold Alt to place freely)";

#[note("adjacent-grid")]
pub fn adjacent_grid(grid: f32, up: bool) -> f32 {
    let next = if up {
        GRID_STEPS.iter().find(|&&s| s > grid)
    } else {
        GRID_STEPS.iter().rev().find(|&&s| s < grid)
    };
    next.copied().unwrap_or(grid)
}

#[note("snapghost")]
#[derive(Debug, Clone, PartialEq)]
pub struct SnapGhost {
    pub node: NodeId,
    pub dx: f32,
    pub dy: f32,
}

impl SnapGhost {
    #[note("snapghost-rects")]
    pub fn rects(&self, doc: &Document) -> Option<(Rect, Rect)> {
        let l = doc.rect_in_form(&self.node)?;
        Some((l, Rect::new(l.x + self.dx, l.y + self.dy, l.w, l.h)))
    }
}

pub struct Designer {
    /// Component catalog — used only for typographic alignment text
    /// metrics. The designer still does not know how to draw.
    pub registry: Option<std::rc::Rc<Registry>>,
    pub grid: f32,
    pub snap: bool,
    /// `Alt` held: snap is suspended for this gesture, the classic IDE's way
    /// to place a control off the grid. The shell sets it with each move.
    pub free: bool,
    /// Gesture counter, to stamp transactions of a drag.
    ///
    /// It is not state that anyone queries: it exists only so that two drags
    /// never carry the same number. Starts at zero and the first gesture is 1, so
    /// that "no gesture yet" does not look like a gesture.
    gesture: u64,
    /// Handle size in design units (used in hit-testing and drawing).
    pub handle_size: f32,
    /// Minimum size of a component.
    pub min_size: f32,
    /// The `Edit ▸ Lock Controls` of the classic IDE: a switch of the DESIGNER, not
    /// a component property.
    ///
    /// That is why it lives here and not in the [`Document`]: none of this enters
    /// the `.crawform` nor the generated code. It is protection for whoever is editing, and
    /// the form saved by a locked IDE must be byte-for-byte identical to the one
    /// saved by an unlocked one.
    ///
    /// **Does not survive closing the IDE**, like `active_rail` and the center tab
    /// in the shell — and here the reason is stronger than "persistence has not been
    /// designed yet": a switch that turns off dragging and comes back turned on in
    /// a session where nobody remembers having turned it on is indistinguishable from
    /// a broken IDE.
    pub locked: bool,
    drag: Option<Drag>,
    /// True between `pointer_down` and the first useful movement: prevents
    /// recording drag transactions when it was just a click.
    moved: bool,
}

#[note("tabaction")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabAction {
    /// Switch the active page.
    Open { control: NodeId, page: usize },
    /// Create a new page, via the "+" at the end of the strip.
    Create { control: NodeId },
}

impl Designer {
    #[note("designer-with-registry")]
    pub fn with_registry(registry: std::rc::Rc<Registry>) -> Self {
        Self {
            registry: Some(registry),
            ..Default::default()
        }
    }
}

impl Default for Designer {
    fn default() -> Self {
        Self {
            registry: None,
            grid: 8.0,
            snap: true,
            free: false,
            handle_size: crawrad_runtime::HANDLE_TARGET / 2.0,
            min_size: 8.0,
            locked: false,
            gesture: 0,
            drag: None,
            moved: false,
        }
    }
}

#[note("outcome")]
#[derive(Debug, Default)]
pub struct Outcome {
    /// New selection, when the click changed the selection.
    pub select: Option<Vec<NodeId>>,
    /// Transaction to apply to the document.
    pub tx: Option<Transaction>,

    /// Dragging in progress (the IDE may want to show guidelines, cursor…).
    pub dragging: bool,
    /// Rectangle of the marquee selection, for the runtime to draw. `None` when
    /// there is no marquee — and the `Some` disappears on its own on `pointer_up`.
    pub marquee: Option<Rect>,
    /// Status bar text — diagnostic of the gesture.
    pub status: Option<String>,
    /// Guidelines to draw while dragging happens. Empty
    /// means "clear whichever are on screen".
    pub guides: Vec<Guide>,
    /// While a move snaps: the anchor and how far the pointer is from where it
    /// lands. `None` when nothing snapped (Snap off, `Alt`, no grid).
    pub ghost: Option<SnapGhost>,
}

impl Designer {
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }

    #[note("designer-snapping")]
    pub fn snapping(&self) -> bool {
        self.snap && !self.free && self.grid > 0.0
    }

    fn snap(&self, v: f32) -> f32 {
        if self.snapping() {
            (v / self.grid).round() * self.grid
        } else {
            v
        }
    }

    #[note("designer-locked-for")]
    fn locked_for(&self, doc: &Document, id: &NodeId) -> bool {
        self.locked && id != &doc.root
    }

    #[note("designer-refuses")]
    pub fn refuses(&self, doc: &Document, tx: &Transaction) -> bool {
        tx.commands.iter().any(|c| match c {
            Command::SetRect { node, .. } => self.locked_for(doc, node),
            // Carries rectangle AND parent: this is the drag that drops the control inside
            // a container (ADR 0012). It moves, therefore it is refused.
            Command::Reparent { node, .. } => self.locked_for(doc, node),
            // Nothing here is geometry chosen by the gesture. `SetProp` is the
            // Inspector; `AddNode` is the palette (in the classic IDE you continue
            // dropping components with the lock on — what it prevents is
            // MOVING what is already placed); `RemoveNode` is Delete, which the
            // classic IDE also does not lock; `Reorder` is Z-order and tab order, which
            // do not touch the rectangle; `SetEvent` and `Rename` are code and name.
            Command::SetProp { .. }
            | Command::AddNode { .. }
            | Command::RemoveNode { .. }
            | Command::Reorder { .. }
            | Command::SetEvent { .. }
            | Command::SetBinding { .. }
            | Command::Rename { .. } => false,
        })
    }

    #[note("designer-gesture-permitted")]
    fn gesture_permitted(&self, doc: &Document, d: &Drag) -> bool {
        match d {
            Drag::Move { start, .. } => start.iter().all(|(id, _)| !self.locked_for(doc, id)),
            Drag::Resize { node, .. } => !self.locked_for(doc, node),
            // The marquee does not move anything: it selects. Locking it would lock multiple
            // selection, which is precisely what the lock exists to preserve.
            Drag::Marquee { .. } => true,
        }
    }

    #[note("designer-start-drag")]
    fn start_drag(&mut self, doc: &Document, d: Drag) -> bool {
        if !self.gesture_permitted(doc, &d) {
            return false;
        }
        self.drag = Some(d);
        // A new number per gesture, and the only requirement is to be monotonic:
        // `Doc::apply` only compares it against the top of the undo stack. It is here
        // because this is where the gesture begins — whoever knows where it ends is the
        // same port, and so numbering cannot escape from it.
        self.gesture += 1;
        true
    }

    #[note("designer-current-gesture")]
    pub fn current_gesture(&self) -> u64 {
        self.gesture
    }

    #[note("designer-derived-geometry")]
    pub fn derived_geometry(&self, doc: &Document, id: &str) -> bool {
        doc.node(id)
            .and_then(|n| self.registry.as_ref()?.get(&n.class))
            .is_some_and(|d| d.derived_geometry)
    }

    #[note("designer-hit")]
    pub fn hit(&self, doc: &Document, x: f32, y: f32) -> Option<NodeId> {
        let under = |id: &NodeId| {
            self.on_canvas(doc, id)
                && doc
                    .rect_in_form(id)
                    .map(|r| r.contains(x, y))
                    .unwrap_or(false)
        };
        let order = doc.in_paint_order();
        let top = order.iter().rev();
        top.clone()
            .find(|id| !self.is_visual(doc, id) && under(id))
            .or_else(|| top.clone().find(|id| under(id)))
            .cloned()
    }

    #[note("designer-is-visual")]
    fn is_visual(&self, doc: &Document, id: &str) -> bool {
        doc.node(id)
            .and_then(|n| self.registry.as_ref()?.get(&n.class))
            .is_none_or(|d| d.visual)
    }

    #[note("designer-on-canvas")]
    pub fn on_canvas(&self, doc: &Document, id: &str) -> bool {
        crawrad_runtime::on_surface(doc, id, |class| {
            self.registry
                .as_ref()
                .and_then(|r| r.get(class))
                .is_none_or(|d| d.visual)
        }) && !self.drawn_by_its_owner(doc, id)
    }

    #[note("designer-drawn-by-its-owner")]
    fn drawn_by_its_owner(&self, doc: &Document, id: &str) -> bool {
        let (Some(node), Some(reg)) = (doc.node(id), self.registry.as_ref()) else {
            return false;
        };
        doc.parent_of(id)
            .and_then(|p| doc.node(p))
            .and_then(|p| reg.get(&p.class))
            .is_some_and(|owner| owner.visual && owner.item_class() == Some(node.class.as_str()))
    }

    #[note("designer-hit-tab")]
    pub fn hit_tab(&self, doc: &Document, x: f32, y: f32) -> Option<TabAction> {
        let id = self.hit(doc, x, y)?;
        let n = doc.node(&id)?;
        if n.class != "PageControl" {
            return None;
        }
        let r = doc.rect_in_form(&id)?;
        // The strip is at the top, or at the bottom
        let top = r.y + crawrad_registry::tab_strip_top(r.h, crawrad_registry::tabs_at_bottom(n));
        if y < top || y > top + crawrad_registry::TAB_STRIP_HEIGHT {
            return None;
        }
        let pages = doc
            .node(&id)?
            .children
            .iter()
            .filter(|c| doc.node(c).is_some_and(|n| n.class == "TabSheet"))
            .count();
        // The "+" first, because it is on the RIGHT edge and not after the
        // last tab. It used to be there, and with 92pt tab in a 320 control the box
        // of the fifth began outside the control: the `PageControl` was stuck at
        // four pages, since "+" is the only way to create.
        if x >= r.x + r.w - crawrad_registry::PLUS_BUTTON_WIDTH {
            return Some(TabAction::Create { control: id });
        }
        // Out of range falls back to the first page, as the canvas does
        let active = match n.props.get("active_page") {
            Some(crawrad_ir::Value::Int(i)) if *i >= 0 && (*i as usize) < pages => *i as usize,
            _ => 0,
        };
        let w = crawrad_registry::tab_window(r.w, pages, active, true);
        if w.arrows {
            let lx = r.x + crawrad_registry::tab_scroll_left_x(r.w, true);
            let s = crawrad_registry::TAB_SCROLL_BUTTON_WIDTH;
            if x >= lx && x < lx + s {
                return (active > 0).then(|| TabAction::Open {
                    control: id,
                    page: active - 1,
                });
            }
            if x >= lx + s && x < lx + 2.0 * s {
                return (active + 1 < pages).then(|| TabAction::Open {
                    control: id,
                    page: active + 1,
                });
            }
        }
        let slot = ((x - r.x) / crawrad_registry::TAB_WIDTH).floor();
        if slot < 0.0 {
            return None;
        }
        let slot = slot as usize;
        if slot >= w.count {
            return None;
        }
        Some(TabAction::Open {
            control: id,
            page: w.first + slot,
        })
    }

    #[note("designer-hit-form")]
    pub fn hit_form(&self, doc: &Document, x: f32, y: f32) -> bool {
        doc.node(&doc.root)
            .and_then(|n| n.rect())
            .map(|r| r.contains(x, y))
            .unwrap_or(false)
    }

    #[note("designer-hit-form-handle")]
    pub fn hit_form_handle(&self, doc: &Document, x: f32, y: f32) -> Option<Handle> {
        let r = doc.node(&doc.root)?.rect()?;
        let h = self.handle_size;

        let (se_x, se_y) = Handle::SE.anchor(r);
        if (x - se_x).abs() <= h && (y - se_y).abs() <= h {
            return Some(Handle::SE);
        }

        let right_edge = r.x + r.w;
        let (e_x, e_y) = Handle::E.anchor(r);
        let on_e_handle = (x - e_x).abs() <= h && (y - e_y).abs() <= h;
        let on_e_border = (x - right_edge).abs() <= 6.0 && y >= r.y && y <= (r.y + r.h);
        if on_e_handle || on_e_border {
            return Some(Handle::E);
        }

        let bottom_edge = r.y + r.h;
        let (s_x, s_y) = Handle::S.anchor(r);
        let on_s_handle = (x - s_x).abs() <= h && (y - s_y).abs() <= h;
        let on_s_border = (y - bottom_edge).abs() <= 6.0 && x >= r.x && x <= (r.x + r.w);
        if on_s_handle || on_s_border {
            return Some(Handle::S);
        }

        None
    }

    #[note("designer-hit-handle")]
    pub fn hit_handle(&self, doc: &Document, sel: &[NodeId], x: f32, y: f32) -> Option<Handle> {
        let id = sel.first()?;
        if id == &doc.root {
            return None;
        }
        // The FOURTH tree path, and the easiest to forget: handle of
        // a node not on screen is a handle floating alone. Non-visuals
        // do not resize (the ActionList icon has the size it has, as in
        // the classic IDE), and what is in a closed page neither.
        if !self.visual(doc, id) || !self.on_canvas(doc, id) || self.derived_geometry(doc, id) {
            return None;
        }
        // In FORM coordinates: pointer arrives like this, and the node's
        // `rect` is relative to parent. For root children it is the same; inside
        // a panel, comparing both compares different spaces.
        let r = doc.rect_in_form(id)?;
        let h = self.handle_size;
        Handle::ALL.into_iter().find(|hd| {
            let (ax, ay) = hd.anchor(r);
            (x - ax).abs() <= h && (y - ay).abs() <= h
        })
    }

    #[note("designer-visual")]
    pub fn visual(&self, doc: &Document, id: &NodeId) -> bool {
        let Some(reg) = &self.registry else {
            return true;
        };
        doc.node(id)
            .and_then(|n| reg.get(&n.class))
            .map(|d| d.visual)
            .unwrap_or(true)
    }

    pub fn pointer_down(
        &mut self,
        doc: &Document,
        sel: &[NodeId],
        x: f32,
        y: f32,
        ctrl: bool,
    ) -> Outcome {
        self.moved = false;

        // 1. handle of the selected component — and only of COMPONENT: the
        //    form exits through step 2, with the three handles that it has.
        if let Some(h) = self.hit_handle(doc, sel, x, y)
            && let Some(id) = sel.first()
            && let Some(r) = doc.node(id).and_then(|n| n.rect())
        {
            let d = Drag::Resize {
                handle: h,
                origin: (x, y),
                node: id.clone(),
                start: r,
            };
            if self.start_drag(doc, d) {
                return Outcome {
                    dragging: true,
                    ..Default::default()
                };
            }
            // Locked. Exits here anyway, instead of falling through to the
            // hit-test below: **the handle is OUTSIDE the component's rectangle**
            // (which is why it is reachable on a component flush with its neighbor),
            // so clicking on it finds nothing and would open a marquee — which on
            // `pointer_up` would erase the selection the user had.
            return Outcome {
                status: Some(LOCKED_WARNING.to_string()),
                ..Default::default()
            };
        }

        // 2. the three form handles — E, S and SE —, **without requiring that the
        //    form be selected**.
        //
        // The condition used to exist, and that was the defect: pointing to the border of
        // the form without having selected it first resized nothing. The
        // click passed through to step 3, and as the handle lives 3.5 units
        // OUTSIDE the border, it fell into the void and opened a marquee
        // starting outside the form — with the cursor, which the adapter
        // already painted as resize arrow here, promising the opposite.
        // In the classic IDE you grab the form border and drag; nobody
        // selects the window before changing its size.
        //
        // **After step 1, and this order is the point.** Every control
        // flush with the right or bottom border has a handle at the same point as that
        // of the form. Testing the form first would take away from the editor the
        // resizing of any component touching the margin — swapping one defect
        // for another. Selected wins, and it is the same order that the
        // adapter already uses to pick the cursor arrow: the component's handle
        // is drawn on top.
        if let Some(h) = self.hit_form_handle(doc, x, y)
            && let Some(r) = doc.node(&doc.root).and_then(|n| n.rect())
        {
            let d = Drag::Resize {
                handle: h,
                origin: (x, y),
                node: doc.root.clone(),
                start: r,
            };
            if self.start_drag(doc, d) {
                // Changing the form's size transfers selection to it,
                // as in the classic IDE — the Object Inspector shows the Form. And there is
                // the internal reason: the handles the hand is holding are
                // the form's, so leaving the selection on a component
                // would mean the screen drawing the handles of one object while the
                // gesture modifies another.
                let already_it = sel.len() == 1 && sel.first() == Some(&doc.root);
                return Outcome {
                    select: (!already_it).then(|| vec![doc.root.clone()]),
                    dragging: true,
                    ..Default::default()
                };
            }
        }

        // 3. component under the cursor
        match self.hit(doc, x, y) {
            None => {
                // Form background: starts the marquee, and the dry click that does not
                // turn into a marquee selects the form itself.
                let base: Vec<NodeId> = if ctrl { sel.to_vec() } else { Vec::new() };
                self.start_drag(
                    doc,
                    Drag::Marquee {
                        origin: (x, y),
                        current: (x, y),
                        add: ctrl,
                        base: base.clone(),
                    },
                );
                let select_form =
                    !ctrl && self.hit_form(doc, x, y) && sel.first() != Some(&doc.root);
                Outcome {
                    select: if select_form {
                        Some(vec![doc.root.clone()])
                    } else {
                        (!ctrl && !sel.is_empty()).then(Vec::new)
                    },
                    dragging: true,
                    ..Default::default()
                }
            }
            Some(id) => {
                let fixed = layout::is_fixed(doc, &id);
                let mut new_sel: Vec<NodeId> = sel.to_vec();
                if ctrl {
                    // Ctrl toggles, as in the classic IDE
                    if let Some(p) = new_sel.iter().position(|s| s == &id) {
                        new_sel.remove(p);
                    } else {
                        new_sel.push(id.clone());
                    }
                } else if !new_sel.contains(&id) {
                    new_sel = vec![id.clone()];
                }

                // A component with Align is positioned by layout: dragging
                // cannot move it, as in the classic IDE.
                let start: Vec<(NodeId, Rect)> = new_sel
                    .iter()
                    // `is_fixed` is `Align` locking movement; derived geometry
                    // is the container imposing the rectangle. Different reasons,
                    // same conclusion: does not drag.
                    .filter(|i| !layout::is_fixed(doc, i) && !self.derived_geometry(doc, i))
                    .filter_map(|i| doc.node(i).and_then(|n| n.rect()).map(|r| (i.clone(), r)))
                    .collect();
                // The gesture starts, or the lock refuses it — and refusing IS the result
                // that the bar must report. Notice that the selection emerges from here
                // either way: locked, the click continues selecting, which
                // keeps the Object Inspector editing the component.
                let refused = !start.is_empty()
                    && !self.start_drag(
                        doc,
                        Drag::Move {
                            origin: (x, y),
                            start,
                        },
                    );
                let _ = fixed;

                Outcome {
                    select: (new_sel != sel).then_some(new_sel),
                    // Refused, nothing is being dragged — and whoever reads this
                    // field to decide whether to draw guidelines or switch cursors
                    // would draw the interface of a non-existent gesture.
                    dragging: !refused,
                    status: refused.then(|| LOCKED_WARNING.to_string()),
                    ..Default::default()
                }
            }
        }
    }

    #[note("designer-pointer-move")]
    pub fn pointer_move(&mut self, doc: &Document, x: f32, y: f32) -> Outcome {
        let _doc = doc;
        // The marquee does not modify the document: only grows and asks for drawing.
        if let Some(Drag::Marquee {
            origin, current, ..
        }) = &mut self.drag
        {
            *current = (x, y);
            let r = Rect::from_points(origin.0, origin.1, x, y);
            // only turns into a real marquee after moving further than the threshold; until then
            // it is a click, and a click can never select the entire screen
            if r.w >= MARQUEE_THRESHOLD || r.h >= MARQUEE_THRESHOLD {
                self.moved = true;
            }
            return Outcome {
                marquee: Some(r),
                dragging: true,
                ..Default::default()
            };
        }

        let Some(drag) = &self.drag else {
            return Outcome::default();
        };

        let tx = match drag {
            Drag::Move { origin, start } => {
                let (dx, dy) = (x - origin.0, y - origin.1);
                if !self.moved && dx.abs() < 1.0 && dy.abs() < 1.0 {
                    return Outcome {
                        dragging: true,
                        ..Default::default()
                    };
                }

                // The anchor of the gesture is the first of the selection: from it spring
                // guidelines, and the adjustment applies to the whole group.
                let moved_nodes: Vec<NodeId> = start.iter().map(|(i, _)| i.clone()).collect();
                let (adjust_x, adjust_y, guidelines) = match start.first() {
                    Some((_, r0)) => {
                        let candidate =
                            Rect::new(self.snap(r0.x + dx), self.snap(r0.y + dy), r0.w, r0.h);
                        self.align_to_neighbors(
                            doc,
                            self.registry.as_deref(),
                            &moved_nodes,
                            candidate,
                        )
                    }
                    None => (0.0, 0.0, Vec::new()),
                };

                let mut tx = Transaction::user("Move").no_gesto(self.current_gesture());
                for (id, r0) in start {
                    let nx = self.snap(r0.x + dx) + adjust_x;
                    let ny = self.snap(r0.y + dy) + adjust_y;
                    tx.push(Command::SetRect {
                        node: id.clone(),
                        rect: Rect::new(nx, ny, r0.w, r0.h),
                    });
                }
                let ghost = start
                    .first()
                    .filter(|_| self.snapping())
                    .map(|(id, r0)| SnapGhost {
                        node: id.clone(),
                        dx: r0.x + dx - (self.snap(r0.x + dx) + adjust_x),
                        dy: r0.y + dy - (self.snap(r0.y + dy) + adjust_y),
                    });
                self.moved = true;
                return Outcome {
                    tx: Some(tx),
                    dragging: true,
                    guides: guidelines,
                    ghost,
                    ..Default::default()
                };
            }
            Drag::Resize {
                handle,
                origin,
                node,
                start,
            } => {
                let (dx, dy) = (x - origin.0, y - origin.1);
                let mut r = *start;
                if handle.moves_west() {
                    let nx = self.snap(start.x + dx);
                    r.w = (start.x + start.w - nx).max(self.min_size);
                    r.x = start.x + start.w - r.w;
                }
                if handle.moves_east() {
                    r.w = self.snap(start.w + dx).max(self.min_size);
                }
                if handle.moves_north() {
                    let ny = self.snap(start.y + dy);
                    r.h = (start.y + start.h - ny).max(self.min_size);
                    r.y = start.y + start.h - r.h;
                }
                if handle.moves_south() {
                    r.h = self.snap(start.h + dy).max(self.min_size);
                }
                if r == *start {
                    return Outcome {
                        dragging: true,
                        ..Default::default()
                    };
                }
                // The form has fixed origin: only changes size.
                if node == &_doc.root {
                    r.x = start.x;
                    r.y = start.y;
                }
                Transaction::user("Resize")
                    .no_gesto(self.current_gesture())
                    .with(Command::SetRect {
                        node: node.clone(),
                        rect: r,
                    })
            }
            // handled at the beginning of the function
            Drag::Marquee { .. } => return Outcome::default(),
        };

        self.moved = true;
        Outcome {
            tx: Some(tx),
            dragging: true,
            ..Default::default()
        }
    }

    #[note("designer-reparent-dragged")]
    fn reparent_dragged(&self, doc: &Document, start: &[(NodeId, Rect)]) -> Option<Transaction> {
        let mut tx = Transaction::user("Move into container");
        for (id, _) in start {
            let current = doc.rect_in_form(id)?;
            let (parent, rx, ry) = self.parent_for_drop(doc, current.x, current.y);
            // The component itself, or something inside it, cannot become the parent.
            if parent == *id || doc.ancestors(&parent).contains(id) {
                continue;
            }
            let current_parent = doc
                .parent_of(id)
                .cloned()
                .unwrap_or_else(|| doc.root.clone());
            if parent == current_parent {
                continue;
            }
            let index = doc.node(&parent).map(|n| n.children.len()).unwrap_or(0);
            tx.push(Command::Reparent {
                node: id.clone(),
                parent,
                index,
                rect: Rect::new(rx, ry, current.w, current.h),
            });
        }
        (!tx.is_empty()).then_some(tx)
    }

    #[note("designer-pointer-up")]
    pub fn pointer_up(&mut self, doc: &Document) -> Outcome {
        let did_drag = self.moved;
        self.moved = false;
        let drag = self.drag.take();

        // Dropping over a container changes the PARENT, not only position.
        if let Some(Drag::Move { start, .. }) = &drag
            && did_drag
            && let Some(tx) = self.reparent_dragged(doc, start)
        {
            return Outcome {
                tx: Some(tx),
                ..Default::default()
            };
        }

        let Some(Drag::Marquee {
            origin,
            current,
            add,
            base,
        }) = drag
        else {
            return Outcome::default();
        };

        let marquee_rect = Rect::from_points(origin.0, origin.1, current.0, current.1);
        // Click is not a marquee. Two conditions, and both must hold: there was
        // tracked motion during capture AND the rectangle has size.
        // Without this, a `pointer_up` arriving with the mouse far from origin —
        // lost capture, button released outside window — would become a selection of
        // everything in its path.
        if !did_drag || (marquee_rect.w < MARQUEE_THRESHOLD && marquee_rect.h < MARQUEE_THRESHOLD) {
            return Outcome {
                marquee: Some(Rect::new(0.0, 0.0, 0.0, 0.0)),
                ..Default::default()
            };
        }

        let mut new_sel = base;
        if let Some(root) = doc.node(&doc.root) {
            for id in &root.children {
                let Some(r) = doc.node(id).and_then(|n| n.rect()) else {
                    continue;
                };
                if marquee_rect.intersects(&non_inverted(r)) && !new_sel.contains(id) {
                    new_sel.push(id.clone());
                }
            }
        }
        let _ = add;

        Outcome {
            status: Some(format!(
                "marquee {:.0}x{:.0} at ({:.0},{:.0}) → {} selected",
                marquee_rect.w,
                marquee_rect.h,
                marquee_rect.x,
                marquee_rect.y,
                new_sel.len()
            )),
            select: Some(new_sel),
            // empty rectangle = erases the marquee from the screen
            marquee: Some(Rect::new(0.0, 0.0, 0.0, 0.0)),
            ..Default::default()
        }
    }

    #[note("designer-cancel")]
    pub fn cancel(&mut self) -> Outcome {
        let tx = match self.drag.take() {
            Some(Drag::Move { start, .. }) => {
                // Stamped with the SAME gesture, and that is the less wrong of the two
                // paths. Without the stamp, the stack would hold the drag step
                // plus the cancel step — and a `Ctrl+Z` after
                // an `Esc` would REDO the drag that the user just
                // cancelled. With it, the whole gesture is a single step whose
                // inverse leads to the starting point where the document already is.
                let mut tx = Transaction::user("Cancel drag").no_gesto(self.current_gesture());
                for (id, r) in start {
                    tx.push(Command::SetRect { node: id, rect: r });
                }
                Some(tx)
            }
            Some(Drag::Resize { node, start, .. }) => Some(
                Transaction::user("Cancel resize")
                    .no_gesto(self.current_gesture())
                    .with(Command::SetRect { node, rect: start }),
            ),
            Some(Drag::Marquee { .. }) | None => None,
        };
        self.moved = false;
        Outcome {
            tx,
            marquee: Some(Rect::new(0.0, 0.0, 0.0, 0.0)),
            ..Default::default()
        }
    }

    #[note("designer-nudge")]
    pub fn nudge(&self, doc: &Document, sel: &[NodeId], dx: f32, dy: f32) -> Option<Transaction> {
        if sel.is_empty() {
            return None;
        }
        let mut tx = Transaction::user("Nudge");
        for id in sel {
            // Page does not move: the movement would be undone by normalization
            // on the next turn, leaving two undo entries and no
            // result.
            if self.derived_geometry(doc, id) {
                continue;
            }
            if let Some(r) = doc.node(id).and_then(|n| n.rect()) {
                tx.push(Command::SetRect {
                    node: id.clone(),
                    rect: Rect::new(r.x + dx, r.y + dy, r.w, r.h),
                });
            }
        }
        (!tx.is_empty()).then_some(tx)
    }

    #[note("designer-nudge-size")]
    pub fn nudge_size(
        &self,
        doc: &Document,
        sel: &[NodeId],
        dw: f32,
        dh: f32,
    ) -> Option<Transaction> {
        if sel.is_empty() {
            return None;
        }
        let mut tx = Transaction::user("Resize");
        for id in sel {
            // Same refusal as `nudge`: the page has parent geometry, and changing
            // its would yield an undo entry with no result at all.
            if self.derived_geometry(doc, id) {
                continue;
            }
            if let Some(r) = doc.node(id).and_then(|n| n.rect()) {
                let new_rect = non_inverted(Rect::new(r.x, r.y, r.w + dw, r.h + dh));
                // Was already at the floor and the arrow says shrink: nothing changes, and a
                // transaction that changes nothing is an undo entry that undoes
                // nothing.
                if new_rect.w == r.w && new_rect.h == r.h {
                    continue;
                }
                tx.push(Command::SetRect {
                    node: id.clone(),
                    rect: new_rect,
                });
            }
        }
        (!tx.is_empty()).then_some(tx)
    }

    #[note("designer-arrow-step")]
    pub fn arrow_step(&self, mode: ArrowNudge) -> f32 {
        // With snap TURNED OFF the grid commands nothing, and the arrow moves one
        // by one. It is the same pair of states that the mouse respects, and the classic IDE
        // as well: without `Snap to grid`, positioning is free.
        let grid_val = if self.snap { self.grid.max(1.0) } else { 1.0 };
        match mode {
            ArrowNudge::Move | ArrowNudge::Resize => grid_val,
            ArrowNudge::MoveFine => 1.0,
            ArrowNudge::MoveFast => grid_val * FAST_STEP,
        }
    }

    #[note("designer-align-relations")]
    fn align_relations(
        &self,
        doc: &Document,
        reg: Option<&Registry>,
        moved_id: Option<&NodeId>,
        r: Rect,
        other: &NodeId,
        o: Rect,
        ext: GuideSpan,
    ) -> Vec<(f32, Guide)> {
        let mut out = Vec::new();

        let (from_v, to_v) = match ext {
            GuideSpan::Gap => guide_extent(r.y, r.y + r.h, o.y, o.y + o.h),
            GuideSpan::Cross => (r.y.min(o.y), (r.y + r.h).max(o.y + o.h)),
        };
        let (from_h, to_h) = match ext {
            GuideSpan::Gap => guide_extent(r.x, r.x + r.w, o.x, o.x + o.w),
            GuideSpan::Cross => (r.x.min(o.x), (r.x + r.w).max(o.x + o.w)),
        };

        // edges and centers
        for (c, a) in [
            (r.x, o.x),
            (r.x + r.w / 2.0, o.x + o.w / 2.0),
            (r.x + r.w, o.x + o.w),
            (r.x, o.x + o.w),
            (r.x + r.w, o.x),
        ] {
            out.push((
                a - c,
                Guide {
                    vertical: true,
                    pos: a,
                    from: from_v,
                    to: to_v,
                    kind: GuideKind::Component,
                    thickness: 0.0,
                },
            ));
        }
        for (c, a) in [
            (r.y, o.y),
            (r.y + r.h / 2.0, o.y + o.h / 2.0),
            (r.y + r.h, o.y + o.h),
            (r.y, o.y + o.h),
            (r.y + r.h, o.y),
        ] {
            out.push((
                a - c,
                Guide {
                    vertical: false,
                    pos: a,
                    from: from_h,
                    to: to_h,
                    kind: GuideKind::Component,
                    thickness: 0.0,
                },
            ));
        }

        // text baseline — always crossing both
        if let (Some(mine), Some(theirs)) = (
            moved_id.and_then(|i| text_line(doc, reg, i, r)),
            text_line(doc, reg, other, o),
        ) {
            out.push((
                theirs - mine,
                Guide {
                    vertical: false,
                    pos: theirs,
                    from: r.x.min(o.x),
                    to: (r.x + r.w).max(o.x + o.w),
                    kind: GuideKind::Text,
                    thickness: 0.0,
                },
            ));
        }

        // The gap between neighbors — the classic IDE hatches the strip when you hit it.
        //
        // **The number comes from the component, not from the designer.** It was a field
        // `margin: 8.0`, chosen by eye, and it contradicted the program itself:
        // `Align` with `AlignWithMargins` spaces by the sum of touching
        // margins — 6, with the classic library default of 3 — and the guide
        // promised 8 on the same form. Two answers for "what is the right
        // spacing here" is one too many.
        //
        // The calculation is the SAME as `layout::align_children`: the two margins that
        // **touch**, and not the largest nor that of one alone. Being to its left
        // abuts my right to its left; to the right, the opposite.
        //
        // Here it is [`layout::Margins::from_value`], and not `effective`: the guide's
        // question is "how much space does this control want around itself", which holds
        // with `AlignWithMargins` turned off. `effective` answers another
        // question — "should `Align` honour this" — and using it here would make the
        // guide disappear for most components, which is the default.
        let margins = |id: Option<&NodeId>| {
            id.and_then(|i| doc.node(i))
                .map(|n| layout::Margins::from_value(n.props.get("margins")))
                .unwrap_or(layout::Margins::DEFAULT)
        };
        let mine = margins(moved_id);
        let theirs = margins(Some(other));

        // The adjustment is `gap - margin` times the sign, not `margin - gap`. It was
        // inverted, and nobody knew: `GuideKind::Spacing` did not have a single
        // test — arriving with 8 gap where 6 was right, the component
        // snapped to 10, MOVING AWAY from the spacing that the hatched strip
        // promised. It was found writing the test for gap coming from component.
        //
        // The math, for whoever touches this: `r` to the left of `o` has gap
        // `o.x - (r.x + r.w)`, and moving `r` by `ax` leaves the gap at `gap - ax`;
        // equating to margin yields `ax = gap - margin`. On the other side gap grows
        // with `ax`, and the sign takes care of that.
        let overlaps_y = r.y < o.y + o.h && o.y < r.y + r.h;
        let overlaps_x = r.x < o.x + o.w && o.x < r.x + r.w;
        if overlaps_y {
            let (to_left, to_right) = (mine.right + theirs.left, mine.left + theirs.right);
            for (gap, sign, margin, target_start) in [
                (o.x - (r.x + r.w), 1.0f32, to_left, o.x - to_left),
                (r.x - (o.x + o.w), -1.0f32, to_right, o.x + o.w),
            ] {
                out.push((
                    (gap - margin) * sign,
                    Guide {
                        vertical: true,
                        pos: target_start,
                        from: r.y.max(o.y),
                        to: (r.y + r.h).min(o.y + o.h),
                        kind: GuideKind::Spacing,
                        thickness: margin,
                    },
                ));
            }
        }
        if overlaps_x {
            let (above, below) = (mine.bottom + theirs.top, mine.top + theirs.bottom);
            for (gap, sign, margin, target_start) in [
                (o.y - (r.y + r.h), 1.0f32, above, o.y - above),
                (r.y - (o.y + o.h), -1.0f32, below, o.y + o.h),
            ] {
                out.push((
                    (gap - margin) * sign,
                    Guide {
                        vertical: false,
                        pos: target_start,
                        from: r.x.max(o.x),
                        to: (r.x + r.w).min(o.x + o.w),
                        kind: GuideKind::Spacing,
                        thickness: margin,
                    },
                ));
            }
        }

        out
    }

    #[note("designer-align-to-neighbors")]
    fn align_to_neighbors(
        &self,
        doc: &Document,
        reg: Option<&Registry>,
        moved: &[NodeId],
        r: Rect,
    ) -> (f32, f32, Vec<Guide>) {
        let Some(root) = doc.node(&doc.root) else {
            return (0.0, 0.0, Vec::new());
        };
        let mut best_x: Option<(f32, Guide)> = None;
        let mut best_y: Option<(f32, Guide)> = None;

        for other in &root.children {
            if moved.contains(other) {
                continue;
            }
            let Some(o) = doc.node(other).and_then(|n| n.rect()) else {
                continue;
            };

            for (d, g) in self.align_relations(doc, reg, moved.first(), r, other, o, GuideSpan::Gap)
            {
                if d.abs() > GUIDE_TOLERANCE {
                    continue;
                }
                let target = if g.vertical { &mut best_x } else { &mut best_y };
                // Distance ties are common: in a column, three components
                // share the same left. Then the CLOSEST neighbor wins
                // — otherwise the stroke crosses whoever is in the middle and looks disconnected.
                let new_gap = g.to - g.from;
                let wins = match target.as_ref() {
                    None => true,
                    // Between components with text, baseline rules:
                    // aligning the box by one pixel would leave text crooked, which
                    // is precisely what the classic IDE avoids.
                    Some((_, current)) if current.kind == GuideKind::Text => {
                        g.kind == GuideKind::Text && d.abs() < 0.001
                    }
                    Some((m, current)) => {
                        g.kind == GuideKind::Text
                            || d.abs() < m.abs() - 0.001
                            || ((d.abs() - m.abs()).abs() < 0.001
                                && new_gap < current.to - current.from)
                    }
                };
                if wins {
                    *target = Some((d, g));
                }
            }
        }

        let ax = best_x.map(|(d, _)| d).unwrap_or(0.0);
        let ay = best_y.map(|(d, _)| d).unwrap_or(0.0);

        // Adjustment is one per axis — one can only snap to one place. But DRAWING
        // shows everything that became valid after snapping: top, bottom, center,
        // baseline and spacing appear together, as in the classic IDE.
        let final_rect = Rect::new(r.x + ax, r.y + ay, r.w, r.h);
        let mut guides: Vec<Guide> = Vec::new();
        for other in &root.children {
            if moved.contains(other) {
                continue;
            }
            let Some(o) = doc.node(other).and_then(|n| n.rect()) else {
                continue;
            };
            for (d, g) in self.align_relations(
                doc,
                reg,
                moved.first(),
                final_rect,
                other,
                o,
                GuideSpan::Gap,
            ) {
                if d.abs() < 0.5 && !guides.iter().any(|x| same_guide(x, &g)) {
                    guides.push(g);
                }
            }
        }

        (ax, ay, guides)
    }

    #[note("designer-static-guides")]
    pub fn static_guides(&self, doc: &Document, sel: &[NodeId]) -> Vec<Guide> {
        let reg = self.registry.as_deref();
        let (Some(id), Some(root)) = (sel.first(), doc.node(&doc.root)) else {
            return Vec::new();
        };
        let Some(r) = doc.node(id).and_then(|n| n.rect()) else {
            return Vec::new();
        };

        let mut guides = Vec::new();
        for other in &root.children {
            if sel.contains(other) {
                continue;
            }
            let Some(o) = doc.node(other).and_then(|n| n.rect()) else {
                continue;
            };
            guides.extend(
                self.align_relations(doc, reg, Some(id), r, other, o, GuideSpan::Cross)
                    .into_iter()
                    .filter(|(d, _)| d.abs() < 0.5)
                    .map(|(_, g)| g),
            );
        }
        guides
    }

    #[note("designer-parent-for-drop")]
    pub fn parent_for_drop(&self, doc: &Document, x: f32, y: f32) -> (NodeId, f32, f32) {
        let mut best: Option<(NodeId, usize, f32, f32)> = None;
        for id in doc.in_paint_order() {
            // The FIFTH path of the tree. Container not on screen cannot
            // receive children: pages of a `PageControl` ALL have
            // the same rectangle (derived geometry), so without this filter the
            // tie-breaker "top one" handed the component to the LAST page —
            // closed. The component vanished upon dropping, and the same path
            // serves `reparent_dragged`, so dragging inside also vanished.
            if !self.on_canvas(doc, &id) {
                continue;
            }
            let Some(n) = doc.node(&id) else { continue };
            let accepts = self
                .registry
                .as_ref()
                .and_then(|r| r.get(&n.class))
                .map(|d| d.children_area())
                .unwrap_or(false);
            if !accepts {
                continue;
            }
            let Some(r) = doc.rect_in_form(&id) else {
                continue;
            };
            if x < r.x || y < r.y || x > r.x + r.w || y > r.y + r.h {
                continue;
            }
            let depth = doc.ancestors(&id).len();
            // `>=` and not `>`: drawing order already comes from bottom to top,
            // so ties stay with the last — the one on top.
            let wins = best.as_ref().is_none_or(|(_, p, _, _)| depth >= *p);
            if wins {
                best = Some((id, depth, r.x, r.y));
            }
        }
        match best {
            Some((id, _, px, py)) => (id, x - px, y - py),
            None => (doc.root.clone(), x, y),
        }
    }

    pub fn drop_rect(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(self.snap(x), self.snap(y), w, h)
    }
}

#[note("non-inverted")]
fn non_inverted(r: Rect) -> Rect {
    Rect::new(r.x, r.y, r.w.max(0.0), r.h.max(0.0))
}

#[note("same-guide")]
fn same_guide(a: &Guide, b: &Guide) -> bool {
    a.vertical == b.vertical
        && a.kind == b.kind
        && (a.pos - b.pos).abs() < 0.5
        && (a.from - b.from).abs() < 0.5
        && (a.to - b.to).abs() < 0.5
}

#[note("guidespan")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuideSpan {
    /// Only the space between the two components — what is seen when dragging.
    Gap,
    /// Crossing both — what reveals alignment with Ctrl.
    Cross,
}

#[note("text-line")]
fn text_line(doc: &Document, reg: Option<&Registry>, id: &str, r: Rect) -> Option<f32> {
    let class_name = &doc.node(id)?.class;
    let tl = reg?.get(class_name)?.text_line?;
    Some(tl.line(r.y, r.h))
}

#[note("guide-extent")]
fn guide_extent(a_ini: f32, a_fim: f32, b_ini: f32, b_fim: f32) -> (f32, f32) {
    let gap_ini = a_fim.min(b_fim);
    let gap_fim = a_ini.max(b_ini);
    let (from, to) = if gap_ini <= gap_fim {
        (gap_ini, gap_fim)
    } else {
        (a_ini.max(b_ini), a_fim.min(b_fim))
    };
    // a stroke of 2 pixels is not visible: guarantees a minimum
    if to - from < 6.0 {
        let mid = (from + to) / 2.0;
        (mid - 3.0, mid + 3.0)
    } else {
        (from, to)
    }
}

#[cfg(test)]
#[path = "test/lib/test.rs"]
mod tests;

#[cfg(test)]
#[path = "test/marquee/test.rs"]
mod tests_marquee;
