//! Interactive preview example of the form in Makepad.
//!
//! To run:
//! ```powershell
//! cargo run -p crawrad-runtime-makepad --example preview
//! ```

pub use makepad_widgets;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;

    App = {{App}} {
        ui: <Root> {
            janela = <Window> {
                window: {inner_size: vec2(640, 480)},
                pass: {clear_color: #1e1e2e},
            body = <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                padding: 24,
                spacing: 16,
                align: {x: 0.5, y: 0.0},

                <Label> {
                    draw_text: {
                        color: #cdd6f4,
                        text_style: {font_size: 22.0}
                    },
                    text: "CrawRAD — Makepad Runtime Engine Preview"
                }

                <Label> {
                    draw_text: {
                        color: #a6adc8,
                        text_style: {font_size: 13.0}
                    },
                    text: "Native test form rendered directly by Makepad with GPU acceleration."
                }

                form_card = <View> {
                    width: 500,
                    height: 320,
                    flow: Down,
                    padding: 20,
                    spacing: 14,
                    align: {x: 0.0, y: 0.0},

                    <Label> {
                        draw_text: {
                            color: #89b4fa,
                            text_style: {font_size: 15.0}
                        },
                        text: "MainForm (Makepad Mode)"
                    }

                    <Label> {
                        draw_text: {
                            color: #bac2de,
                            text_style: {font_size: 12.0}
                        },
                        text: "User Name:"
                    }

                    txt_nome = <TextInput> {
                        width: 440,
                        height: 34,
                        text: "Rust Developer",
                        draw_text: {
                            color: #cdd6f4,
                            text_style: {font_size: 12.0}
                        }
                    }

                    chk_ativo = <CheckBox> {
                        draw_text: {
                            color: #cdd6f4,
                            text_style: {font_size: 12.0}
                        },
                        text: "Enable cross-platform mode (Wasm / Mobile / Desktop)"
                    }

                    botoes = <View> {
                        width: Fill,
                        height: 40,
                        flow: Right,
                        spacing: 12,
                        align: {x: 1.0, y: 0.5},

                        btn_salvar = <Button> {
                            width: 120,
                            height: 32,
                            text: "Save (Click Me)"
                        }

                        btn_cancelar = <Button> {
                            width: 100,
                            height: 32,
                            text: "Cancel"
                        }
                    }

                    lbl_status = <Label> {
                        draw_text: {
                            color: #a6e3a1,
                            text_style: {font_size: 12.0}
                        },
                        text: "Status: Ready for interaction."
                    }
                }
            }
        }
    }
}
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    cliques: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn_salvar)).clicked(actions) {
            self.cliques += 1;
            let msg = format!(
                "Status: Saved successfully! Makepad clicks: {}",
                self.cliques
            );
            self.ui.label(id!(lbl_status)).set_text(cx, &msg);
        }

        if self.ui.button(id!(btn_cancelar)).clicked(actions) {
            self.ui
                .label(id!(lbl_status))
                .set_text(cx, "Status: Action cancelled.");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn main() {
    app_main();
}
