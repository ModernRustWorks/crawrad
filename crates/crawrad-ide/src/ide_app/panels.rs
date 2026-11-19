//! **THE SIX DOCK PANELS, in DSL** — partitioned into submodules under `panels/`.
//!
//! Under ADR 0043, this root module acts as the entry point and registry
//! for the IDE panel templates, keeping each file well below the ceiling of
//! 1,500 lines.
//!
//! # Submodules
//!
//! - `side_panels`: Components, Inspector, Explorer, Packages, and Debug.
//! - `design`: The central panel for Form / Designer / Code Editor / Minimap.
//! - `chat`: The AI / Aefos Chat panel.
//! - `terminal`: The Terminal and Output panel.

use makepad_widgets::*;

#[path = "panels/chat.rs"]
pub mod chat;
#[path = "panels/design.rs"]
pub mod design;
#[path = "panels/side_panels.rs"]
pub mod side_panels;
#[path = "panels/terminal.rs"]
pub mod terminal;

pub fn live_design(cx: &mut Cx) {
    side_panels::live_design(cx);
    design::live_design(cx);
    chat::live_design(cx);
    terminal::live_design(cx);
}
