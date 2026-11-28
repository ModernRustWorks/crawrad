//! Window placement and state management for Slint applications.

/// Represents the window state of a form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormWindowState {
    #[default]
    Normal,
    Minimized,
    Maximized,
    FullScreen,
}

impl FormWindowState {
    pub fn from_property(prop: &str) -> Self {
        match prop {
            "wsMaximized" => Self::Maximized,
            "wsMinimized" => Self::Minimized,
            "wsFullScreen" => Self::FullScreen,
            _ => Self::Normal,
        }
    }
}

/// Computes centered window coordinates given monitor dimensions and form dimensions.
pub fn calculate_screen_center(
    monitor_w: f32,
    monitor_h: f32,
    form_w: f32,
    form_h: f32,
) -> (f32, f32) {
    let x = ((monitor_w - form_w) * 0.5).max(0.0);
    let y = ((monitor_h - form_h) * 0.5).max(0.0);
    (x, y)
}
