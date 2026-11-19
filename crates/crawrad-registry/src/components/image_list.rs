use crate::{ComponentDesc, ICON_SIZE, Props, non_visual_props};
use crawrad_note::note;

#[note("components-image-list-image-list")]
pub fn image_list() -> ComponentDesc {
    crate::component!(
        "ImageList",
        display: "ImageList",
        base_name: "image_list",
        palette: "Standard",
        prefix: "iml",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        // Non-visual icon collection that now generates struct and state in the target.
        emits_code: true,
        base: non_visual_props(),
        props: [
            Props::image_width(16),
            Props::image_height(16),
            // comctl32 vocabulary: egui has neither colour depth nor
            // transparency colour for image lists. Namespaced, the file
            // preserves them and whoever does not understand them ignores them (ADR 0002).
            // Icons are a model collection, not a target one. Edited by the
            // ImageList Editor; the inspector only displays it.
            Props::images_data(),
        ],
    )
}
