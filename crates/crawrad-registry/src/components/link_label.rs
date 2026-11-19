use crate::components::base::control_props;
use crate::{ComponentDesc, Events, Props, TextLine, component};
use crawrad_note::note;

#[note("components-link-label-link-label")]
pub fn link_label() -> ComponentDesc {
    let mut d = component!(
        "LinkLabel",
        display: "LinkLabel",
        base_name: "link_label",
        palette: "Additional",
        prefix: "lnk",
        size: (120.0, 20.0),
        text: TextLine::Top(2.0),
        base: control_props(),
        props: [
            Props::caption("LinkLabel"),
            Props::url(""),
        ],
        events: [Events::MOUSE_CLICK],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
