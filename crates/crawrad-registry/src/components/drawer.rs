use crate::components::base::control_props;
use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-drawer-drawer")]
pub fn drawer() -> ComponentDesc {
    let mut d = component!(
        "Drawer",
        display: "Drawer",
        base_name: "drawer",
        palette: "Additional",
        prefix: "drw",
        size: (240.0, 320.0),
        children: Children::Area,
        // Click target, not a focus stop: no TabOrder
        base: control_props(),
        props: [
            Props::caption("Drawer"),
            Props::side("dsLeft"),
            Props::opened(false),
            Props::show_header(true),
            Props::overlay(true),
        ],
        events: [Events::CLOSE],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    d
}
