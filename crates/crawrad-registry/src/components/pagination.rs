use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-pagination-pagination")]
pub fn pagination() -> ComponentDesc {
    let mut d = component!(
        "Pagination",
        display: "Pagination",
        base_name: "pagination",
        palette: "Additional",
        prefix: "pag",
        size: (280.0, 28.0),
        props: [
            Props::page_count(10),
            Props::page_index(0),
            Props::max_buttons(7),
            Props::show_arrows(true),
        ],
        events: [
            Events::CHANGE,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
