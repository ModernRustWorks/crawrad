use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-combo-box-combo-box")]
pub fn combo_box() -> ComponentDesc {
    let mut d = component!(
        "ComboBox",
        display: "ComboBox",
        base_name: "combo_box",
        palette: "Standard",
        prefix: "cbo",
        size: (145.0, 24.0),
        props: [
            Props::flat_items_sample(&["Item1", "Item2", "Item3"]),
            // `-1` means none selected, as in the classic RAD IDE.
            Props::item_index(-1),
            Props::text(""),
            Props::combo_box_style("DropDownList"),
            Props::sorted(false),
            Props::drop_down_count(8),
        ],
        events: [
            Events::CHANGE,
            Events::MOUSE_CLICK,
            Events::SELECT,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
