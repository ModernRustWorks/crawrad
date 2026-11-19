use crate::{ComponentDesc, Events, Props, TextLine, component};
use crawrad_note::note;

#[note("components-bit-btn-bit-btn")]
pub fn bit_btn() -> ComponentDesc {
    component!(
        "BitBtn",
        display: "BitBtn",
        base_name: "bit_btn",
        palette: "Additional",
        prefix: "bbtn",
        size: (96.0, 32.0),
        text: TextLine::Centered,
        props: [
            Props::action(),
            Props::caption("BitBtn"),
            Props::bit_btn_kind("bkCustom"),
            Props::glyph(),
            Props::num_glyphs(1),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::ENTER,
            Events::EXIT,
            Events::KEY_DOWN,
            Events::KEY_PRESS,
            Events::KEY_UP,
            Events::MOUSE_ACTIVATE,
            Events::MOUSE_DOWN,
            Events::MOUSE_ENTER,
            Events::MOUSE_LEAVE,
            Events::MOUSE_MOVE,
            Events::MOUSE_UP,
        ],
    )
}
