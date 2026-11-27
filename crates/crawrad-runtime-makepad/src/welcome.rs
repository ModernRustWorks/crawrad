//! The Welcome page: where the IDE starts when there is no project to
//! reopen.
//!
//! Left, the Start commands and the `On start` box; right, the recent
//! projects. The page draws rows already in words and decides nothing: a
//! click comes back as a [`WelcomeAction`], and the shell runs the catalog's
//! action for it (`project.new`, `project.open_folder`, `file.open_form`,
//! `project.forget_recent`), the same ones the menus and the MCP run.

use makepad_widgets::*;

/// The crab drawing: the empty Welcome list and the chat's Rust Teacher card.
pub const WELCOME_CRAB_PNG: &[u8] = include_bytes!("../resources/welcome-crab.png");

live_design! {
    link crawrad;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use crate::borderless::*;

    WelcomeLink = <CrawradBorderlessButton> {
        width: Fit, height: 28.0,
        padding: { left: 0.0, right: 0.0 },
        align: { x: 0.0, y: 0.5 },
        draw_bg: {
            color: #0000, color_hover: #0000, color_down: #0000, color_focus: #0000,
        }
        draw_text: {
            color: #ff8a5c, color_hover: #ffab88, color_down: #ffab88, color_focus: #ff8a5c,
            text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
        }
        text: ""
    }

    WelcomeButton = <Button> {
        width: Fit, height: 28.0,
        padding: { left: 14.0, right: 14.0 },
        draw_bg: {
            color: #2b2d30, color_hover: #35373b, color_down: #3c3f41,
            border_color_1: #4a4d52, border_color_2: #4a4d52,
        }
        draw_text: { color: #dfe1e5, text_style: <THEME_FONT_REGULAR> { font_size: 10.0 } }
        text: ""
    }

    WelcomeHeading = <Label> {
        width: Fit, height: Fit,
        draw_text: { color: #c9ccd1, text_style: <THEME_FONT_BOLD> { font_size: 10.0 } }
        text: ""
    }

    WelcomeRecentRow = <View> {
        width: Fill, height: 50.0,
        flow: Overlay,
        show_bg: true,
        draw_bg: { color: #232427 }
        btn = <CrawradBorderlessButton> {
            width: Fill, height: Fill,
            draw_bg: {
                color: #0000, color_hover: #2e436e, color_down: #35507f, color_focus: #0000,
            }
            draw_text: { text_style: { font_size: 0.0 } }
            text: ""
        }
        cols = <View> {
            width: Fill, height: Fill,
            flow: Right,
            align: { y: 0.5 },
            padding: { left: 12.0, right: 4.0 },
            names = <View> {
                width: Fill, height: Fit,
                flow: Down,
                spacing: 2.0,
                lbl_name = <Label> { width: Fill, height: Fit, draw_text: { color: #f0f1f3, text_style: <THEME_FONT_BOLD> { font_size: 10.0 } }, text: "" }
                lbl_place = <Label> { width: Fill, height: Fit, draw_text: { color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
            }
            lbl_forms = <Label> { width: 90.0, height: Fit, draw_text: { color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 9.5 } }, text: "" }
            lbl_age = <Label> { width: 110.0, height: Fit, draw_text: { color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 9.5 } }, text: "" }
            btn_forget = <CrawradBorderlessButton> {
                width: 28.0, height: 28.0,
                draw_bg: {
                    color: #0000, color_hover: #ffffff18, color_down: #ffffff24, color_focus: #0000,
                }
                draw_text: { color: #6f737b, color_hover: #f0f1f3, color_down: #f0f1f3, color_focus: #6f737b, text_style: <THEME_FONT_REGULAR> { font_size: 10.0 } }
                text: "×"
            }
        }
        sep = <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #333538 } }
    }

    pub CrawradWelcomePageBase = {{CrawradWelcomePage}} {}

    pub CrawradWelcomePage = <CrawradWelcomePageBase> {
        width: Fill, height: Fill,
        flow: Down,
        show_bg: true,
        // The page reproduces the card it sits on (CrawradOutlined, radius
        // CRAWRAD_CARD_RADIUS = 6): same radius, all four corners, flush against
        // it. Painting a square page over a rounded card was what squared off
        // Welcome; rounding it differently only let the card show around it.
        draw_bg: {
            color: #1e1f22
            uniform border_radius: 6.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        tab_bar = <View> {
            width: Fill, height: 32.0,
            flow: Right,
            align: { y: 1.0 },
            padding: { left: 8.0 },
            show_bg: true,
            draw_bg: { color: #2b2d30 }
            tab = <View> {
                width: Fit, height: 28.0,
                flow: Right,
                align: { y: 0.5 },
                spacing: 8.0,
                padding: { left: 12.0, right: 6.0 },
                show_bg: true,
                draw_bg: { color: #1e1f22 }
                <Label> { draw_text: { color: #f0f1f3, text_style: <THEME_FONT_REGULAR> { font_size: 10.0 } }, text: "Welcome" }
                btn_close = <CrawradBorderlessButton> {
                    width: 20.0, height: 20.0,
                    draw_bg: {
                        color: #0000, color_hover: #ffffff18, color_down: #ffffff24, color_focus: #0000,
                    }
                    draw_text: { color: #9da0a8, color_hover: #f0f1f3, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                    text: "×"
                }
            }
        }

        body = <View> {
            width: Fill, height: Fill,
            flow: Right,
            spacing: 56.0,
            padding: { left: 64.0, right: 64.0, top: 48.0, bottom: 48.0 },

            left = <View> {
                width: 300.0, height: Fill,
                flow: Down,
                spacing: 28.0,

                <View> {
                    width: Fill, height: Fit,
                    flow: Down,
                    spacing: 4.0,
                    <Label> { draw_text: { color: #f0f1f3, text_style: <THEME_FONT_BOLD> { font_size: 22.0 } }, text: "CrawRAD" }
                    <Label> { draw_text: { color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 10.0 } }, text: "Visual forms for Rust" }
                }

                <View> {
                    width: Fill, height: Fit,
                    flow: Down,
                    spacing: 4.0,
                    <WelcomeHeading> { text: "Start" }
                    btn_new_project = <WelcomeLink> { text: "New Project…" }
                    btn_open_folder = <WelcomeLink> { text: "Open Folder…" }
                    btn_open_form = <WelcomeLink> { text: "Open Form…" }
                }

                <View> {
                    width: Fill, height: Fit,
                    flow: Down,
                    spacing: 8.0,
                    <WelcomeHeading> { text: "On start" }
                    chk_reopen = <CheckBox> {
                        text: "Reopen the last project",
                        draw_text: { color: #dfe1e5, text_style: <THEME_FONT_REGULAR> { font_size: 10.0 } }
                    }
                    lbl_reopen_note = <Label> {
                        width: Fill, height: Fit,
                        draw_text: { wrap: Word, color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                        text: ""
                    }
                }
            }

            right = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 10.0,

                <View> {
                    width: Fill, height: Fit,
                    flow: Right,
                    align: { y: 0.5 },
                    <WelcomeHeading> { text: "Recent" }
                    <View> { width: Fill, height: Fit }
                    lbl_recent_note = <Label> { draw_text: { color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "Modified date of the newest form" }
                }

                recent_box = <View> {
                    width: Fill, height: Fit,
                    flow: Down,
                    show_bg: true,
                    draw_bg: { color: #3c3f41 }
                    padding: 1.0,
                    recent_list = <PortalList> {
                        width: Fill, height: 50.0,
                        row = <WelcomeRecentRow> {}
                    }
                }

                lbl_recent_help = <Label> {
                    width: Fill, height: Fit,
                    draw_text: { wrap: Word, color: #a8abb3, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                    text: "Click opens the project. Folders that no longer exist leave the list by themselves. × removes a project from the list, never from disk."
                }

                empty_box = <View> {
                    visible: false,
                    width: Fill, height: 300.0,
                    flow: Down,
                    align: { x: 0.5, y: 0.5 },
                    spacing: 12.0,
                    show_bg: true,
                    draw_bg: { color: #232427 }
                    empty_crab = <Image> { width: 120.0, height: 103.0, fit: Stretch }
                    <Label> { draw_text: { color: #c9ccd1, text_style: <THEME_FONT_REGULAR> { font_size: 10.0 } }, text: "No project opened yet." }
                    <View> {
                        width: Fit, height: Fit,
                        flow: Right,
                        spacing: 10.0,
                        btn_empty_new = <WelcomeButton> {
                            draw_bg: { color: #e5582a, color_hover: #f06a3d, color_down: #c94c23, border_color_1: #0000, border_color_2: #0000 }
                            draw_text: { color: #ffffff }
                            text: "New Project…"
                        }
                        btn_empty_open = <WelcomeButton> { text: "Open Folder…" }
                    }
                }
            }
        }
    }
}

#[crawrad_note::note("welcome-welcomerowview")]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct WelcomeRowView {
    pub name: String,
    pub place: String,
    pub forms: String,
    pub age: String,
}

#[crawrad_note::note("welcome-welcomeaction")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WelcomeAction {
    NewProject,
    OpenFolder,
    OpenForm,
    /// The recent row at this index.
    Open(usize),
    /// Take the recent row at this index out of the list.
    Forget(usize),
    /// The `Reopen the last project` box, as it now stands.
    Reopen(bool),
    /// The tab's ×.
    Close,
}

#[crawrad_note::note("welcome-reopen-note")]
pub fn reopen_note(reopen: bool) -> &'static str {
    if reopen {
        "On: the last project opens. Also in Options › IDE › Saving and Recovering."
    } else {
        "Off: this page opens. Also in Options › IDE › Saving and Recovering."
    }
}

#[crawrad_note::note("welcome-list-height")]
pub fn list_height(rows: usize) -> f64 {
    rows.clamp(1, 8) as f64 * 50.0
}

#[crawrad_note::note("welcome-crawradwelcomepage")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradWelcomePage {
    #[deref]
    pub view: View,

    #[rust]
    rows: Vec<WelcomeRowView>,

    #[rust]
    row_areas: Vec<Area>,

    #[rust]
    forget_areas: Vec<Area>,

    #[rust]
    crab_loaded: bool,
}

impl Widget for CrawradWelcomePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let recent = self.view.portal_list(id!(recent_list));
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = recent.borrow_mut_if_eq(&step) {
                list.set_item_range(cx, 0, self.rows.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(row) = self.rows.get(i) else {
                        continue;
                    };
                    let item = list.item(cx, i, live_id!(row));
                    item.label(id!(lbl_name)).set_text(cx, &row.name);
                    item.label(id!(lbl_place)).set_text(cx, &row.place);
                    item.label(id!(lbl_forms)).set_text(cx, &row.forms);
                    item.label(id!(lbl_age)).set_text(cx, &row.age);
                    item.draw_all(cx, scope);
                    if self.row_areas.len() <= i {
                        self.row_areas.resize(i + 1, Area::Empty);
                        self.forget_areas.resize(i + 1, Area::Empty);
                    }
                    self.row_areas[i] = item.area();
                    self.forget_areas[i] = item.button(id!(btn_forget)).area();
                }
            }
        }
        DrawStep::done()
    }
}

impl CrawradWelcomePage {
    #[crawrad_note::note("welcome-crawradwelcomepage-set-rows")]
    pub fn set_rows(&mut self, cx: &mut Cx, rows: &[WelcomeRowView]) {
        self.rows = rows.to_vec();
        self.row_areas = vec![Area::Empty; rows.len()];
        self.forget_areas = vec![Area::Empty; rows.len()];
        let empty = rows.is_empty();
        self.view.view(id!(recent_box)).set_visible(cx, !empty);
        self.view
            .label(id!(lbl_recent_help))
            .set_visible(cx, !empty);
        self.view
            .label(id!(lbl_recent_note))
            .set_visible(cx, !empty);
        self.view.view(id!(empty_box)).set_visible(cx, empty);
        // The crab greets an empty list only, and is decoded once.
        if empty && !self.crab_loaded {
            self.crab_loaded = self
                .view
                .image(id!(empty_crab))
                .load_png_from_data(cx, WELCOME_CRAB_PNG)
                .is_ok();
        }
        // The list is as tall as its rows, up to eight: no grey under the last.
        let height = list_height(rows.len());
        self.view
            .portal_list(id!(recent_list))
            .apply_over(cx, live! { height: (height) });
        self.view.redraw(cx);
    }

    #[crawrad_note::note("welcome-crawradwelcomepage-set-reopen")]
    pub fn set_reopen(&mut self, cx: &mut Cx, reopen: bool) {
        self.view.check_box(id!(chk_reopen)).set_active(cx, reopen);
        self.view
            .label(id!(lbl_reopen_note))
            .set_text(cx, reopen_note(reopen));
        self.view.redraw(cx);
    }

    #[crawrad_note::note("welcome-crawradwelcomepage-drawn-controls")]
    pub fn drawn_controls(&self, cx: &Cx) -> Vec<(String, Rect)> {
        let mut controls: Vec<(String, Rect)> = [
            ("welcome_close", id!(btn_close)),
            ("welcome_new_project", id!(btn_new_project)),
            ("welcome_open_folder", id!(btn_open_folder)),
            ("welcome_open_form", id!(btn_open_form)),
            ("welcome_empty_new_project", id!(btn_empty_new)),
            ("welcome_empty_open_folder", id!(btn_empty_open)),
        ]
        .into_iter()
        .map(|(name, path)| (name.to_string(), self.view.button(path).area().rect(cx)))
        .collect();
        controls.push((
            "welcome_reopen".to_string(),
            self.view.check_box(id!(chk_reopen)).area().rect(cx),
        ));
        for (i, (row, forget)) in self.row_areas.iter().zip(&self.forget_areas).enumerate() {
            controls.push((format!("welcome_recent_{i}"), row.rect(cx)));
            controls.push((format!("welcome_forget_{i}"), forget.rect(cx)));
        }
        controls
    }

    #[crawrad_note::note("welcome-crawradwelcomepage-clicked")]
    pub fn clicked(&self, actions: &Actions) -> Option<WelcomeAction> {
        if self.view.button(id!(btn_close)).clicked(actions) {
            return Some(WelcomeAction::Close);
        }
        if self.view.button(id!(btn_new_project)).clicked(actions)
            || self.view.button(id!(btn_empty_new)).clicked(actions)
        {
            return Some(WelcomeAction::NewProject);
        }
        if self.view.button(id!(btn_open_folder)).clicked(actions)
            || self.view.button(id!(btn_empty_open)).clicked(actions)
        {
            return Some(WelcomeAction::OpenFolder);
        }
        if self.view.button(id!(btn_open_form)).clicked(actions) {
            return Some(WelcomeAction::OpenForm);
        }
        if let Some(on) = self.view.check_box(id!(chk_reopen)).changed(actions) {
            return Some(WelcomeAction::Reopen(on));
        }
        let recent = self.view.portal_list(id!(recent_list));
        for (i, item) in recent.items_with_actions(actions) {
            // The × sits on top of the row's button: it is asked first, so
            // removing a project never opens it too.
            if item.button(id!(btn_forget)).clicked(actions) {
                return Some(WelcomeAction::Forget(i));
            }
            if item.button(id!(btn)).clicked(actions) {
                return Some(WelcomeAction::Open(i));
            }
        }
        None
    }
}

impl CrawradWelcomePageRef {
    #[crawrad_note::note("welcome-crawradwelcomepageref-set-rows")]
    pub fn set_rows(&self, cx: &mut Cx, rows: &[WelcomeRowView]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_rows(cx, rows);
        }
    }

    #[crawrad_note::note("welcome-crawradwelcomepageref-set-reopen")]
    pub fn set_reopen(&self, cx: &mut Cx, reopen: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_reopen(cx, reopen);
        }
    }

    #[crawrad_note::note("welcome-crawradwelcomepageref-drawn-controls")]
    pub fn drawn_controls(&self, cx: &Cx) -> Vec<(String, Rect)> {
        self.borrow()
            .map_or(Vec::new(), |inner| inner.drawn_controls(cx))
    }

    #[crawrad_note::note("welcome-crawradwelcomepageref-clicked")]
    pub fn clicked(&self, actions: &Actions) -> Option<WelcomeAction> {
        self.borrow().and_then(|inner| inner.clicked(actions))
    }
}

#[cfg(test)]
#[path = "test/welcome/test.rs"]
mod tests;
