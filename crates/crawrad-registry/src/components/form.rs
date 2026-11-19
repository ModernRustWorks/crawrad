use crate::components::base::window_props;
use crate::{Children, ComponentDesc, Events, Props, UNIVERSAL_TARGETS};

pub fn form() -> ComponentDesc {
    let mut props = window_props();
    // A window control's layout and Tab keys mean nothing on the form
    // itself — the form IS the window. Loading drops them from old files.
    props.retain(|p| !crawrad_ir::FORM_RETIRED_KEYS.contains(&p.key));
    props.push(Props::caption("Form1"));
    // The window's colour (transparent = the theme's own), where it
    // opens and how
    props.push(Props::color(crawrad_ir::Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }));
    // The font the form's controls take when their ParentFont is on
    props.push(Props::font(crate::components::base::theme_font()));
    props.push(Props::form_position("poDesigned"));
    props.push(Props::window_state("wsNormal"));
    props.push(Props::form_theme("thSystem"));
    ComponentDesc {
        inherits: None,
        class: "Form",
        display: "Form",
        base_name: "form",
        palette: "", // does not appear on the palette: already exists in the document
        prefix: "form",
        default_size: (640.0, 420.0),
        props,
        events: vec![
            Events::CREATE,
            Events::CLOSE,
            Events::SHOW,
            Events::HIDE,
            Events::ACTIVATE,
            Events::DEACTIVATE,
            Events::CLOSE_QUERY,
            Events::DESTROY,
            Events::RESIZE,
            Events::PAINT,
            Events::KEY_DOWN,
            Events::KEY_PRESS,
            Events::KEY_UP,
            Events::MOUSE_ACTIVATE,
            Events::MOUSE_DOWN,
            Events::MOUSE_ENTER,
            Events::MOUSE_LEAVE,
            Events::MOUSE_MOVE,
            Events::MOUSE_UP,
            Events::MOUSE_WHEEL,
            Events::MOUSE_DOUBLE_CLICK,
            Events::MOUSE_CLICK,
        ],
        targets: UNIVERSAL_TARGETS,
        children: Children::Area,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        // Native has no hidden field: its type is emitted by us.
        private_state: false,
        // Native offers no capability beyond its own class.
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    }
}
