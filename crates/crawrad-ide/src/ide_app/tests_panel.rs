//! The Tests panel glue: state and decisions in `crawrad_project::test_run`,
//! drawing in `tests_panel_view`; here only the wires.

use crawrad_actions::Panel;
use crawrad_project::test_run::{PanelEffect, PanelInput as I};
use makepad_code_editor::tests_panel_view::TestsPanelAction as A;
use makepad_code_editor::tests_panel_view::*;
use makepad_widgets::*;

use super::{IdeApp, layout, xywh};

impl IdeApp {
    pub(super) fn open_tests(&mut self, cx: &mut Cx) {
        let effect = self.tests.open(self.alvo_do_cargo());
        self.test_effect(cx, effect);
    }

    pub(super) fn pump_tests(&mut self, cx: &mut Cx) {
        if let Some(effect) = self.tests.pump() {
            self.test_effect(cx, effect);
        }
    }

    fn test_effect(&mut self, cx: &mut Cx, effect: PanelEffect) {
        match effect {
            PanelEffect::Paint => {}
            PanelEffect::Status(text) => {
                self.status = Some(text);
                self.apply_diagnostics_decorations(cx);
                self.refresh(cx);
            }
            PanelEffect::Go {
                place: (file, line, column),
                output,
            } => {
                if output {
                    self.show_panel(cx, Panel::TestOutput);
                }
                let (line, byte) = (line.saturating_sub(1), column.saturating_sub(1));
                self.go_to_place(cx, crawrad_host::navigation::Place { file, line, byte });
            }
        }
        let ((head, rows, output), dock) = (self.tests.view(), self.dock());
        let tab = dock.item(layout::tab_of(Panel::Tests));
        tab.tests_panel_view(id!(tests_view))
            .set_view(cx, head, rows);
        let out = dock.item(layout::tab_of(Panel::TestOutput));
        out.test_output_view(id!(output_view))
            .set_output(cx, output);
    }

    pub(super) fn tests_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let dock = self.dock();
        let out = dock.item(layout::tab_of(Panel::TestOutput));
        let tab = dock.item(layout::tab_of(Panel::Tests));
        let input = match tab.tests_panel_view(id!(tests_view)).action(actions) {
            _ if out.test_output_view(id!(output_view)).spot_clicked(actions) => I::Spot,
            A::RunAll => I::RunAll,
            A::RunFailed => I::RunFailed,
            A::Refresh => I::Refresh,
            A::Chip(i) => I::Chip(i),
            A::Filter(text) => I::Filter(text),
            A::Row(i) => I::Row(i),
            A::RunRow(i) => I::RunRow(i),
            A::None | A::OpenSpot => return,
        };
        let effect = self.tests.input(input, self.alvo_do_cargo());
        self.test_effect(cx, effect);
    }

    pub(super) fn tests_view(&self) -> TestsPanelViewRef {
        let tab = self.dock().item(layout::tab_of(Panel::Tests));
        tab.tests_panel_view(id!(tests_view))
    }

    pub(crate) fn measured_tests_rects(&self, cx: &Cx) -> Vec<(usize, [f64; 4])> {
        if !self.tests_rects.is_empty() {
            return (self.tests_rects.iter())
                .filter(|(_, r)| r.size.x > 0.0 && r.size.y > 0.0)
                .map(|(i, r)| (*i, xywh(*r)))
                .collect();
        }
        let view = self.tests_view();
        let list = view.portal_list();
        let count = view.rows_count();
        let mut rects = Vec::new();
        for i in 0..count {
            if let Some((_, item)) = list.get_item(i) {
                let r = item.area().rect(cx);
                if r.size.x > 0.0 && r.size.y > 0.0 {
                    rects.push((i, xywh(r)));
                }
            }
        }
        rects
    }

    pub(super) fn tests_json(&self, cx: &Cx) -> serde_json::Value {
        let rects = self.measured_tests_rects(cx);
        serde_json::from_str(&self.tests_view().rows_json_with_rects(&rects)).unwrap_or_default()
    }
}
