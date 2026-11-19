use crate::components::base::control_props;
use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-category-panel-category-panel")]
pub fn category_panel() -> ComponentDesc {
    let mut d = component!(
        "CategoryPanel",
        display: "CategoryPanel",
        base_name: "category_panel",
        palette: "Additional",
        prefix: "cat",
        size: (200.0, 160.0),
        children: Children::Area,
        // its header is a click target, not a focus stop: no TabOrder
        base: control_props(),
        props: [
            Props::caption("CategoryPanel"),
            Props::collapsed(false),
        ],
        events: [Events::COLLAPSED, Events::EXPANDED],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
