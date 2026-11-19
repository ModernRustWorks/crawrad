//! A `MaskEdit` on the designer canvas: the `Edit`'s box, showing its
//! text shaped by the `EditMask` — the blank mask when the text is empty, as
//! the classic designer shows it.

use crawrad_ir::Value;
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::edit::draw_text_box;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-mask-edit-class")]
pub const CLASS: &str = "MaskEdit";

#[note("components-mask-edit-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let mask = match c.props.get("edit_mask") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        draw_text_box(p, c, r, &crawrad_runtime::mask::shown_text(mask, &c.text));
    }
}
