use crate::{ComponentDesc, Events, Props, TextLine, component};

pub fn button() -> ComponentDesc {
    let mut d = component!(
        "Button",
        display: "Button",
        base_name: "button",
        palette: "Standard",
        prefix: "btn",
        size: (96.0, 32.0),
        text: TextLine::Centered,
        props: [
            Props::action(),
            Props::caption("Button"),
        ],
        events: [
            Events::MOUSE_CLICK,
            Events::ENTER,
            Events::EXIT,
            Events::KEY_DOWN,
            Events::KEY_PRESS,
            Events::KEY_UP,
            Events::MOUSE_ACTIVATE,
            Events::MOUSE_DOWN,
            Events::MOUSE_ENTER,
            Events::MOUSE_LEAVE,
            Events::MOUSE_MOVE,
            Events::MOUSE_UP,
        ],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    d
}
