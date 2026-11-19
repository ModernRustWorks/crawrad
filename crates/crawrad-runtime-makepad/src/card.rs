//! **The palette's Card as a widget** — `CrawradFormCard` (ADR 0051).
//!
//! The generated apps and the IDE use the SAME widget: a rounded surface with
//! border and shadow in ONE view (so it works with `height: Fit`), a `header`
//! (title, subtitle, divider) and the children on top (`flow: Overlay`, each
//! child placed by its margin). Before, the generator wrote ~80 lines of DSL per
//! Card and the IDE had its own cards — two ideas of one component.
//!
//! State goes through [`CrawradFormCardRef`]: the look (radius, elevation), the
//! header, the colours (selection) and the click.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub CrawradFormCardBase = {{CrawradFormCard}} {}

    pub CrawradFormCard = <CrawradFormCardBase> {
        width: 220, height: 140,
        flow: Overlay,
        clip_x: false, clip_y: false,
        show_bg: true,
        // The surface: makepad-widgets 1.0's `RoundedShadowView` shader
        // (view_ui.rs, MIT OR Apache-2.0) — a widget type cannot inherit
        // another's `draw_bg`, so it is restated here.
        draw_bg: {
            color: #2f2f45
            uniform border_radius: 8.0
            uniform border_size: 1.0
            uniform border_color: #6e7290
            uniform shadow_color: #00000033
            uniform shadow_radius: 4.0
            uniform shadow_offset: vec2(0.0, 2.0)
            varying rect_size2: vec2,
            varying rect_size3: vec2,
            varying rect_pos2: vec2,
            varying rect_shift: vec2,
            varying sdf_rect_pos: vec2,
            varying sdf_rect_size: vec2,
            fn vertex(self) -> vec4 {
                let min_offset = min(self.shadow_offset, vec2(0));
                self.rect_size2 = self.rect_size + 2.0 * vec2(self.shadow_radius);
                self.rect_size3 = self.rect_size2 + abs(self.shadow_offset);
                self.rect_pos2 = self.rect_pos - vec2(self.shadow_radius) + min_offset;
                self.sdf_rect_size = self.rect_size2 - vec2(self.shadow_radius * 2.0 + self.border_size * 2.0)
                self.sdf_rect_pos = -min_offset + vec2(self.border_size + self.shadow_radius);
                self.rect_shift = -min_offset;
                return self.clip_and_transform_vertex(self.rect_pos2, self.rect_size3)
            }
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size3)
                sdf.box(self.sdf_rect_pos.x, self.sdf_rect_pos.y, self.sdf_rect_size.x, self.sdf_rect_size.y, max(1.0, self.border_radius))
                if sdf.shape > -1.0 {
                    let m = self.shadow_radius;
                    let o = self.shadow_offset + self.rect_shift;
                    let v = GaussShadow::rounded_box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3 + vec2(m)), self.shadow_radius * 0.5, self.border_radius * 2.0);
                    sdf.clear(self.shadow_color * v)
                }
                sdf.fill_keep(self.color)
                if self.border_size > 0.0 {
                    sdf.stroke(self.border_color, self.border_size)
                }
                return sdf.result
            }
        }
        header = <View> {
            width: Fill, height: 28.0,
            flow: Overlay,
            labels = <View> {
                width: Fill, height: Fill,
                flow: Down, spacing: 2.0,
                padding: { left: 12.0, top: 6.0, right: 12.0 },
                title = <Label> {
                    padding: 0, margin: 0,
                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 10.0 }, color: #f2f2fa }
                    text: "Card"
                }
                subtitle = <Label> {
                    padding: 0, margin: 0,
                    draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }, color: #a6adc7 }
                    text: ""
                }
            }
            divider = <View> {
                margin: { top: 27.0 },
                width: Fill, height: 1.0,
                show_bg: true,
                draw_bg: { color: #6e729099 }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradFormCardAction {
    Clicked,
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct CrawradFormCard {
    #[deref]
    view: View,
}

impl Widget for CrawradFormCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // The children answer first (a button inside the card still works);
        // a tap on the card's own surface is the card's click.
        self.view.handle_event(cx, event, scope);
        if let Hit::FingerUp(e) = event.hits(cx, self.view.area())
            && e.is_over
        {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradFormCardAction::Clicked,
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[crawrad_note::note("card-header-height")]
pub fn header_height(subtitle: &str) -> f64 {
    if subtitle.is_empty() { 28.0 } else { 42.0 }
}

#[crawrad_note::note("card-shadow")]
pub fn shadow(elevation: i64) -> (f32, f64) {
    let e = elevation.clamp(0, 3);
    if e == 0 {
        return (0.0, 0.0);
    }
    ((0.12 + 0.08 * e as f32).min(0.4), e as f64 * 2.0)
}

impl CrawradFormCardRef {
    #[crawrad_note::note("card-crawformcardref-set-look")]
    pub fn set_look(&self, cx: &mut Cx, corner_radius: i64, elevation: i64) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        let r = corner_radius.max(0) as f64;
        let (alpha, offset) = shadow(elevation);
        let (o, blur) = (offset as f32, offset * 2.0);
        inner.view.apply_over(
            cx,
            live! { draw_bg: {
                border_radius: (r),
                shadow_color: (vec4(0.0, 0.0, 0.0, alpha)),
                shadow_offset: (vec2(0.0, o)),
                shadow_radius: (blur),
            } },
        );
    }

    #[crawrad_note::note("card-crawformcardref-set-header")]
    pub fn set_header(&self, cx: &mut Cx, title: &str, subtitle: &str, show: bool) {
        let Some(inner) = self.borrow() else {
            return;
        };
        let header = inner.view.view(id!(header));
        header.set_visible(cx, show);
        let h = header_height(subtitle);
        header.apply_over(cx, live! { height: (h) });
        header
            .view(id!(divider))
            .apply_over(cx, live! { margin: { top: (h - 1.0) } });
        header.label(id!(title)).set_text(cx, title);
        let sub = header.label(id!(subtitle));
        sub.set_text(cx, subtitle);
        sub.set_visible(cx, !subtitle.is_empty());
    }

    #[crawrad_note::note("card-crawformcardref-set-colors")]
    pub fn set_colors(&self, cx: &mut Cx, fill: Vec4, border: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.apply_over(
                cx,
                live! { draw_bg: { color: (fill), border_color: (border) } },
            );
        }
    }

    #[crawrad_note::note("card-crawformcardref-clicked")]
    pub fn clicked(&self, actions: &Actions) -> bool {
        let uid = self.widget_uid();
        actions
            .iter()
            .filter_map(|a| a.downcast_ref::<WidgetAction>())
            .filter(|w| w.widget_uid == uid)
            .any(|w| {
                matches!(
                    w.action.downcast_ref::<CrawradFormCardAction>(),
                    Some(CrawradFormCardAction::Clicked)
                )
            })
    }
}

#[crawrad_note::note("card-apply")]
pub fn apply(cx: &mut Cx, card: &WidgetRef, corner_radius: i64, elevation: i64) {
    card.as_crawrad_form_card()
        .set_look(cx, corner_radius, elevation);
}

#[cfg(test)]
#[path = "test/card/test.rs"]
mod tests;
