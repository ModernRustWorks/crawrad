//! Control for the legacy Permissions Dialog (replaced by the unified `permission_card` in chat).

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-permission-ideapp-draw-permission")]
    pub(in super::super) fn draw_permission(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(permission_dialog), false);
    }

    #[note("ide-app-dialogs-permission-ideapp-atender-permission")]
    pub(in super::super) fn atender_permission(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}
