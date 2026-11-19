//! # `TreeView` — Canonical Reference Implementation
//!
//! Hierarchical tree view component with expandable/collapsible nodes,
//! indentation lines, node selection, image list binding, and change events.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A visual tree control displaying a hierarchical collection of expandable items.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "tv",
    size = (140.0, 160.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_expanding")]
#[event(name = "on_collapsing")]
#[event(name = "on_mouse_click")]
pub struct TreeView {
    /// Reference to an `ImageList` component supplying node icons.
    #[prop(display = "Images", category = "appearance", default = "")]
    pub images: String,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for TreeView {
    fn default() -> Self {
        Self {
            images: String::new(),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for TreeView {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 140.0;
        let h = 160.0;

        // 1. Background surface and border
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);

        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.0, 0.0, w, h);

        // Collapse box [-]
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.stroke_rect(6.0, 8.0, 10.0, 10.0);
        canvas.fill_rect(8.0, 12.0, 6.0, 1.5);

        // 2. Root node "Node 1" (expanded)
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("[-]", 6.0, 6.0, 14.0, 16.0, TextAlign::Left);
        canvas.fill_text_in_rect("Node 1", 22.0, 6.0, 100.0, 16.0, TextAlign::Left);

        // 3. Tree hierarchy connecting lines
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        // Vertical line down from Node 1
        canvas.fill_rect(12.0, 22.0, 1.0, 48.0);
        // Horizontal line to Child 1
        canvas.fill_rect(12.0, 34.0, 10.0, 1.0);
        // Horizontal line to Child 2
        canvas.fill_rect(12.0, 54.0, 10.0, 1.0);

        // 4. Child 1 (selected)
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(24.0, 26.0, 110.0, 18.0, 2.0);
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_text_in_rect("Child 1", 28.0, 28.0, 80.0, 14.0, TextAlign::Left);

        // 5. Child 2
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("Child 2", 28.0, 48.0, 80.0, 14.0, TextAlign::Left);

        // 6. Root node "Node 2" (collapsed)
        canvas.fill_text_in_rect("[+]", 6.0, 70.0, 14.0, 16.0, TextAlign::Left);
        canvas.fill_text_in_rect("Node 2", 22.0, 70.0, 100.0, 16.0, TextAlign::Left);
        // Plus glyph: horizontal and vertical lines
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_rect(8.0, 76.0, 6.0, 1.5);
        canvas.fill_rect(10.0, 74.0, 1.5, 6.0);
    }
}
