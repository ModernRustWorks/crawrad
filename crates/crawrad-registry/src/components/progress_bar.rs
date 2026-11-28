use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-progress-bar-progress-bar")]
pub fn progress_bar() -> ComponentDesc {
    component!(
        "ProgressBar",
        display: "ProgressBar",
        base_name: "progress_bar",
        palette: "Win32",
        prefix: "pb",
        size: (150.0, 16.0),
        props: [
            Props::min(0),
            Props::max(100),
            Props::position(0),
            Props::step(10),
            Props::orientation("Horizontal"),
            Props::smooth(true),
            Props::slint_border_radius(3),
        ],
        events: [
            Events::MOUSE_CLICK,
        ],
    )
}
