use crate::components::base::{ICON_SIZE, non_visual_props};
use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-timer-timer")]
pub fn timer() -> ComponentDesc {
    component!(
        "Timer",
        display: "Timer",
        base_name: "timer",
        palette: "System",
        prefix: "tmr",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        base: non_visual_props(),
        props: [
            Props::enabled(true),
            Props::interval(1000),
        ],
        events: [Events::TIMER],
    )
}
