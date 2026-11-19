use crate::{ComponentDesc, Events, component, control_props};
use crawrad_note::note;

#[note("components-paint-box-paint-box")]
pub fn paint_box() -> ComponentDesc {
    component!(
        "PaintBox",
        display: "PaintBox",
        base_name: "paint_box",
        palette: "Additional",
        prefix: "pbx",
        size: (105.0, 105.0),
        // Drawn on its parent (classic RAD's `TGraphicControl`): it never
        // takes the focus, so it has no place in the Tab sequence.
        base: control_props(),
        props: [],
        events: [
            Events::PAINT,
            Events::MOUSE_CLICK,
            Events::MOUSE_DOWN,
            Events::MOUSE_UP,
            Events::MOUSE_MOVE,
        ],
    )
}
