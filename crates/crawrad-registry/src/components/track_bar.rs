use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-track-bar-track-bar")]
pub fn track_bar() -> ComponentDesc {
    component!(
        "TrackBar",
        display: "TrackBar",
        base_name: "track_bar",
        palette: "Win32",
        prefix: "tb",
        size: (150.0, 32.0),
        props: [
            Props::min(0),
            Props::max(10),
            Props::position(0),
            Props::orientation("Horizontal"),
            Props::page_size(2),
            Props::makepad_slider_look("Minimal"),
        ],
        events: [
            Events::CHANGE,
            Events::ENTER,
            Events::EXIT,
            Events::KEY_DOWN,
            Events::KEY_UP,
        ],
    )
}
