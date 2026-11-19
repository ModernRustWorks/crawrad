use crate::components::base::control_props;
use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-shape-shape")]
pub fn shape() -> ComponentDesc {
    component!(
        "Shape",
        display: "Shape",
        base_name: "shape",
        palette: "Additional",
        prefix: "shp",
        size: (64.0, 64.0),
        // a graphic control, as in the classic RAD IDE: it takes no focus, so
        // it has no TabOrder
        base: control_props(),
        props: [
            Props::shape_kind("Rectangle"),
            Props::pen_width(1),
            // The classic Brush (white) and Pen (black)
            Props::brush_color(crawrad_ir::Color::rgb(255, 255, 255)),
            Props::pen_color(crawrad_ir::Color::rgb(0, 0, 0)),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::MOUSE_DOWN,
            Events::MOUSE_UP,
        ],
    )
}
