//! **Color picker widgets**: a saturation/value square for one hue
//! and a vertical hue strip.
//!
//! Used by the Color dialog to let the user select a custom colour by clicking
//! or dragging on the HSV surfaces.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use makepad_draw::shader::std::*;

    pub DrawColorSquare = {{DrawColorSquare}} {
        fn pixel(self) -> vec4 {
            let h = fract(self.hue / 360.0);
            let s = clamp(self.pos.x, 0.0, 1.0);
            let v = clamp(1.0 - self.pos.y, 0.0, 1.0);
            let r = clamp(abs(fract(h) * 6.0 - 3.0) - 1.0, 0.0, 1.0);
            let g = clamp(abs(fract(h + 2.0 / 3.0) * 6.0 - 3.0) - 1.0, 0.0, 1.0);
            let b = clamp(abs(fract(h + 1.0 / 3.0) * 6.0 - 3.0) - 1.0, 0.0, 1.0);
            let rgb = mix(vec3(1.0, 1.0, 1.0), vec3(r, g, b), s) * v;
            let p = self.pos * self.rect_size;
            if p.x < 1.0 || p.x > self.rect_size.x - 1.0 || p.y < 1.0 || p.y > self.rect_size.y - 1.0 {
                return vec4(0.29, 0.30, 0.32, 1.0);
            }
            return vec4(rgb, 1.0);
        }
    }

    pub DrawColorSquareMarker = {{DrawColorSquareMarker}} {
        fn pixel(self) -> vec4 {
            let p = self.pos * self.rect_size;
            let c = self.rect_size * 0.5;
            let d = length(p - c);
            if d > 7.0 || d < 3.0 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
            if d >= 4.0 && d <= 6.0 {
                return vec4(1.0, 1.0, 1.0, 1.0);
            }
            return vec4(0.15, 0.15, 0.15, 1.0);
        }
    }

    pub DrawHueStrip = {{DrawHueStrip}} {
        fn pixel(self) -> vec4 {
            let h = clamp(self.pos.y, 0.0, 1.0);
            let r = clamp(abs(fract(h) * 6.0 - 3.0) - 1.0, 0.0, 1.0);
            let g = clamp(abs(fract(h + 2.0 / 3.0) * 6.0 - 3.0) - 1.0, 0.0, 1.0);
            let b = clamp(abs(fract(h + 1.0 / 3.0) * 6.0 - 3.0) - 1.0, 0.0, 1.0);
            let p = self.pos * self.rect_size;
            if p.x < 1.0 || p.x > self.rect_size.x - 1.0 || p.y < 1.0 || p.y > self.rect_size.y - 1.0 {
                return vec4(0.29, 0.30, 0.32, 1.0);
            }
            return vec4(r, g, b, 1.0);
        }
    }

    pub DrawHueStripMarker = {{DrawHueStripMarker}} {
        fn pixel(self) -> vec4 {
            let p = self.pos * self.rect_size;
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            if p.x < 1.0 || p.x > w - 1.0 || p.y < 1.0 || p.y > h - 1.0 {
                return vec4(0.15, 0.15, 0.15, 1.0);
            }
            return vec4(1.0, 1.0, 1.0, 1.0);
        }
    }

    pub CrawradColorSquareBase = {{CrawradColorSquare}} {}

    pub CrawradColorSquare = <CrawradColorSquareBase> {
        width: 220.0, height: 220.0
    }

    pub CrawradHueStripBase = {{CrawradHueStrip}} {}

    pub CrawradHueStrip = <CrawradHueStripBase> {
        width: 18.0, height: 220.0
    }
}

#[crawrad_note::note("color-picker-drawcolorsquare")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawColorSquare {
    #[deref]
    pub draw_super: DrawQuad,
    /// Current hue in degrees.
    #[live]
    pub hue: f32,
}

#[crawrad_note::note("color-picker-drawcolorsquaremarker")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawColorSquareMarker {
    #[deref]
    pub draw_super: DrawQuad,
}

#[crawrad_note::note("color-picker-drawhuestrip")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawHueStrip {
    #[deref]
    pub draw_super: DrawQuad,
}

#[crawrad_note::note("color-picker-drawhuestripmarker")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawHueStripMarker {
    #[deref]
    pub draw_super: DrawQuad,
}

#[crawrad_note::note("color-picker-crawradcolorsquareaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradColorSquareAction {
    /// A saturation/value pair was picked by user interaction.
    Picked {
        /// Saturation fraction in `0.0..=1.0`.
        s: f32,
        /// Value fraction in `0.0..=1.0`.
        v: f32,
    },
    /// No action occurred.
    None,
}

#[crawrad_note::note("color-picker-crawradcolorsquare")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradColorSquare {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_square: DrawColorSquare,
    #[live]
    draw_marker: DrawColorSquareMarker,
    #[rust]
    area: Area,
    #[rust]
    hue: f32,
    #[rust]
    s: f32,
    #[rust]
    v: f32,
    #[rust]
    dragging: bool,
}

impl Widget for CrawradColorSquare {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) | Hit::FingerHoverOver(_) => {
                cx.set_cursor(MouseCursor::Crosshair);
            }
            Hit::FingerHoverOut(_) if !self.dragging => {
                cx.set_cursor(MouseCursor::Default);
            }
            Hit::FingerDown(e) => {
                cx.set_cursor(MouseCursor::Crosshair);
                self.dragging = true;
                let rect = self.area.rect(cx);
                let (s, v) = square_fractions(
                    (rect.pos.x, rect.pos.y),
                    (rect.size.x, rect.size.y),
                    (e.abs.x, e.abs.y),
                );
                self.s = s;
                self.v = v;
                self.redraw(cx);
                cx.widget_action(uid, &scope.path, CrawradColorSquareAction::Picked { s, v });
            }
            Hit::FingerMove(e) => {
                cx.set_cursor(MouseCursor::Crosshair);
                if self.dragging {
                    let rect = self.area.rect(cx);
                    let (s, v) = square_fractions(
                        (rect.pos.x, rect.pos.y),
                        (rect.size.x, rect.size.y),
                        (e.abs.x, e.abs.y),
                    );
                    self.s = s;
                    self.v = v;
                    self.redraw(cx);
                    cx.widget_action(uid, &scope.path, CrawradColorSquareAction::Picked { s, v });
                }
            }
            Hit::FingerUp(_) => {
                self.dragging = false;
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
            return DrawStep::done();
        }

        self.draw_square.hue = self.hue;
        self.draw_square.draw_abs(cx, rect);

        let (mx, my) = square_marker(
            (rect.pos.x, rect.pos.y),
            (rect.size.x, rect.size.y),
            self.s,
            self.v,
        );
        let marker_rect = Rect {
            pos: dvec2(mx - 7.0, my - 7.0),
            size: dvec2(14.0, 14.0),
        };
        self.draw_marker.draw_abs(cx, marker_rect);

        DrawStep::done()
    }
}

impl CrawradColorSquareRef {
    #[crawrad_note::note("color-picker-crawradcolorsquareref-set")]
    pub fn set(&self, cx: &mut Cx, hue: f32, s: f32, v: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hue = hue.rem_euclid(360.0);
            inner.s = s.clamp(0.0, 1.0);
            inner.v = v.clamp(0.0, 1.0);
            inner.redraw(cx);
        }
    }

    #[crawrad_note::note("color-picker-crawradcolorsquareref-picked")]
    pub fn picked(&self, actions: &Actions) -> Option<(f32, f32)> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradColorSquareAction::Picked { s, v } => Some((s, v)),
            _ => None,
        }
    }
}

#[crawrad_note::note("color-picker-crawradhuestripaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradHueStripAction {
    /// A hue strip fraction in `0.0..=1.0` was picked by user interaction.
    Picked(f32),
    /// No action occurred.
    None,
}

#[crawrad_note::note("color-picker-crawradhuestrip")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradHueStrip {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_strip: DrawHueStrip,
    #[live]
    draw_marker: DrawHueStripMarker,
    #[rust]
    area: Area,
    #[rust]
    fraction: f32,
    #[rust]
    dragging: bool,
}

impl Widget for CrawradHueStrip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        match event.hits(cx, self.area) {
            Hit::FingerDown(e) => {
                self.dragging = true;
                let rect = self.area.rect(cx);
                let frac = strip_fraction(rect.pos.y, rect.size.y, e.abs.y);
                self.fraction = frac;
                self.redraw(cx);
                cx.widget_action(uid, &scope.path, CrawradHueStripAction::Picked(frac));
            }
            Hit::FingerMove(e) if self.dragging => {
                let rect = self.area.rect(cx);
                let frac = strip_fraction(rect.pos.y, rect.size.y, e.abs.y);
                self.fraction = frac;
                self.redraw(cx);
                cx.widget_action(uid, &scope.path, CrawradHueStripAction::Picked(frac));
            }
            Hit::FingerUp(_) => {
                self.dragging = false;
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
            return DrawStep::done();
        }

        self.draw_strip.draw_abs(cx, rect);

        let marker_y = (rect.pos.y + (self.fraction as f64) * rect.size.y - 2.0).round();
        let marker_rect = Rect {
            pos: dvec2(rect.pos.x, marker_y),
            size: dvec2(rect.size.x, 4.0),
        };
        self.draw_marker.draw_abs(cx, marker_rect);

        DrawStep::done()
    }
}

impl CrawradHueStripRef {
    #[crawrad_note::note("color-picker-crawradhuestripref-set")]
    pub fn set(&self, cx: &mut Cx, fraction: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.fraction = fraction.clamp(0.0, 1.0);
            inner.redraw(cx);
        }
    }

    #[crawrad_note::note("color-picker-crawradhuestripref-picked")]
    pub fn picked(&self, actions: &Actions) -> Option<f32> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradHueStripAction::Picked(fraction) => Some(fraction),
            _ => None,
        }
    }
}

#[crawrad_note::note("color-picker-square-fractions")]
pub fn square_fractions(rect_pos: (f64, f64), rect_size: (f64, f64), at: (f64, f64)) -> (f32, f32) {
    if rect_size.0 <= 0.0 || rect_size.1 <= 0.0 {
        return (0.0, 0.0);
    }
    let s = ((at.0 - rect_pos.0) / rect_size.0).clamp(0.0, 1.0) as f32;
    let v = (1.0 - (at.1 - rect_pos.1) / rect_size.1).clamp(0.0, 1.0) as f32;
    (s, v)
}

#[crawrad_note::note("color-picker-strip-fraction")]
pub fn strip_fraction(top: f64, height: f64, y: f64) -> f32 {
    if height <= 0.0 {
        return 0.0;
    }
    ((y - top) / height).clamp(0.0, 1.0) as f32
}

#[crawrad_note::note("color-picker-square-marker")]
pub fn square_marker(rect_pos: (f64, f64), rect_size: (f64, f64), s: f32, v: f32) -> (f64, f64) {
    let s_clamped = f64::from(s.clamp(0.0, 1.0));
    let v_clamped = f64::from(v.clamp(0.0, 1.0));
    let x = rect_pos.0 + s_clamped * rect_size.0;
    let y = rect_pos.1 + (1.0 - v_clamped) * rect_size.1;
    (x, y)
}

#[cfg(test)]
#[path = "test/color_picker/test.rs"]
mod tests;
