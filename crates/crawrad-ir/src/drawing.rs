//! **DECLARED DRAWING** — how a third-party component paints itself without a
//! line of Rust.
//!
//! It is the other half of ADR 0036.
//! The first half — the component turning into CODE — was proven on 2026-09-18
//! with `Connection`. This is the one that was missing: the **visual** third-party
//! component, which up until now appeared on the canvas as a dashed rectangle with the
//! class name inside (`generic::Drawing`).
//!
//! # Why a CLOSED vocabulary, and not code
//!
//! The alternative would be for the component to embed a paint function, and that
//! would force loading third-party code inside the IDE process — the
//! path that ADR 0036 rejected after measuring the price in those who paid it: the
//! WinForms designer was rewritten into a separate process, and in Lazarus
//! *installing a design-time package means linking it into the IDE*.
//!
//! The canvas of this codebase had already chosen the opposite, and before there were
//! third parties: `Brush` (`crawrad-runtime-makepad/src/canvas/brush.rs`) is a
//! closed set of paint primitives, and a component does NOT receive
//! `Cx2d` or shaders. This enum is that vocabulary turned into DATA — the same
//! drawing, now declarable in a `manifest.json`.
//!
//! # Colors belong to the THEME, and this is no small detail
//!
//! A verb does not accept `#ff0000`. It accepts an [`Ink`], which is a role —
//! *surface*, *border*, *text* —, and whoever resolves the value is the canvas.
//!
//! The reason has been written in `desenho_comum.rs` of the egui target since before this
//! file existed: *"no frozen colors… the day the theme becomes a
//! form property, a file with `Color32::from_rgb` inside
//! would need to be rewritten entirely"*. A third-party component with a
//! literal color would be that file, published to crates.io and beyond our reach.

use crawrad_note::note;
#[note("drawing-ink")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ink {
    /// The component background.
    Surface,
    /// Its border.
    Border,
    /// The main text.
    Text,
    /// Secondary text — caption, hint, whatever is less important.
    TextDim,
    /// The home accent: selection, focus, whatever draws attention.
    Accent,
}

impl Ink {
    #[note("drawing-ink-from-name")]
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "surface" => Some(Self::Surface),
            "border" => Some(Self::Border),
            "text" => Some(Self::Text),
            "text_dim" => Some(Self::TextDim),
            "accent" => Some(Self::Accent),
            _ => None,
        }
    }

    #[note("drawing-ink-name")]
    pub fn name(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Border => "border",
            Self::Text => "text",
            Self::TextDim => "text_dim",
            Self::Accent => "accent",
        }
    }

    #[note("drawing-ink-all")]
    pub const ALL: &'static [Self] = &[
        Self::Surface,
        Self::Border,
        Self::Text,
        Self::TextDim,
        Self::Accent,
    ];
}

#[note("drawing-textsource")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextSource {
    /// Written in the manifest, identical for every instance.
    Fixed(String),
    /// The value of the property with this key, in the drawn node.
    Property(String),
}

#[note("drawing-area")]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Area {
    /// The entire rectangle. The default, and what the five did before.
    #[default]
    All,
    /// Inset by this many design units on all sides.
    Inset(f32),
    /// **A FRACTION of the rectangle**, from the start of the axis up to where the value dictates.
    ///
    /// It is what a gauge, a progress bar, and a scroll thumb
    /// need — three components that the house would have written three times.
    ///
    /// The value comes from the property `from`, normalized between `min` and `max`. Outside
    /// the range it is clamped: a component does not paint outside its own box,
    /// and that rule already applies to native ones (`clips_to_own_box`).
    Fraction {
        from: String,
        axis: Axis,
        min: Limit,
        max: Limit,
    },
}

#[note("drawing-limit")]
#[derive(Debug, Clone, PartialEq)]
pub enum Limit {
    /// Written in the manifest, identical for every instance.
    Fixed(f32),
    /// The value of the property with this key, in the drawn node.
    Property(String),
}

impl Limit {
    #[note("drawing-limit-property")]
    pub fn property(&self) -> Option<&str> {
        match self {
            Self::Fixed(_) => None,
            Self::Property(k) => Some(k),
        }
    }
}

#[note("drawing-axis")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// From left to right.
    X,
    /// From top to bottom.
    Y,
}

impl Axis {
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "x" => Some(Self::X),
            "y" => Some(Self::Y),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
        }
    }
}

#[note("drawing-verb")]
#[derive(Debug, Clone, PartialEq)]
pub struct Verb {
    pub area: Area,
    pub stroke: Stroke,
}

impl Verb {
    #[note("drawing-verb-all")]
    pub fn all(stroke: Stroke) -> Self {
        Self {
            area: Area::All,
            stroke,
        }
    }
}

#[note("drawing-stroke")]
#[derive(Debug, Clone, PartialEq)]
pub enum Stroke {
    /// The component box: fill, border, and radius.
    Box { fill: Ink, border: Ink, radius: f32 },
    /// Just the outline, with thickness.
    Frame { color: Ink, thickness: f32 },
    /// Text inside the component area.
    Text {
        from: TextSource,
        color: Ink,
        centered: bool,
    },
    /// The dashed rectangle — what `generic::Drawing` already did, now
    /// available for anyone who wants to say *"this is a placeholder"*.
    Dashed { color: Ink },
    /// The check mark, for what has an on/off state.
    Check { color: Ink },
}

#[note("drawing-drawing")]
pub type Drawing = Vec<Verb>;

/// What the canvas needs to paint the catalogue classes it has no Rust drawing for.
///
/// The shell fills it from the catalogue; the canvas adapter never imports the catalogue (ADR 0027).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeclaredLooks {
    /// Class → the drawing its manifest declares (third-party component, ADR 0036).
    pub drawings: std::collections::BTreeMap<String, Drawing>,
    /// Inheriting class → its native base: in v1 the Designer paints it AS the base.
    pub bases: std::collections::BTreeMap<String, String>,
    /// Class → its palette icon, so a non-visual badge wears the same drawing as its tile.
    pub icons: std::collections::BTreeMap<String, Vec<crate::icon::IconStep>>,
    /// Classes the catalogue declares `visual: false`: with no drawing of their own
    /// the canvas paints them as the non-visual badge, never as a placeholder.
    pub non_visual: std::collections::BTreeSet<String>,
}

impl DeclaredLooks {
    /// The class the canvas paints for `class`: its base when it inherits one, else itself.
    pub fn painted_class<'a>(&'a self, class: &'a str) -> &'a str {
        self.bases.get(class).map_or(class, String::as_str)
    }

    #[note("drawing-declared-looks-icon-of")]
    pub fn icon_of(&self, class: &str) -> &[crate::icon::IconStep] {
        self.icons.get(class).map_or(&[], Vec::as_slice)
    }
}
