//! # `ActionList` & `Action` — Canonical Reference Implementation
//!
//! Centralized command and action management architecture, decoupling UI buttons
//! and menus from business logic handlers, shortcut keys, and state synchronization.

use crawrad_component::Shortcut;
use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual container hosting centralized action definitions and image references.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "acl",
    size = (28.0, 28.0),
    children = "collection:Action",
    targets = ["*"]
)]
pub struct ActionList {
    /// Enables or disables all contained actions.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Reference to an `ImageList` supplying icons for actions.
    #[prop(display = "Images", category = "appearance", default = "")]
    pub images: String,
}

impl Default for ActionList {
    fn default() -> Self {
        Self {
            enabled: true,
            images: String::new(),
        }
    }
}

impl Paint for ActionList {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Action symbol: stylized lightning bolt / action execution arrow
        canvas.set_fill_style(ThemeRole::Accent);
        // Lightning bolt upper diagonal
        canvas.fill_rect(13.0, 5.0, 3.0, 8.0);
        // Horizontal junction
        canvas.fill_rect(10.0, 11.0, 8.0, 3.0);
        // Lower diagonal
        canvas.fill_rect(11.0, 13.0, 3.0, 9.0);
    }
}

/// An individual action representing a user command with shortcut, caption, and event handlers.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "",
    prefix = "act",
    size = (28.0, 28.0),
    targets = ["*"]
)]
#[event(name = "on_execute")]
#[event(name = "on_update")]
pub struct Action {
    /// Command caption text displayed on bound controls.
    #[prop(display = "Caption", category = "appearance", default = "")]
    pub text: String,

    /// Status bar or tooltip hint text.
    #[prop(display = "Hint", category = "appearance", default = "")]
    pub hint: String,

    /// Primary keyboard shortcut triggering this action (e.g., "Ctrl+S").
    #[prop(display = "ShortCut", category = "behavior")]
    pub shortcut: Shortcut,

    /// Organization category in the ActionList editor.
    #[prop(display = "Category", category = "behavior", default = "")]
    pub category: String,

    /// Icon index within the parent ActionList's ImageList.
    #[prop(display = "ImageIndex", category = "appearance", default = -1)]
    pub image_index: i64,

    /// Controls whether bound controls are clickable.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Controls visibility of bound buttons and menu items.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Toggle or checkmark state for switchable actions.
    #[prop(display = "Checked", category = "behavior", default = false)]
    pub checked: bool,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            text: String::new(),
            hint: String::new(),
            shortcut: Shortcut::NONE,
            category: String::new(),
            image_index: -1,
            enabled: true,
            visible: true,
            checked: false,
        }
    }
}

impl Paint for Action {
    /// Design-time visual rendering inside collection editor.
    fn paint(&self, canvas: &mut Canvas) {
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);
        canvas.set_stroke_style(ThemeRole::Accent);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);
    }
}
