// Recorded drawing operations produced by the Canvas API.

use super::types::{Axis, FillStyle, StrokeStyle, TextAlign};

/// An operation recorded by the Canvas drawing context.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::CanvasCommand;
///
/// let cmd = CanvasCommand::Save;
/// assert_eq!(cmd, CanvasCommand::Save);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CanvasCommand {
    /// Push the current drawing state onto the state stack.
    Save,
    /// Pop the topmost drawing state from the stack and restore properties.
    Restore,
    /// Set the current fill style (color or theme role).
    SetFillStyle(FillStyle),
    /// Set the current stroke style (color or theme role).
    SetStrokeStyle(StrokeStyle),
    /// Set the stroke line thickness.
    SetLineWidth(f32),
    /// Fill a rectangle with the active fill style.
    FillRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Outline a rectangle with the active stroke style.
    StrokeRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Clear a rectangular area.
    ClearRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Fill a rectangle with rounded corners.
    FillRoundRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
    },
    /// Stroke a rectangle with rounded corners.
    StrokeRoundRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
    },
    /// Start a new drawing path.
    BeginPath,
    /// Close the current subpath by drawing a line back to the starting point.
    ClosePath,
    /// Move the pen without drawing a line.
    MoveTo { x: f32, y: f32 },
    /// Add a straight line segment to the current subpath.
    LineTo { x: f32, y: f32 },
    /// Add a circular arc to the current subpath.
    Arc {
        x: f32,
        y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    },
    /// Fill the current path with the active fill style.
    Fill,
    /// Stroke the current path with the active stroke style.
    Stroke,
    /// Render filled text at a given point.
    FillText { text: String, x: f32, y: f32 },
    /// Render filled text bounded inside a rectangle with specified alignment.
    FillTextInRect {
        text: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        align: TextAlign,
    },
    /// Render text dynamically linked to a component property.
    FillTextProp {
        prop_name: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        align: TextAlign,
    },
    /// Fill a proportional progress bar area linked to component range properties.
    FillFractionRect {
        axis: Axis,
        prop_value: String,
        prop_min: String,
        prop_max: String,
    },
    /// Draw a line between two points using the active stroke style.
    DrawLine { x1: f32, y1: f32, x2: f32, y2: f32 },
    /// Draw a checkmark indicator inside a bounding rectangle.
    DrawCheck {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Draw a dashed boundary rectangle (selection or placeholder box).
    DrawDashedRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
}
