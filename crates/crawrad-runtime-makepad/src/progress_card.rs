//! **A corner progress card component**: live progress, timing, and lingering status.

use makepad_widgets::*;

live_design! {
    link crawrad;
    use link::widgets::*;
    use link::theme::*;

    pub CrawradProgressCardBase = {{CrawradProgressCard}} {}

    pub CrawradProgressCard = <CrawradProgressCardBase> {
        width: 320.0, height: Fit,
        flow: Down,
        padding: 8.0,
        spacing: 6.0,
        show_bg: true,
        draw_bg: { color: #26292b }

        top_row = <View> {
            width: Fill, height: Fit,
            flow: Right,
            align: { y: 0.5 },
            spacing: 6.0,

            icon = <Label> {
                width: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 10.0 }
                    color: #f0f0f4
                }
                text: ""
            }
            headline = <Label> {
                width: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 10.0 }
                    color: #f0f0f4
                }
                text: ""
            }
            <View> { width: Fill, height: Fit }
            clock = <Label> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    color: #9da1aa
                }
                text: ""
            }
        }

        // a Label has no `visible`: the Release card hides the row
        detail_row = <View> {
            width: Fill, height: Fit,
            detail = <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    color: #9da1aa
                }
                text: ""
            }
        }

        card_bar = <View> {
            width: Fill, height: 4.0,
            show_bg: true,
            draw_bg: { color: #4d4f52 }
            bar = <View> {
                width: 0.0, height: Fill,
                show_bg: true,
                draw_bg: { color: #de5a2b }
            }
        }

        size_row = <View> {
            visible: false,
            width: Fill, height: Fit,
            flow: Right,
            align: { y: 0.5 },
            spacing: 8.0,

            size_label = <Label> {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: #f0f0f4
                }
                text: ""
            }
            change_label = <Label> {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                    color: #de5a2b
                }
                text: ""
            }
        }

        mostly_row = <View> {
            visible: false,
            width: Fill, height: Fit,

            mostly_label = <Label> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    color: #9da1aa
                }
                text: ""
            }
        }

        bottom_row = <View> {
            width: Fill, height: Fit,
            flow: Right,
            align: { y: 0.5 },
            spacing: 6.0,

            <View> { width: Fill, height: Fit }
            btn_cancel = <Button> { width: Fit, height: 22.0, text: "" }
            btn_show = <Button> { width: Fit, height: 22.0, text: "" }
        }
    }
}

#[crawrad_note::note("progress-card-crawradprogresscard")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradProgressCard {
    #[deref]
    view: View,
}

impl Widget for CrawradProgressCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[crawrad_note::note("progress-card-progress-bar-width")]
pub fn progress_bar_width(area_width: f64, fraction: Option<f64>) -> Option<f64> {
    fraction.map(|f| (area_width * f).max(2.0))
}

impl CrawradProgressCardRef {
    #[crawrad_note::note("progress-card-crawradprogresscardref-set")]
    pub fn set(
        &self,
        cx: &mut Cx,
        icon: &str,
        headline: &str,
        detail: &str,
        clock: &str,
        fraction: Option<f64>,
        running: bool,
    ) {
        let Some(inner) = self.borrow() else {
            return;
        };
        inner.view.label(id!(icon)).set_text(cx, icon);
        inner.view.label(id!(headline)).set_text(cx, headline);
        inner.view.label(id!(detail)).set_text(cx, detail);
        inner.view.label(id!(clock)).set_text(cx, clock);

        let cancel_btn = inner.view.button(id!(btn_cancel));
        cancel_btn.set_visible(cx, running);

        inner.view.view(id!(size_row)).set_visible(cx, false);
        inner.view.view(id!(mostly_row)).set_visible(cx, false);
        inner.view.view(id!(detail_row)).set_visible(cx, true);

        let bar_view = inner.view.view(id!(card_bar));
        bar_view.set_visible(cx, fraction.is_some());
        if let Some(w) = progress_bar_width(bar_view.area().rect(cx).size.x, fraction) {
            inner
                .view
                .view(id!(card_bar.bar))
                .apply_over(cx, live! { width: (w), height: Fill });
        }
    }

    /// Sets the progress card into the finished Release build state.
    pub fn set_release(
        &self,
        cx: &mut Cx,
        headline: &str,
        size: &str,
        change: Option<(&str, bool /* grew */)>,
        mostly: Option<&str>,
    ) {
        let Some(inner) = self.borrow() else {
            return;
        };
        inner.view.label(id!(icon)).set_text(cx, "✓");
        inner.view.label(id!(headline)).set_text(cx, headline);
        inner.view.view(id!(detail_row)).set_visible(cx, false);
        inner.view.label(id!(clock)).set_text(cx, "");

        inner.view.button(id!(btn_cancel)).set_visible(cx, true);
        inner.view.view(id!(card_bar)).set_visible(cx, false);

        let size_row = inner.view.view(id!(size_row));
        size_row.set_visible(cx, true);
        inner
            .view
            .label(id!(size_row.size_label))
            .set_text(cx, size);

        let change_label = inner.view.label(id!(size_row.change_label));
        if let Some((chg_text, grew)) = change {
            change_label.set_text(cx, chg_text);
            let color = if grew {
                vec4(0.871, 0.353, 0.169, 1.0)
            } else {
                vec4(0.486, 0.706, 0.420, 1.0)
            };
            change_label.apply_over(cx, live! { draw_text: { color: (color) } });
        } else {
            change_label.set_text(cx, "");
        }

        let mostly_row = inner.view.view(id!(mostly_row));
        if let Some(m_text) = mostly {
            mostly_row.set_visible(cx, true);
            inner
                .view
                .label(id!(mostly_row.mostly_label))
                .set_text(cx, m_text);
        } else {
            mostly_row.set_visible(cx, false);
            inner
                .view
                .label(id!(mostly_row.mostly_label))
                .set_text(cx, "");
        }
    }

    #[crawrad_note::note("progress-card-crawradprogresscardref-set-button-texts")]
    pub fn set_button_texts(&self, cx: &mut Cx, show: &str, cancel: &str) {
        let Some(inner) = self.borrow() else {
            return;
        };
        inner.view.button(id!(btn_show)).set_text(cx, show);
        inner.view.button(id!(btn_cancel)).set_text(cx, cancel);
    }

    #[crawrad_note::note("progress-card-crawradprogresscardref-show-clicked")]
    pub fn show_clicked(&self, actions: &Actions) -> bool {
        self.borrow()
            .is_some_and(|inner| inner.view.button(id!(btn_show)).clicked(actions))
    }

    #[crawrad_note::note("progress-card-crawradprogresscardref-cancel-clicked")]
    pub fn cancel_clicked(&self, actions: &Actions) -> bool {
        self.borrow()
            .is_some_and(|inner| inner.view.button(id!(btn_cancel)).clicked(actions))
    }

    #[crawrad_note::note("progress-card-crawradprogresscardref-set-visible")]
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.set_visible(cx, visible);
        }
    }
}

#[cfg(test)]
#[path = "test/progress_card/test.rs"]
mod tests;
