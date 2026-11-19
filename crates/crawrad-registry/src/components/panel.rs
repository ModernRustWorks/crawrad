use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-panel-panel")]
pub fn panel() -> ComponentDesc {
    let mut d = component!(
        "Panel",
        display: "Panel",
        base_name: "panel",
        palette: "Standard",
        prefix: "pnl",
        size: (240.0, 120.0),
        children: Children::Area,
        props: [
            Props::caption(""),
            // The border of `TPanel`. A number, and not the two bevels of
            // the classic RAD IDE (`BevelInner`/`BevelOuter`/`BevelWidth`): that is
            // 1995 3D drawing vocabulary, and what remains useful is the
            // border thickness.
            Props::border_width(1),
        ],
        // The target already delivered it
        events: [Events::MOUSE_CLICK, Events::RESIZE],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    // See `without_tab_stop`
    crate::components::base::without_tab_stop(&mut d.props);
    d
}
