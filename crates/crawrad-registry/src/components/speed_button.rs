use crate::{ComponentDesc, Events, Props, TextLine, component};
use crawrad_note::note;

#[note("components-speed-button-speed-button")]
pub fn speed_button() -> ComponentDesc {
    component!(
        "SpeedButton",
        display: "SpeedButton",
        base_name: "speed_button",
        palette: "Additional",
        prefix: "sbtn",
        size: (28.0, 28.0),
        text: TextLine::Centered,
        props: [
            Props::action(),
            Props::caption(""),
            Props::glyph(),
            Props::num_glyphs(1),
            Props::flat(true),
            Props::down(false),
            Props::group_index(0),
            Props::allow_all_up(false),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::ENTER,
            Events::EXIT,
            Events::MOUSE_DOWN,
            Events::MOUSE_ENTER,
            Events::MOUSE_LEAVE,
            Events::MOUSE_UP,
        ],
    )
}
