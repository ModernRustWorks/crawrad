use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-group-box-group-box")]
pub fn group_box() -> ComponentDesc {
    let mut d = component!(
        "GroupBox",
        display: "GroupBox",
        base_name: "group_box",
        palette: "Standard",
        prefix: "gbx",
        size: (240.0, 120.0),
        children: Children::Area,
        props: [Props::caption("GroupBox")],
        // The target already delivered it
        events: [Events::MOUSE_CLICK],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    // See `without_tab_stop`
    crate::components::base::without_tab_stop(&mut d.props);
    d
}
