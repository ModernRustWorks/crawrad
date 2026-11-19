//! MainMenu component - top-level window menu bar.
//!
//! The component itself is a badge, as every non-visual; its bar is
//! drawn along the top of the form by [`draw_bar`], the strip the generated
//! applications show.

use crawrad_ir::{Rect as RectIr, Value};
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::Scene;
use crate::components::comum::*;
use crate::components::non_visual::Badge;
use crawrad_note::note;

pub static DESENHO: Badge = Badge::for_class("MainMenu").with_form_strip(draw_bar);

#[note("components-main-menu-bar-height")]
pub const BAR_HEIGHT: f32 = 22.0;

#[note("components-main-menu-entry-padding")]
const ENTRY_PADDING: f32 = 8.0;

#[note("components-main-menu-draw-bar")]
pub fn draw_bar(p: &mut Brush, menu: &MakepadCtl, form: RectIr, scene: &Scene) -> bool {
    let entries: Vec<&str> = scene
        .children_of_class(menu, "MenuItem")
        .filter(|i| !matches!(i.props.get("visible"), Some(Value::Bool(false))))
        .filter_map(|i| match i.props.get("text") {
            Some(Value::Str(s)) if s != "-" => Some(s.as_str()),
            _ => None,
        })
        .collect();
    if entries.is_empty() {
        return false;
    }
    let bar = p.na_tela(RectIr::new(form.x, form.y, form.w, BAR_HEIGHT));
    p.rounded_box(bar, MAKEPAD_PANEL, MAKEPAD_PANEL, 0.0);
    let pad = p.medida(ENTRY_PADDING);
    let mut x = bar.pos.x;
    for text in entries {
        let w = p.measure(&without_accelerator(text)).x + 2.0 * pad;
        let area = Rect {
            pos: dvec2(x + pad, bar.pos.y),
            size: dvec2(w - 2.0 * pad, bar.size.y),
        };
        p.write_caption(area, text, MAKEPAD_TEXT, false);
        x += w;
    }
    true
}

#[note("components-main-menu-without-accelerator")]
fn without_accelerator(text: &str) -> String {
    text.replace('&', "")
}
