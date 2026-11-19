//! A `Chips` component on the designer canvas.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::chips::{CHIP_GAP, CHIP_HEIGHT, CHIP_PAD_X, chip_width, chips_layout};
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-chips-class")]
pub const CLASS: &str = "Chips";

#[note("components-chips-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let items: Vec<String> = match c.props.get("items") {
            Some(v) => crawrad_ir::Item::list_from(v)
                .into_iter()
                .map(|it| it.text)
                .collect(),
            None => Vec::new(),
        };

        let removable = matches!(c.props.get("removable"), Some(Value::Bool(true)));
        let item_index = match c.props.get("item_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };

        let alpha = if c.enabled { 1.0 } else { 0.5 };

        let widths: Vec<f32> = items
            .iter()
            .map(|it| chip_width(it, removable, 0.0))
            .collect();
        let positions = chips_layout(&widths, r.size.x as f32, CHIP_HEIGHT, CHIP_GAP);

        let sel_bg = [0.227, 0.349, 0.549, alpha]; // #3a598c
        let unsel_bg = [0.192, 0.192, 0.251, alpha]; // #313140
        let sel_fg = [1.0, 1.0, 1.0, alpha];
        let unsel_fg = [0.804, 0.839, 0.957, alpha]; // #cdd6f4
        let remove_fg = [0.424, 0.439, 0.525, alpha]; // #6c7086

        for (i, (&(x, y), it)) in positions.iter().zip(items.iter()).enumerate() {
            if y + CHIP_HEIGHT > r.size.y as f32 {
                break;
            }
            let w = widths[i];
            let chip_r = Rect {
                pos: dvec2(r.pos.x + x as f64, r.pos.y + y as f64),
                size: dvec2(w as f64, CHIP_HEIGHT as f64),
            };

            let is_sel = item_index >= 0 && item_index as usize == i;
            let bg = if is_sel { sel_bg } else { unsel_bg };
            let fg = if is_sel { sel_fg } else { unsel_fg };

            let radius = (CHIP_HEIGHT * 0.5) as f64;
            p.rounded_box(chip_r, bg, bg, radius);

            if removable {
                let text_r = Rect {
                    pos: chip_r.pos,
                    size: dvec2((w - 14.0) as f64, CHIP_HEIGHT as f64),
                };
                p.write(text_r, it, fg, true);
                let remove_r = Rect {
                    pos: dvec2(chip_r.pos.x + (w - CHIP_PAD_X * 2.0) as f64, chip_r.pos.y),
                    size: dvec2((CHIP_PAD_X * 2.0) as f64, CHIP_HEIGHT as f64),
                };
                p.write(remove_r, "×", remove_fg, true);
            } else {
                p.write(chip_r, it, fg, true);
            }
        }
    }
}
