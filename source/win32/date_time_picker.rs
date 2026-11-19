//! # `DateTimePicker` — Canonical Reference Implementation
//!
//! A date/time input box with masked editing and field stepping.
//! Shows `dd/mm/yyyy` for `dtkDate` and `HH:MM:SS` for `dtkTime`.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A single-line date/time input box.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "dtp",
    size = (200.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
#[event(name = "on_key_down")]
#[event(name = "on_key_press")]
#[event(name = "on_key_up")]
pub struct DateTimePicker {
    /// Date or time mode (`dtkDate` or `dtkTime`).
    #[prop(display = "Kind", category = "behavior", default = "dtkDate")]
    pub kind: String,

    /// Canonical value in ISO format (`YYYY-MM-DD` or `HH:MM:SS`).
    #[prop(display = "DateTime", category = "text", default = "2000-01-01")]
    pub date_time: String,

    /// Optional minimum date limit in ISO format.
    #[prop(display = "MinDate", category = "behavior", default = "")]
    pub min_date: String,

    /// Optional maximum date limit in ISO format.
    #[prop(display = "MaxDate", category = "behavior", default = "")]
    pub max_date: String,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for DateTimePicker {
    fn default() -> Self {
        Self {
            kind: String::from("dtkDate"),
            date_time: String::from("2000-01-01"),
            min_date: String::new(),
            max_date: String::new(),
            visible: true,
            enabled: true,
        }
    }
}

impl DateTimePicker {
    /// The formatted text shown in the box: `dd/mm/yyyy` for date, `HH:MM:SS` for time.
    pub fn shown(&self) -> String {
        if self.kind == "dtkTime" {
            if self.date_time.len() >= 8 {
                self.date_time[..8].to_string()
            } else {
                "00:00:00".to_string()
            }
        } else if self.date_time.len() >= 10 {
            let parts: Vec<&str> = self.date_time[..10].split('-').collect();
            if parts.len() == 3 {
                format!("{}/{}/{}", parts[2], parts[1], parts[0])
            } else {
                self.date_time.clone()
            }
        } else {
            "01/01/2000".to_string()
        }
    }
}

impl Paint for DateTimePicker {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Text box outline
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 200.0, 24.0, 2.0);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 200.0, 24.0, 2.0);

        // 2. The formatted date/time text
        canvas.set_fill_style(ThemeRole::Text);
        let shown = self.shown();
        if !shown.is_empty() {
            canvas.fill_text_in_rect(&shown, 6.0, 4.0, 188.0, 16.0, TextAlign::Left);
        }
    }
}
