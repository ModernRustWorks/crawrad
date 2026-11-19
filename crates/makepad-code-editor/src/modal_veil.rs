//! The ModalVeil widget: a modal background overlay that locks pointer sweep
//! and intercepts any hits outside the child dialog while visible.

use makepad_widgets::*;

live_design! {
    use link::widgets::*;

    // The look (veil colour, centring) lives in `CrawradDialogVeil`.
    pub ModalVeil = {{ModalVeil}} {}
}

/// Modal veil widget that holds a pointer sweep lock and consumes outside pointer hits while visible.
#[derive(Live, LiveHook, Widget)]
pub struct ModalVeil {
    #[deref]
    pub view: View,
    #[rust]
    sweep_locked: bool,
}

impl Widget for ModalVeil {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.view.visible() {
            if self.sweep_locked {
                cx.sweep_unlock(self.view.area());
                self.sweep_locked = false;
            }
            return;
        }

        let area = self.view.area();

        if !self.sweep_locked && area != Area::Empty {
            cx.sweep_lock(area);
            self.sweep_locked = true;
        }

        cx.sweep_unlock(area);
        self.view.handle_event(cx, event, scope);

        if !self.view.visible() {
            if self.sweep_locked {
                cx.sweep_unlock(area);
                self.sweep_locked = false;
            }
            return;
        }

        cx.sweep_lock(area);
        // Consume every hit on the veil so pointer events do not reach views underneath.
        let _ = event.hits_with_sweep_area(cx, area, area);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let is_visible = self.view.visible();
        let area = self.view.area();
        if is_visible {
            if !self.sweep_locked && area != Area::Empty {
                cx.sweep_lock(area);
                self.sweep_locked = true;
            }
        } else if self.sweep_locked {
            cx.sweep_unlock(area);
            self.sweep_locked = false;
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
