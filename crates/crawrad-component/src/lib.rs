//! # `crawrad-component` — The Component Authoring Toolkit
//!
//! The foundational toolbox for authoring CrawRAD components.
//! When authoring a component, you do not start from scratch: you build on the
//! pre-built property types, the 2D canvas, and the typed collections that come
//! in this crate.
//!
//! ## Architecture Overview
//!
//! The crate is organized into two core pillars:
//!
//! ```text
//! crawrad-component
//! │
//! ├── Property Pieces
//! │   ├── Strings
//! │   ├── Anchors
//! │   ├── Edges
//! │   ├── Color
//! │   ├── Collection<T>
//! │   ├── DateTime
//! │   ├── FilePath
//! │   ├── Cursor
//! │   └── ModalResult
//! │
//! └── Drawing API
//!     ├── Canvas
//!     ├── Paint
//!     ├── CanvasCommand
//!     ├── FillStyle
//!     ├── StrokeStyle
//!     ├── TextAlign
//!     └── ThemeRole
//! ```
//!
//! ### 1. Property Pieces (`pieces`)
//! Typed properties that define component state, validated by the compiler and
//! recognized automatically by the IDE Object Inspector and form persistence.
//!
//! ### 2. Drawing API (`canvas` / `drawing`)
//! A lightweight, declarative 2D vector drawing engine. Allows components to define
//! their visual rendering in pure Rust without binding directly to heavy backend
//! windowing or GPU engines.
//!
//! ## What it IS, and what it is NOT
//!
//! | In this crate | Why |
//! |---|---|
//! | **Property pieces** | Ready-made typed building blocks for component state |
//! | **2D Canvas drawing** | Declarative rendering commands emitted in pure Rust |
//! | **Manifest emitter** | Emits `manifest.json` metadata for the IDE palette |
//!
//! | NOT in this crate | Why |
//! |---|---|
//! | `Registry`, `Designer`, `Document` | Belong exclusively to the IDE shell |
//! | Target runtimes (Makepad, egui) | Components declare verbs; the engine renders |
//! | Heavy third-party dependencies | Keeps compile times under a second and binaries minimal |
//!

// No `unsafe`, and the compiler guarantees it -- as in every crate of this house.
#![forbid(unsafe_code)]

pub mod canvas;
pub mod fraction;
pub mod manifest;
pub mod pieces;
pub mod text_line;

/// Alias for the 2D Drawing API subsystem.
pub use canvas as drawing;

pub use canvas::{
    Canvas, CanvasCommand, CanvasDetector, CanvasExtract, CanvasFallback, FillStyle, Paint,
    StrokeStyle, TextAlign, TextBaseline, ThemeRole, merge_manifest_draw,
};
pub use fraction::fraction;
pub use manifest::{GENERATED_NOTICE, SDK};
pub use pieces::{
    Anchors, BusyTiming, Collection, Color, Cursor, DateTime, Edges, FilePath, Font, Key, KeyCode,
    ModalResult, Picture, Shortcut, Strings, anchored,
};

#[cfg(test)]
#[path = "../test/canvas/test.rs"]
mod canvas_test;
