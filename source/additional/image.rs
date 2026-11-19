//! # `Image` — Canonical Reference Implementation
//!
//! Standard graphic picture display component supporting raster and vector assets,
//! proportional scaling, centering, stretching, and mouse click events.

use crawrad_component::Picture;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A visual component that displays graphical images (PNG, JPEG, SVG, bitmap).
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "img",
    size = (105.0, 105.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_mouse_down")]
#[event(name = "on_mouse_up")]
#[event(name = "on_mouse_move")]
pub struct Image {
    /// Graphic picture asset holding raw binary bytes or resource source.
    #[prop(display = "Picture", category = "appearance")]
    pub picture: Picture,

    /// Scales the image to fill the component dimensions.
    #[prop(display = "Stretch", category = "appearance", default = false)]
    pub stretch: bool,

    /// Maintains original aspect ratio when stretching.
    #[prop(display = "Proportional", category = "appearance", default = false)]
    pub proportional: bool,

    /// Centers the graphic within the component boundaries.
    #[prop(display = "Center", category = "appearance", default = false)]
    pub center: bool,

    /// Automatically resizes the control to match graphic dimensions.
    #[prop(display = "AutoSize", category = "behavior", default = false)]
    pub auto_size: bool,

    /// Enables transparent background rendering.
    #[prop(display = "Transparent", category = "appearance", default = false)]
    pub transparent: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            picture: Picture::empty(),
            stretch: false,
            proportional: false,
            center: false,
            auto_size: false,
            transparent: false,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Image {
    /// Design-time visual rendering on the IDE form surface.
    ///
    /// Renders an elegant graphic picture placeholder with a bordered canvas frame,
    /// a stylized landscape vector silhouette (mountains and sun), and center guide.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 105.0;
        let h = 105.0;

        // 1. Frame background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, w, h, 2.0);

        // 2. Dashed or thin outer border
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, w, h, 2.0);

        // 3. Stylized picture icon / graphic placeholder
        // Sun circle
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(64.0, 24.0, 14.0, 14.0, 7.0);

        // Mountains (two triangular polygons using fill_rect/segments)
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_round_rect(16.0, 52.0, 42.0, 36.0, 2.0);
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(44.0, 42.0, 45.0, 46.0, 2.0);

        // 4. Caption label "(Image)" at bottom
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("(Image)", 0.0, 78.0, w, 18.0, TextAlign::Center);
    }
}
