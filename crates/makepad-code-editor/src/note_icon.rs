//! Note-kind gutter icons (Documentation, Note, Comment).

use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub NoteFileIcon = {{DrawColor}} {
        draw_depth: 1.05,
        color: #F0B43C,
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            let f = w * 0.32;
            sdf.move_to(0.5, 0.5);
            sdf.line_to(w - f, 0.5);
            sdf.line_to(w - 0.5, f);
            sdf.line_to(w - 0.5, h - 0.5);
            sdf.line_to(0.5, h - 0.5);
            sdf.close_path();
            sdf.fill(self.color);
            sdf.move_to(w - f, 0.8);
            sdf.line_to(w - f, f);
            sdf.line_to(w - 0.8, f);
            sdf.stroke(#9A6A12, 1.4);
            return sdf.result * self.color.w;
        }
    }

    pub NoteDocIcon = {{DrawColor}} {
        draw_depth: 1.05,
        color: #4DA3FF,
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            sdf.box(0.5, 0.5, w - 1.0, h - 1.0, 1.8);
            sdf.fill(self.color);
            sdf.move_to(w * 0.22, h * 0.30);
            sdf.line_to(w * 0.78, h * 0.30);
            sdf.move_to(w * 0.22, h * 0.50);
            sdf.line_to(w * 0.78, h * 0.50);
            sdf.move_to(w * 0.22, h * 0.70);
            sdf.line_to(w * 0.54, h * 0.70);
            sdf.stroke(#0E2440, 1.2);
            return sdf.result * self.color.w;
        }
    }

    pub NoteCommentIcon = {{DrawColor}} {
        draw_depth: 1.05,
        color: #3DC9B0,
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            let r = min(w, h) * 0.18;
            let bh = h * 0.74;
            sdf.move_to(r, 0.5);
            sdf.line_to(w - r, 0.5);
            sdf.line_to(w - 0.5, r);
            sdf.line_to(w - 0.5, bh - r);
            sdf.line_to(w - r, bh);
            sdf.line_to(w * 0.42, bh);
            sdf.line_to(0.5, h - 0.5);
            sdf.line_to(w * 0.18, bh);
            sdf.line_to(0.5, bh - r);
            sdf.line_to(0.5, r);
            sdf.close_path();
            sdf.stroke(self.color, 1.6);
            return sdf.result * self.color.w;
        }
    }

    pub NoteKindIcon = {{NoteKindIcon}} {
        width: 20.0,
        height: 20.0,
        draw_doc: <NoteDocIcon> {},
        draw_note: <NoteFileIcon> {},
        draw_comment: <NoteCommentIcon> {},
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct NoteKindIcon {
    #[walk]
    walk: Walk,
    #[live(0)]
    pub kind: i64,
    #[live(1.0)]
    pub alpha: f32,
    #[live]
    draw_doc: DrawColor,
    #[live]
    draw_note: DrawColor,
    #[live]
    draw_comment: DrawColor,
    #[redraw]
    #[rust]
    area: Area,
}

impl Widget for NoteKindIcon {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        let alpha = self.alpha;
        let draw = match self.kind {
            0 => &mut self.draw_doc,
            2 => &mut self.draw_comment,
            _ => &mut self.draw_note,
        };
        draw.color.w = alpha;
        // Its own draw call: batched into the gutter's, it drew under the dialog.
        draw.new_draw_call(cx);
        draw.draw_abs(cx, rect);
        DrawStep::done()
    }
}

impl NoteKindIcon {
    pub fn set_kind(&mut self, cx: &mut Cx, kind: i64) {
        if self.kind != kind {
            self.kind = kind;
            self.area.redraw(cx);
        }
    }

    pub fn set_alpha(&mut self, cx: &mut Cx, alpha: f32) {
        if (self.alpha - alpha).abs() > 0.001 {
            self.alpha = alpha;
            self.area.redraw(cx);
        }
    }
}

impl NoteKindIconRef {
    pub fn set_kind(&self, cx: &mut Cx, kind: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_kind(cx, kind);
        }
    }

    pub fn set_alpha(&self, cx: &mut Cx, alpha: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_alpha(cx, alpha);
        }
    }
}
