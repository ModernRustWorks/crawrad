use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-scroll-box-scroll-box")]
pub fn scroll_box() -> ComponentDesc {
    let mut d = component!(
        "ScrollBox",
        display: "ScrollBox",
        base_name: "scroll_box",
        palette: "Additional",
        prefix: "sbx",
        size: (200.0, 150.0),
        children: Children::Area,
        props: [
            Props::auto_scroll(true),
            Props::horz_scroll_bar(true),
            Props::vert_scroll_bar(true),
            Props::border_width(1),
        ],
        events: [Events::RESIZE],
    );
    // Its own background (it draws no text, so no font)
    d.props.extend(crate::components::base::color_props());
    // See `without_tab_stop`
    crate::components::base::without_tab_stop(&mut d.props);
    d
}
