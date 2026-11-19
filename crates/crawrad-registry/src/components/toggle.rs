use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-toggle-toggle")]
pub fn toggle() -> ComponentDesc {
    let mut d = component!(
        "Toggle",
        display: "Toggle",
        base_name: "toggle",
        palette: "Additional",
        prefix: "tgl",
        size: (120.0, 24.0),
        props: [
            Props::caption("Toggle"),
            Props::checked(false),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::CHANGE,
        ],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
