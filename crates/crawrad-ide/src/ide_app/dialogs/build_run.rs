//! The centered build modal dialog: draws live compilation progress for Run.

use super::super::*;

impl IdeApp {
    pub(in super::super) fn draw_build_run(&mut self, cx: &mut Cx) {
        let is_run = self
            .build
            .running
            .as_ref()
            .is_some_and(|(m, _, _)| *m == crate::ide_app::dialogs::build::BuildMode::Run);
        let run = self
            .build
            .run(crate::ide_app::dialogs::build::BuildMode::Run);
        let elapsed = self
            .build
            .running
            .as_ref()
            .map(|(_, _, t)| t.elapsed())
            .unwrap_or_default();
        let target_dir = self
            .alvo_do_cargo()
            .map(|(r, _)| r.join("target").join("debug"));
        let content = run.map(|p| {
            crawrad_project::build_dialog::dialog_content(p, target_dir.as_deref(), elapsed)
        });
        let aberto = is_run && content.as_ref().is_some_and(|c| c.should_stay_open);
        self.show_veil(cx, id!(build_run_dialog), aberto);
        if !aberto {
            return;
        }
        self.ui.widget(id!(build_card)).set_visible(cx, false);
        let Some(c) = content else { return };
        let dlg = self.ui.view(id!(build_run_dialog.dlg));
        dlg.label(id!(run_title)).set_text(cx, &c.title);
        dlg.label(id!(run_action))
            .set_text(cx, c.current_action.as_deref().unwrap_or(""));
        dlg.label(id!(run_counter))
            .set_text(cx, c.counter.as_deref().unwrap_or(""));
        let bar_view = dlg.view(id!(run_bar));
        bar_view.set_visible(cx, c.fraction.is_some());
        if let Some(f) = c.fraction {
            let w = (bar_view.area().rect(cx).size.x * f).max(2.0);
            dlg.view(id!(run_bar.bar))
                .apply_over(cx, live! { width: (w), height: Fill });
        }
        dlg.label(id!(run_output)).set_text(cx, &c.output_path);
        dlg.label(id!(run_elapsed)).set_text(cx, &c.elapsed);
        dlg.button(id!(run_cancel)).set_text(cx, c.button_label);
    }

    pub(in super::super) fn check_build_run_cancel(&mut self, cx: &mut Cx) {
        let is_run = self
            .build
            .running
            .as_ref()
            .is_some_and(|(m, _, _)| *m == crate::ide_app::dialogs::build::BuildMode::Run);
        if is_run
            && self
                .ui
                .view(id!(build_run_dialog.dlg))
                .button(id!(run_cancel))
                .clicked(&cx.new_actions)
            && let Some((_, id, _)) = self.build.running
        {
            self.build_tasks.cancel(id);
        }
    }
}
