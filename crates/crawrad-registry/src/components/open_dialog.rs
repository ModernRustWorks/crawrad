use crate::components::base::{ICON_SIZE, non_visual_props};
use crate::{ComponentDesc, Props, component};
use crawrad_note::note;

#[note("components-open-dialog-open-dialog")]
pub fn open_dialog() -> ComponentDesc {
    component!(
        "OpenDialog",
        display: "OpenDialog",
        base_name: "open_dialog",
        palette: "Dialogs",
        prefix: "dlg_open",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        base: non_visual_props(),
        props: [
            Props::title("Open"),
            Props::filter("All files (*.*)|*.*"),
            Props::file_name(""),
            Props::initial_dir(""),
            Props::default_ext(""),
        ],
    )
}
