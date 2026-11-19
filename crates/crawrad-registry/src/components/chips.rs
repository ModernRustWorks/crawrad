use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-chips-chips")]
pub fn chips() -> ComponentDesc {
    let mut d = component!(
        "Chips",
        display: "Chips",
        base_name: "chips",
        palette: "Additional",
        prefix: "chp",
        size: (220.0, 56.0),
        props: [
            Props::flat_items_sample(&["Rust", "egui", "Makepad"]),
            Props::item_index(-1),
            Props::removable(false),
        ],
        events: [
            Events::CHANGE,
            Events::MOUSE_CLICK,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
