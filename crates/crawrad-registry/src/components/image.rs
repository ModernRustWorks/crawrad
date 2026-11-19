use crate::components::base::control_props;
use crate::{ComponentDesc, Props, component};
use crawrad_ir::Picture;

pub fn image() -> ComponentDesc {
    component!(
        "Image",
        display: "Image",
        base_name: "image",
        palette: "Additional",
        prefix: "img",
        size: (105.0, 105.0),
        base: control_props(),
        props: [
            Props::picture(Picture::empty()),
            Props::stretch(false),
            Props::proportional(false),
            Props::center(false),
            Props::auto_size(false),
            Props::transparent(false),
        ],
    )
}
