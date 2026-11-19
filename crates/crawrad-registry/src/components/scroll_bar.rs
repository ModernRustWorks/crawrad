use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-scroll-bar-scroll-bar")]
pub fn scroll_bar() -> ComponentDesc {
    component!(
        "ScrollBar",
        display: "ScrollBar",
        base_name: "scroll_bar",
        palette: "Standard",
        prefix: "sb",
        size: (120.0, 16.0),
        props: [
            Props::scroll_bar_kind(),
            Props::min(0),
            Props::max(100),
            Props::position(0),
            Props::small_change(1),
            Props::large_change(10),
        ],
        events: [
            Events::CHANGE,
            Events::SCROLL,
        ],
    )
}
