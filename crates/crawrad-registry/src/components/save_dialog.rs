use crate::components::base::{ICON_SIZE, non_visual_props};
use crate::{ComponentDesc, Props, component};
use crawrad_note::note;

#[note("components-save-dialog-save-dialog")]
pub fn save_dialog() -> ComponentDesc {
    component!(
        "SaveDialog",
        display: "SaveDialog",
        base_name: "save_dialog",
        palette: "Dialogs",
        prefix: "dlg_save",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        base: non_visual_props(),
        props: [
            Props::title("Save As"),
            Props::filter("All files (*.*)|*.*"),
            Props::file_name(""),
            Props::initial_dir(""),
            Props::default_ext(""),
        ],
    )
}
