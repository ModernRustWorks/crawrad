//! Visual style, theme, and color definitions for Native Makepad.
//!
//! In Makepad, widget styling and visuals are declared
//! via live_design DSL shaders and properties (`draw_bg`, color palette).

use crawrad_note::note;
#[note("targets-makepad-drawing-form-bg-color")]
pub const FORM_BG_COLOR: &str = "#242424";

#[note("targets-makepad-drawing-panel-bg-color")]
pub const PANEL_BG_COLOR: &str = "#252538";

#[note("targets-makepad-drawing-border-color")]
pub const BORDER_COLOR: &str = "#3b3b4f";

#[note("targets-makepad-drawing-input-bg-color")]
pub const INPUT_BG_COLOR: &str = "#181825";

// Legacy aliases for existing references if any
pub const COR_FUNDO_FORM: &str = FORM_BG_COLOR;
pub const COR_PAINEL: &str = PANEL_BG_COLOR;
pub const COR_BORDA: &str = BORDER_COLOR;
