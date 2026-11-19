//! **The `Gauge`** — a meter that shows a value inside a range.
//!
//! It is the first component of the suite to do **both halves**: it draws
//! itself in the designer from `manifest.json` — with no Rust for that — and it
//! becomes this type inside the generated application.
//!
//! | half | who answers |
//! |---|---|
//! | **drawing in the designer** | the verbs below, painted by the IDE's `Brush` |
//! | **the type in the application** | this `struct` |
//!
//! And both read the **same properties**, because both come out of the same
//! struct. That sentence used to be a lie: until the manifest was generated,
//! this file claimed `visible` and `enabled` were declared on both sides and
//! the manifest declared neither. Nothing could have caught it — one side was
//! prose and the other was TOML.
//!
//! # It is written ON THE BOX, and only on it
//!
//! `crawrad-component` brings the pieces; `crawrad-component-derive` writes the
//! manifest. **Neither is the IDE.** There is no Makepad here, no egui, no
//! `Document` and no `Registry`. `cargo add gauge-crawrad` works in any
//! program, with CrawRAD nowhere in sight — which is what keeps the suite from
//! becoming an appendix of the IDE.
//!
//! # `manifest.json` is generated from this file
//!
//! Change the struct below and run `CRAWRAD_BLESS=1 cargo test` to rewrite it.
//! Editing the manifest by hand is how the two halves drifted apart in the
//! first place.

use crawrad_component::canvas::{Axis, Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A meter: a caption and a value inside a range.
///
/// # The range is clamped, not refused
///
/// A `value` outside `min..=max` is not an error: [`Self::fraction`] returns
/// `0.0` or `1.0`. A meter that refuses a value is a meter that stops showing
/// anything exactly when something left the expected range — which is when
/// somebody is looking at it.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "gau",
    size = (140.0, 48.0),
    targets = ["*"]
)]
// The palette icon: the dial, the needle, the hub. The paints take empty
// parentheses so Clippy does not read the second `stroke` as a duplicate.
#[icon(
    circle(8.0, 9.0, 6.0),
    stroke(),
    move_to(8.0, 9.0),
    line_to(11.5, 5.5),
    stroke(),
    circle(8.0, 9.0, 1.2),
    fill()
)]
// **Every visual control in a classic RAD IDE has `OnClick`**, and it is the same here: the
// surface of a declared component is its own rectangle, and what the framework
// can say about a rectangle is what the pointer did on it.
//
// A meter's `OnClick` is not decoration: it is how whoever uses the component
// switches the unit, resets the counter, or opens the detail — with no widget
// underneath.
#[event(name = "on_click")]
pub struct Gauge {
    /// The caption, written over the bar.
    #[prop(display = "Caption", category = "appearance", default = "Gauge")]
    pub text: String,

    /// The value right now.
    ///
    /// A meter that shows no value is a box with a caption, which is why this
    /// property is what broke the five-verb vocabulary open.
    #[prop(
        display = "Value",
        category = "behavior",
        default = 65,
        min = 0,
        max = 100
    )]
    pub value: i64,

    /// The start of the range.
    ///
    /// It used to exist here and NOT in the manifest, and that missing half was
    /// a small lie of an expensive family: changing `min` at run time moved the
    /// component's `fraction()` and left the painted bar alone, because the
    /// drawing declared `min = 0.0` as a fixed number. While the values were the
    /// defaults the two agreed — right until somebody used it.
    #[prop(display = "Min", category = "behavior", default = 0, min = -1000000, max = 1000000)]
    pub min: i64,

    /// The end of the range.
    #[prop(display = "Max", category = "behavior", default = 100, min = -1000000, max = 1000000)]
    pub max: i64,

    /// Shows on screen.
    ///
    /// The generated code already guarded the drawing with this field before the
    /// manifest declared it — so the application honoured a property the Object
    /// Inspector could not set.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Responds to interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Gauge {
    fn default() -> Self {
        Self {
            visible: true,
            enabled: true,
            text: String::from("Gauge"),
            value: 65,
            min: 0,
            max: 100,
        }
    }
}

impl Gauge {
    /// **How much of the bar is filled**, from `0.0` to `1.0`.
    ///
    /// It is the same sum the designer makes to draw the fraction — and it is
    /// what keeps what you see on the canvas equal to what the application
    /// shows.
    ///
    /// A zero-width range returns `0.0` and not `NaN`: an empty bar is an
    /// answer; a strange symbol on screen is not.
    pub fn fraction(&self) -> f32 {
        // **THE SUM COMES FROM THE BOX**, and is not rewritten here.
        //
        // It used to be the THIRD copy: the designer canvas had one, the
        // generated code received one, and this method had this one. Three sums
        // that ought to agree are three chances to disagree — and the divergence
        // would show up as a meter reading one thing in the designer and another
        // on the user's screen.
        crawrad_component::fraction(self.value as f32, self.min as f32, self.max as f32)
    }

    /// The value as a whole percentage, for whoever wants to write `65%`.
    pub fn percent(&self) -> i32 {
        (self.fraction() * 100.0).round() as i32
    }
}

impl Paint for Gauge {
    /// Visual representation painted using the standard Canvas 2D API.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Background surface with rounded corners
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 140.0, 48.0, 4.0);

        // 2. The progress bar filling the fraction of the value
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_fraction_rect(Axis::X, "value", "min", "max");

        // 3. Accent frame outline
        canvas.set_stroke_style(ThemeRole::Accent);
        canvas.set_line_width(2.0);
        canvas.stroke_round_rect(0.0, 0.0, 140.0, 48.0, 4.0);

        // 4. Centered caption text
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 0.0, 0.0, 140.0, 48.0, TextAlign::Center);
    }
}

// **The manifest is emitted from the struct above.**
//
// This line is what turns `manifest.json` from a file somebody maintains into an
// artifact a test keeps honest. See `crawrad_component::manifest`.
crawrad_component::manifest!(Gauge);

#[cfg(test)]
mod tests {
    use super::*;

    /// The fraction follows the value, and is clamped on both sides.
    ///
    /// The same cases the IDE canvas measures — because it is the same sum, and
    /// two sums that ought to agree need two measurements that agree.
    #[test]
    fn the_fraction_follows_the_value_and_is_clamped() {
        let g = |v: i64| Gauge {
            value: v,
            ..Default::default()
        };
        assert_eq!(g(0).fraction(), 0.0);
        assert_eq!(g(20).fraction(), 0.2);
        assert_eq!(g(100).fraction(), 1.0);
        assert_eq!(g(-30).fraction(), 0.0, "below the range stays empty");
        assert_eq!(g(500).fraction(), 1.0, "above the range stays full");
    }

    /// A range with no width does not divide by zero.
    #[test]
    fn a_range_with_no_width_gives_an_empty_bar() {
        let g = Gauge {
            min: 10,
            max: 10,
            value: 50,
            ..Default::default()
        };
        assert_eq!(g.fraction(), 0.0);
        assert_eq!(g.percent(), 0);
    }

    /// The range does not have to start at zero.
    #[test]
    fn the_range_can_start_anywhere() {
        let g = Gauge {
            min: -50,
            max: 50,
            value: 0,
            ..Default::default()
        };
        assert_eq!(g.fraction(), 0.5, "the middle of -50..50 is zero");
        assert_eq!(g.percent(), 50);
    }

    /// **The drawing still follows the properties, and not two fixed numbers.**
    ///
    /// The derive guarantees that every property of the manifest is a field of
    /// this type, and that the defaults agree. What it cannot guarantee is that
    /// the *drawing* keeps reading the range from the properties — `area` takes
    /// a number just as happily, and a number here is the bug this project
    /// already paid for once.
    #[test]
    fn the_drawing_reads_the_range_from_the_properties() {
        let mut canvas = Canvas::new(140.0, 48.0);
        Gauge::default().paint(&mut canvas);
        let manifest = canvas.to_manifest_json();
        assert!(
            manifest.contains(r#""min": "min", "max": "max""#),
            "the drawing stopped following the range properties:\n{manifest}"
        );
    }

    /// The default already shows something.
    ///
    /// A component born empty looks broken in the designer — the same reason the
    /// catalog's `Button` is born with its own name written on it.
    #[test]
    fn the_default_already_shows_something() {
        let g = Gauge::default();
        assert!(g.visible && g.enabled);
        assert!(!g.text.is_empty());
        assert!(g.fraction() > 0.0);
    }
}
