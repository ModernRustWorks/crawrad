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
// A COLOUR -- eight bits per channel, the way the designer stores it.

/// A colour: red, green, blue and alpha, eight bits each.
///
/// # This is a component's DATA, and never how it paints
///
/// A component declares what it draws with theme ROLES -- surface, border,
/// text, accent -- and the IDE resolves them. A literal colour inside a drawing
/// would be immune to the theme and out of reach of whoever maintains the
/// application.
///
/// So this type is for the other thing: a colour the component's own code uses
/// and the user picks. A chart series, a threshold, a legend entry.
///
/// # Eight bits, and not four floats
///
/// It is what the designer stores and what a colour picker hands back, so the
/// value that travels is the value that was chosen -- with no rounding in the
/// middle to explain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Opaque, from the three channels.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// With an alpha of its own.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Fully transparent.
    pub const fn transparent() -> Self {
        Self::rgba(0, 0, 0, 0)
    }

    /// The four channels as `0.0..=1.0`, which is what most painters want.
    pub fn to_f32(self) -> [f32; 4] {
        [
            f32::from(self.r) / 255.0,
            f32::from(self.g) / 255.0,
            f32::from(self.b) / 255.0,
            f32::from(self.a) / 255.0,
        ]
    }
}

impl From<Color> for [u8; 4] {
    /// The four bytes, in the order a `Canvas` takes a colour.
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl Default for Color {
    /// Opaque black, which is what a colour with nothing said is everywhere.
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}
