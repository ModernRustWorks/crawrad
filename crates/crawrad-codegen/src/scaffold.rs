//! Scaffolding for the templates of newly generated projects (ADR 0017).

use crate::AlvoGerador;
use crawrad_note::note;

#[note("scaffold-scaffold-main-rs")]
pub fn scaffold_main_rs(alvo: AlvoGerador, nome: &str, template_id: &str) -> String {
    match template_id {
        "console_app" => format!("fn main() {{\n    println!(\"Hello from {nome}!\");\n}}\n"),
        _ => match alvo {
            AlvoGerador::Egui => format!(
                "use eframe::egui;\n\n\
                fn main() -> eframe::Result<()> {{\n    \
                    let native_options = eframe::NativeOptions::default();\n    \
                    eframe::run_native(\n        \
                        \"{nome}\",\n        \
                        native_options,\n        \
                        Box::new(|_cc| Ok(Box::new(MyApp::default()))),\n    \
                    )\n\
                }}\n\n\
                #[derive(Default)]\n\
                struct MyApp {{}}\n\n\
                impl eframe::App for MyApp {{\n    \
                    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {{\n        \
                        ui.heading(\"{nome}\");\n    \
                    }}\n\
                }}\n"
            ),
            _ => "mod app;\n\nfn main() {\n    app::app_main();\n}\n".to_string(),
        },
    }
}

/// The opening every Makepad template's `app.rs` shares, up to the `ui:` of its `App`.
const APP_HEAD: &str = "use makepad_widgets::*;\n\nlive_design! {\n    link widgets;\n    use link::widgets::*;\n    use link::theme::*;\n\n    App = {{App}} {\n        ";

#[note("scaffold-scaffold-app-rs")]
pub fn scaffold_app_rs(alvo: AlvoGerador, nome: &str, template_id: &str) -> Option<String> {
    if alvo == AlvoGerador::Egui {
        return None;
    }
    match template_id {
        "wasm_app" => Some(
            APP_HEAD.to_string()
                + &format!(
                    "\
                    ui: <Window> {{\n            \
                        window: {{ inner_size: vec2(1024.0, 768.0), title: \"{nome} (WASM)\" }},\n            \
                        pass: {{ clear_color: #x14141a }},\n            \
                        body = <View> {{\n                \
                            width: Fill, height: Fill,\n                \
                            flow: Down, spacing: 16.0,\n                \
                            align: {{ x: 0.5, y: 0.5 }},\n                \
                            <Label> {{\n                    \
                                draw_text: {{\n                        \
                                    color: #x33cce6,\n                        \
                                    text_style: <THEME_FONT_BOLD> {{ font_size: 20.0 }}\n                    \
                                }}\n                    \
                                text: \"{nome} WebAssembly\"\n                \
                            }}\n                \
                            <Label> {{\n                    \
                                draw_text: {{\n                        \
                                    color: #x9999b3,\n                        \
                                    text_style: <THEME_FONT_REGULAR> {{ font_size: 14.0 }}\n                    \
                                }}\n                    \
                                text: \"Compiled to WASM via Makepad backend\"\n                \
                            }}\n            \
                        }}\n        \
                    }}\n    \
                }}\n\
            }}\n\n"
                )
                + &app_tail(None),
        ),
        "mobile_app" => Some(
            APP_HEAD.to_string()
                + &format!(
                    "\
                    ui: <Root> {{\n            \
                        main_window = <Window> {{\n                \
                            window: {{ inner_size: vec2(390.0, 844.0) }},\n                \
                            pass: {{ clear_color: #x0f172a }},\n                \
                            body = <View> {{\n                    \
                                width: Fill, height: Fill,\n                    \
                                flow: Down, spacing: 12.0,\n                    \
                                header = <RoundedView> {{\n                        \
                                    width: Fill, height: 56.0,\n                        \
                                    flow: Right, align: {{ y: 0.5 }},\n                        \
                                    padding: {{ left: 16.0, right: 16.0 }},\n                        \
                                    draw_bg: {{ color: #x1e293b }},\n                        \
                                    <Label> {{\n                            \
                                        draw_text: {{\n                                \
                                            color: #f8fafc,\n                                \
                                            text_style: <THEME_FONT_BOLD> {{ font_size: 14.0 }}\n                            \
                                        }}\n                            \
                                        text: \"{nome} Mobile\"\n                        \
                                    }}\n                    \
                                }}\n                    \
                                content = <View> {{\n                        \
                                    width: Fill, height: Fill,\n                        \
                                    flow: Down, padding: 16.0, spacing: 16.0,\n                        \
                                    align: {{ x: 0.5, y: 0.5 }},\n                        \
                                    <Label> {{\n                            \
                                        draw_text: {{\n                                \
                                            color: #x94a3b8,\n                                \
                                            text_style: <THEME_FONT_REGULAR> {{ font_size: 12.0 }}\n                            \
                                        }}\n                            \
                                        text: \"Makepad Mobile App for Android & iOS\"\n                        \
                                    }}\n                        \
                                    btn_counter = <Button> {{\n                            \
                                        width: 200.0, height: 48.0,\n                            \
                                        text: \"Tap Me (Mobile Button)\"\n                        \
                                    }}\n                        \
                                    lbl_status = <Label> {{\n                            \
                                        draw_text: {{\n                                \
                                            color: #x38bdf8,\n                                \
                                            text_style: <THEME_FONT_BOLD> {{ font_size: 14.0 }}\n                            \
                                        }}\n                            \
                                        text: \"Taps: 0\"\n                        \
                                    }}\n                    \
                                }}\n                    \
                                bottom_nav = <RoundedView> {{\n                        \
                                    width: Fill, height: 64.0,\n                        \
                                    flow: Right, align: {{ x: 0.5, y: 0.5 }},\n                        \
                                    spacing: 32.0,\n                        \
                                    draw_bg: {{ color: #x1e293b }},\n                        \
                                    <Button> {{ width: 80.0, height: 40.0, text: \"Home\" }}\n                        \
                                    <Button> {{ width: 80.0, height: 40.0, text: \"Explore\" }}\n                        \
                                    <Button> {{ width: 80.0, height: 40.0, text: \"Settings\" }}\n                    \
                                }}\n                \
                            }}\n            \
                        }}\n        \
                        }}\n    \
                    }}\n\
                }}\n\n"
                )
                + &app_tail(Some(("btn_counter", "lbl_status", "Taps"))),
        ),
        "multiwindow_app" => Some(
            APP_HEAD.to_string()
                + &format!(
                    "\
                    ui: <Root> {{\n            \
                        main_window = <Window> {{\n                \
                            window: {{ inner_size: vec2(800.0, 600.0), title: \"{nome} - Main\" }},\n                \
                            pass: {{ clear_color: #x1f1f24 }},\n                \
                            body = <View> {{\n                    \
                                width: Fill, height: Fill,\n                    \
                                flow: Down, spacing: 12.0, align: {{ x: 0.5, y: 0.5 }},\n                    \
                                <Label> {{\n                        \
                                    draw_text: {{ color: #e6e6e6 }},\n                        \
                                    text: \"Multi-Window Desktop Application\"\n                    \
                                }}\n                \
                            }}\n            \
                        }}\n            \
                        inspector_window = <Window> {{\n                \
                            window: {{ inner_size: vec2(360.0, 500.0), title: \"Inspector\" }},\n                \
                            pass: {{ clear_color: #x26262e }},\n                \
                            body = <View> {{\n                    \
                                width: Fill, height: Fill,\n                    \
                                flow: Down, padding: 12.0,\n                    \
                                <Label> {{\n                        \
                                    draw_text: {{ color: #b3b3cc }},\n                        \
                                    text: \"Properties & Tools\"\n                    \
                                }}\n                \
                            }}\n            \
                        }}\n        \
                    }}\n    \
                }}\n\
                }}\n\n"
                )
                + &app_tail(None),
        ),
        _ => Some(
            APP_HEAD.to_string()
                + &format!(
                    "\
                    ui: <Window> {{\n            \
                        window: {{ inner_size: vec2(800.0, 600.0), title: \"{nome}\" }},\n            \
                        pass: {{ clear_color: #x1f1f24 }},\n            \
                        body = <View> {{\n                \
                            width: Fill, height: Fill,\n                \
                            flow: Down, spacing: 16.0,\n                \
                            align: {{ x: 0.5, y: 0.5 }},\n                \
                            <Label> {{\n                    \
                                draw_text: {{\n                        \
                                    color: #e6e6e6,\n                        \
                                    text_style: <THEME_FONT_BOLD> {{ font_size: 18.0 }}\n                    \
                                }}\n                    \
                                text: \"Welcome to {nome}\"\n                \
                            }}\n                \
                            btn = <Button> {{\n                    \
                                width: 160.0, height: 40.0,\n                    \
                                text: \"Click Me\"\n                \
                            }}\n                \
                            lbl = <Label> {{\n                    \
                                draw_text: {{\n                        \
                                    color: #x4db3e6,\n                        \
                                    text_style: <THEME_FONT_REGULAR> {{ font_size: 14.0 }}\n                    \
                                }}\n                    \
                                text: \"Clicks: 0\"\n                \
                            }}\n            \
                        }}\n        \
                    }}\n    \
                }}\n\
            }}\n\n"
                )
                + &app_tail(Some(("btn", "lbl", "Clicks"))),
        ),
    }
}

/// The `App` struct and trait impls every Makepad template ends with.
fn app_tail(counter: Option<(&str, &str, &str)>) -> String {
    let (field, actions) = match counter {
        Some((button, label, word)) => (
            "    #[rust] count: usize,\n",
            format!(
                "    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {{\n        \
                    if self.ui.button(id!({button})).clicked(actions) {{\n            \
                        self.count += 1;\n            \
                        self.ui.label(id!({label})).set_text(cx, &format!(\"{word}: {{}}\", self.count));\n        \
                    }}\n    \
                }}\n"
            ),
        ),
        None => (
            "",
            "    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}\n".to_string(),
        ),
    };
    format!(
        "app_main!(App);\n\n\
        #[derive(Live, LiveHook)]\n\
        pub struct App {{\n    \
            #[live] ui: WidgetRef,\n{field}\
        }}\n\n\
        impl LiveRegister for App {{\n    \
            fn live_register(cx: &mut Cx) {{\n        \
                makepad_widgets::live_design(cx);\n    \
            }}\n\
        }}\n\n\
        impl MatchEvent for App {{\n{actions}\
        }}\n\n\
        impl AppMain for App {{\n    \
            fn handle_event(&mut self, cx: &mut Cx, event: &Event) {{\n        \
                self.match_event(cx, event);\n        \
                self.ui.handle_event(cx, event, &mut Scope::empty());\n    \
            }}\n\
        }}\n"
    )
}
