use makepad_widgets::*;

#[path = "dsl/editor_dialogs.rs"]
pub mod editor_dialogs;
#[path = "dsl/options_dialog.rs"]
pub mod options_dialog;
#[path = "dsl/project_dialogs.rs"]
pub mod project_dialogs;
#[path = "dsl/tools_dialogs.rs"]
pub mod tools_dialogs;

pub fn live_design(cx: &mut Cx) {
    editor_dialogs::live_design(cx);
    options_dialog::live_design(cx);
    project_dialogs::live_design(cx);
    tools_dialogs::live_design(cx);
}
