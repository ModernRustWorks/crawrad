//! The window's live hooks: `LiveHook` and `LiveRegister`. Cut out of `ide_app.rs`.

use super::*;
use crawrad_note::note;
impl LiveHook for IdeApp {
    #[note("ide-app-live-ideapp-after-apply")]
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.ui.view(id!(caption_bar)).set_visible(cx, false);
        self.ui.view(id!(windows_buttons)).set_visible(cx, false);
    }
}
impl LiveRegister for IdeApp {
    #[note("ide-app-live-ideapp-live-register")]
    fn live_register(cx: &mut Cx) {
        // Order matters: a `link` resolves only what is already registered (see the note).
        makepad_widgets::live_design(cx);
        cx.link(live_id!(theme), live_id!(crawrad_theme_dark));
        // The dialog veil's widget, before the theme's templates use it.
        makepad_code_editor::modal_veil::live_design(cx);
        tema::live_design(cx);
        dialogos::dsl::live_design(cx);
        makepad_code_editor::minimap::live_design(cx);
        paineis::live_design(cx);
        makepad_code_editor::live_design(cx);
        crawrad_runtime_makepad::live_design(cx);
        layout::register_tab_names();
        dsl_main::live_design(cx);
    }
}
