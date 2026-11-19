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
// FOUR SIDES -- what a margin, a padding or an inset is made of.

/// Four sides: left, top, right, bottom.
///
/// It is what a classic RAD IDE writes as `Margins` and as `Padding`, and what an inset
/// is: one number per edge, in design units.
///
/// # Why a type and not four fields
///
/// Because the Object Inspector edits it as ONE row that folds open into four,
/// and a component that spelled the four out would get four unrelated rows. The
/// shape of the value is what decides the shape of the editor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edges {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Edges {
    /// The same number on all four sides.
    ///
    /// It is the common case by a wide margin, and it is the one a classic RAD IDE's
    /// Object Inspector shows first.
    pub const fn uniform(v: f32) -> Self {
        Self {
            left: v,
            top: v,
            right: v,
            bottom: v,
        }
    }

    /// Nothing on any side.
    pub const fn none() -> Self {
        Self::uniform(0.0)
    }

    /// How much the four sides take away horizontally.
    pub fn horizontal(self) -> f32 {
        self.left + self.right
    }

    /// How much they take away vertically.
    pub fn vertical(self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for Edges {
    fn default() -> Self {
        Self::none()
    }
}
