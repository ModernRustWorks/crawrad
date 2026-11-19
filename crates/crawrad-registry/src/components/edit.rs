use crate::{ComponentDesc, Events, Props, TextLine, component};

pub fn edit() -> ComponentDesc {
    let mut d = component!(
        "Edit",
        display: "Edit",
        base_name: "edit",
        palette: "Standard",
        prefix: "edt",
        size: (200.0, 24.0),
        text: TextLine::Centered,
        props: [
            Props::text("").bindable(),
            Props::password_char(""),
            Props::readonly(false),
            Props::text_hint(""),
            Props::numbers_only(false),
            Props::max_length(0),
            Props::alignment("taLeftJustify"),
            Props::char_case("ecNormal"),
        ],
        events: [
            Events::CHANGE,
            Events::ENTER,
            Events::EXIT,
            Events::KEY_DOWN,
            Events::KEY_UP,
            Events::KEY_PRESS,
            Events::RETURN,
        ],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    d
}
