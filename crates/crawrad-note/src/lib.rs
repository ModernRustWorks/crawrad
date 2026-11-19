//! Turns CrawRAD IDE notes into rustdoc: `#[note("id")]` reads `.crawrad/notes/<id>.md`.
//!
//! # Usage
//!
//! The note is looked up under the crate being compiled; this example uses
//! one of this crate's own test notes, `.crawrad/notes/test-free-fn.md`.
//!
//! ```
//! use crawrad_note::note;
//!
//! #[note("test-free-fn")]
//! fn compute_sum(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//!
//! assert_eq!(compute_sum(40, 2), 42);
//! ```

#![forbid(unsafe_code)]

use proc_macro::TokenStream;

mod expand;
mod parse;

/// Transforms an IDE note from `.crawrad/notes/<id>.md` into rustdoc (`#[doc = "..."]`)
/// and makes cargo rebuild the crate when the note changes.
#[proc_macro_attribute]
pub fn note(attr: TokenStream, item: TokenStream) -> TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    expand::expand(attr.into(), item.into(), &manifest_dir).into()
}
