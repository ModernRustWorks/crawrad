use crate::components::base::control_props;
use crate::{ComponentDesc, Props, component};
use crawrad_note::note;

#[note("components-bevel-bevel")]
pub fn bevel() -> ComponentDesc {
    component!(
        "Bevel",
        display: "Bevel",
        base_name: "bevel",
        palette: "Additional",
        prefix: "bvl",
        size: (50.0, 50.0),
        base: control_props(),
        props: [
            Props::bevel_shape("Box"),
            Props::bevel_style("Lowered"),
        ],
        events: [],
    )
}
