use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-multi-select-multi-select")]
pub fn multi_select() -> ComponentDesc {
    let mut d = component!(
        "MultiSelect",
        display: "MultiSelect",
        base_name: "multi_select",
        palette: "Additional",
        prefix: "mse",
        size: (180.0, 24.0),
        props: [
            Props::flat_items_sample(&["Rust", "egui", "Makepad", "Slint", "Iced"]),
            Props::selected_items(),
            Props::text_hint("Select…"),
            Props::drop_down_count(6),
            Props::slint_border_radius(4),
        ],
        events: [
            Events::CHANGE,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
