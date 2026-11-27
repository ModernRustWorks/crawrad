//! Action dispatch for dialogs, modals, and shell pickers.

use super::super::*;

impl IdeApp {
    pub(in super::super) fn handle_dialog_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // THE TWO INSPECTOR WINDOWS — the text pad and the `Align` picker.
        self.atender_pad(cx, actions);
        self.atender_items(cx, actions);
        self.atender_lines(cx, actions);
        self.handle_color(cx, actions);
        self.atender_rename(cx, actions);
        self.atender_component_rename(cx, actions);
        self.atender_actions(cx, actions);
        self.atender_standard(cx, actions);
        self.atender_imagelist(cx, actions);
        self.atender_quit(cx, actions);
        self.handle_recover_dialog(cx, actions);
        self.atender_options(cx, actions);
        self.atender_project_options(cx, actions);
        self.atender_novo_projeto(cx, actions);
        self.atender_criacao(cx, actions);
        self.atender_icones(cx, actions);
        self.atender_memory(cx, actions);
        self.atender_environment(cx, actions);
        self.atender_mcp(cx, actions);
        self.atender_acp_diagnostic(cx, actions);
        self.atender_nota(cx, actions);
        if self.hover_note_clicked(actions) {
            self.open_hover_note(cx);
        }
        self.atender_goto(cx, actions);
        self.atender_quick_open(cx, actions);
        self.atender_inherit_form(cx, actions);
        self.atender_new_component(cx, actions);
        self.atender_build(cx, actions);
        self.handle_shrink(cx, actions);
        self.atender_permission(cx, actions);
        self.handle_templates(cx, actions);
        self.atender_find_replace(cx, actions);
        self.atender_autocomplete(cx, actions);
    }
}
