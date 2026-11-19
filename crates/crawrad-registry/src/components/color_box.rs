use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-color-box-color-box")]
pub fn color_box() -> ComponentDesc {
    let mut d = component!(
        "ColorBox",
        display: "ColorBox",
        base_name: "color_box",
        palette: "Additional",
        prefix: "clb",
        size: (145.0, 24.0),
        props: [
            Props::selected_color(crawrad_ir::Color::rgb(0, 0, 0)),
        ],
        events: [
            Events::CHANGE,
            Events::ENTER,
            Events::EXIT,
        ],
    );
    d.props.extend(crate::components::base::font_props());
    d
}
