use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-spin-edit-spin-edit")]
pub fn spin_edit() -> ComponentDesc {
    let mut d = component!(
        "SpinEdit",
        display: "SpinEdit",
        base_name: "spin_edit",
        palette: "Additional",
        prefix: "spn",
        size: (121.0, 24.0),
        props: [
            Props::spin_value(0),
            Props::min_value(0),
            Props::max_value(0),
            Props::increment(1),
            Props::editor_enabled(true),
        ],
        events: [
            Events::CHANGE,
            Events::ENTER,
            Events::EXIT,
        ],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
