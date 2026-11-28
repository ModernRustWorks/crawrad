use crate::{ComponentDesc, Events, Props, TextLine, component};
use crawrad_note::note;

#[note("components-radio-button-radio-button")]
pub fn radio_button() -> ComponentDesc {
    let mut d = component!(
        "RadioButton",
        display: "RadioButton",
        base_name: "radio_button",
        palette: "Standard",
        prefix: "rb",
        size: (120.0, 20.0),
        text: TextLine::Centered,
        props: [
            Props::caption("RadioButton"),
            Props::checked(false),
            Props::slint_border_radius(8),
        ],
        events: [Events::MOUSE_CLICK, Events::CHANGE],
    );
    // Font and ParentFont (its draw_bg is the box itself, not a
    // background, so it takes no Color)
    d.props.extend(crate::components::base::font_props());
    d
}
