//! The ActionBulb widget for the code editor's gutter lightbulb actions.
//!
//! Replaces font glyphs with a bespoke vector-drawn lightbulb:
//! - Glass dome with centered capital 'R'
//! - Two-ring screw base below
//! - Soft glow around the glass
//! - A pulsing glow for quick fixes (`preferred == true`)
//! - Amber lit look for code actions (`preferred == false`)

use makepad_widgets::*;

live_design! {
    use link::shaders::*;
    use link::widgets::*;

    pub ActionBulb = {{ActionBulb}} {
        width: 16.0,
        height: 16.0,
        cursor: Hand,
        show_bg: true,
        draw_bg: {
            instance preferred: 0.0
            instance dark: 1.0
            instance hover: 0.0
            instance pulse: 1.0

            fn pixel(self) -> vec4 {
                // The drawing (y 1.2..24.2 of a 24 box, glow included) fills the line's
                // height.
                let u = self.rect_size.y / 23.0;
                let px = self.pos * self.rect_size;
                let p = vec2(12.0 + (px.x - self.rect_size.x * 0.5) / (u * 0.9), 1.2 + px.y / u);

                let sdf = Sdf2d::viewport(p);

                // Pick the four looks with `mix` (the shader language has no
                // `if` expression): preferred x dark.
                let pd = self.preferred;
                let dk = self.dark;
                let fill_col = mix(mix(#ffd27a, #3a2a10, dk), mix(#ffc24d, #5a3d0c, dk), pd);
                let outline_col = mix(mix(#b86e00, #ffb547, dk), mix(#a35f00, #ffcf5c, dk), pd);
                let r_col = mix(mix(#6b3d00, #fff4dc, dk), mix(#5a3300, #fffaf0, dk), pd);
                let base1_col = mix(#9a9aa0, #d4d4da, dk);
                let base2_col = mix(#7d7d83, #b4b4ba, dk);

                // 1. Soft glow around the glass in the outline colour (low alpha)
                let glow_col = vec4(outline_col.rgb, mix(0.20, mix(0.20, 0.45, mix(self.pulse, 1.0, self.hover)), pd));

                sdf.move_to(12.0, 2.6);
                sdf.line_to(14.5, 3.1);
                sdf.line_to(16.8, 4.7);
                sdf.line_to(18.1, 7.0);
                sdf.line_to(18.4, 9.4);
                sdf.line_to(17.6, 11.8);
                sdf.line_to(15.9, 14.1);
                sdf.line_to(15.4, 15.2);
                sdf.line_to(14.9, 16.7);
                sdf.line_to(9.1, 16.7);
                sdf.line_to(8.6, 15.2);
                sdf.line_to(8.1, 14.1);
                sdf.line_to(6.4, 11.8);
                sdf.line_to(5.6, 9.4);
                sdf.line_to(5.9, 7.0);
                sdf.line_to(7.2, 4.7);
                sdf.line_to(9.5, 3.1);
                sdf.close_path();
                sdf.stroke(glow_col, 1.6);

                // 2. Bulb glass body (fill and outline)
                sdf.move_to(12.0, 2.6);
                sdf.line_to(14.5, 3.1);
                sdf.line_to(16.8, 4.7);
                sdf.line_to(18.1, 7.0);
                sdf.line_to(18.4, 9.4);
                sdf.line_to(17.6, 11.8);
                sdf.line_to(15.9, 14.1);
                sdf.line_to(15.4, 15.2);
                sdf.line_to(14.9, 16.7);
                sdf.line_to(9.1, 16.7);
                sdf.line_to(8.6, 15.2);
                sdf.line_to(8.1, 14.1);
                sdf.line_to(6.4, 11.8);
                sdf.line_to(5.6, 9.4);
                sdf.line_to(5.9, 7.0);
                sdf.line_to(7.2, 4.7);
                sdf.line_to(9.5, 3.1);
                sdf.close_path();
                sdf.fill_keep(fill_col);
                sdf.stroke(outline_col, 1.0);

                // 3. Bold capital R centred in the glass
                sdf.move_to(9.6, 5.4);
                sdf.line_to(9.6, 12.6);
                sdf.stroke(r_col, 2.1);
                sdf.move_to(9.6, 5.6);
                sdf.line_to(12.6, 5.6);
                sdf.line_to(14.2, 7.3);
                sdf.line_to(12.6, 9.0);
                sdf.line_to(9.6, 9.0);
                sdf.stroke(r_col, 1.9);
                sdf.move_to(11.8, 9.0);
                sdf.line_to(14.4, 12.6);
                sdf.stroke(r_col, 2.1);

                // 4. Grey two-ring screw base below the glass
                sdf.rect(8.6, 17.4, 6.8, 1.9);
                sdf.fill(base1_col);
                sdf.rect(9.0, 19.7, 6.0, 1.9);
                sdf.fill(base2_col);
                sdf.rect(10.4, 22.0, 3.2, 1.2);
                sdf.fill(base2_col);

                return sdf.result;
            }
        }
    }
}

/// Returns the (fill, outline, r) color hex strings for the light bulb.
pub fn bulb_colors(preferred: bool, dark: bool) -> (&'static str, &'static str, &'static str) {
    match (preferred, dark) {
        (false, true) => ("#3a2a10", "#ffb547", "#fff4dc"),
        (false, false) => ("#ffd27a", "#b86e00", "#6b3d00"),
        (true, true) => ("#5a3d0c", "#ffcf5c", "#fffaf0"),
        (true, false) => ("#ffc24d", "#a35f00", "#5a3300"),
    }
}

/// Computes the ray pulse opacity at elapsed time `t_secs`.
/// Period is 1.6s, oscillating between 0.55 (at t=0, t=1.6) and 1.0 (at t=0.8).
pub fn pulse_alpha(t_secs: f32) -> f32 {
    let freq = std::f32::consts::PI * 2.0 / 1.6;
    0.775 - 0.225 * (t_secs * freq).cos()
}

/// Actions emitted by [`ActionBulb`].
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum ActionBulbAction {
    None,
    Clicked,
}

/// The ActionBulb widget representing the editor gutter action indicator.
#[derive(Live, LiveHook, Widget)]
pub struct ActionBulb {
    #[deref]
    view: View,
    #[live(false)]
    pub preferred: bool,
    #[live(true)]
    pub dark: bool,
    #[rust]
    pub hover: bool,
    #[rust(1.0)]
    pub pulse: f32,
    #[rust]
    next_frame: NextFrame,
    #[rust]
    start_time: Option<std::time::Instant>,
}

impl Widget for ActionBulb {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.next_frame.is_event(event).is_some()
            && self.view.visible()
            && self.preferred
            && !self.hover
        {
            if let Some(start) = self.start_time {
                let new_pulse = pulse_alpha(start.elapsed().as_secs_f32());
                if (self.pulse - new_pulse).abs() > 0.005 {
                    self.pulse = new_pulse;
                    self.sync_shader(cx);
                }
            }
            self.next_frame = cx.new_next_frame();
        }

        if !self.view.visible() {
            return;
        }

        let area = self.view.area();
        match event.hits(cx, area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                if !self.hover {
                    self.hover = true;
                    self.pulse = 1.0;
                    self.sync_shader(cx);
                }
            }
            Hit::FingerHoverOut(_) if self.hover => {
                self.hover = false;
                if self.preferred {
                    if let Some(start) = self.start_time {
                        self.pulse = pulse_alpha(start.elapsed().as_secs_f32());
                    }
                    self.next_frame = cx.new_next_frame();
                } else {
                    self.pulse = 1.0;
                }
                self.sync_shader(cx);
            }
            Hit::FingerUp(fe) if fe.is_over => {
                cx.widget_action(self.widget_uid(), &scope.path, ActionBulbAction::Clicked);
            }
            _ => {}
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ActionBulb {
    fn sync_shader(&mut self, cx: &mut Cx) {
        let pref_val = if self.preferred { 1.0_f64 } else { 0.0_f64 };
        let dark_val = if self.dark { 1.0_f64 } else { 0.0_f64 };
        let hover_val = if self.hover { 1.0_f64 } else { 0.0_f64 };
        let pulse_val = f64::from(self.pulse);
        self.view.apply_over(
            cx,
            live! {
                draw_bg: {
                    preferred: (pref_val),
                    dark: (dark_val),
                    hover: (hover_val),
                    pulse: (pulse_val),
                }
            },
        );
        self.view.redraw(cx);
    }

    /// Quick fix (`true`, pulsing glow) or only actions (`false`).
    pub fn set_preferred(&mut self, cx: &mut Cx, preferred: bool) {
        self.set_kind(cx, preferred, self.dark);
    }

    /// Follows the editor theme: dark or light colours.
    pub fn set_dark(&mut self, cx: &mut Cx, dark: bool) {
        self.set_kind(cx, self.preferred, dark);
    }

    fn set_kind(&mut self, cx: &mut Cx, preferred: bool, dark: bool) {
        let changed = self.preferred != preferred || self.dark != dark;
        self.preferred = preferred;
        self.dark = dark;
        if changed {
            if self.start_time.is_none() {
                self.start_time = Some(std::time::Instant::now());
            }
            if self.view.visible() && self.preferred && !self.hover {
                self.next_frame = cx.new_next_frame();
            } else if !self.preferred {
                self.pulse = 1.0;
            }
            self.sync_shader(cx);
        }
    }

    /// Controls visibility and animation lifecycle.
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.view.visible() != visible {
            self.view.set_visible(cx, visible);
            if visible {
                if self.start_time.is_none() {
                    self.start_time = Some(std::time::Instant::now());
                }
                if self.preferred && !self.hover {
                    self.next_frame = cx.new_next_frame();
                }
            }
            self.sync_shader(cx);
        }
    }

    /// Checks if this bulb was clicked in the given actions.
    pub fn clicked(&self, actions: &Actions) -> bool {
        for a in actions {
            if let Some(w) = a.downcast_ref::<WidgetAction>()
                && w.widget_uid == self.widget_uid()
                && let Some(act) = w.action.downcast_ref::<ActionBulbAction>()
                && *act == ActionBulbAction::Clicked
            {
                return true;
            }
        }
        false
    }
}

impl ActionBulbRef {
    /// Quick fix or only actions, on the referenced bulb.
    pub fn set_preferred(&self, cx: &mut Cx, preferred: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_preferred(cx, preferred);
        }
    }

    /// Dark or light colours, on the referenced bulb.
    pub fn set_dark(&self, cx: &mut Cx, dark: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_dark(cx, dark);
        }
    }

    /// Returns true if the referenced bulb was clicked.
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.clicked(actions))
    }

    /// Sets visibility on the referenced bulb widget.
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_visible(cx, visible);
        }
    }

    /// Returns whether the referenced bulb is currently visible.
    pub fn visible(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.view.visible())
    }
}

#[cfg(test)]
#[path = "test/action_bulb_view/test.rs"]
mod tests;
