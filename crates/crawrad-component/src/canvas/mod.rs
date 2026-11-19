// 2D Drawing Canvas aligned with the WHATWG / MDN Canvas 2D specification.

pub mod command;
pub mod types;

pub use command::CanvasCommand;
pub use types::{Axis, CanvasState, FillStyle, StrokeStyle, TextAlign, TextBaseline, ThemeRole};

/// Standard 2D drawing canvas context for custom CrawRAD components.
///
/// Built on the standard WHATWG / MDN Canvas 2D specification with typed Rust
/// idioms, eliminating proprietary DSLs and raw macros.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::Canvas;
///
/// let canvas = Canvas::new(100.0, 50.0);
/// assert_eq!(canvas.width(), 100.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    width: f32,
    height: f32,
    state: CanvasState,
    state_stack: Vec<CanvasState>,
    commands: Vec<CanvasCommand>,
}

impl Canvas {
    /// Create a new canvas with the specified design dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(320.0, 240.0);
    /// assert_eq!(canvas.width(), 320.0);
    /// assert_eq!(canvas.height(), 240.0);
    /// ```
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            state: CanvasState::default(),
            state_stack: Vec::new(),
            commands: Vec::new(),
        }
    }

    /// Canvas width in points/pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(200.0, 100.0);
    /// assert_eq!(canvas.width(), 200.0);
    /// ```
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Canvas height in points/pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(200.0, 100.0);
    /// assert_eq!(canvas.height(), 100.0);
    /// ```
    pub fn height(&self) -> f32 {
        self.height
    }

    // ── STATE AND STYLES (WHATWG: CanvasFillStrokeStyles, CanvasState) ────────

    /// Set the current fill style (color or theme role).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, FillStyle, ThemeRole};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_fill_style(ThemeRole::Border);
    /// assert_eq!(*canvas.fill_style(), FillStyle::Theme(ThemeRole::Border));
    /// ```
    pub fn set_fill_style<S: Into<FillStyle>>(&mut self, style: S) {
        let s = style.into();
        self.state.fill_style = s.clone();
        self.commands.push(CanvasCommand::SetFillStyle(s));
    }

    /// Current fill style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, FillStyle, ThemeRole};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_fill_style(ThemeRole::Accent);
    /// assert_eq!(*canvas.fill_style(), FillStyle::Theme(ThemeRole::Accent));
    /// ```
    pub fn fill_style(&self) -> &FillStyle {
        &self.state.fill_style
    }

    /// Set the current stroke style (color or theme role).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, StrokeStyle, ThemeRole};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_stroke_style(ThemeRole::Border);
    /// assert_eq!(*canvas.stroke_style(), StrokeStyle::Theme(ThemeRole::Border));
    /// ```
    pub fn set_stroke_style<S: Into<StrokeStyle>>(&mut self, style: S) {
        let s = style.into();
        self.state.stroke_style = s.clone();
        self.commands.push(CanvasCommand::SetStrokeStyle(s));
    }

    /// Current stroke style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, StrokeStyle, ThemeRole};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_stroke_style(ThemeRole::Border);
    /// assert_eq!(*canvas.stroke_style(), StrokeStyle::Theme(ThemeRole::Border));
    /// ```
    pub fn stroke_style(&self) -> &StrokeStyle {
        &self.state.stroke_style
    }

    /// Set the stroke line width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_line_width(2.5);
    /// assert_eq!(canvas.line_width(), 2.5);
    /// ```
    pub fn set_line_width(&mut self, width: f32) {
        let w = width.max(0.0);
        self.state.line_width = w;
        self.commands.push(CanvasCommand::SetLineWidth(w));
    }

    /// Current stroke line width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(100.0, 100.0);
    /// assert_eq!(canvas.line_width(), 1.0);
    /// ```
    pub fn line_width(&self) -> f32 {
        self.state.line_width
    }

    /// Set horizontal text alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, TextAlign};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_text_align(TextAlign::Center);
    /// assert_eq!(canvas.text_align(), TextAlign::Center);
    /// ```
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.state.text_align = align;
    }

    /// Current horizontal text alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, TextAlign};
    ///
    /// let canvas = Canvas::new(100.0, 100.0);
    /// assert_eq!(canvas.text_align(), TextAlign::Left);
    /// ```
    pub fn text_align(&self) -> TextAlign {
        self.state.text_align
    }

    /// Set vertical text baseline.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, TextBaseline};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.set_text_baseline(TextBaseline::Middle);
    /// assert_eq!(canvas.text_baseline(), TextBaseline::Middle);
    /// ```
    pub fn set_text_baseline(&mut self, baseline: TextBaseline) {
        self.state.text_baseline = baseline;
    }

    /// Current vertical text baseline.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, TextBaseline};
    ///
    /// let canvas = Canvas::new(100.0, 100.0);
    /// assert_eq!(canvas.text_baseline(), TextBaseline::Top);
    /// ```
    pub fn text_baseline(&self) -> TextBaseline {
        self.state.text_baseline
    }

    /// Save the current canvas state onto the state stack.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.save();
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn save(&mut self) {
        self.state_stack.push(self.state.clone());
        self.commands.push(CanvasCommand::Save);
    }

    /// Restore the most recently saved canvas state from the stack.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.save();
    /// canvas.restore();
    /// assert_eq!(canvas.commands().len(), 2);
    /// ```
    pub fn restore(&mut self) {
        if let Some(prev) = self.state_stack.pop() {
            self.state = prev;
            self.commands.push(CanvasCommand::Restore);
        }
    }

    // ── RECTANGLES (WHATWG: CanvasRect) ───────────────────────────────────────

    /// Fill a rectangle with the current fill style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill_rect(10.0, 10.0, 50.0, 30.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(CanvasCommand::FillRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Stroke a rectangle outline with the current stroke style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.stroke_rect(5.0, 5.0, 40.0, 20.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(CanvasCommand::StrokeRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Clear all pixels within the specified rectangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.clear_rect(0.0, 0.0, 100.0, 100.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn clear_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(CanvasCommand::ClearRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Fill a rectangle with rounded corners using the specified radius.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill_round_rect(0.0, 0.0, 80.0, 40.0, 4.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill_round_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32) {
        self.commands.push(CanvasCommand::FillRoundRect {
            x,
            y,
            width,
            height,
            radius,
        });
    }

    /// Stroke a rectangle with rounded corners using the specified radius.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.stroke_round_rect(0.0, 0.0, 80.0, 40.0, 4.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn stroke_round_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32) {
        self.commands.push(CanvasCommand::StrokeRoundRect {
            x,
            y,
            width,
            height,
            radius,
        });
    }

    // ── PATHS (WHATWG: CanvasPath, CanvasDrawPath) ────────────────────────────

    /// Starts a new path by emptying the list of subpaths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.begin_path();
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn begin_path(&mut self) {
        self.commands.push(CanvasCommand::BeginPath);
    }

    /// Connects the current point back to the starting point of the subpath.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.close_path();
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn close_path(&mut self) {
        self.commands.push(CanvasCommand::ClosePath);
    }

    /// Moves the starting point of a new subpath to the (x, y) coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.move_to(10.0, 15.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(CanvasCommand::MoveTo { x, y });
    }

    /// Connects the last point in the subpath to (x, y) with a straight line.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.line_to(20.0, 25.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(CanvasCommand::LineTo { x, y });
    }

    /// Adds a circular arc to the current path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.arc(50.0, 50.0, 20.0, 0.0, std::f32::consts::PI);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) {
        self.commands.push(CanvasCommand::Arc {
            x,
            y,
            radius,
            start_angle,
            end_angle,
        });
    }

    /// Fills the current or given path with the current fill style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill();
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill(&mut self) {
        self.commands.push(CanvasCommand::Fill);
    }

    /// Strokes the current or given path with the current stroke style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.stroke();
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn stroke(&mut self) {
        self.commands.push(CanvasCommand::Stroke);
    }

    // ── TEXT (WHATWG: CanvasText) ─────────────────────────────────────────────

    /// Fill text at the specified coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill_text("Hello", 10.0, 20.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill_text(&mut self, text: &str, x: f32, y: f32) {
        self.commands.push(CanvasCommand::FillText {
            text: text.to_string(),
            x,
            y,
        });
    }

    /// Fill text bounded inside a rectangle with alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, TextAlign};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill_text_in_rect("Label", 0.0, 0.0, 100.0, 30.0, TextAlign::Center);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill_text_in_rect(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        align: TextAlign,
    ) {
        self.commands.push(CanvasCommand::FillTextInRect {
            text: text.to_string(),
            x,
            y,
            width,
            height,
            align,
        });
    }

    /// Fill text dynamically bound to a component property (e.g. "text" / "caption").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, TextAlign};
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill_text_prop("caption", 0.0, 0.0, 100.0, 30.0, TextAlign::Left);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill_text_prop(
        &mut self,
        prop_name: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        align: TextAlign,
    ) {
        self.commands.push(CanvasCommand::FillTextProp {
            prop_name: prop_name.to_string(),
            x,
            y,
            width,
            height,
            align,
        });
    }

    // ── PROPORTIONAL PROGRESS & METERS ────────────────────────────────────────

    /// Fill a proportional progress bar area linked to component range properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    /// use crawrad_component::canvas::Axis;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.fill_fraction_rect(Axis::X, "progress", "0", "100");
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn fill_fraction_rect(
        &mut self,
        axis: Axis,
        prop_value: &str,
        prop_min: &str,
        prop_max: &str,
    ) {
        self.commands.push(CanvasCommand::FillFractionRect {
            axis,
            prop_value: prop_value.to_string(),
            prop_min: prop_min.to_string(),
            prop_max: prop_max.to_string(),
        });
    }

    // ── CONVENIENCE UI HELPERS ────────────────────────────────────────────────

    /// Draw a line between two coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.draw_line(0.0, 0.0, 10.0, 10.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.commands
            .push(CanvasCommand::DrawLine { x1, y1, x2, y2 });
    }

    /// Draw a checkmark indicator inside a bounding rectangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.draw_check(2.0, 2.0, 16.0, 16.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn draw_check(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(CanvasCommand::DrawCheck {
            x,
            y,
            width,
            height,
        });
    }

    /// Draw a dashed boundary rectangle (selection or placeholder box).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let mut canvas = Canvas::new(100.0, 100.0);
    /// canvas.draw_dashed_rect(0.0, 0.0, 50.0, 50.0);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    pub fn draw_dashed_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(CanvasCommand::DrawDashedRect {
            x,
            y,
            width,
            height,
        });
    }

    // ── INSPECTION & EXPORT ───────────────────────────────────────────────────

    /// Borrow the recorded commands list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(100.0, 100.0);
    /// assert!(canvas.commands().is_empty());
    /// ```
    pub fn commands(&self) -> &[CanvasCommand] {
        &self.commands
    }

    /// Consume the canvas and return the recorded commands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(100.0, 100.0);
    /// let commands = canvas.into_commands();
    /// assert!(commands.is_empty());
    /// ```
    pub fn into_commands(self) -> Vec<CanvasCommand> {
        self.commands
    }

    /// Convert the recorded canvas commands into manifest `draw` JSON items.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::Canvas;
    ///
    /// let canvas = Canvas::new(100.0, 100.0);
    /// let json = canvas.to_manifest_json();
    /// assert!(json.starts_with('['));
    /// ```
    pub fn to_manifest_json(&self) -> String {
        let mut out = String::from("[\n");
        let mut active_fill: Option<String> = None;
        let mut active_stroke: Option<String> = None;
        let mut active_line_width: f32 = 1.0;
        let mut first = true;

        for cmd in &self.commands {
            match cmd {
                CanvasCommand::SetFillStyle(s) => match s {
                    FillStyle::Theme(role) => active_fill = Some(role.as_str().to_string()),
                    FillStyle::Color(c) => {
                        active_fill = Some(format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b))
                    }
                },
                CanvasCommand::SetStrokeStyle(s) => match s {
                    StrokeStyle::Theme(role) => active_stroke = Some(role.as_str().to_string()),
                    StrokeStyle::Color(c) => {
                        active_stroke = Some(format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b))
                    }
                },
                CanvasCommand::SetLineWidth(w) => active_line_width = *w,
                CanvasCommand::FillRect {
                    x: _,
                    y: _,
                    width: _,
                    height: _,
                } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let fill_str = active_fill.as_deref().unwrap_or("surface");
                    out.push_str(&format!(
                        r#"        {{ "verb": "box", "fill": "{fill_str}" }}"#
                    ));
                }
                CanvasCommand::FillRoundRect {
                    x: _,
                    y: _,
                    width: _,
                    height: _,
                    radius,
                } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let fill_str = active_fill.as_deref().unwrap_or("surface");
                    let border_str = active_stroke.as_deref().unwrap_or("border");
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"box\",\n          \"fill\": \"{fill_str}\",\n          \"border\": \"{border_str}\",\n          \"radius\": {radius:.1}\n        }}"
                    ));
                }
                CanvasCommand::FillFractionRect {
                    axis,
                    prop_value,
                    prop_min,
                    prop_max,
                } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let fill_str = active_fill.as_deref().unwrap_or("accent");
                    let axis_str = axis.as_str();
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"box\",\n          \"fill\": \"{fill_str}\",\n          \"area\": {{ \"fraction_of\": \"{prop_value}\", \"axis\": \"{axis_str}\", \"min\": \"{prop_min}\", \"max\": \"{prop_max}\" }}\n        }}"
                    ));
                }
                CanvasCommand::StrokeRoundRect {
                    x: _,
                    y: _,
                    width: _,
                    height: _,
                    radius: _,
                }
                | CanvasCommand::StrokeRect {
                    x: _,
                    y: _,
                    width: _,
                    height: _,
                } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let color_str = active_stroke.as_deref().unwrap_or("accent");
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"frame\",\n          \"color\": \"{color_str}\",\n          \"thickness\": {active_line_width:.1}\n        }}"
                    ));
                }
                CanvasCommand::FillTextInRect {
                    text,
                    x: _,
                    y: _,
                    width: _,
                    height: _,
                    align,
                } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let color_str = active_fill.as_deref().unwrap_or("text");
                    let centered = *align == TextAlign::Center;
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"text\",\n          \"text\": \"{text}\",\n          \"color\": \"{color_str}\",\n          \"centered\": {centered}\n        }}"
                    ));
                }
                CanvasCommand::FillTextProp {
                    prop_name,
                    x: _,
                    y: _,
                    width: _,
                    height: _,
                    align,
                } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let color_str = active_fill.as_deref().unwrap_or("text");
                    let centered = *align == TextAlign::Center;
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"text\",\n          \"prop\": \"{prop_name}\",\n          \"color\": \"{color_str}\",\n          \"centered\": {centered}\n        }}"
                    ));
                }
                CanvasCommand::DrawCheck { .. } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let color_str = active_stroke.as_deref().unwrap_or("accent");
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"check\",\n          \"color\": \"{color_str}\"\n        }}"
                    ));
                }
                CanvasCommand::DrawDashedRect { .. } => {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    let color_str = active_stroke.as_deref().unwrap_or("text_dim");
                    out.push_str(&format!(
                        "        {{\n          \"verb\": \"dashed\",\n          \"color\": \"{color_str}\"\n        }}"
                    ));
                }
                _ => {}
            }
        }

        out.push_str("\n      ]");
        out
    }
}

/// Trait implemented by components that define custom drawing via the Canvas API.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::{Canvas, Paint};
///
/// struct MyBox;
/// impl Paint for MyBox {
///     fn paint(&self, canvas: &mut Canvas) {
///         canvas.fill_rect(0.0, 0.0, 10.0, 10.0);
///     }
/// }
///
/// let mut canvas = Canvas::new(50.0, 50.0);
/// MyBox.paint(&mut canvas);
/// assert_eq!(canvas.commands().len(), 1);
/// ```
pub trait Paint {
    /// Render this component's visual appearance on the canvas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_component::{Canvas, Paint};
    ///
    /// struct MyBox;
    /// impl Paint for MyBox {
    ///     fn paint(&self, canvas: &mut Canvas) {
    ///         canvas.fill_rect(0.0, 0.0, 10.0, 10.0);
    ///     }
    /// }
    ///
    /// let mut canvas = Canvas::new(50.0, 50.0);
    /// MyBox.paint(&mut canvas);
    /// assert_eq!(canvas.commands().len(), 1);
    /// ```
    fn paint(&self, canvas: &mut Canvas);
}

/// Detects whether a component type implements `Paint`.
///
/// Helper wrapper that uses autoreferencing specialization to detect whether
/// a component type implements `Paint`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::CanvasDetector;
///
/// let _detector = CanvasDetector::<()>::new();
/// assert_eq!(std::mem::size_of::<CanvasDetector<()>>(), 0);
/// ```
pub struct CanvasDetector<T>(pub std::marker::PhantomData<T>);

impl<T> Default for CanvasDetector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CanvasDetector<T> {
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

pub trait CanvasFallback {
    fn extract_draw_json(&self) -> Option<String> {
        None
    }
}

impl<T> CanvasFallback for CanvasDetector<T> {}

pub trait CanvasExtract {
    fn extract_draw_json(&self) -> Option<String>;
}

impl<T: Paint + Default> CanvasExtract for &CanvasDetector<T> {
    fn extract_draw_json(&self) -> Option<String> {
        let mut canvas = Canvas::new(140.0, 48.0);
        let instance = T::default();
        instance.paint(&mut canvas);
        if canvas.commands().is_empty() {
            None
        } else {
            Some(canvas.to_manifest_json())
        }
    }
}

/// Helper to merge an extracted canvas draw JSON into a raw component JSON.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::merge_manifest_draw;
///
/// let raw = "{}";
/// let merged = merge_manifest_draw(raw, None);
/// assert_eq!(merged, "{}");
/// ```
pub fn merge_manifest_draw(raw_json: &str, draw_json: Option<&str>) -> String {
    let Some(draws) = draw_json else {
        return raw_json.to_string();
    };
    if raw_json.contains(r#""draw":"#) {
        return raw_json.to_string();
    }
    if let Some(icon_pos) = raw_json.find(r#""icon":"#) {
        let line_start = raw_json[..icon_pos]
            .rfind('\n')
            .map(|p| p + 1)
            .unwrap_or(icon_pos);
        let (head, tail) = raw_json.split_at(line_start);
        format!("{head}      \"draw\": {draws},\n{tail}")
    } else if let Some(pos) = raw_json.rfind('}') {
        let (head, tail) = raw_json.split_at(pos);
        let trimmed_head = head.trim_end();
        let separator = if trimmed_head.ends_with(',') { "" } else { "," };
        format!("{trimmed_head}{separator}\n      \"draw\": {draws}\n    {tail}")
    } else {
        raw_json.to_string()
    }
}
