use crate::components::base::{ICON_SIZE, non_visual_props};
use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-popup-menu-popup-menu")]
pub fn popup_menu() -> ComponentDesc {
    component!(
        "PopupMenu",
        display: "PopupMenu",
        base_name: "popup_menu",
        palette: "Standard",
        prefix: "pop",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        children: Children::Collection { item: "MenuItem" },
        base: non_visual_props(),
        props: [
            Props::images(),
        ],
        events: [
            Events::POPUP,
        ],
    )
}
