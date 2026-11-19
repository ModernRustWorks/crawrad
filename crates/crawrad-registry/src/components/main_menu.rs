use crate::components::base::{ICON_SIZE, non_visual_props};
use crate::{Children, ComponentDesc, Events, Props, UNIVERSAL_TARGETS, component};
use crawrad_note::note;

#[note("components-main-menu-main-menu")]
pub fn main_menu() -> ComponentDesc {
    component!(
        "MainMenu",
        display: "MainMenu",
        base_name: "main_menu",
        palette: "Standard",
        prefix: "mnu",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        children: Children::Collection { item: "MenuItem" },
        base: non_visual_props(),
        props: [
            Props::images(),
        ],
    )
}

#[note("components-main-menu-menu-item")]
pub fn menu_item() -> ComponentDesc {
    let mut props = non_visual_props();
    props.extend([
        Props::caption(""),
        Props::shortcut(""),
        Props::checked(false),
        Props::enabled(true),
        Props::visible(true),
        Props::action(),
    ]);
    ComponentDesc {
        inherits: None,
        class: "MenuItem",
        // Off the palette: an item is born inside a menu, never loose on the
        // form.
        palette: "",
        display: "MenuItem",
        base_name: "menu_item",
        prefix: "mni",
        default_size: ICON_SIZE,
        props,
        events: vec![Events::MOUSE_CLICK],
        targets: UNIVERSAL_TARGETS,
        children: Children::Collection { item: "MenuItem" },
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
