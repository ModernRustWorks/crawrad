// This file is EMITTED VERBATIM into the user's project. Everything in it --
// comments included -- ends up in front of whoever opens the generated code, so
// it is English from the first line (ADR 0030). The house rationale, in
// Portuguese, lives in `pieces/mod.rs`, which is never emitted.
//
// Authoring rules, the same ones `strings.rs` carries:
//
// 1. No `//!` and no `#![...]`. This text lands in the MIDDLE of a file, after
//    other items, and an inner attribute after an item is `error[E0753]`.
// 2. No `use`, and no type of ours. Only `std`. On the other side none of our
//    crates exist -- the generated project depends on none of them, and that
//    is the proof this is generation and not an interpreter in disguise.
//    Do not NAME one either, not even in a comment: the gate that proves it
//    reads the whole emitted file, and it is right to.
//
// THE ANCHORS -- which edges a component holds on to when the form resizes. It
// is the classic RAD `Anchors`, whole: an anchored edge keeps its distance to the
// form's edge, so the component stretches or follows.

/// A component's anchors: left, top, right, bottom.
///
/// # `Anchors = []` leaves the component still, and that is the RULE
///
/// Here it once said that a classic RAD IDE keeps the proportion and that standing still
/// was "the predictable behaviour" — as if it were a pragmatic departure from
/// its component library. It is not. In a classic RAD IDE, `Anchors = []` leaves the component where it
/// is: the library only moves the edge that was anchored, and with no anchor there
/// is nothing to move.
///
/// That sentence was not harmless. It was the argument behind a twin arm in the
/// code generator that added `dw / 2.0` and threw the component 50 pt sideways
/// when the user opened the application in a larger window — the designer
/// showing one thing and the application another, which is the one thing this
/// product must never do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anchors {
    pub left: bool,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
}

impl Default for Anchors {
    /// `[akLeft, akTop]`, as in a classic RAD IDE.
    fn default() -> Self {
        Self {
            left: true,
            top: true,
            right: false,
            bottom: false,
        }
    }
}

/// **Where a rectangle ends up after the form grew by `dw` × `dh`.**
///
/// Anchored on both sides, the component **stretches**; anchored only on the
/// far side, it **follows** that edge; anchored only on the near side — **or on
/// no side at all** — it stays put.
///
/// # It is exact and reversible, and there is NO floor here
///
/// `anchored(…, -n, -n)` followed by `anchored(…, n, n)` gives the rectangle
/// back. It can, because nothing is clipped: a width anchored on both sides
/// crosses zero and comes back the same way.
///
/// A `w.max(1.0)` used to live in the designer's copy, and it was
/// **destructive**: resizing is incremental — the shell keeps the form's
/// previous size and sends deltas, and the result is written back to the
/// document — so whatever the clamp ate was gone. Measured on 09/09/2026: a
/// component of 380 × 280 anchored on all four, shrunk by 500 and grown back by
/// 500, came back **501 × 501**. The owner saw it on screen with a `TreeView`
/// and said the component "gets lost".
///
/// **The floor belongs to whoever paints**, where it destroys nothing: the
/// preview clips at `max(0.0)` and the generated application clips at
/// `max(1.0)`, and both recompute from design-time constants instead of writing
/// the result back.
///
/// # Why this sum is HERE
///
/// It was written twice — once in the IDE's designer and once inside the text
/// the generator emits — and the two were equalised by hand on 09/09/2026 after
/// the copies disagreed. Two sums that must agree are two chances to disagree,
/// and the divergence shows up as a form that looks one way in the designer and
/// another in the application.
pub fn anchored(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    a: Anchors,
    dw: f32,
    dh: f32,
) -> (f32, f32, f32, f32) {
    let (mut x, mut w) = (x, w);
    match (a.left, a.right) {
        (true, true) => w += dw,
        (false, true) => x += dw,
        // The two arms stay APART, and a gate asks for it. Folding them into a
        // `|` -- or into a `_` -- hides `(false, false)`, which is where the
        // defect lived: the generator added `dw / 2.0` and threw the component
        // 50 pt sideways when the window opened larger.
        (true, false) => {}
        (false, false) => {}
    }
    let (mut y, mut h) = (y, h);
    match (a.top, a.bottom) {
        (true, true) => h += dh,
        (false, true) => y += dh,
        (true, false) => {}
        (false, false) => {}
    }
    (x, y, w, h)
}
