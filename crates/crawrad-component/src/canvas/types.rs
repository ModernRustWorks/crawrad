// Standard Canvas types aligned with the WHATWG / W3C HTML Canvas 2D specification.

use crate::Color;

/// Theme color roles recognized by the CrawRAD canvas and theme system.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::ThemeRole;
///
/// let role = ThemeRole::Surface;
/// assert_eq!(role.as_str(), "surface");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ThemeRole {
    /// Background surface color of the control.
    #[default]
    Surface,
    /// Outline and border color.
    Border,
    /// Primary foreground text color.
    Text,
    /// Dimmed or secondary text color (hints, placeholders).
    TextDim,
    /// Highlight and accent color (selection, focus, active states).
    Accent,
}

impl ThemeRole {
    /// The canonical identifier used in manifests and theme stylesheets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::ThemeRole;
    ///
    /// assert_eq!(ThemeRole::Accent.as_str(), "accent");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Border => "border",
            Self::Text => "text",
            Self::TextDim => "text_dim",
            Self::Accent => "accent",
        }
    }

    /// Parse from a theme role string name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::ThemeRole;
    ///
    /// assert_eq!(ThemeRole::from_str_name("surface"), Some(ThemeRole::Surface));
    /// assert_eq!(ThemeRole::from_str_name("unknown"), None);
    /// ```
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s {
            "surface" | "Surface" => Some(Self::Surface),
            "border" | "Border" => Some(Self::Border),
            "text" | "Text" => Some(Self::Text),
            "text_dim" | "TextDim" => Some(Self::TextDim),
            "accent" | "Accent" => Some(Self::Accent),
            _ => None,
        }
    }
}

/// A fill style for shapes and text, matching `CanvasRenderingContext2D.fillStyle`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::{FillStyle, ThemeRole};
///
/// let fill = FillStyle::Theme(ThemeRole::Surface);
/// assert_eq!(fill, FillStyle::default());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum FillStyle {
    /// An explicit RGBA color.
    Color(Color),
    /// A semantic theme color role.
    Theme(ThemeRole),
}

impl Default for FillStyle {
    fn default() -> Self {
        Self::Theme(ThemeRole::Surface)
    }
}

impl From<Color> for FillStyle {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}

impl From<ThemeRole> for FillStyle {
    fn from(role: ThemeRole) -> Self {
        Self::Theme(role)
    }
}

/// A stroke style for lines and borders.
///
/// A stroke style for lines and borders, matching `CanvasRenderingContext2D.strokeStyle`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::{StrokeStyle, ThemeRole};
///
/// let stroke = StrokeStyle::Theme(ThemeRole::Border);
/// assert_eq!(stroke, StrokeStyle::default());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum StrokeStyle {
    /// An explicit RGBA color.
    Color(Color),
    /// A semantic theme color role.
    Theme(ThemeRole),
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self::Theme(ThemeRole::Border)
    }
}

impl From<Color> for StrokeStyle {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}

impl From<ThemeRole> for StrokeStyle {
    fn from(role: ThemeRole) -> Self {
        Self::Theme(role)
    }
}

/// Horizontal text alignment, matching `CanvasRenderingContext2D.textAlign`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::TextAlign;
///
/// assert_eq!(TextAlign::default(), TextAlign::Left);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextAlign {
    /// Align text to the left boundary.
    #[default]
    Left,
    /// Center text horizontally.
    Center,
    /// Align text to the right boundary.
    Right,
}

impl TextAlign {
    /// The string identifier matching WHATWG Canvas specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::TextAlign;
    ///
    /// assert_eq!(TextAlign::Center.as_str(), "center");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

/// Vertical text baseline alignment.
///
/// Vertical text baseline alignment, matching `CanvasRenderingContext2D.textBaseline`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::TextBaseline;
///
/// assert_eq!(TextBaseline::default(), TextBaseline::Top);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextBaseline {
    /// Top of the em square.
    #[default]
    Top,
    /// Middle of the em square.
    Middle,
    /// Bottom of the bounding box.
    Bottom,
}

impl TextBaseline {
    /// The string identifier matching WHATWG Canvas specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::TextBaseline;
    ///
    /// assert_eq!(TextBaseline::Top.as_str(), "top");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Middle => "middle",
            Self::Bottom => "bottom",
        }
    }
}

/// Coordinate axis for proportional bar and progress measurements.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::canvas::Axis;
///
/// assert_eq!(Axis::default(), Axis::X);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Axis {
    /// Horizontal axis from left to right.
    #[default]
    X,
    /// Vertical axis from top to bottom.
    Y,
}

impl Axis {
    /// The canonical axis string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::canvas::Axis;
    ///
    /// assert_eq!(Axis::X.as_str(), "x");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
        }
    }
}

/// Snapshot of the canvas drawing state.
///
/// Snapshot of the canvas drawing state, saved and restored via `save()` and `restore()`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::canvas::CanvasState;
///
/// let state = CanvasState::default();
/// assert_eq!(state.line_width, 1.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CanvasState {
    /// Current fill style.
    pub fill_style: FillStyle,
    /// Current stroke style.
    pub stroke_style: StrokeStyle,
    /// Stroke line width in points/pixels.
    pub line_width: f32,
    /// Text horizontal alignment.
    pub text_align: TextAlign,
    /// Text vertical baseline.
    pub text_baseline: TextBaseline,
    /// Global alpha transparency (0.0 to 1.0).
    pub global_alpha: f32,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            fill_style: FillStyle::default(),
            stroke_style: StrokeStyle::default(),
            line_width: 1.0,
            text_align: TextAlign::default(),
            text_baseline: TextBaseline::default(),
            global_alpha: 1.0,
        }
    }
}
