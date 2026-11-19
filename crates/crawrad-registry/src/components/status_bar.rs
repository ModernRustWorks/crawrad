use crate::{
    Children, ComponentDesc, Events, ICON_SIZE, Props, UNIVERSAL_TARGETS, component,
    non_visual_props,
};
use crawrad_note::note;

#[note("components-status-bar-status-bar")]
pub fn status_bar() -> ComponentDesc {
    let mut d = component!(
        "StatusBar",
        display: "StatusBar",
        base_name: "status_bar",
        palette: "Win32",
        prefix: "stb",
        size: (320.0, 20.0),
        children: Children::Collection { item: "StatusPanel" },
        props: [
            Props::simple_text(""),
            Props::simple_panel(true),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::MOUSE_DOUBLE_CLICK,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}

#[note("components-status-bar-status-panel")]
pub fn status_panel() -> ComponentDesc {
    let mut props = non_visual_props();
    props.extend([
        Props::caption(""),
        Props::panel_width(50),
        Props::alignment("taLeftJustify"),
    ]);
    ComponentDesc {
        inherits: None,
        class: "StatusPanel",
        // Off the palette: a panel is born inside a StatusBar, never loose on
        // the form.
        palette: "",
        display: "StatusPanel",
        base_name: "status_panel",
        prefix: "stp",
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
