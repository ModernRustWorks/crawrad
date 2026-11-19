//! Interactive mobile application example in Makepad (Android / iOS).
//!
//! Simulates the geometry and usability of a mobile device (390x844 portrait)
//! with navigation bar, tactile cards, and responsive counter.
//!
//! To run on desktop for immediate testing:
//! ```powershell
//! cargo run -p crawrad-runtime-makepad --example mobile_counter
//! ```

pub use makepad_widgets;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;

    App = {{App}} {
        ui: <Root> {
            janela_mobile = <Window> {
                // Default dimensions of a modern smartphone (iPhone 14 / Android portrait: 390x844)
                window: { inner_size: vec2(390, 844) },
                pass: { clear_color: #0b0f17 },

                body = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    spacing: 0,

                    // 1. TOP BAR (Status & Header)
                    header = <RoundedView> {
                        width: Fill,
                        height: 60.0,
                        flow: Right,
                        align: { y: 0.5 },
                        padding: { left: 18.0, right: 18.0, top: 4.0, bottom: 4.0 },
                        draw_bg: { color: #141c2b },

                        <Label> {
                            draw_text: {
                                color: #38bdf8,
                                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                            },
                            text: "📱 CrawRAD Mobile"
                        }

                        <View> { width: Fill, height: 1.0 }

                        lbl_bateria = <Label> {
                            draw_text: {
                                color: #94a3b8,
                                text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                            },
                            text: "100% 🔋"
                        }
                    }

                    // 2. SCROLLABLE BODY / CONTENT
                    conteudo = <ScrollYView> {
                        width: Fill,
                        height: Fill,
                        flow: Down,
                        padding: { left: 16.0, right: 16.0, top: 16.0, bottom: 16.0 },
                        spacing: 16.0,

                        // Welcome Card
                        card_hero = <RoundedView> {
                            width: Fill,
                            height: Fit,
                            flow: Down,
                            padding: 16.0,
                            spacing: 8.0,
                            draw_bg: {
                                color: #182234,
                                border_radius: 12.0,
                                border_size: 1.0,
                                border_color: #232f42,
                            },

                            <Label> {
                                draw_text: {
                                    color: #f8fafc,
                                    text_style: <THEME_FONT_BOLD> { font_size: 15.0 }
                                },
                                text: "Makepad Mobile Runtime"
                            }

                            <Label> {
                                width: Fill,
                                draw_text: {
                                    color: #94a3b8,
                                    text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
                                },
                                text: "This application demonstrates native mobile support in the CrawRAD ecosystem. Rendered via GPU with full support for touch, gestures, and high frame rates."
                            }
                        }

                        // Interactive Action Card (Tactile Counter)
                        card_contador = <RoundedView> {
                            width: Fill,
                            height: Fit,
                            flow: Down,
                            align: { x: 0.5 },
                            padding: 20.0,
                            spacing: 14.0,
                            draw_bg: {
                                color: #182234,
                                border_radius: 12.0,
                                border_size: 1.0,
                                border_color: (vec4(0.145, 0.388, 0.922, 1.0)),
                            },

                            <Label> {
                                draw_text: {
                                    color: #38bdf8,
                                    text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
                                },
                                text: "INTERACTIVE COUNTER"
                            }

                            lbl_contador = <Label> {
                                draw_text: {
                                    color: #ffffff,
                                    text_style: <THEME_FONT_BOLD> { font_size: 36.0 }
                                },
                                text: "0"
                            }

                            lbl_feedback = <Label> {
                                draw_text: {
                                    color: #94a3b8,
                                    text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
                                },
                                text: "Tap the button to increment"
                            }

                            btn_incrementar = <Button> {
                                width: 220.0,
                                height: 48.0,
                                text: "Increment (+1)",
                                draw_bg: {
                                    color: #1d68e1,
                                    color_hover: (vec4(0.145, 0.388, 0.922, 1.0)),
                                    border_radius: 8.0,
                                    border_size: 0.0,
                                }
                                draw_text: {
                                    color: #ffffff,
                                    text_style: <THEME_FONT_BOLD> { font_size: 13.0 }
                                }
                            }

                            btn_resetar = <Button> {
                                width: 140.0,
                                height: 34.0,
                                text: "Reset",
                                draw_bg: {
                                    color: #1e293b,
                                    color_hover: #334155,
                                    border_radius: 6.0,
                                    border_size: 1.0,
                                    border_color: #334155,
                                }
                                draw_text: {
                                    color: #94a3b8,
                                    text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
                                }
                            }
                        }

                        // Platform Information Card
                        card_info = <RoundedView> {
                            width: Fill,
                            height: Fit,
                            flow: Down,
                            padding: 14.0,
                            spacing: 6.0,
                            draw_bg: {
                                color: #141c2b,
                                border_radius: 10.0,
                                border_size: 1.0,
                                border_color: #1e293b,
                            },

                            <Label> {
                                draw_text: {
                                    color: #f1f5f9,
                                    text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                                },
                                text: "Supported Targets:"
                            }

                            <Label> {
                                draw_text: {
                                    color: #64748b,
                                    text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                                },
                                text: "🤖 Android (NDK / aarch64, armv7, x86_64)\n🍎 iOS (Metal / aarch64, sim)"
                            }
                        }
                    }

                    // 3. BOTTOM NAVIGATION BAR (Bottom Navigation Bar)
                    bottom_nav = <RoundedView> {
                        width: Fill,
                        height: 64.0,
                        flow: Right,
                        align: { x: 0.5, y: 0.5 },
                        spacing: 24.0,
                        padding: { left: 16.0, right: 16.0 },
                        draw_bg: {
                            color: #141c2b,
                            border_size: 1.0,
                            border_color: #1e293b,
                        },

                        btn_nav_home = <Button> {
                            width: 80.0, height: 40.0,
                            text: "🏠 Home",
                            draw_bg: { color: #1e293b, border_radius: 6.0 }
                            draw_text: { color: #38bdf8 }
                        }
                        btn_nav_explore = <Button> {
                            width: 80.0, height: 40.0,
                            text: "🔍 Explore",
                            draw_bg: { color: #00000000, border_radius: 6.0 }
                            draw_text: { color: #94a3b8 }
                        }
                        btn_nav_settings = <Button> {
                            width: 80.0, height: 40.0,
                            text: "⚙ Settings",
                            draw_bg: { color: #00000000, border_radius: 6.0 }
                            draw_text: { color: #94a3b8 }
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
    toques: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn_incrementar)).clicked(actions) {
            self.toques += 1;
            let count_str = self.toques.to_string();
            let feedback = format!("Total taps registered: {}", self.toques);
            self.ui.label(id!(lbl_contador)).set_text(cx, &count_str);
            self.ui.label(id!(lbl_feedback)).set_text(cx, &feedback);
        }

        if self.ui.button(id!(btn_resetar)).clicked(actions) {
            self.toques = 0;
            self.ui.label(id!(lbl_contador)).set_text(cx, "0");
            self.ui
                .label(id!(lbl_feedback))
                .set_text(cx, "Counter reset.");
        }

        if self.ui.button(id!(btn_nav_home)).clicked(actions) {
            self.ui
                .label(id!(lbl_feedback))
                .set_text(cx, "Selected tab: Home");
        }

        if self.ui.button(id!(btn_nav_explore)).clicked(actions) {
            self.ui
                .label(id!(lbl_feedback))
                .set_text(cx, "Selected tab: Explore");
        }

        if self.ui.button(id!(btn_nav_settings)).clicked(actions) {
            self.ui
                .label(id!(lbl_feedback))
                .set_text(cx, "Selected tab: Settings");
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
