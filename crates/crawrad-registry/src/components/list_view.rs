use crate::{
    Children, ComponentDesc, Events, ICON_SIZE, Props, UNIVERSAL_TARGETS, component,
    non_visual_props,
};
use crawrad_note::note;

#[note("components-list-view-list-view")]
pub fn list_view() -> ComponentDesc {
    let mut d = component!(
        "ListView",
        display: "ListView",
        base_name: "list_view",
        palette: "Win32",
        prefix: "lvw",
        size: (240.0, 150.0),
        children: Children::Collection { item: "ListColumn" },
        props: [
            Props::list_view_items_sample(),
            Props::item_index(-1),
            Props::row_select(true),
            Props::grid_lines(false),
            Props::show_column_headers(true),
        ],
        events: [
            Events::CHANGE,
            Events::COLUMN_CLICK,
            Events::MOUSE_CLICK,
            Events::MOUSE_DOUBLE_CLICK,
            Events::SELECT_ITEM,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}

#[note("components-list-view-list-column")]
pub fn list_column() -> ComponentDesc {
    let mut props = non_visual_props();
    props.extend([
        Props::caption(""),
        Props::panel_width(80),
        Props::alignment("taLeftJustify"),
    ]);
    ComponentDesc {
        inherits: None,
        class: "ListColumn",
        palette: "",
        display: "ListColumn",
        base_name: "list_column",
        prefix: "lvc",
        default_size: ICON_SIZE,
        props,
        events: Vec::new(),
        targets: UNIVERSAL_TARGETS,
        children: Children::NoChildren,
        derived_geometry: false,
        visual: false,
        emits_code: true,
        private_state: false,
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    }
}
