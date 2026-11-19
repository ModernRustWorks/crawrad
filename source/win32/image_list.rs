//! # `ImageList` — Canonical Reference Implementation
//!
//! Centralized icon and bitmap repository for fast indexed lookup across
//! toolbars, buttons, menus, and tree view items.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual repository holding a collection of equal-sized graphic icons.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "iml",
    size = (28.0, 28.0),
    targets = ["*"]
)]
pub struct ImageList {
    /// Width of individual icons in pixels (typically 16, 24, 32, or 48).
    #[prop(display = "Width", category = "appearance", default = 16, min = 1)]
    pub image_width: i64,

    /// Height of individual icons in pixels (typically 16, 24, 32, or 48).
    #[prop(display = "Height", category = "appearance", default = 16, min = 1)]
    pub image_height: i64,

    /// Serialized collection of image assets.
    #[prop(display = "Images", category = "appearance", default = "")]
    pub images_data: String,
}

impl Default for ImageList {
    fn default() -> Self {
        Self {
            image_width: 16,
            image_height: 16,
            images_data: String::new(),
        }
    }
}

impl Paint for ImageList {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    ///
    /// Renders the classic RAD ImageList visual metaphor: a miniature 2x2 photo/icon reel.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. 2x2 grid representing graphic image reel
        // Top-left icon
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(5.0, 5.0, 8.0, 8.0, 1.0);

        // Top-right icon
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(15.0, 5.0, 8.0, 8.0, 1.0);

        // Bottom-left icon
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_round_rect(5.0, 15.0, 8.0, 8.0, 1.0);

        // Bottom-right icon
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(15.0, 15.0, 8.0, 8.0, 1.0);
    }
}
