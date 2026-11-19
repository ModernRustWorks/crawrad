//! # Interactive Data Binding Visualization Example in Makepad (ADR 0041)
//!
//! Live demonstration of **CrawRAD Blocks** connected to `DatasetAdapter`:
//! - Real-time bidirectional editing (TwoWay).
//! - Auto-Edit when typing in Edit / CheckBox.
//! - Cursor navigation (`first`, `prior`, `next`, `last`).
//! - Reversible transactions with `post()` and `cancel()` (snapshot rollback).
//!
//! To run:
//! ```powershell
//! cargo run -p crawrad-runtime-makepad --example binding_demo
//! ```

use crawrad_binding::adapter::{DatasetAdapter, DatasetState};
pub use makepad_widgets;
use makepad_widgets::*;
use std::collections::BTreeMap;
use std::sync::Arc;

type DataRecord = BTreeMap<String, String>;

fn make_record(name: &str, city: &str, active: &str) -> DataRecord {
    let mut map = BTreeMap::new();
    map.insert("NAME".to_string(), name.to_string());
    map.insert("CITY".to_string(), city.to_string());
    map.insert("ACTIVE".to_string(), active.to_string());
    map
}

live_design! {
    use link::widgets::*;
    use link::theme::*;

    App = {{App}} {
        ui: <Root> {
            janela = <Window> {
                window: {inner_size: vec2(780, 640)},
                pass: {clear_color: #181825},
            body = <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                padding: 24,
                spacing: 16,
                align: {x: 0.5, y: 0.0},

                // Header
                <Label> {
                    draw_text: {
                        color: #cdd6f4,
                        text_style: {font_size: 20.0}
                    },
                    text: "CrawRAD Blocks — Live Data Binding Demo (ADR 0041)"
                }

                <Label> {
                    draw_text: {
                        color: #a6adc8,
                        text_style: {font_size: 13.0}
                    },
                    text: "Unified visual family: the same UI components connected to a transactional DatasetAdapter."
                }

                // Navigation and Transactional Commands Toolbar
                toolbar = <View> {
                    width: 720,
                    height: 48,
                    flow: Right,
                    padding: 8,
                    spacing: 8,
                    align: {x: 0.0, y: 0.5},
                    show_bg: true,
                    draw_bg: { color: #1e1e2e },

                    btn_first = <Button> { text: "First" }
                    btn_prior = <Button> { text: "Prior" }
                    btn_next = <Button> { text: "Next" }
                    btn_last = <Button> { text: "Last" }

                    spacer = <View> { width: 16.0 }

                    btn_insert = <Button> { text: "Insert" }
                    btn_delete = <Button> { text: "Delete" }
                    btn_post = <Button> { text: "Post" }
                    btn_cancel = <Button> { text: "Cancel" }
                }

                // Dataset State Indicator
                state_bar = <View> {
                    width: 720,
                    height: 36,
                    flow: Right,
                    padding: 8,
                    spacing: 16,
                    align: {x: 0.0, y: 0.5},
                    show_bg: true,
                    draw_bg: { color: #313244 },

                    lbl_state = <Label> {
                        draw_text: {
                            color: #f9e2af,
                            text_style: {font_size: 13.0}
                        },
                        text: "Dataset State: BROWSE"
                    }

                    lbl_record = <Label> {
                        draw_text: {
                            color: #89dceb,
                            text_style: {font_size: 13.0}
                        },
                        text: "Record: 1 of 3"
                    }
                }

                // Form Card
                form_card = <View> {
                    width: 720,
                    height: 240,
                    flow: Down,
                    padding: 20,
                    spacing: 12,
                    show_bg: true,
                    draw_bg: { color: #1e1e2e },

                    <Label> {
                        draw_text: {
                            color: #89b4fa,
                            text_style: {font_size: 15.0}
                        },
                        text: "Registration Form (Bidirectional Live Binding)"
                    }

                    <Label> {
                        draw_text: {
                            color: #bac2de,
                            text_style: {font_size: 12.0}
                        },
                        text: "Customer Name (Field: NAME):"
                    }

                    txt_name = <TextInput> {
                        width: 460,
                        height: 34,
                        text: ""
                    }

                    <Label> {
                        draw_text: {
                            color: #bac2de,
                            text_style: {font_size: 12.0}
                        },
                        text: "City / State (Field: CITY):"
                    }

                    txt_city = <TextInput> {
                        width: 460,
                        height: 34,
                        text: ""
                    }

                    chk_active = <CheckBox> {
                        text: "Customer Active in Registry (Field: ACTIVE)"
                    }
                }

                // In-Memory Table Viewer
                table_view = <View> {
                    width: 720,
                    height: 140,
                    flow: Down,
                    padding: 14,
                    spacing: 6,
                    show_bg: true,
                    draw_bg: { color: #181825 },

                    <Label> {
                        draw_text: {
                            color: #a6adc8,
                            text_style: {font_size: 12.0}
                        },
                        text: "TABLE OVERVIEW (Connection):"
                    }

                    lbl_row0 = <Label> {
                        draw_text: {
                            color: #cdd6f4,
                            text_style: {font_size: 12.0}
                        },
                        text: ""
                    }
                    lbl_row1 = <Label> {
                        draw_text: {
                            color: #cdd6f4,
                            text_style: {font_size: 12.0}
                        },
                        text: ""
                    }
                    lbl_row2 = <Label> {
                        draw_text: {
                            color: #cdd6f4,
                            text_style: {font_size: 12.0}
                        },
                        text: ""
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
    adapter: Option<Arc<DatasetAdapter>>,
}

impl App {
    fn init_dataset(&mut self) {
        if self.adapter.is_some() {
            return;
        }

        let columns = &["NAME", "CITY", "ACTIVE"];
        let initial_rows = vec![
            make_record("Acme Logística Ltda", "São Paulo - SP", "true"),
            make_record("TechCorp Sistemas", "Belo Horizonte - MG", "true"),
            make_record("Varejo Central", "Curitiba - PR", "false"),
        ];

        let ds = Arc::new(DatasetAdapter::new(columns, initial_rows));
        self.adapter = Some(ds);
    }

    fn sync_view_from_dataset(&mut self, cx: &mut Cx) {
        let Some(ds) = &self.adapter else { return };

        let state = ds.state().get();
        let cursor = ds.collection().cursor().unwrap_or(0);
        let count = ds.collection().len();

        // 1. Update status bars
        let state_str = match state {
            DatasetState::Inactive => "INACTIVE",
            DatasetState::Browse => "BROWSE",
            DatasetState::Edit => "EDIT (Typing... Auto-Edit enabled)",
            DatasetState::Insert => "INSERT (New Record)",
        };
        self.ui
            .label(id!(lbl_state))
            .set_text(cx, &format!("Dataset State: {state_str}"));
        self.ui
            .label(id!(lbl_record))
            .set_text(cx, &format!("Record: {} of {}", cursor + 1, count));

        // 2. Update form fields (TwoWay Bindings)
        let name_val = ds.field("NAME").map(|o| o.get()).unwrap_or_default();
        let city_val = ds.field("CITY").map(|o| o.get()).unwrap_or_default();
        let active_val = ds.field("ACTIVE").map(|o| o.get()).unwrap_or_default() == "true";

        let txt_name = self.ui.text_input(id!(txt_name));
        if txt_name.text() != name_val {
            txt_name.set_text(cx, &name_val);
        }

        let txt_city = self.ui.text_input(id!(txt_city));
        if txt_city.text() != city_val {
            txt_city.set_text(cx, &city_val);
        }

        self.ui
            .check_box(id!(chk_active))
            .set_active(cx, active_val);

        // 3. Update table view
        let rows = ds.collection().window(0, count);
        let format_row = |idx: usize, row_opt: Option<&DataRecord>| -> String {
            let Some(r) = row_opt else {
                return String::new();
            };
            let pointer = if idx == cursor { "► " } else { "    " };
            let name = r.get("NAME").cloned().unwrap_or_default();
            let city = r.get("CITY").cloned().unwrap_or_default();
            let active = r.get("ACTIVE").cloned().unwrap_or_default();
            format!(
                "{pointer}[ID {}]  Name: {:<25}  City: {:<22}  Active: {}",
                idx + 1,
                name,
                city,
                active
            )
        };

        self.ui
            .label(id!(lbl_row0))
            .set_text(cx, &format_row(0, rows.first()));
        self.ui
            .label(id!(lbl_row1))
            .set_text(cx, &format_row(1, rows.get(1)));
        self.ui
            .label(id!(lbl_row2))
            .set_text(cx, &format_row(2, rows.get(2)));

        self.ui.redraw(cx);
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.init_dataset();
        let Some(ds) = self.adapter.clone() else {
            return;
        };

        // Navigation Buttons
        if self.ui.button(id!(btn_first)).clicked(actions) {
            ds.collection().first();
            self.sync_view_from_dataset(cx);
        }
        if self.ui.button(id!(btn_prior)).clicked(actions) {
            ds.collection().previous();
            self.sync_view_from_dataset(cx);
        }
        if self.ui.button(id!(btn_next)).clicked(actions) {
            ds.collection().next();
            self.sync_view_from_dataset(cx);
        }
        if self.ui.button(id!(btn_last)).clicked(actions) {
            ds.collection().last();
            self.sync_view_from_dataset(cx);
        }

        // Transactional Action Buttons
        if self.ui.button(id!(btn_insert)).clicked(actions) {
            let _ = ds.insert();
            self.sync_view_from_dataset(cx);
        }
        if self.ui.button(id!(btn_delete)).clicked(actions) {
            let _ = ds.delete();
            self.sync_view_from_dataset(cx);
        }
        if self.ui.button(id!(btn_post)).clicked(actions) {
            let _ = ds.post();
            self.sync_view_from_dataset(cx);
        }
        if self.ui.button(id!(btn_cancel)).clicked(actions) {
            let _ = ds.cancel();
            self.sync_view_from_dataset(cx);
        }

        // TwoWay Data Binding on TextInputs (with transparent Auto-Edit while typing!)
        if let Some(new_name) = self.ui.text_input(id!(txt_name)).changed(actions) {
            if let Some(obs) = ds.field("NAME") {
                obs.set(new_name);
            }
            self.sync_view_from_dataset(cx);
        }

        if let Some(new_city) = self.ui.text_input(id!(txt_city)).changed(actions) {
            if let Some(obs) = ds.field("CITY") {
                obs.set(new_city);
            }
            self.sync_view_from_dataset(cx);
        }

        // TwoWay Data Binding on CheckBox
        if let Some(is_active) = self.ui.check_box(id!(chk_active)).changed(actions) {
            if let Some(obs) = ds.field("ACTIVE") {
                obs.set(if is_active {
                    "true".to_string()
                } else {
                    "false".to_string()
                });
            }
            self.sync_view_from_dataset(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.init_dataset();
            self.sync_view_from_dataset(cx);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn main() {
    app_main();
}
