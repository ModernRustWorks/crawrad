//! The pure model behind a Color dialog.
//!
//! Maintains the state of the Color dialog opened by the Object Inspector and
//! Options pages without any direct GUI dependencies.

use crawrad_ir::Color;
use crawrad_note::note;

#[note("color-dialog-recent-max")]
pub const RECENT_MAX: usize = 8;

#[note("color-dialog-channel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Red,
    Green,
    Blue,
    Alpha,
}

#[note("color-dialog-colorfields")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorFields {
    pub hex: String,
    pub red: String,
    pub green: String,
    pub blue: String,
    pub alpha: String,
}

#[note("color-dialog-colordialog")]
#[derive(Debug, Clone, PartialEq)]
pub struct ColorDialog {
    /// Window title for the dialog (e.g. "Color · Keywords").
    pub title: String,
    /// What the caller had initially; never changes.
    pub current: Color,
    /// The chosen color patch being edited.
    pub chosen: Color,
    /// Hue kept separately so dragging saturation or value on a grey does not lose the hue.
    hue: f32,
    /// Recent colors list, up to [`RECENT_MAX`] entries.
    pub recent: Vec<Color>,
}

impl ColorDialog {
    #[note("color-dialog-colordialog-new")]
    pub fn new(title: impl Into<String>, current: Color, recent: Vec<Color>) -> Self {
        let (hue, _, _) = current.to_hsv();
        let mut recent = recent;
        recent.truncate(RECENT_MAX);
        Self {
            title: title.into(),
            current,
            chosen: current,
            hue,
            recent,
        }
    }

    #[note("color-dialog-colordialog-hue")]
    pub fn hue(&self) -> f32 {
        self.hue
    }

    #[note("color-dialog-colordialog-saturation-value")]
    pub fn saturation_value(&self) -> (f32, f32) {
        let (_, s, v) = self.chosen.to_hsv();
        (s, v)
    }

    #[note("color-dialog-colordialog-pick-square")]
    pub fn pick_square(&mut self, s: f32, v: f32) {
        self.chosen = Color::from_hsv(self.hue, s, v, self.chosen.a);
    }

    #[note("color-dialog-colordialog-pick-hue")]
    pub fn pick_hue(&mut self, fraction: f32) {
        let hue = fraction * 360.0;
        self.hue = hue;
        let (_, s, v) = self.chosen.to_hsv();
        self.chosen = Color::from_hsv(hue, s, v, self.chosen.a);
    }

    #[note("color-dialog-colordialog-pick")]
    pub fn pick(&mut self, c: Color) {
        let alpha = self.chosen.a;
        self.chosen = Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: alpha,
        };
        if c.r != c.g || c.g != c.b {
            let (h, _, _) = c.to_hsv();
            self.hue = h;
        }
    }

    #[note("color-dialog-colordialog-set-hex")]
    pub fn set_hex(&mut self, text: &str) -> Result<(), String> {
        let c =
            Color::from_hex(text.trim()).ok_or_else(|| "Not a colour: use #RRGGBB".to_string())?;
        self.chosen = c;
        if c.r != c.g || c.g != c.b {
            let (h, _, _) = c.to_hsv();
            self.hue = h;
        }
        Ok(())
    }

    #[note("color-dialog-colordialog-set-channel")]
    pub fn set_channel(&mut self, ch: Channel, text: &str) -> Result<(), String> {
        let val: u8 = text
            .trim()
            .parse()
            .map_err(|_| "Use a number from 0 to 255".to_string())?;
        match ch {
            Channel::Red => self.chosen.r = val,
            Channel::Green => self.chosen.g = val,
            Channel::Blue => self.chosen.b = val,
            Channel::Alpha => self.chosen.a = val,
        }
        if ch != Channel::Alpha
            && (self.chosen.r != self.chosen.g || self.chosen.g != self.chosen.b)
        {
            let (h, _, _) = self.chosen.to_hsv();
            self.hue = h;
        }
        Ok(())
    }

    #[note("color-dialog-colordialog-fields")]
    pub fn fields(&self) -> ColorFields {
        ColorFields {
            hex: self.chosen.to_hex(),
            red: self.chosen.r.to_string(),
            green: self.chosen.g.to_string(),
            blue: self.chosen.b.to_string(),
            alpha: self.chosen.a.to_string(),
        }
    }

    #[note("color-dialog-colordialog-accept")]
    pub fn accept(self) -> (Color, Vec<Color>) {
        let chosen = self.chosen;
        let mut recent = vec![chosen];
        for c in self.recent {
            if !recent.contains(&c) {
                recent.push(c);
                if recent.len() == RECENT_MAX {
                    break;
                }
            }
        }
        (chosen, recent)
    }

    #[note("color-dialog-colordialog-basic")]
    pub fn basic() -> [(&'static str, Color); 16] {
        Color::STANDARD
    }
}

#[note("color-dialog-colortarget")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorTarget {
    /// A component or form property in the Object Inspector.
    Property { node: String, key: String },
    /// A 0-based field in the Color Scheme options page.
    SchemeField(usize),
}

#[cfg(test)]
#[path = "test/color_dialog/test.rs"]
mod tests;
