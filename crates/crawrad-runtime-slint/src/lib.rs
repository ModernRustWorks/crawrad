//! CrawRAD runtime layer for Slint applications.
//!
//! Provides support for component styling, dialog dispatch, timer scheduling,
//! data models, and window management in applications generated with the Slint target.

#![forbid(unsafe_code)]

pub mod dialogs;
pub mod events;
pub mod look;
pub mod models;
pub mod timer;
pub mod window;

pub use dialogs::*;
pub use events::*;
pub use look::*;
pub use models::*;
pub use timer::*;
pub use window::*;
