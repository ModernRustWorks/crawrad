use crate::components::base::control_props;
use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-badge-badge")]
pub fn badge() -> ComponentDesc {
    let mut d = component!(
        "Badge",
        display: "Badge",
        base_name: "badge",
        palette: "Additional",
        prefix: "bdg",
        size: (56.0, 20.0),
        base: control_props(),
        props: [
            Props::caption("New"),
            Props::badge_kind("bkPrimary"),
            Props::outline(false),
            Props::count(0),
            Props::max_count(99),
            Props::associate(),
        ],
        events: [Events::MOUSE_CLICK],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
