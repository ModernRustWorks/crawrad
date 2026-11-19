use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-breadcrumb-breadcrumb")]
pub fn breadcrumb() -> ComponentDesc {
    let mut d = component!(
        "Breadcrumb",
        display: "Breadcrumb",
        base_name: "breadcrumb",
        palette: "Additional",
        prefix: "brc",
        size: (260.0, 24.0),
        props: [
            Props::flat_items_sample(&["Home", "Projects", "CrawRAD"]),
            Props::separator("›"),
            Props::item_index(-1),
            Props::breadcrumb_style("bsText"),
            Props::segment_color(crawrad_ir::Color::rgb(0x45, 0x46, 0x4B)),
            Props::current_color(crawrad_ir::Color::rgb(0xE0, 0x61, 0x2B)),
        ],
        events: [
            Events::CHANGE,
            Events::MOUSE_CLICK,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
