use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-list-box-list-box")]
pub fn list_box() -> ComponentDesc {
    let mut d = component!(
        "ListBox",
        display: "ListBox",
        base_name: "list_box",
        palette: "Standard",
        prefix: "lst",
        size: (120.0, 96.0),
        props: [
            Props::flat_items_sample(&["Item1", "Item2", "Item3"]),
            Props::item_index(-1),
            Props::multi_select(false),
            Props::sorted(false),
        ],
        events: [
            Events::CHANGE,
            Events::MOUSE_CLICK,
            Events::MOUSE_DOUBLE_CLICK,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
