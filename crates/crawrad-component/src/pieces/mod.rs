//! **THE PROPERTY PIECES**: the types a component uses to hold state.
//!
//! # They live HERE, at the bottom of the stack, and until 18/09/2026 they did not
//!
//! `strings.rs` lived in `crawrad-runtime/src/pieces/`, and this box reached it
//! through `include!("../../crawrad-runtime/…")`. That is: **the bottom of the
//! stack reached upwards.**
//!
//! That was not ugly, it was wrong in direction. The box is the crate that
//! depends on nothing and goes into the binary of whoever only dragged an icon;
//! `crawrad-runtime` sits above it. A property piece belongs to the box by
//! definition ([ADR 0037], [ADR 0038] section 1): whoever wanted it would have
//! had to reach a crate that is not its own.
//!
//! Today it is an ordinary `mod`. The `include!` and the `#[cfg(any())] mod`
//! that existed only so `cargo fmt` could see the file reached by path are gone.
//!
//! # One source, three readers
//!
//! | who | how |
//! |---|---|
//! | **this box** | `mod`: the component author gets the type, and the compiler checks it |
//! | `crawrad-runtime` | `pub use crawrad_component::Strings`: an ordinary Cargo dependency |
//! | `crawrad-codegen` | `include_str!`: the file becomes **text** inside the user's project |
//!
//! The third is what forces the piece to be a file, and not only a type: a form
//! made only of native components **depends on no crate of ours**, and the proof
//! is that its type is EMITTED, not imported.
//!
//! # The emitted code speaks English
//!
//! `strings.rs` is emitted **whole**, comments and all, into the project of
//! whoever uses the IDE. Whoever opens it is reading Rust, and an API borrowed
//! from another language would force that person to learn a dialect that exists
//! nowhere. The gate `o_codigo_gerado_nao_tem_portugues` enforces it.
//!

pub mod anchors;
pub mod busy;
pub mod collection;
pub mod color;
pub mod cursor;
pub mod date_time;
pub mod edges;
pub mod file_path;
pub mod font;
pub mod modal_result;
pub mod picture;
pub mod shortcut;
pub mod strings;

pub use anchors::{Anchors, anchored};
pub use busy::BusyTiming;
pub use collection::Collection;
pub use color::Color;
pub use cursor::Cursor;
pub use date_time::DateTime;
pub use edges::Edges;
pub use file_path::FilePath;
pub use font::Font;
pub use modal_result::ModalResult;
pub use picture::Picture;
pub use shortcut::{Key, Key as KeyCode, Shortcut};
pub use strings::Strings;

#[cfg(test)]
#[path = "../../test/busy/test.rs"]
mod busy_test;

#[cfg(test)]
#[path = "../../test/collection/test.rs"]
mod collection_test;

#[cfg(test)]
#[path = "../../test/cursor/test.rs"]
mod cursor_test;

#[cfg(test)]
#[path = "../../test/date_time/test.rs"]
mod date_time_test;

#[cfg(test)]
#[path = "../../test/file_path/test.rs"]
mod file_path_test;

#[cfg(test)]
#[path = "../../test/font/test.rs"]
mod font_test;

#[cfg(test)]
#[path = "../../test/modal_result/test.rs"]
mod modal_result_test;

#[cfg(test)]
#[path = "../../test/picture/test.rs"]
mod picture_test;

#[cfg(test)]
#[path = "../../test/shortcut/test.rs"]
mod shortcut_test;
