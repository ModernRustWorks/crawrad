//! **A PaintBox draws what its `OnPaint` drew**.
//!
//! The generated app runs the user's `OnPaint` against a `Canvas` (a list
//! of commands, the same text on both targets) and hands the commands here;
//! [`CrawradPaintBox`] draws them in its own box, every frame, until the next
//! set arrives. One `Sdf2d` shader draws the rectangles, circles and lines,
//! and `DrawText` the text.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    pub DrawPaint = {{DrawPaint}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            let e = self.width;
            if self.kind < 0.5 {
                // `Sdf2d::box` is no signed distance inside (std.rs:394), so
                // a square corner uses `rect`, and `box` rounds by 2r
                if self.radius < 1.5 {
                    sdf.rect(e * 0.5, e * 0.5, w - e, h - e);
                } else {
                    sdf.box(e * 0.5, e * 0.5, w - e, h - e, self.radius * 0.5);
                }
                sdf.fill_keep(self.fill);
                if e > 0.0 {
                    sdf.stroke(self.stroke, e);
                }
            } else if self.kind < 1.5 {
                sdf.circle(w * 0.5, h * 0.5, (min(w, h) - e) * 0.5);
                sdf.fill_keep(self.fill);
                if e > 0.0 {
                    sdf.stroke(self.stroke, e);
                }
            } else {
                sdf.move_to(self.p0.x, self.p0.y);
                sdf.line_to(self.p1.x, self.p1.y);
                sdf.stroke(self.stroke, max(e, 1.0));
            }
            return sdf.result;
        }
    }

    pub CrawradPaintBoxBase = {{CrawradPaintBox}} {}

    pub CrawradPaintBox = <CrawradPaintBoxBase> {
        width: Fill, height: Fill,
        draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 12.0 } }
    }
}

#[crawrad_note::note("paint-command")]
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        fill: Option<[u8; 4]>,
        stroke: Option<([u8; 4], f32)>,
    },
    Circle {
        x: f32,
        y: f32,
        r: f32,
        fill: Option<[u8; 4]>,
        stroke: Option<([u8; 4], f32)>,
    },
    Line {
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: [u8; 4],
        width: f32,
    },
    Text {
        text: String,
        x: f32,
        y: f32,
        color: [u8; 4],
        size: f32,
    },
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPaint {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub kind: f32,
    #[live]
    pub fill: Vec4,
    #[live]
    pub stroke: Vec4,
    #[live]
    pub width: f32,
    #[live]
    pub radius: f32,
    #[live]
    pub p0: Vec2,
    #[live]
    pub p1: Vec2,
}

#[crawrad_note::note("paint-crawradpaintbox")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradPaintBox {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_paint: DrawPaint,
    #[live]
    draw_text: DrawText,
    #[rust]
    commands: Vec<Command>,
}

fn color(c: [u8; 4]) -> Vec4 {
    vec4(
        f32::from(c[0]) / 255.0,
        f32::from(c[1]) / 255.0,
        f32::from(c[2]) / 255.0,
        f32::from(c[3]) / 255.0,
    )
}

const NONE: Vec4 = Vec4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

impl Widget for CrawradPaintBox {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let area = cx.walk_turtle(walk);
        paint(
            cx,
            &mut self.draw_paint,
            &mut self.draw_text,
            area.pos,
            &self.commands,
        );
        DrawStep::done()
    }
}

#[crawrad_note::note("paint-paint")]
pub fn paint(
    cx: &mut Cx2d,
    draw_paint: &mut DrawPaint,
    draw_text: &mut DrawText,
    origin: DVec2,
    commands: &[Command],
) {
    let at = |x: f32, y: f32| dvec2(origin.x + x as f64, origin.y + y as f64);
    for c in commands {
        match c {
            Command::Rect {
                x,
                y,
                w,
                h,
                radius,
                fill,
                stroke,
            } => {
                draw_paint.kind = 0.0;
                draw_paint.radius = *radius;
                draw_paint.fill = fill.map_or(NONE, color);
                let (s, sw) = stroke.map_or((NONE, 0.0), |(c, w)| (color(c), w));
                draw_paint.stroke = s;
                draw_paint.width = sw;
                let r = Rect {
                    pos: at(*x, *y),
                    size: dvec2(*w as f64, *h as f64),
                };
                draw_paint.draw_abs(cx, r);
            }
            Command::Circle {
                x,
                y,
                r,
                fill,
                stroke,
            } => {
                draw_paint.kind = 1.0;
                draw_paint.fill = fill.map_or(NONE, color);
                let (s, sw) = stroke.map_or((NONE, 0.0), |(c, w)| (color(c), w));
                draw_paint.stroke = s;
                draw_paint.width = sw;
                let rect = Rect {
                    pos: at(x - r, y - r),
                    size: dvec2(2.0 * *r as f64, 2.0 * *r as f64),
                };
                draw_paint.draw_abs(cx, rect);
            }
            Command::Line {
                x0,
                y0,
                x1,
                y1,
                color: c,
                width,
            } => {
                // a quad around the segment, with room for its width
                let pad = width.max(1.0) + 1.0;
                let (lx, ly) = (x0.min(*x1) - pad, y0.min(*y1) - pad);
                let (hx, hy) = (x0.max(*x1) + pad, y0.max(*y1) + pad);
                draw_paint.kind = 2.0;
                draw_paint.stroke = color(*c);
                draw_paint.width = *width;
                draw_paint.p0 = vec2(x0 - lx, y0 - ly);
                draw_paint.p1 = vec2(x1 - lx, y1 - ly);
                let r = Rect {
                    pos: at(lx, ly),
                    size: dvec2((hx - lx) as f64, (hy - ly) as f64),
                };
                draw_paint.draw_abs(cx, r);
            }
            Command::Text {
                text,
                x,
                y,
                color: c,
                size,
            } => {
                draw_text.color = color(*c);
                draw_text.text_style.font_size = *size;
                draw_text.draw_abs(cx, at(*x, *y), text);
            }
        }
    }
}

impl CrawradPaintBoxRef {
    #[crawrad_note::note("paint-crawradpaintboxref-set-commands")]
    pub fn set_commands(&self, cx: &mut Cx, commands: Vec<Command>) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.commands != commands
        {
            inner.commands = commands;
            inner.draw_paint.redraw(cx);
        }
    }
}
