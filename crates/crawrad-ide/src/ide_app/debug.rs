//! The Run under the debugger: the glue between the window and
//! the session the host keeps (`crawrad_host::Debug`).
//!
//! What is decided lives in the crates — the session in `crawrad-dap`, its state
//! in `crawrad-host`, the values in `crawrad-dap-rust`. This file only starts it,
//! pumps it once a frame and shows where the program stopped.

use crawrad_os::tasks::wake;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crawrad_actions::Panel;
use crawrad_dap::{Command, DebugController, LaunchConfig};
use crawrad_host::DebugEvent;
use crawrad_host::debug_panel::Row;
use makepad_widgets::*;

use super::{IdeApp, icon, layout};
use crawrad_note::note;
use crawrad_project::toolbar;

impl IdeApp {
    #[note("ide-app-debug-ideapp-run-or-continue")]
    pub(super) fn run_or_continue(&mut self, cx: &mut Cx) {
        let debug = self.host.debug_mut();
        if debug.is_stopped() {
            debug.send(Command::Continue);
        } else if debug.is_active() {
            self.status = Some("the program is running under the debugger".into());
        } else {
            self.rodar_projeto(cx);
        }
    }

    #[note("ide-app-debug-ideapp-debug-command")]
    pub(super) fn debug_command(&mut self, command: Command) {
        if !self.host.debug_mut().send(command) {
            self.status = Some("no program is being debugged — Run starts one".into());
        }
    }

    #[note("ide-app-debug-ideapp-start-debugging")]
    pub(super) fn start_debugging(
        &mut self,
        cx: &mut Cx,
        program: PathBuf,
        cwd: &Path,
    ) -> Result<(), String> {
        let adapter = crawrad_dap::adapter::locate().map_err(|missing| missing.to_string())?;
        let breakpoints: BTreeMap<PathBuf, Vec<i64>> = self
            .host
            .breakpoints()
            .files()
            .map(|(file, lines)| {
                (
                    file.to_path_buf(),
                    lines.into_iter().map(i64::from).collect(),
                )
            })
            .collect();
        let presenter = Box::new(crawrad_dap_rust::RustPresenter::for_program(&program));
        let config = LaunchConfig {
            program,
            cwd: Some(cwd.to_path_buf()),
            // A panic then names the project's frames.
            extra: serde_json::Map::from_iter([(
                "env".to_string(),
                serde_json::json!(["RUST_BACKTRACE=1"]),
            )]),
            ..LaunchConfig::default()
        };
        let session =
            DebugController::launch_process(adapter, config, breakpoints, presenter, wake);
        self.host.debug_mut().start(session);
        // A classic RAD IDE turns to its debugging layout on Run: Run and Debug comes forward.
        self.execute_with(
            cx,
            "layout.show_panel",
            crawrad_actions::Args::text("panel", "debug"),
        );
        self.write_debug_bar(cx);
        Ok(())
    }

    #[note("ide-app-debug-ideapp-pump-debugging")]
    pub(super) fn pump_debugging(&mut self, cx: &mut Cx) {
        for event in self.host.debug_mut().pump() {
            match event {
                DebugEvent::Stopped {
                    file: Some(file),
                    line,
                    reason,
                } => self.show_stopped_at(cx, file, line, &reason),
                DebugEvent::Stopped { file: None, .. } => {}
                DebugEvent::Resumed => self.status = Some("running under the debugger".into()),
                DebugEvent::Inspected => {}
                DebugEvent::Ended(text) => {
                    self.status = Some(text);
                    let stderr = self.host.debug_mut().output().concat();
                    if let Some((root, package)) = self.alvo_do_cargo() {
                        let end = crawrad_project::app_run::AppEnd::read(package, root, stderr);
                        self.report_panics(cx, end);
                    }
                }
            }
            self.write_debug_bar(cx);
            self.redraw_panel_list(cx, Panel::Debug, id!(debug_body.debug_list));
            self.refresh(cx);
        }
    }

    #[note("ide-app-debug-ideapp-show-stopped-at")]
    fn show_stopped_at(&mut self, cx: &mut Cx, file: PathBuf, line: u32, reason: &str) {
        self.status = Some(crawrad_host::debug::stop_text(&file, line, reason));
        self.abrir_no_editor(cx, file);
        self.pousar_cursor(cx, &crate::ide_app::PousoCodigo::cursor(line as usize, 1));
    }
}

#[note("ide-app-debug-control-bar")]
const CONTROL_BAR: &[(&[LiveId], icon::Chrome, &str)] = &[
    (
        id!(btn_dbg_continue),
        icon::Chrome::DebugContinue,
        "project.run",
    ),
    (id!(btn_dbg_pause), icon::Chrome::DebugPause, "debug.pause"),
    (
        id!(btn_dbg_step_over),
        icon::Chrome::StepOver,
        "debug.step_over",
    ),
    (
        id!(btn_dbg_step_into),
        icon::Chrome::StepInto,
        "debug.step_into",
    ),
    (
        id!(btn_dbg_step_out),
        icon::Chrome::StepOut,
        "debug.step_out",
    ),
    (id!(btn_dbg_stop), icon::Chrome::DebugStop, "debug.stop"),
];
impl IdeApp {
    #[note("ide-app-debug-ideapp-write-debug-bar")]
    pub(super) fn write_debug_bar(&mut self, cx: &mut Cx) {
        let debug = self.host.debug();
        let (stopped, active) = (debug.is_stopped(), debug.is_active());
        let blue = f64::from(u8::from(active));
        self.ui
            .view(id!(bottom_status_bar))
            .apply_over(cx, live! { draw_bg: { debugging: (blue) } });
        let can = |action: &str| match action {
            "project.run" => stopped || !active,
            "debug.pause" => active && !stopped,
            "debug.stop" => active,
            _ => stopped,
        };
        let panel = self.dock().item(layout::tab_of(Panel::Debug));
        for (button, role, action) in CONTROL_BAR {
            let b = panel.button(button);
            b.set_text(cx, role.glyph());
            Self::habilitar(cx, &b, can(action));
        }
        // The toolbar's debugger group follows the same judgement.
        for c in toolbar::Command::ALL
            .iter()
            .filter(|c| c.action().starts_with("debug."))
        {
            Self::habilitar(cx, &self.bar_button(*c), can(c.action()));
        }
    }

    #[note("ide-app-debug-ideapp-debug-bar-clicks")]
    pub(super) fn debug_bar_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let panel = self.dock().item(layout::tab_of(Panel::Debug));
        let pressed = CONTROL_BAR
            .iter()
            .find(|(button, ..)| panel.button(button).clicked(actions));
        if let Some((_, _, action)) = pressed {
            self.execute_with(cx, action, crawrad_actions::Args::none());
        }
    }

    #[note("ide-app-debug-ideapp-debug-list-clicks")]
    pub(super) fn debug_list_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let list = self
            .dock()
            .item(layout::tab_of(Panel::Debug))
            .portal_list(id!(debug_body.debug_list));
        if !list.any_items_with_actions(actions) {
            return;
        }
        let rows = crawrad_host::debug_panel::rows(self.host.debug(), self.host.breakpoints());
        let items = list.items_with_actions(actions);
        let closed = items
            .iter()
            .find(|(_, item)| item.button(id!(face.fechar)).clicked(actions))
            .and_then(|(i, _)| rows.get(*i).cloned());
        if let Some(Row::Watch { index, .. }) = closed {
            self.remove_watch(cx, index);
            return;
        }
        let picked = items
            .into_iter()
            .find(|(_, item)| item.button(id!(alvo)).clicked(actions))
            .and_then(|(i, _)| rows.get(i).cloned());
        match picked {
            Some(Row::Frame { index, .. }) => {
                if let Some((file, line)) = self.host.debug_mut().select_frame(index) {
                    self.abrir_no_editor(cx, file);
                    self.pousar_cursor(cx, &crate::ide_app::PousoCodigo::cursor(line as usize, 1));
                }
            }
            Some(Row::Variable { reference, .. } | Row::Watch { reference, .. })
                if reference != 0 =>
            {
                self.host.debug_mut().toggle_variable(reference);
            }
            _ => return,
        }
        self.redraw_panel_list(cx, Panel::Debug, id!(debug_body.debug_list));
    }

    #[note("ide-app-debug-ideapp-add-watch")]
    pub(super) fn add_watch(&mut self, cx: &mut Cx, expression: Option<String>) {
        let expression = expression
            .or_else(|| self.extrair_selecao_do_editor().map(|s| s.selected_text))
            .or_else(|| self.word_under_caret());
        self.status = Some(match expression {
            Some(e) if self.host.debug_mut().add_watch(&e) => {
                let panel = crawrad_actions::Args::text("panel", Panel::Debug.id());
                self.execute_with(cx, "layout.show_panel", panel);
                format!("watching `{}`", e.trim())
            }
            Some(e) => format!("`{}` is already watched", e.trim()),
            None => "Add Watch: select an expression, or put the caret on a word".into(),
        });
        self.redraw_panel_list(cx, Panel::Debug, id!(debug_body.debug_list));
    }

    #[note("ide-app-debug-ideapp-remove-watch")]
    pub(super) fn remove_watch(&mut self, cx: &mut Cx, index: usize) {
        if let Some(e) = self.host.debug_mut().remove_watch(index) {
            self.status = Some(format!("no longer watching `{e}`"));
        }
        self.redraw_panel_list(cx, Panel::Debug, id!(debug_body.debug_list));
    }

    #[note("ide-app-debug-ideapp-word-under-caret")]
    fn word_under_caret(&self) -> Option<String> {
        if !self.center().is_text() {
            return None;
        }
        self.with_code_view(|cv| cv.session.as_ref()?.word_at_cursor())?
    }

    #[note("ide-app-debug-ideapp-write-debug-row")]
    pub(super) fn write_debug_row(cx: &mut Cx, item: &WidgetRef, row: &Row) {
        if let Row::Watch { .. } = row {
            item.button(id!(face.fechar))
                .set_text(cx, icon::Chrome::Close.glyph());
        }
        let opens = match row {
            Row::Variable {
                depth,
                reference,
                open,
                ..
            } => Some((*depth, *reference, *open)),
            Row::Watch {
                reference, open, ..
            } => Some((0, *reference, *open)),
            _ => None,
        };
        if let Some((depth, reference, open)) = opens {
            let chevron = match (reference != 0, open) {
                (false, _) => "",
                (true, false) => icon::Chrome::ChevronRight.glyph(),
                (true, true) => icon::Chrome::ChevronDown.glyph(),
            };
            item.label(id!(face.marca)).set_text(cx, chevron);
            let indent = 6.0 + 12.0 * depth as f64;
            item.view(id!(face))
                .apply_over(cx, live! { padding: { left: (indent) } });
        }
        let (name, value) = match row {
            Row::Section(t) => return item.label(id!(face.nome)).set_text(cx, t),
            Row::Note(t) => return item.label(id!(face.nome)).set_text(cx, t),
            Row::Variable { name, value, .. } => (name.clone(), format!(" = {value}")),
            Row::Watch {
                expression, value, ..
            } => (expression.clone(), format!(" = {value}")),
            Row::Frame {
                function, place, ..
            } => (function.clone(), format!("  {place}")),
            Row::Breakpoint { file, line } => (format!("\u{25cf} {file}"), line.to_string()),
        };
        item.label(id!(face.nome)).set_text(cx, &name);
        item.label(id!(face.valor)).set_text(cx, &value);
    }
}
