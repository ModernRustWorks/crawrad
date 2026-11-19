//! The Tab order (`tab_order.rs`): Tab and Shift+Tab walk the controls
//! in `TabOrder`, through a real egui `Context`.

use super::*;

fn rect(x: f32, w: f32) -> egui::Rect {
    egui::Rect::from_min_size(egui::pos2(x, 40.0), egui::vec2(w, 24.0))
}

/// A form drawn left to right — A, B, C — with the Tab order B, C, A. C is
/// a multiline edit that keeps Tab when `c_keeps_tab`. The panel behind A
/// is a stop too, and has no widget of its own.
struct Form {
    ctx: egui::Context,
    order: CrawradTabOrder,
    text: [String; 3],
    c_keeps_tab: bool,
    /// The widgets of A, B and C, as the last frame drew them.
    ids: [egui::Id; 3],
    /// A Label's click this frame: the name its `FocusControl` holds.
    link: Option<&'static str>,
}

impl Form {
    fn new(c_keeps_tab: bool) -> Self {
        let mut f = Form {
            ctx: egui::Context::default(),
            order: CrawradTabOrder::default(),
            text: Default::default(),
            c_keeps_tab,
            ids: [egui::Id::NULL; 3],
            link: None,
        };
        f.frame(Vec::new());
        f
    }

    /// One frame with `events`; the widget with the focus after it.
    fn frame(&mut self, events: Vec<egui::Event>) -> Option<egui::Id> {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..Default::default()
        };
        let (order, text, ids, keeps, link) = (
            &mut self.order,
            &mut self.text,
            &mut self.ids,
            self.c_keeps_tab,
            self.link.take(),
        );
        self.ctx
            .run_ui(input, |ui| {
                order.before(ui);
                if let Some(name) = link {
                    order.focus(ui, name);
                }
                let mut stops: Vec<CrawradTabStop> = Vec::new();
                let [a, b, c] = text;
                let panel =
                    egui::Rect::from_min_size(egui::pos2(0.0, 30.0), egui::vec2(150.0, 60.0));
                stops.push((&[2], panel, false, ""));
                ids[0] = ui.put(rect(10.0, 120.0), egui::TextEdit::singleline(a)).id;
                stops.push((&[2, 0], rect(10.0, 120.0), false, "a"));
                ids[1] = ui.put(rect(200.0, 120.0), egui::TextEdit::singleline(b)).id;
                stops.push((&[0], rect(200.0, 120.0), false, "b"));
                ids[2] = ui
                    .put(
                        rect(400.0, 120.0),
                        egui::TextEdit::multiline(c).lock_focus(keeps),
                    )
                    .id;
                stops.push((&[1], rect(400.0, 120.0), keeps, "c"));
                order.after(ui, &mut stops);
            })
            .textures_delta
            .clear();
        self.ctx.memory(|m| m.focused())
    }

    fn tab(&mut self, shift: bool) -> Option<egui::Id> {
        let modifiers = if shift {
            egui::Modifiers::SHIFT
        } else {
            egui::Modifiers::NONE
        };
        let key = |pressed| egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers,
        };
        self.frame(vec![key(true), key(false)]);
        // the focus asked for lands on the next frame, as egui's own does
        self.frame(Vec::new())
    }
}

#[test]
fn tab_walks_the_tab_order_and_wraps_round() {
    let mut f = Form::new(false);
    let [a, b, c] = f.ids;
    assert_eq!(
        f.tab(false),
        Some(b),
        "first the lowest TabOrder, not the leftmost"
    );
    assert_eq!(f.tab(false), Some(c));
    assert_eq!(
        f.tab(false),
        Some(a),
        "the panel's edit, once: the panel has no widget"
    );
    assert_eq!(f.tab(false), Some(b), "and round again");
}

#[test]
fn shift_tab_walks_it_backwards() {
    let mut f = Form::new(false);
    let [a, b, c] = f.ids;
    assert_eq!(f.tab(true), Some(a), "with no focus, the last");
    assert_eq!(f.tab(true), Some(c));
    assert_eq!(f.tab(true), Some(b));
    assert_eq!(f.tab(true), Some(a), "round from the first to the last");
}

#[test]
fn a_control_that_keeps_tab_keeps_the_focus() {
    let mut f = Form::new(true);
    let [_, _, c] = f.ids;
    f.tab(false);
    assert_eq!(f.tab(false), Some(c));
    assert_eq!(
        f.tab(false),
        Some(c),
        "a Memo with WantTabs types the Tab instead"
    );
}

#[test]
fn the_next_stop_wraps_round_both_ways() {
    assert_eq!(crawrad_tab_next(None, 0, true), None);
    assert_eq!(crawrad_tab_next(None, 3, true), Some(0));
    assert_eq!(crawrad_tab_next(None, 3, false), Some(2));
    assert_eq!(crawrad_tab_next(Some(2), 3, true), Some(0));
    assert_eq!(crawrad_tab_next(Some(0), 3, false), Some(2));
}

#[test]
fn a_label_click_focuses_its_focus_control_by_name() {
    let mut f = Form::new(false);
    let [_, _, c] = f.ids;
    f.link = Some("c");
    f.frame(Vec::new());
    assert_eq!(
        f.frame(Vec::new()),
        Some(c),
        "the FocusControl takes the keyboard"
    );
    f.link = Some("gone");
    f.frame(Vec::new());
    assert_eq!(
        f.frame(Vec::new()),
        Some(c),
        "a name that is no stop does nothing"
    );
}
