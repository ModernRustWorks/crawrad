//! Makepad Image component, drawn on the canvas.

use crawrad_ir::Value;
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::canvas::atlas::fit_rect;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-image-classe")]
pub const CLASSE: &str = "Image";

#[note("components-image-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let bytes = match c.props.get("picture") {
            Some(Value::Picture(pic)) if !pic.is_empty() => pic.bytes(),
            _ => &[],
        };
        let stretch = matches!(c.props.get("stretch"), Some(Value::Bool(true)));
        let proportional = matches!(c.props.get("proportional"), Some(Value::Bool(true)));
        let center = matches!(c.props.get("center"), Some(Value::Bool(true)));

        let mut painted = false;
        if !bytes.is_empty()
            && let Some((w, h)) = p.picture_size(bytes)
        {
            let img_w = p.medida(w as f32);
            let img_h = p.medida(h as f32);
            let target = fit_rect(r, img_w, img_h, stretch, proportional, center);
            p.with_clip(r, |p| {
                painted = p.picture(target, bytes);
            });
        }

        if !painted {
            // An empty or undecodable Image draws a thin frame, never the generic dashed box.
            p.rounded_box(r, [0.0; 4], MAKEPAD_BORDER, 0.0);
        }
    }
}
