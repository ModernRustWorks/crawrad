//! **A Drawer's scrim, side and shadow offset at run time**.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    pub DrawScrim = {{DrawScrim}} {
        fn pixel(self) -> vec4 {
            return self.color;
        }
    }

    pub CrawradScrimBase = {{CrawradScrim}} {}

    pub CrawradScrim = <CrawradScrimBase> {
        width: Fill, height: Fill,
        draw_bg: {
            color: #00000066
        }
    }
}

#[crawrad_note::note("drawer-drawscrim")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawScrim {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
}

#[crawrad_note::note("drawer-crawradscrimaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradScrimAction {
    Clicked,
    None,
}

#[crawrad_note::note("drawer-crawradscrim")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradScrim {
    #[redraw]
    #[live]
    pub draw_bg: DrawScrim,

    #[walk]
    pub walk: Walk,

    #[live]
    pub visible: bool,
}

impl Widget for CrawradScrim {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let area = self.draw_bg.area();
        if let Hit::FingerUp(_) = event.hits(cx, area) {
            cx.widget_action(self.widget_uid(), &scope.path, CrawradScrimAction::Clicked);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_bg.begin(cx, walk, Layout::default());
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl CrawradScrimRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), CrawradScrimAction::Clicked)
        } else {
            false
        }
    }

    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }
}

#[crawrad_note::note("drawer-apply")]
pub fn apply(cx: &mut Cx, drawer: &WidgetRef, side: &str) {
    let is_right = side == "dsRight";
    let shadow_offset_x = if is_right { -4.0 } else { 4.0 };

    drawer.view(id!(shadow)).apply_over(
        cx,
        live! {
            margin: { left: (shadow_offset_x), top: 0.0 },
            draw_bg: {
                color: (vec4(0.0, 0.0, 0.0, 0.2)),
            }
        },
    );

    // The border is the DSL's: `border_size`/`border_color` are uniforms.
    drawer.view(id!(surface)).apply_over(
        cx,
        live! {
            draw_bg: {
                color: (vec4(0.184, 0.184, 0.271, 1.0)),
            }
        },
    );
}

#[cfg(test)]
#[path = "test/drawer/test.rs"]
mod drawer_test;
