//! **THE PROPERTY PIECES** — and they no longer live here.
//!
//! Until 18/09/2026 `strings.rs` belonged to this crate, and THE BOX
//! (`crawrad-component`) reached it via `include!("../../crawrad-runtime/…")` — the
//! bottom of the stack reaching upward.
//!
//! The direction was inverted. A property piece belongs to the box by definition
//! ([ADR 0037], [ADR 0038] §1): it is what the author of a third-party component
//! receives with `cargo add crawrad-component`, and it is the crate that depends on
//! nothing. `crawrad-runtime` is **above** it.
//!
//! Today the piece moved down, and this module is just the re-export that keeps
//! `crawrad_runtime::pieces::Strings` working for whoever was already using it.
//!

pub use crawrad_component::Strings;
