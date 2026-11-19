use crate::components::base::{ICON_SIZE, non_visual_props};
use crate::{ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-message-dialog-message-dialog")]
pub fn message_dialog() -> ComponentDesc {
    component!(
        "MessageDialog",
        display: "MessageDialog",
        base_name: "message_dialog",
        palette: "Dialogs",
        prefix: "dlg_msg",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        base: non_visual_props(),
        props: [
            Props::title(""),
            Props::message_text(""),
            Props::message_dialog_kind("mtInformation"),
            Props::message_dialog_buttons("mbOK"),
        ],
        events: [
            Events::CLOSE,
        ],
    )
}
