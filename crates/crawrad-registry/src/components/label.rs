use crate::components::base::control_props;
use crate::{ComponentDesc, Events, Props, TextLine, component};

pub fn label() -> ComponentDesc {
    let mut d = component!(
        "Label",
        display: "Label",
        base_name: "label",
        palette: "Standard",
        prefix: "lbl",
        size: (100.0, 20.0),
        text: TextLine::Top(2.0),
        // Label does not receive focus: stops at the control rung. In the classic RAD IDE it is the
        // difference between descending from graphic control and window control.
        base: control_props(),
        props: [
            Props::caption("Label"),
            // Where the text sits, and whether it breaks or runs on
            Props::alignment("taLeftJustify"),
            Props::text_layout("tlTop"),
            Props::word_wrap(false),
            // Off by default, unlike a classic RAD `Label`: the designer does
            // not measure text (the tab strip has the same reason), so the box
            // it draws IS the size; on, the generated app fits the text.
            Props::auto_size(false),
            // A click on the label focuses this control
            Props::focus_control(),
        ],
        // The pointer router delivers both, for a class that declares them
        events: [Events::MOUSE_CLICK, Events::MOUSE_DOUBLE_CLICK],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    d
}
