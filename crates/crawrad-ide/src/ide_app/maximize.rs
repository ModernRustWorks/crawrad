//! MAXIMIZE WITHOUT COVERING THE TASKBAR
use super::*;
use crawrad_note::note;
use crawrad_os::ScreenRect;

pub use crawrad_runtime_makepad::window::{centre_in_physical_pixels, followed, on_screen};

impl IdeApp {
    #[note("ide-app-maximize-ideapp-toggle-maximized")]
    pub(super) fn toggle_maximized(&mut self, cx: &mut Cx) {
        let window = self.window_id(cx);
        let geom = cx.windows[window].window_geom.clone();
        let colocou = if self.maximizada {
            match self.restore_to.take() {
                Some(anterior) => crawrad_os::place_main_window(anterior),
                None => false,
            }
        } else {
            let antes = on_screen(&geom);
            match crawrad_os::monitor_at(centre_in_physical_pixels(&geom)) {
                Some(area) if crawrad_os::place_main_window(area.work) => {
                    self.restore_to = Some(antes);
                    true
                }
                _ => false,
            }
        };
        if !colocou {
            self.restore_to = None;
            cx.push_unique_platform_op(if self.maximizada {
                CxOsOp::RestoreWindow(window)
            } else {
                CxOsOp::MaximizeWindow(window)
            });
        }
        self.maximizada = !self.maximizada;
        self.show_window_button(cx);
    }

    #[note("ide-app-maximize-ideapp-fit-on-screen-once")]
    pub(super) fn fit_on_screen_once(&mut self, cx: &mut Cx) {
        if !self.fit_on_screen_pending {
            return;
        }
        let window = self.window_id(cx);
        let geom = cx.windows[window].window_geom.clone();
        if geom.dpi_factor <= 0.0 || geom.inner_size.x <= 0.0 || geom.inner_size.y <= 0.0 {
            // Nothing has been described yet. Try again on the next event.
            return;
        }
        crawrad_os::prepare_main_window();
        if self.maximizada || self.place_startup_window() {
            self.fit_on_screen_pending = false;
            return;
        }
        let antes = on_screen(&geom);
        let Some(area) = crawrad_os::monitor_at(centre_in_physical_pixels(&geom)) else {
            self.fit_on_screen_pending = false;
            self.toggle_maximized(cx);
            return;
        };
        if !crawrad_os::place_main_window(area.work) {
            return;
        }
        self.fit_on_screen_pending = false;
        let kept = self.startup_window.take();
        let kept = kept.map(|w| ScreenRect::of_window([w.x, w.y], [w.width, w.height], 1.0));
        self.restore_to = Some(kept.unwrap_or(antes));
        self.maximizada = true;
        self.show_window_button(cx);
    }
}
