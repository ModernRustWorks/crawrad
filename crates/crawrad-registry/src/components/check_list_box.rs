use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-check-list-box-check-list-box")]
pub fn check_list_box() -> ComponentDesc {
    let mut d = component!(
        "CheckListBox",
        display: "CheckListBox",
        base_name: "check_list_box",
        palette: "Additional",
        prefix: "clb",
        size: (180.0, 120.0),
        props: [
            Props::flat_items_sample(&["Rust", "egui", "Makepad"]),
            Props::checked_items(),
            Props::slint_border_radius(4),
        ],
        events: [
            Events::CHANGE,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
