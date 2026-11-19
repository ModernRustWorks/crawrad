//! # Canonical Reference Implementations of CrawRAD Components
//!
//! This crate contains reference implementations of native visual and non-visual
//! components for the CrawRAD ecosystem, showcasing idiomatic Rust architecture,
//! metadata derivation via `#[derive(Component)]`, and 2D vector rendering with `Canvas`.

#[path = "../standard/mod.rs"]
pub mod standard;

#[path = "../additional/mod.rs"]
pub mod additional;

#[path = "../win32/mod.rs"]
pub mod win32;

#[path = "../system/mod.rs"]
pub mod system;

#[path = "../dialogs/mod.rs"]
pub mod dialogs;
