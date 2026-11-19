use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-tool-bar-tool-bar")]
pub fn tool_bar() -> ComponentDesc {
    let mut d = component!(
        "ToolBar",
        display: "ToolBar",
        base_name: "tool_bar",
        palette: "Win32",
        prefix: "tbr",
        size: (320.0, 32.0),
        children: Children::Area,
        props: [
            Props::flat(true),
            Props::show_captions(false),
            Props::button_width(24),
            Props::button_height(24),
        ],
        events: [
            Events::MOUSE_CLICK,
        ],
    );
    // See `without_tab_stop`
    crate::components::base::without_tab_stop(&mut d.props);
    d
}
