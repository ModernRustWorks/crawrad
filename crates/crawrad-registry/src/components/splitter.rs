use crate::components::base::control_props;
use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-splitter-splitter")]
pub fn splitter() -> ComponentDesc {
    component!(
        "Splitter",
        display: "Splitter",
        base_name: "splitter",
        palette: "Additional",
        prefix: "spl",
        size: (5.0, 100.0),
        // a graphic control, as in the classic RAD IDE: it takes no focus, so
        // it has no TabOrder
        base: control_props(),
        props: [
            Props::min_size(30),
            Props::auto_snap(true),
            Props::beveled(true),
        ],
        events: [Events::MOVED],
    )
}
