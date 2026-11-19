use crate::components::base::control_props;
use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-card-card")]
pub fn card() -> ComponentDesc {
    let mut d = component!(
        "Card",
        display: "Card",
        base_name: "card",
        palette: "Additional",
        prefix: "crd",
        size: (220.0, 140.0),
        children: Children::Area,
        // Click target, not a focus stop: no TabOrder
        base: control_props(),
        props: [
            Props::caption("Card"),
            Props::subtitle(""),
            Props::corner_radius(8),
            Props::elevation(1),
            Props::show_header(true),
        ],
        events: [Events::MOUSE_CLICK],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    d
}
