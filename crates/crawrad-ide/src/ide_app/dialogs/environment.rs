//! Environment Check dialog wiring.
use super::super::*;
use crawrad_note::note;
use crawrad_project::environment::{InstallStep, Probe};
use crawrad_project::environment_install::SystemProbe;
use crawrad_project::environment_session::Pump;
use makepad_code_editor::environment_view::{
    EnvironmentCheckViewWidgetRefExt, EnvironmentViewAction,
};

/// Constructs a system probe with IDE LSP and DAP locators wired up.
pub fn make_system_probe() -> Box<dyn Probe + Send> {
    Box::new(SystemProbe {
        rust_analyzer: || crawrad_lsp::locate::locate().ok().map(|s| s.path),
        lldb_dap: || crawrad_dap::adapter::locate().ok(),
    })
}

impl IdeApp {
    #[note("ide-app-dialogs-environment-ideapp-check-environment-on-startup")]
    pub(in super::super) fn check_environment_on_startup(&mut self, cx: &mut Cx) {
        if self
            .environment
            .borrow_mut()
            .begin_startup(make_system_probe)
        {
            self.refresh(cx);
        }
    }

    #[note("ide-app-dialogs-environment-ideapp-trigger-environment-check")]
    pub(in super::super) fn trigger_environment_check(&mut self, cx: &mut Cx) {
        self.environment.borrow_mut().start_probe(make_system_probe);
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-environment-ideapp-open-environment")]
    pub(in super::super) fn open_environment(&mut self, cx: &mut Cx) {
        self.environment_open = true;
        self.trigger_environment_check(cx);
    }

    #[note("ide-app-dialogs-environment-ideapp-close-environment")]
    pub(in super::super) fn close_environment(&mut self, cx: &mut Cx) {
        self.environment_open = false;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-environment-ideapp-pump-environment")]
    pub(in super::super) fn pump_environment(&mut self, cx: &mut Cx) {
        let pump = self.environment.borrow_mut().pump();
        match pump {
            Pump::Open => self.open_environment(cx),
            Pump::Redraw if self.environment_open => {
                self.draw_environment(cx);
                self.ui.redraw(cx);
            }
            _ => {}
        }
    }

    #[note("ide-app-dialogs-environment-ideapp-draw-environment")]
    pub(in super::super) fn draw_environment(&mut self, cx: &mut Cx) {
        let dialog = self.ui.widget(id!(environment_dialog));
        dialog.set_visible(cx, self.environment_open);
        if self.environment_open {
            let (head, rows) = self.environment.borrow().view_here();
            let view = self.ui.environment_check_view(id!(environment_dialog.view));
            view.set_view(cx, head, rows);
        }
    }

    #[note("ide-app-dialogs-environment-ideapp-atender-environment")]
    pub(in super::super) fn atender_environment(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.environment_open {
            return;
        }
        let view = self.ui.environment_check_view(id!(environment_dialog.view));
        let mut s = self.environment.borrow_mut();
        match view.action(actions) {
            EnvironmentViewAction::None => return,
            EnvironmentViewAction::Cancel => {
                drop(s);
                return self.close_environment(cx);
            }
            EnvironmentViewAction::CheckAgain => s.start_probe(make_system_probe),
            EnvironmentViewAction::StopInstall => s.stop_install(),
            EnvironmentViewAction::InstallAll => s.install_all(),
            EnvironmentViewAction::Primary(i) => match s.press_primary(i) {
                Some(InstallStep::OpenUrl { url }) => {
                    crawrad_runtime::shell::open_url(url);
                }
                Some(InstallStep::CopyCommand { command }) => cx.copy_to_clipboard(command),
                _ => {}
            },
        }
        drop(s);
        self.draw_environment(cx);
        self.refresh(cx);
    }
}
