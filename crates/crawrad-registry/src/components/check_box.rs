use crate::{ComponentDesc, Events, Props, TextLine, component};
use crawrad_note::note;

#[note("components-check-box-check-box")]
pub fn check_box() -> ComponentDesc {
    let mut d = component!(
        "CheckBox",
        display: "CheckBox",
        base_name: "check_box",
        palette: "Standard",
        prefix: "chk",
        size: (120.0, 20.0),
        text: TextLine::Centered,
        props: [
            Props::caption("CheckBox"),
            Props::checked(false).bindable(),
            // State says the same as Checked; setting one sets the other
            Props::check_state("cbUnchecked"),
            Props::allow_grayed(false),
            Props::check_alignment("taRightJustify"),
        ],
        events: [Events::MOUSE_CLICK, Events::CHANGE],
    );
    // Font and ParentFont (its draw_bg is the box itself, not a
    // background, so it takes no Color)
    d.props.extend(crate::components::base::font_props());
    d
}
