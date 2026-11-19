//! Size preview widget displaying multiple viewport frames side-by-side.

use crawrad_ir::{Document, NodeId};
use makepad_widgets::*;

use crate::canvas::DesignCanvas;
use crawrad_runtime::Runtime;

live_design! {
    link crawrad;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use crate::canvas::*;
    use crate::borderless::*;

    SizeRowSlot = <RoundedView> {
        visible: false,
        width: Fill, height: 44.0,
        flow: Overlay,
        show_bg: true,
        draw_bg: {
            color: #0000,
            // 4 px corner: Makepad's RoundedView draws twice the border_radius
            border_radius: 2.0
        }

        row_content = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 10.0, right: 10.0 },
            spacing: 10.0,

            tick_box = <RoundedView> {
                width: 16.0, height: 16.0,
                flow: Overlay,
                align: { x: 0.5, y: 0.5 },
                show_bg: true,
                draw_bg: {
                    color: #0000,
                    border_color: #4a4f54,
                    border_size: 1.0,
                    // 2 px corner: Makepad's RoundedView draws twice the border_radius
                    border_radius: 1.0
                }
                lbl_check = <Label> {
                    draw_text: {
                        color: #1b1d1f,
                        text_style: <THEME_FONT_BOLD> { font_size: 9.0 }
                    }
                    text: ""
                }
            }

            pic_box = <View> {
                width: 34.0, height: 34.0,
                align: { x: 0.5, y: 0.5 },
                device_pic = <RoundedView> {
                    width: 30.0, height: 30.0,
                    show_bg: true,
                    draw_bg: {
                        color: #0000,
                        border_color: #a9abae,
                        border_size: 1.5,
                        // Picture corner: Makepad's RoundedView draws twice the border_radius
                        border_radius: 1.0
                    }
                }
            }

            info_col = <View> {
                width: Fill, height: Fit,
                flow: Down, spacing: 1.0,
                align: { y: 0.5 },
                lbl_name = <Label> {
                    width: Fit, height: Fit, padding: 0.0,
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                    }
                    text: ""
                }
                lbl_dims = <Label> {
                    width: Fit, height: Fit, padding: 0.0,
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_CODE> { font_size: 8.0 }
                    }
                    text: ""
                }
            }

            lbl_kind = <Label> {
                width: Fit, height: Fit,
                draw_text: {
                    color: #6f7377,
                    text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
                }
                text: "preset"
            }
        }

        btn = <Button> {
            width: Fill, height: Fill,
            draw_bg: {
                color: #0000,
                color_hover: #ffffff08,
                color_down: #ffffff14,
                border_color_1: #0000,
                border_color_2: #0000,
            }
            text: ""
        }
    }

    SizeFrameSlot = <View> {
        visible: false,
        width: Fit, height: Fill,
        flow: Down,
        align: { x: 0.0, y: 0.0 }

        slot_header = <View> {
            width: Fit, height: 24.0,
            flow: Right, align: { y: 0.5 }
            spacing: 8.0,
            lbl_name = <Label> {
                draw_text: {
                    color: #e6e7e8,
                    text_style: <THEME_FONT_BOLD> { font_size: 9.5 }
                }
                text: ""
            }
            lbl_dims = <Label> {
                draw_text: {
                    color: #8d9093,
                    text_style: <THEME_FONT_CODE> { font_size: 9.0 }
                }
                text: ""
            }
            lbl_zoom = <Label> {
                draw_text: {
                    color: #6f7377,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                }
                text: ""
            }
        }

        frame_box = <RoundedView> {
            width: Fit, height: Fit,
            show_bg: true,
            draw_bg: {
                color: #252538,
                border_color: #3a3e42,
                border_size: 1.0,
                // 6 px card corner: Makepad's RoundedView draws twice the border_radius
                border_radius: 3.0
            }
            canvas = <DesignCanvas> {
                width: 390.0, height: 844.0
            }
        }

        note_box = <View> {
            width: Fit, height: Fit,
            margin: { top: 6.0 },
            lbl_note = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #8d9093,
                    text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                }
                text: ""
            }
        }
    }

    pub SizePreviewView = {{SizePreviewView}} {
        width: Fill, height: Fill,
        flow: Overlay,

        main_layout = <View> {
            width: Fill, height: Fill,
            flow: Down,

            toolbar = <CrawradTopRounded> {
                width: Fill, height: 38.0,
                flow: Right, align: { y: 0.5 }
                padding: { left: 14.0, right: 14.0 },
                spacing: 8.0,
                draw_bg: { color: #161819 }

                lbl_sizes = <Label> {
                    draw_text: {
                        color: #6f7377,
                        text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                    }
                    text: "Sizes"
                }

                combo_box = <View> {
                width: 300.0, height: 28.0,
                flow: Overlay, align: { x: 1.0, y: 0.5 },
                sizes_combo = <Button> {
                    width: 300.0, height: 28.0,
                    align: { x: 0.0, y: 0.5 },
                    padding: { left: 10.0, right: 10.0 },
                    draw_bg: {
                        border_size: 1.0,
                        // 4 px corner: Makepad's RoundedView draws twice the border_radius
                        border_radius: 2.0,
                        color: #2b2d30,
                        color_hover: #35383d,
                        color_down: #3e4247,
                        border_color_1: #5a3a20,
                        border_color_2: #5a3a20,
                    }
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: ""
                }
                // The fonts have no ▾: the chevron is drawn, like the ComboBox's caret.
                combo_chevron = <View> {
                    width: 10.0, height: 6.0,
                    margin: { right: 12.0 },
                    show_bg: true,
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.move_to(1.0, 1.0);
                            sdf.line_to(self.rect_size.x * 0.5, self.rect_size.y - 1.0);
                            sdf.line_to(self.rect_size.x - 1.0, 1.0);
                            sdf.stroke(#a9abae, 1.5);
                            return sdf.result;
                        }
                    }
                }
                }

                <View> { width: Fill, height: Fit }

                lbl_readonly_view = <View> {
                    width: Fit, height: Fit,
                    lbl_readonly = <Label> {
                        draw_text: {
                            color: #6f7377,
                            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                        }
                        text: "Read-only · follows every edit"
                    }
                }

                btn_fit = <Button> {
                    width: 44.0, height: 26.0,
                    align: { x: 0.5, y: 0.5 }
                    draw_bg: {
                        border_size: 1.0,
                        // 4 px corner: Makepad draws twice the border_radius
                        border_radius: 2.0
                        color: #2b2d30,
                        color_hover: #35383d,
                        color_down: #3e4247,
                        border_color_1: #4a4f54,
                        border_color_2: #4a4f54,
                    }
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_BOLD> { font_size: 9.0 }
                    }
                    text: "Fit"
                }
            }

            strip_scroll = <ScrollXView> {
                width: Fill, height: Fill,
                padding: { left: 24.0, right: 24.0, top: 16.0, bottom: 16.0 },
                show_bg: true,
                draw_bg: {
                    color: #1d1f20,
                    fn pixel(self) -> vec4 {
                        let p = self.pos * self.rect_size;
                        let m = mod(p, vec2(8.0, 8.0));
                        let d = length(m - vec2(4.0, 4.0));
                        let dot = clamp(1.0 - (d - 0.5), 0.0, 1.0);
                        let bg = vec4(0.114, 0.122, 0.125, 1.0);
                        let grid = vec4(0.478, 0.498, 0.604, 0.35);
                        return mix(bg, grid, dot);
                    }
                }

                strip_row = <View> {
                    width: Fit, height: Fill,
                    flow: Right,
                    spacing: 28.0,

                    frame_0 = <SizeFrameSlot> {}
                    frame_1 = <SizeFrameSlot> {}
                    frame_2 = <SizeFrameSlot> {}
                    frame_3 = <SizeFrameSlot> {}
                    frame_4 = <SizeFrameSlot> {}
                    frame_5 = <SizeFrameSlot> {}
                }
            }
        }

        popup_backdrop = <View> {
            visible: false,
            width: Fill, height: Fill,
            flow: Overlay,

            btn_dismiss_popup = <Button> {
                width: Fill, height: Fill,
                draw_bg: {
                    color: #0000,
                    color_hover: #0000,
                    color_down: #0000,
                    border_color_1: #0000,
                    border_color_2: #0000,
                }
                text: ""
            }

            popup_card = <RoundedView> {
                width: 340.0, height: Fit,
                flow: Down,
                margin: { left: 54.0, top: 40.0 },
                show_bg: true,
                draw_bg: {
                    color: #2a2d30,
                    border_color: #4a4f54,
                    border_size: 1.0,
                    // 6 px card corner: Makepad's RoundedView draws twice the border_radius
                    border_radius: 3.0
                }

                size_row_0 = <SizeRowSlot> {}
                size_row_1 = <SizeRowSlot> {}
                size_row_2 = <SizeRowSlot> {}
                size_row_3 = <SizeRowSlot> {}
                size_row_4 = <SizeRowSlot> {}
                size_row_5 = <SizeRowSlot> {}
                size_row_6 = <SizeRowSlot> {}
                size_row_7 = <SizeRowSlot> {}
                size_row_8 = <SizeRowSlot> {}
                size_row_9 = <SizeRowSlot> {}
                size_row_10 = <SizeRowSlot> {}
                size_row_11 = <SizeRowSlot> {}

                sep = <View> {
                    width: Fill, height: 1.0,
                    show_bg: true,
                    draw_bg: { color: #3a3e42 }
                }

                btn_add_custom_row = <Button> {
                    width: Fill, height: 36.0,
                    padding: { left: 14.0 },
                    align: { x: 0.0, y: 0.5 },
                    draw_bg: {
                        color: #0000,
                        color_hover: #ffffff08,
                        color_down: #ffffff14,
                        border_color_1: #0000,
                        border_color_2: #0000,
                    }
                    draw_text: {
                        color: #e8914a,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                    }
                    text: "+ Custom size…"
                }
            }
        }

        custom_card_backdrop = <View> {
            visible: false,
            width: Fill, height: Fill,
            flow: Overlay,
            show_bg: true,
            draw_bg: { color: #0000004d }

            custom_card = <RoundedView> {
                width: 280.0, height: Fit,
                flow: Down,
                padding: 14.0,
                spacing: 8.0,
                margin: { left: 40.0, top: 44.0 },
                show_bg: true,
                draw_bg: {
                    color: #252538,
                    border_color: #4a4f54,
                    border_size: 1.0,
                    // 6 px card corner: Makepad's RoundedView draws twice the border_radius
                    border_radius: 3.0
                }

                card_title = <Label> {
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_BOLD> { font_size: 10.0 }
                    }
                    text: "Add Custom Size"
                }

                <View> { width: Fill, height: 2.0 }

                <Label> {
                    draw_text: { color: #8d9093, text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }
                    text: "Name"
                }
                input_name = <TextInput> {
                    width: Fill, height: 26.0,
                    empty_text: "e.g. Small Mobile"
                    draw_text: { color: #e6e7e8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                }

                dims_row = <View> {
                    width: Fill, height: Fit,
                    flow: Right, spacing: 8.0,
                    w_col = <View> {
                        width: Fill, height: Fit, flow: Down, spacing: 4.0,
                        <Label> {
                            draw_text: { color: #8d9093, text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }
                            text: "Width"
                        }
                        input_width = <TextInput> {
                            width: Fill, height: 26.0,
                            empty_text: "390"
                            draw_text: { color: #e6e7e8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                        }
                    }
                    h_col = <View> {
                        width: Fill, height: Fit, flow: Down, spacing: 4.0,
                        <Label> {
                            draw_text: { color: #8d9093, text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }
                            text: "Height"
                        }
                        input_height = <TextInput> {
                            width: Fill, height: 26.0,
                            empty_text: "844"
                            draw_text: { color: #e6e7e8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                        }
                    }
                }

                error_box = <View> {
                    visible: false,
                    width: Fill, height: Fit,
                    margin: { top: 2.0 },
                    lbl_error = <Label> {
                        draw_text: { color: #f0a39c, text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }
                        text: ""
                    }
                }

                card_buttons = <View> {
                    width: Fill, height: 28.0,
                    flow: Right, align: { y: 0.5 }
                    spacing: 8.0,
                    margin: { top: 6.0 },

                    <View> { width: Fill, height: Fit }

                    btn_cancel = <Button> {
                        width: 70.0, height: 26.0,
                        draw_text: { color: #a9abae, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                        text: "Cancel"
                    }
                    btn_add = <Button> {
                        width: 70.0, height: 26.0,
                        draw_bg: {
                            color: #f3b27a, color_hover: #f7c396, color_down: #e09f65,
                            border_color_1: #0000, border_color_2: #0000,
                        }
                        draw_text: { color: #1b1d1f, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }
                        text: "Add"
                    }
                }
            }
        }
    }
}

/// User actions emitted from the sizes preview view.
#[derive(Clone, Debug, PartialEq, Eq, DefaultNone)]
pub enum SizePreviewAction {
    /// No gesture (the widget-action default).
    None,
    Toggle(usize),
    ToggleCustomCard,
    CloseCustomCard,
    AddCustom {
        name: String,
        width: u32,
        height: u32,
    },
    Fit,
}

/// A size item in the combo dropdown and toolbar summary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeItem {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub shown: bool,
    pub custom: bool,
}

/// Formats the summary text for the sizes combo: "{n} shown — {a} · {b} · {c}" or "None shown".
pub fn combo_label(items: &[SizeItem]) -> String {
    let shown: Vec<&str> = items
        .iter()
        .filter(|it| it.shown)
        .map(|it| it.name.as_str())
        .collect();
    let Some((first, rest)) = shown.split_first() else {
        return "None shown".to_string();
    };
    // The combo is 300 px wide: names go in while they fit, the rest become "+N".
    let mut names = first.to_string();
    let mut listed = 1;
    for name in rest {
        if names.chars().count() + name.chars().count() + 3 > COMBO_NAMES_MAX {
            break;
        }
        names.push_str(" · ");
        names.push_str(name);
        listed += 1;
    }
    let more = shown.len() - listed;
    let more = if more > 0 {
        format!(" +{more}")
    } else {
        String::new()
    };
    format!("{} shown — {names}{more}", shown.len())
}

/// How many characters of names fit in the combo's 300 px next to "N shown — ".
const COMBO_NAMES_MAX: usize = 26;

/// Calculates the (width, height, border_radius) for a device picture in the size combo.
///
/// The longer side is 30 px, the shorter side is scaled proportionally (minimum 8 px).
/// The border_radius is already halved because Makepad's RoundedView draws twice the radius.
pub fn device_picture(width: u32, height: u32) -> (f64, f64, f64) {
    let (wf, hf) = (width as f64, height as f64);
    let (w, h) = if wf >= hf {
        let h_calc = if wf > 0.0 {
            (30.0 * hf / wf).max(8.0)
        } else {
            8.0
        };
        (30.0, h_calc)
    } else {
        let w_calc = if hf > 0.0 {
            (30.0 * wf / hf).max(8.0)
        } else {
            8.0
        };
        (w_calc, 30.0)
    };
    let border_radius = if width > height && width >= 1024 {
        // Desktop corner 1 px halved: Makepad's RoundedView draws twice the border_radius
        0.5
    } else {
        // Phone/tablet corner min(w, h)/5 px halved: Makepad's RoundedView draws twice the border_radius
        (w.min(h) / 5.0) / 2.0
    };
    (w, h, border_radius)
}

/// One frame of the strip, already laid out by the designer (the widget decides nothing).
pub struct SizeFrame<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
    pub doc: &'a Document,
    pub note: &'a str,
    pub warning: bool,
    pub overflow: Vec<NodeId>,
}

#[derive(Live, LiveHook, Widget)]
pub struct SizePreviewView {
    #[deref]
    pub view: View,
    /// The strip height the frames were last scaled to.
    #[rust]
    scaled_h: f64,
    /// The strip came out at another height: ask for a fit on the next frame.
    #[rust]
    refit: bool,
}

impl Widget for SizePreviewView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::NextFrame(_) = event
            && std::mem::take(&mut self.refit)
        {
            cx.widget_action(self.widget_uid(), &scope.path, SizePreviewAction::Fit);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let step = self.view.draw_walk(cx, scope, walk);
        // The frames are scaled to the strip's height, known only once it is drawn.
        let h = Self::frame_height(self.view.view(id!(strip_scroll)).area().rect(cx).size.y);
        if (h - self.scaled_h).abs() > 1.0 && !self.refit {
            self.refit = true;
            cx.new_next_frame();
        }
        step
    }
}

impl SizePreviewView {
    /// Formats the summary text for the sizes combo: "{n} shown — {a} · {b} · {c}" or "None shown".
    pub fn combo_label(items: &[SizeItem]) -> String {
        combo_label(items)
    }

    /// Toggles the visibility of the sizes combo popup card.
    pub fn toggle_popup(&mut self, cx: &mut Cx) {
        let is_visible = self.view.view(id!(popup_backdrop)).visible();
        self.set_popup_visible(cx, !is_visible);
    }

    /// Sets the visibility of the sizes combo popup card.
    pub fn set_popup_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.view.view(id!(popup_backdrop)).set_visible(cx, visible);
        self.view.redraw(cx);
    }

    /// Toggles the visibility of the custom size addition card.
    pub fn toggle_custom_card(&mut self, cx: &mut Cx) {
        let is_visible = self.view.view(id!(custom_card_backdrop)).visible();
        self.set_custom_card_visible(cx, !is_visible);
    }

    /// Sets the visibility of the custom size addition card.
    pub fn set_custom_card_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.view
            .view(id!(custom_card_backdrop))
            .set_visible(cx, visible);
        if visible {
            self.clear_custom_error(cx);
            self.view.text_input(id!(input_name)).set_text(cx, "");
            self.view.text_input(id!(input_width)).set_text(cx, "");
            self.view.text_input(id!(input_height)).set_text(cx, "");
        }
        self.view.redraw(cx);
    }

    /// Displays an error message inside the custom card.
    pub fn set_custom_error(&mut self, cx: &mut Cx, error: &str) {
        self.view.view(id!(error_box)).set_visible(cx, true);
        self.view.label(id!(lbl_error)).set_text(cx, error);
        self.view.redraw(cx);
    }

    /// Clears any error message in the custom card.
    pub fn clear_custom_error(&mut self, cx: &mut Cx) {
        self.view.view(id!(error_box)).set_visible(cx, false);
        self.view.label(id!(lbl_error)).set_text(cx, "");
        self.view.redraw(cx);
    }

    /// The height a frame gets in a strip of `strip_h` px: room for the header and the note.
    pub fn frame_height(strip_h: f64) -> f64 {
        (strip_h - 150.0).max(120.0)
    }

    /// Inspects UI actions and returns any triggered `SizePreviewAction`.
    pub fn action(&mut self, cx: &mut Cx, actions: &Actions) -> Option<SizePreviewAction> {
        if let SizePreviewAction::Fit = actions.find_widget_action(self.widget_uid()).cast() {
            return Some(SizePreviewAction::Fit);
        }
        if self.view.button(id!(btn_fit)).clicked(actions) {
            return Some(SizePreviewAction::Fit);
        }

        if self.view.button(id!(sizes_combo)).clicked(actions) {
            self.toggle_popup(cx);
            return None;
        }

        if self.view.button(id!(btn_dismiss_popup)).clicked(actions) {
            self.set_popup_visible(cx, false);
            return None;
        }

        let row_ids = [
            id!(size_row_0),
            id!(size_row_1),
            id!(size_row_2),
            id!(size_row_3),
            id!(size_row_4),
            id!(size_row_5),
            id!(size_row_6),
            id!(size_row_7),
            id!(size_row_8),
            id!(size_row_9),
            id!(size_row_10),
            id!(size_row_11),
        ];

        let popup_card = self.view.view(id!(popup_card));
        for (i, &row_id) in row_ids.iter().enumerate() {
            if popup_card.view(row_id).button(id!(btn)).clicked(actions) {
                return Some(SizePreviewAction::Toggle(i));
            }
        }

        if self.view.button(id!(btn_add_custom_row)).clicked(actions) {
            self.set_popup_visible(cx, false);
            return Some(SizePreviewAction::ToggleCustomCard);
        }

        if self.view.button(id!(btn_cancel)).clicked(actions) {
            return Some(SizePreviewAction::CloseCustomCard);
        }

        if self.view.button(id!(btn_add)).clicked(actions) {
            let name = self.view.text_input(id!(input_name)).text();
            let w_str = self.view.text_input(id!(input_width)).text();
            let h_str = self.view.text_input(id!(input_height)).text();

            let width = match w_str.trim().parse::<u32>() {
                Ok(w) if w > 0 => w,
                _ => {
                    self.set_custom_error(cx, "Width must be a positive number");
                    return None;
                }
            };
            let height = match h_str.trim().parse::<u32>() {
                Ok(h) if h > 0 => h,
                _ => {
                    self.set_custom_error(cx, "Height must be a positive number");
                    return None;
                }
            };

            return Some(SizePreviewAction::AddCustom {
                name: name.trim().to_string(),
                width,
                height,
            });
        }

        None
    }

    /// Updates the sizes combo, row slots and all frame slots from the model.
    pub fn set_view(
        &mut self,
        cx: &mut Cx,
        items: &[SizeItem],
        frames: &[SizeFrame],
        declared_drawings: &crawrad_ir::drawing::DeclaredLooks,
    ) {
        // 1. Update sizes combo button text and width
        let label = combo_label(items);
        let combo_w = (label.chars().count() as f64 * 7.5 + 40.0).max(300.0);
        let combo_btn = self.view.button(id!(sizes_combo));
        combo_btn.apply_over(cx, live! { width: (combo_w) });
        combo_btn.set_text(cx, &format!("{label}   ▾"));

        // 2. Update popup row slots
        let row_ids = [
            id!(size_row_0),
            id!(size_row_1),
            id!(size_row_2),
            id!(size_row_3),
            id!(size_row_4),
            id!(size_row_5),
            id!(size_row_6),
            id!(size_row_7),
            id!(size_row_8),
            id!(size_row_9),
            id!(size_row_10),
            id!(size_row_11),
        ];

        let popup_card = self.view.view(id!(popup_card));
        for (i, &row_id) in row_ids.iter().enumerate() {
            let row_view = popup_card.view(row_id);
            if let Some(item) = items.get(i) {
                row_view.set_visible(cx, true);
                if item.shown {
                    // Ticked: orange fill with dark check, row bg #31291f
                    row_view.apply_over(
                        cx,
                        live! {
                            draw_bg: { color: (vec4(0.192, 0.161, 0.122, 1.0)) }
                        },
                    );
                    let tick_box = row_view.view(id!(tick_box));
                    tick_box.apply_over(
                        cx,
                        live! {
                            draw_bg: {
                                color: (vec4(0.910, 0.569, 0.290, 1.0)),
                                border_color: (vec4(0.910, 0.569, 0.290, 1.0)),
                            }
                        },
                    );
                    tick_box.label(id!(lbl_check)).set_text(cx, "✓");
                } else {
                    // Unticked: transparent with #4a4f54 border, row bg transparent
                    row_view.apply_over(
                        cx,
                        live! {
                            draw_bg: { color: (vec4(0.0, 0.0, 0.0, 0.0)) }
                        },
                    );
                    let tick_box = row_view.view(id!(tick_box));
                    tick_box.apply_over(
                        cx,
                        live! {
                            draw_bg: {
                                color: (vec4(0.0, 0.0, 0.0, 0.0)),
                                border_color: (vec4(0.290, 0.310, 0.329, 1.0)),
                            }
                        },
                    );
                    tick_box.label(id!(lbl_check)).set_text(cx, "");
                }

                let (pic_w, pic_h, pic_radius) = device_picture(item.width, item.height);
                row_view.view(id!(device_pic)).apply_over(
                    cx,
                    live! {
                        width: (pic_w),
                        height: (pic_h),
                        draw_bg: {
                            border_radius: (pic_radius),
                        }
                    },
                );

                row_view.label(id!(lbl_name)).set_text(cx, &item.name);
                row_view
                    .label(id!(lbl_dims))
                    .set_text(cx, &format!("{} × {}", item.width, item.height));
                row_view
                    .label(id!(lbl_kind))
                    .set_text(cx, if item.custom { "custom" } else { "preset" });
            } else {
                row_view.set_visible(cx, false);
            }
        }

        // 3. Measure available height for scaling frames
        let strip_h = self.view.view(id!(strip_scroll)).area().rect(cx).size.y;
        let avail_h = Self::frame_height(strip_h);
        self.scaled_h = avail_h;

        // 4. Update the 6 fixed slots
        let slot_ids = [
            id!(frame_0),
            id!(frame_1),
            id!(frame_2),
            id!(frame_3),
            id!(frame_4),
            id!(frame_5),
        ];

        for (i, &slot_id) in slot_ids.iter().enumerate() {
            let slot = self.view.view(id!(strip_row)).view(slot_id);
            if let Some(preset) = frames.get(i) {
                slot.set_visible(cx, true);

                let scale = (avail_h / preset.height as f64).min(1.0);
                let scaled_w = (preset.width as f64 * scale).round();
                let scaled_h = (preset.height as f64 * scale).round();
                let scale_percent = (scale * 100.0).round() as u32;

                slot.label(id!(lbl_name)).set_text(cx, preset.name);
                slot.label(id!(lbl_dims))
                    .set_text(cx, &format!("{}×{}", preset.width, preset.height));
                slot.label(id!(lbl_zoom))
                    .set_text(cx, &format!("{scale_percent}%"));

                slot.view(id!(frame_box)).apply_over(
                    cx,
                    live! {
                        width: (scaled_w),
                        height: (scaled_h),
                    },
                );

                slot.view(id!(note_box)).apply_over(
                    cx,
                    live! {
                        width: (scaled_w),
                    },
                );

                let canvas_widget = slot.widget(id!(canvas));
                canvas_widget.apply_over(
                    cx,
                    live! {
                        width: (scaled_w),
                        height: (scaled_h),
                    },
                );

                {
                    let (framed, note_text, is_warning) = (preset.doc, preset.note, preset.warning);
                    if let Some(mut c) = canvas_widget.borrow_mut::<DesignCanvas>() {
                        c.runtime_mut().com_desenhos(declared_drawings.clone());
                        let _ = c.runtime_mut().open(framed);
                        c.runtime_mut().set_grid(0.0);
                        c.runtime_mut().set_zoom(scale as f32);
                        c.set_read_only(true);
                        c.set_overflow(&preset.overflow);
                    }

                    let lbl_note = slot.label(id!(lbl_note));
                    lbl_note.set_text(cx, note_text);
                    if is_warning {
                        // Red #f0a39c warning
                        lbl_note.apply_over(
                            cx,
                            live! {
                                draw_text: { color: (vec4(0.941, 0.639, 0.612, 1.0)) }
                            },
                        );
                    } else {
                        // Muted #8d9093 Fits
                        lbl_note.apply_over(
                            cx,
                            live! {
                                draw_text: { color: (vec4(0.553, 0.565, 0.576, 1.0)) }
                            },
                        );
                    }
                }
            } else {
                slot.set_visible(cx, false);
            }
        }

        self.view.redraw(cx);
    }
}

impl SizePreviewViewRef {
    pub fn action(&self, cx: &mut Cx, actions: &Actions) -> Option<SizePreviewAction> {
        self.borrow_mut()
            .and_then(|mut inner| inner.action(cx, actions))
    }

    pub fn toggle_popup(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle_popup(cx);
        }
    }

    pub fn set_popup_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_popup_visible(cx, visible);
        }
    }

    pub fn toggle_custom_card(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle_custom_card(cx);
        }
    }

    pub fn set_custom_card_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_custom_card_visible(cx, visible);
        }
    }

    pub fn set_custom_error(&self, cx: &mut Cx, error: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_custom_error(cx, error);
        }
    }

    pub fn clear_custom_error(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_custom_error(cx);
        }
    }

    pub fn set_view(
        &self,
        cx: &mut Cx,
        items: &[SizeItem],
        frames: &[SizeFrame],
        declared_drawings: &crawrad_ir::drawing::DeclaredLooks,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_view(cx, items, frames, declared_drawings);
        }
    }
}

#[cfg(test)]
#[path = "test/size_preview_view/test.rs"]
mod tests;
