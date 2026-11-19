//! A `Breadcrumb` component on the designer canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::breadcrumb::{
    BREADCRUMB_GAP, CHEVRON_OVERLAP, CHEVRON_PADS, CHEVRON_TIP, breadcrumb_layout, chevron_layout,
};
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-breadcrumb-class")]
pub const CLASS: &str = "Breadcrumb";

#[note("components-breadcrumb-drawing")]
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

        let separator = match c.props.get("separator") {
            Some(crawrad_ir::Value::Str(s)) => s.clone(),
            _ => "›".to_string(),
        };

        let style = match c.props.get("style") {
            Some(crawrad_ir::Value::Enum(s)) | Some(crawrad_ir::Value::Str(s)) => s.as_str(),
            _ => "bsText",
        };

        let seg_color = match c.props.get("segment_color") {
            Some(crawrad_ir::Value::Color(k)) => [k.r, k.g, k.b, k.a].map(|v| f32::from(v) / 255.0),
            _ => [0.271, 0.275, 0.294, 1.0], // #45464b
        };

        let cur_color = match c.props.get("current_color") {
            Some(crawrad_ir::Value::Color(k)) => [k.r, k.g, k.b, k.a].map(|v| f32::from(v) / 255.0),
            _ => [0.878, 0.380, 0.169, 1.0], // #e0612b
        };

        let item_index = match c.props.get("item_index") {
            Some(crawrad_ir::Value::Int(i)) => *i,
            _ => -1,
        };

        let alpha = if c.enabled { 1.0 } else { 0.5 };

        // real widths, measured by the canvas fonts (on-screen, zoom included)
        let widths: Vec<f32> = items.iter().map(|it| p.measure(it).x as f32).collect();
        let count = items.len();

        if style == "bsChevron" {
            let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
            let current_idx = if item_index >= 0 && (item_index as usize) < count {
                item_index as usize
            } else {
                count.saturating_sub(1)
            };

            p.with_clip(r, |p| {
                for (i, (&(x, w), it)) in positions.iter().zip(items.iter()).enumerate() {
                    if x as f64 >= r.size.x {
                        break;
                    }
                    let is_current = i == current_idx;
                    let bg = if is_current {
                        [
                            cur_color[0],
                            cur_color[1],
                            cur_color[2],
                            cur_color[3] * alpha,
                        ]
                    } else {
                        [
                            seg_color[0],
                            seg_color[1],
                            seg_color[2],
                            seg_color[3] * alpha,
                        ]
                    };
                    // The block row by row, each a 1-point strip between its two
                    // slanted edges: the canvas has no polygon shader, and a new
                    // one would have to join its shader-ordered layers.
                    let left = r.pos.x + x as f64;
                    let (tip, height) = (f64::from(CHEVRON_TIP), r.size.y);
                    let notched = if i == 0 { 0.0 } else { 1.0 };
                    for row in 0..height.ceil() as usize {
                        let y = row as f64 + 0.5;
                        let slope = ((y - height / 2.0) / (height / 2.0)).abs().min(1.0);
                        let from = left + notched * tip * (1.0 - slope);
                        let to = left + f64::from(w) - tip * slope;
                        let strip = Rect {
                            pos: dvec2(from, r.pos.y + row as f64),
                            size: dvec2((to - from).max(0.0), 1.0),
                        };
                        p.rounded_box(strip, bg, bg, 0.0);
                    }

                    let left_pad = if i == 0 {
                        CHEVRON_PADS.0
                    } else {
                        CHEVRON_PADS.1
                    };
                    let text_r = Rect {
                        pos: dvec2(r.pos.x + x as f64 + left_pad as f64, r.pos.y),
                        size: dvec2(widths[i] as f64, r.size.y),
                    };
                    let fg = if is_current {
                        [1.0, 1.0, 1.0, alpha]
                    } else {
                        [0.804, 0.839, 0.957, alpha]
                    };
                    p.write(text_r, it, fg, false);
                }
            });
            return;
        }

        let sep_w = p.measure(&separator).x as f32;
        let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);

        let link_fg = [0.537, 0.706, 0.980, alpha]; // #89b4fa
        let current_fg = [0.804, 0.839, 0.957, alpha]; // #cdd6f4
        let sep_fg = [0.424, 0.439, 0.525, alpha]; // #6c7086

        let count = items.len();
        p.with_clip(r, |p| {
            for (i, (&x, it)) in positions.iter().zip(items.iter()).enumerate() {
                if x as f64 >= r.size.x {
                    break;
                }
                let is_last = i + 1 == count;
                let w = widths[i];
                let fg = if is_last { current_fg } else { link_fg };
                let seg_r = Rect {
                    pos: dvec2(r.pos.x + x as f64, r.pos.y),
                    size: dvec2(w as f64, r.size.y),
                };
                p.write(seg_r, it, fg, false);

                if !is_last {
                    let sep_x = x + w + BREADCRUMB_GAP;
                    if (sep_x as f64) < r.size.x {
                        let sep_r = Rect {
                            pos: dvec2(r.pos.x + sep_x as f64, r.pos.y),
                            size: dvec2(sep_w as f64, r.size.y),
                        };
                        p.write(sep_r, &separator, sep_fg, false);
                    }
                }
            }
        });
    }
}
