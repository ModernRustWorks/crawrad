use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-radio-group-radio-group")]
pub fn radio_group() -> ComponentDesc {
    let mut d = component!(
        "RadioGroup",
        display: "RadioGroup",
        base_name: "radio_group",
        palette: "Standard",
        prefix: "rgp",
        size: (185.0, 105.0),
        props: [
            Props::caption("RadioGroup"),
            Props::flat_items_sample(&["Option 1", "Option 2", "Option 3"]),
            Props::item_index(-1),
            Props::columns(1),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::CHANGE,
        ],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
