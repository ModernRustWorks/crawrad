//! Native modal message dialog implementation for Makepad projects.
//!
//! Provides Makepad DSL and synchronization logic for native dark message dialogs
//! with SDF rounded boxes, Rust Orange buttons, and Enter/Esc keyboard cycle.

use crawrad_note::note;
use crawrad_runtime::dialogs::{
    DialogButtons, ModalResult, active_dialog, is_dialog_open, resolve_dialog,
};
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub CrawradMessageDialog = <View> {
        visible: false,
        width: Fill, height: Fill,
        flow: Overlay,
        align: { x: 0.5, y: 0.5 },
        show_bg: true,
        draw_bg: { color: vec4(0.0, 0.0, 0.0, 0.6) }

        dlg_box = <View> {
            width: 440.0, height: Fit,
            flow: Down,
            padding: 20.0,
            spacing: 16.0,
            show_bg: true,
            draw_bg: {
                color: #1e1e1e
                instance radius: 4.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                    sdf.fill_keep(self.color);
                    sdf.stroke(#333333, 1.0);
                    return sdf.result;
                }
            }

            dlg_header = <View> {
                width: Fill, height: Fit,
                flow: Right,
                align: { y: 0.5 },
                spacing: 10.0,

                dlg_icon = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #de5a2b,
                        text_style: { font_size: 18.0 }
                    }
                    text: "ℹ"
                }
                dlg_title = <Label> {
                    width: Fill, height: Fit,
                    draw_text: {
                        color: #ffffff,
                        text_style: { font_size: 13.0 }
                    }
                    text: "Information"
                }
            }

            dlg_body = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #cdd6f4,
                    text_style: { font_size: 11.0 }
                }
                text: ""
            }

            dlg_footer = <View> {
                width: Fill, height: 32.0,
                flow: Right,
                align: { x: 1.0, y: 0.5 },
                spacing: 8.0,

                dlg_btn_cancel = <Button> {
                    visible: false,
                    width: 80.0, height: 26.0,
                    draw_bg: {
                        instance radius: 2.0
                    }
                    text: "Cancel"
                }
                dlg_btn_no = <Button> {
                    visible: false,
                    width: 75.0, height: 26.0,
                    draw_bg: {
                        instance radius: 2.0
                    }
                    text: "No"
                }
                dlg_btn_yes = <Button> {
                    visible: false,
                    width: 75.0, height: 26.0,
                    draw_bg: {
                        color: #de5a2b
                        instance radius: 2.0
                    }
                    text: "Yes"
                }
                dlg_btn_ok = <Button> {
                    visible: true,
                    width: 80.0, height: 26.0,
                    draw_bg: {
                        color: #de5a2b
                        instance radius: 2.0
                    }
                    text: "OK"
                }
            }
        }
    }
}

#[note("dialogs-sync-message-dialog")]
pub fn sync_message_dialog(cx: &mut Cx, view: &WidgetRef) {
    let open = is_dialog_open();
    view.set_visible(cx, open);
    if !open {
        return;
    }

    if let Some(req) = active_dialog() {
        view.label(id!(dlg_title)).set_text(cx, &req.title);
        view.label(id!(dlg_body)).set_text(cx, &req.message);
        view.label(id!(dlg_icon))
            .set_text(cx, req.kind.icon_symbol());

        // Set icon color
        let (r, g, b) = req.kind.icon_color_rgb();
        let color = vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);
        view.label(id!(dlg_icon))
            .apply_over(cx, live! { draw_text: { color: (color) } });

        // Update button visibilities
        let show_ok = req.buttons == DialogButtons::Ok || req.buttons == DialogButtons::OkCancel;
        let show_cancel =
            req.buttons == DialogButtons::OkCancel || req.buttons == DialogButtons::YesNoCancel;
        let show_yes =
            req.buttons == DialogButtons::YesNo || req.buttons == DialogButtons::YesNoCancel;
        let show_no =
            req.buttons == DialogButtons::YesNo || req.buttons == DialogButtons::YesNoCancel;

        view.button(id!(dlg_btn_ok)).set_visible(cx, show_ok);
        view.button(id!(dlg_btn_cancel))
            .set_visible(cx, show_cancel);
        view.button(id!(dlg_btn_yes)).set_visible(cx, show_yes);
        view.button(id!(dlg_btn_no)).set_visible(cx, show_no);
    }
}

#[note("dialogs-handle-dialog-actions")]
pub fn handle_dialog_actions(cx: &mut Cx, view: &WidgetRef, actions: &Actions) -> bool {
    if !is_dialog_open() {
        return false;
    }

    if view.button(id!(dlg_btn_ok)).clicked(actions) {
        resolve_dialog(ModalResult::Ok);
        sync_message_dialog(cx, view);
        return true;
    }
    if view.button(id!(dlg_btn_yes)).clicked(actions) {
        resolve_dialog(ModalResult::Yes);
        sync_message_dialog(cx, view);
        return true;
    }
    if view.button(id!(dlg_btn_no)).clicked(actions) {
        resolve_dialog(ModalResult::No);
        sync_message_dialog(cx, view);
        return true;
    }
    if view.button(id!(dlg_btn_cancel)).clicked(actions) {
        resolve_dialog(ModalResult::Cancel);
        sync_message_dialog(cx, view);
        return true;
    }

    false
}

#[note("dialogs-handle-dialog-key")]
pub fn handle_dialog_key(cx: &mut Cx, view: &WidgetRef, event: &Event) -> bool {
    if !is_dialog_open() {
        return false;
    }

    if let Event::KeyDown(ke) = event
        && let Some(req) = active_dialog()
    {
        if ke.key_code == KeyCode::ReturnKey {
            resolve_dialog(req.buttons.default_result());
            sync_message_dialog(cx, view);
            return true;
        } else if ke.key_code == KeyCode::Escape {
            resolve_dialog(req.buttons.cancel_result());
            sync_message_dialog(cx, view);
            return true;
        }
    }

    false
}
