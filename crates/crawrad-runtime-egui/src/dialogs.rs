//! Modal message dialog rendering for egui projects.

use crawrad_note::note;
use crawrad_runtime::dialogs::{active_dialog, resolve_dialog};
use egui::{Align2, Color32, CornerRadius, Id, Key, LayerId, Margin, Order, Stroke, Vec2};

#[note("dialogs-render-message-dialog")]
pub fn render_message_dialog(ctx: &egui::Context) -> bool {
    let Some(req) = active_dialog() else {
        return false;
    };

    let screen_rect = ctx.viewport_rect();
    // 1. Dim the backdrop to focus user attention on the dialog
    let painter = ctx.layer_painter(LayerId::new(
        Order::Middle,
        Id::new("crawrad_modal_backdrop"),
    ));
    painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(150));

    // 2. Keyboard shortcuts: Enter for default button, Escape for cancel
    let mut chosen_result = None;
    if ctx.input(|i| i.key_pressed(Key::Enter)) {
        chosen_result = Some(req.buttons.default_result());
    } else if ctx.input(|i| i.key_pressed(Key::Escape)) {
        chosen_result = Some(req.buttons.cancel_result());
    }

    // 3. Centered modal window
    egui::Window::new(&req.title)
        .id(Id::new("crawrad_native_message_dialog"))
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .order(Order::Foreground)
        .frame(
            egui::Frame::window(&ctx.style_of(ctx.theme()))
                .fill(Color32::from_rgb(0x1e, 0x1e, 0x1e))
                .stroke(Stroke::new(1.0, Color32::from_rgb(0x3a, 0x3a, 0x3a)))
                .corner_radius(CornerRadius::same(4))
                .inner_margin(Margin::same(16)),
        )
        .show(ctx, |ui| {
            ui.set_min_width(320.0);
            ui.set_max_width(480.0);

            // Row with icon + message
            ui.horizontal(|ui| {
                let (r, g, b) = req.kind.icon_color_rgb();
                ui.label(
                    egui::RichText::new(req.kind.icon_symbol())
                        .color(Color32::from_rgb(r, g, b))
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(&req.message)
                        .color(Color32::from_rgb(0xcd, 0xd6, 0xf4))
                        .size(13.0),
                );
            });

            ui.add_space(16.0);

            // Row with buttons aligned to the right
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let buttons = req.buttons.button_labels();
                    // Iterate in reverse for right_to_left
                    for &(label, result) in buttons.iter().rev() {
                        let is_primary = result == req.buttons.default_result();
                        let btn = egui::Button::new(
                            egui::RichText::new(label).color(Color32::WHITE).size(12.5),
                        )
                        .fill(if is_primary {
                            Color32::from_rgb(0xde, 0x5a, 0x2b) // Rust Orange
                        } else {
                            Color32::from_rgb(0x3e, 0x3e, 0x42)
                        })
                        .min_size(Vec2::new(72.0, 26.0))
                        .corner_radius(CornerRadius::same(2));

                        if ui.add(btn).clicked() {
                            chosen_result = Some(result);
                        }
                        ui.add_space(6.0);
                    }
                });
            });
        });

    if let Some(res) = chosen_result {
        resolve_dialog(res);
        ctx.request_repaint();
    }

    true
}
