//! **The `Strings` piece, read from a single file.**
//!
//! It was previously written here and AGAIN in the egui target, with different APIs:
//! this one had `len`/`get`/`clear` and lacked `new`/`add`/`set_text` and `PartialEq`;
//! the other one had the reverse. A form that compiled on one target failed to
//! compile on the other, in BOTH directions.
//!
//! Now both read `crawrad-component/src/pieces/strings.rs`, which is compiled in
//! that crate — ensuring `clippy` checks it before any user project receives it.
//! This is ADR 0037: a piece with two fittings is not a piece.

#[crawrad_note::note("targets-makepad-components-types-strings-type-def")]
pub const STRINGS_TYPE_DEF: &str =
    include_str!("../../../../../crawrad-component/src/pieces/strings.rs");
