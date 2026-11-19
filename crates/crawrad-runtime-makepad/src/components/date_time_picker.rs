//! A `DateTimePicker` on the designer canvas: the `Edit`'s box, showing its
//! formatted date or time string — `dd/mm/yyyy` for dates, `HH:MM:SS` for times.

use crawrad_ir::Value;
use crawrad_runtime::date_time::{DateTimeKind, iso_to_shown};
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::edit::draw_text_box;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-date-time-picker-class")]
pub const CLASS: &str = "DateTimePicker";

#[note("components-date-time-picker-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let kind = match c.props.get("kind") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => DateTimeKind::parse_kind(s),
            _ => DateTimeKind::Date,
        };
        let date_time = match c.props.get("date_time") {
            Some(Value::Str(s)) if !s.is_empty() => s.as_str(),
            _ => kind.default_value(),
        };
        let shown = iso_to_shown(kind, date_time);
        draw_text_box(p, c, r, &shown);
    }
}
