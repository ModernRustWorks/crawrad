//! App translations grid view widget.

use makepad_widgets::*;

live_design! {
    link crawrad;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use crate::borderless::*;

    LangChipSlot = <RoundedView> {
        visible: false,
        width: Fit, height: 26.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 10.0, right: 10.0 },
        show_bg: true,
        draw_bg: {
            color: #2b2d30,
            border_color: #4a4f54,
            border_size: 1.0,
            // 4 px corner: Makepad RoundedView draws twice the border_radius
            border_radius: 2.0,
        }
        lbl_name = <Label> {
            width: Fit, height: Fit, padding: 0.0,
            draw_text: {
                color: #c9cbce,
                text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
            }
            text: ""
        }
        lbl_count = <Label> {
            width: Fit, height: Fit, padding: 0.0,
            draw_text: {
                color: #e8914a,
                text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
            }
            text: ""
        }
    }

    // A language's column title in the grid header.
    LangHeader = <View> {
        visible: false,
        width: Fill, height: Fit,
        lbl = <Label> {
            padding: 0.0,
            draw_text: { color: #8d9093, text_style: <THEME_FONT_REGULAR> { font_size: 9.5 } }
            text: ""
        }
    }

    // A coloured dot and its meaning, in the footer's legend.
    LegendDot = <View> {
        width: Fit, height: Fit,
        flow: Right, align: { y: 0.5 },
        spacing: 4.0,
        dot = <View> {
            width: 6.0, height: 6.0,
            show_bg: true,
            draw_bg: {
                color: #8d9093,
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5 - 0.5);
                    sdf.fill(self.color);
                    return sdf.result;
                }
            }
        }
        lbl = <Label> {
            padding: 0.0,
            draw_text: {
                color: #8d9093,
                text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
            }
            text: ""
        }
    }

    // One state of a cell: the text on its tinted background.
    CellText = <View> {
        visible: false,
        width: Fill, height: Fill,
        align: { y: 0.5 },
        padding: { left: 8.0, right: 8.0 },
        show_bg: true,
        draw_bg: { color: #0000 }
        lbl_text = <Label> {
            width: Fill, height: Fit, padding: 0.0,
            draw_text: {
                color: #c9cbce,
                text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
            }
            text: ""
        }
    }

    CellSlot = <View> {
        visible: false,
        width: Fill, height: Fill,
        flow: Overlay, align: { y: 0.5 },

        display_view = <View> {
            width: Fill, height: Fill,
            flow: Overlay, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0 },
            // One pre-styled view per state, switched with set_visible: recolouring PortalList
            // items with apply_over every frame breaks Makepad's draw order.
            done_view = <CellText> {}
            stale_view = <CellText> {
                draw_bg: { color: #e6c46e12 }
                lbl_text = { draw_text: { color: #e6c46e } }
            }
            missing_view = <CellText> {
                draw_bg: { color: #e5534b14 }
                lbl_text = { draw_text: { color: #e5534b }, text: "missing" }
            }

            btn_cell = <Button> {
                width: Fill, height: Fill,
                draw_bg: {
                    color: #0000,
                    color_hover: #ffffff08,
                    color_down: #ffffff14,
                    border_size: 0.0,
                }
                text: ""
            }
        }

        edit_view = <View> {
            visible: false,
            width: Fill, height: Fill,
            padding: { left: 4.0, right: 4.0 },
            align: { y: 0.5 },
            input_text = <TextInput> {
                width: Fill, height: 26.0,
                empty_text: ""
                padding: { left: 6.0, right: 6.0, top: 4.0, bottom: 4.0 },
                draw_bg: {
                    color: #1e2022,
                    border_color_1: #e8914a,
                    border_color_2: #e8914a,
                    border_size: 1.0,
                    border_radius: 1.0,
                }
                draw_text: {
                    color: #e6e7e8,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                }
            }
        }
    }

    GroupHeaderRow = <View> {
        width: Fill, height: 32.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 16.0, right: 16.0 },
        spacing: 8.0,
        show_bg: true,
        draw_bg: { color: #1c1d1f }

        lbl_form = <Label> {
            width: Fit, height: Fit, padding: 0.0,
            draw_text: {
                color: #e6e7e8,
                text_style: <THEME_FONT_BOLD> { font_size: 10.5 }
            }
            text: ""
        }
        lbl_count = <Label> {
            width: Fit, height: Fit, padding: 0.0,
            draw_text: {
                color: #8d9093,
                text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
            }
            text: ""
        }
    }

    DataRow = <View> {
        width: Fill, height: 32.0,
        flow: Overlay,
        show_bg: true,
        draw_bg: {
            instance selected: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg_color = mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.149, 0.188, 0.231, 1.0), self.selected);
                sdf.box_all(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0, 0.0, 0.0, 0.0);
                sdf.fill_keep(bg_color);
                // bottom border #25282a
                sdf.move_to(0.0, self.rect_size.y - 1.0);
                sdf.line_to(self.rect_size.x, self.rect_size.y - 1.0);
                sdf.stroke(vec4(0.145, 0.157, 0.165, 1.0), 1.0);
                // 2 px orange bar at left when selected
                if self.selected > 0.5 {
                    sdf.move_to(1.0, 0.0);
                    sdf.line_to(1.0, self.rect_size.y);
                    sdf.stroke(vec4(0.910, 0.569, 0.290, 1.0), 2.0);
                }
                return sdf.result;
            }
        }

        row_content = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 16.0, right: 16.0 },

            lbl_key = <Label> {
                width: 300.0, height: Fit, padding: 0.0,
                draw_text: {
                    color: #a9abae,
                    text_style: <THEME_FONT_CODE> { font_size: 9.5 }
                }
                text: ""
            }
            lbl_source = <Label> {
                width: Fill, height: Fit, padding: 0.0,
                draw_text: {
                    color: #c9cbce,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                }
                text: ""
            }
            cell_0 = <CellSlot> {}
            cell_1 = <CellSlot> {}
            cell_2 = <CellSlot> {}
            cell_3 = <CellSlot> {}
        }
    }

    pub TranslationsView = {{TranslationsView}} {
        width: Fill, height: Fill,
        flow: Overlay,

        main_content = <CrawradTopRounded> {
            draw_bg: { color: #1d1f20 }
            width: Fill, height: Fill,
            flow: Down,

            // Header: title, subtitle, and files line
            header_bar = <View> {
                width: Fill, height: 42.0,
                flow: Right, align: { y: 0.5 },
                padding: { left: 16.0, right: 16.0 },
                spacing: 12.0,

                lbl_title = <Label> {
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_BOLD> { font_size: 13.0 }
                    }
                    text: "Translations"
                }
                lbl_dot = <Label> {
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
                    }
                    text: "·"
                }
                lbl_subtitle = <Label> {
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_REGULAR> { font_size: 10.5 }
                    }
                    text: "Every text the user of your app reads, in every language it ships in"
                }

                <View> { width: Fill, height: Fit }

                lbl_files = <Label> {
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_CODE> { font_size: 10.0 }
                    }
                    text: ""
                }
            }

            // Toolbar row: English source chip, lang chips, + Language... button, Missing only toggle
            toolbar_row = <View> {
                width: Fill, height: 38.0,
                flow: Right, align: { y: 0.5 },
                padding: { left: 16.0, right: 16.0 },
                spacing: 8.0,

                chip_english = <RoundedView> {
                    width: Fit, height: 26.0,
                    flow: Right, align: { y: 0.5 },
                    padding: { left: 10.0, right: 10.0 },
                    show_bg: true,
                    draw_bg: {
                        color: #2b2d30,
                        border_color: #4a4f54,
                        border_size: 1.0,
                        border_radius: 2.0,
                    }
                    lbl_en = <Label> {
                        padding: 0.0,
                        draw_text: {
                            color: #c9cbce,
                            text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                        }
                        text: "English source"
                    }
                }

                chip_lang_0 = <LangChipSlot> {}
                chip_lang_1 = <LangChipSlot> {}
                chip_lang_2 = <LangChipSlot> {}
                chip_lang_3 = <LangChipSlot> {}

                btn_add_lang = <Button> {
                    width: Fit, height: 26.0,
                    padding: { left: 8.0, right: 8.0 },
                    draw_bg: {
                        color: #0000,
                        color_hover: #ffffff0a,
                        color_down: #ffffff14,
                        border_size: 0.0,
                    }
                    draw_text: {
                        color: #e8914a,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                    }
                    text: "+ Language…"
                }

                <View> { width: Fill, height: Fit }

                // A Button draws no children: the tick and the label sit under a clear button.
                missing_toggle = <View> {
                    width: Fit, height: 26.0,
                    flow: Overlay, align: { y: 0.5 },
                    missing_row = <View> {
                        width: Fit, height: Fill,
                        flow: Right, align: { y: 0.5 },
                        padding: { left: 6.0, right: 8.0 },
                        tick_box = <RoundedView> {
                            width: 14.0, height: 14.0,
                            margin: { right: 6.0 },
                            show_bg: true,
                            draw_bg: {
                                instance checked: 0.0
                                color: #0000,
                                border_color: #4a4f54,
                                border_size: 1.0,
                                border_radius: 1.0,
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box_all(0.0, 0.0, self.rect_size.x, self.rect_size.y, 1.0, 1.0, 1.0, 1.0);
                                    if self.checked > 0.5 {
                                        sdf.fill(#e8914a);
                                        sdf.move_to(self.rect_size.x * 0.22, self.rect_size.y * 0.48);
                                        sdf.line_to(self.rect_size.x * 0.42, self.rect_size.y * 0.72);
                                        sdf.line_to(self.rect_size.x * 0.78, self.rect_size.y * 0.26);
                                        sdf.stroke(#1b1d1f, 1.4);
                                    } else {
                                        sdf.fill_keep(#0000);
                                        sdf.stroke(#4a4f54, 1.0);
                                    }
                                    return sdf.result;
                                }
                            }
                        }
                        lbl_missing = <Label> {
                            padding: 0.0,
                            draw_text: {
                                color: #c9cbce,
                                text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                            }
                            text: "Missing only (0)"
                        }
                    }
                    btn_missing_only = <Button> {
                        width: Fill, height: Fill,
                        text: ""
                        draw_bg: {
                            color: #0000,
                            color_hover: #ffffff0a,
                            color_down: #ffffff14,
                            border_size: 0.0,
                        }
                    }
                }
            }

            // Fixed Column header row
            col_header_row = <View> {
                width: Fill, height: 28.0,
                flow: Right, align: { y: 0.5 },
                padding: { left: 16.0, right: 16.0 },
                show_bg: true,
                draw_bg: { color: #232527 }

                lbl_col_key = <Label> {
                    width: 300.0, height: Fit, padding: 0.0,
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                    }
                    text: "Key"
                }
                lbl_col_source = <Label> {
                    width: Fill, height: Fit, padding: 0.0,
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                    }
                    text: "English (from the form)"
                }
                header_lang_0 = <LangHeader> {}
                header_lang_1 = <LangHeader> {}
                header_lang_2 = <LangHeader> {}
                header_lang_3 = <LangHeader> {}
            }

            // Grid PortalList
            grid_list = <PortalList> {
                width: Fill, height: Fill,
                GroupHeaderRow = <GroupHeaderRow> {}
                DataRow = <DataRow> {}
                DataRowSelected = <DataRow> { draw_bg: { selected: 1.0 } }
            }

            // Footer
            footer_bar = <View> {
                width: Fill, height: Fit,
                flow: Right, align: { y: 0.5 },
                padding: { left: 16.0, right: 16.0, top: 8.0, bottom: 8.0 },
                spacing: 14.0,
                show_bg: true,
                draw_bg: {
                    color: #1b1d1f,
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.move_to(0.0, 0.0);
                        sdf.line_to(self.rect_size.x, 0.0);
                        sdf.stroke(#25282a, 1.0);
                        return sdf.result;
                    }
                }

                dot_missing_view = <LegendDot> {
                    dot = { draw_bg: { color: #e5534b } }
                    lbl = { text: "missing — the app shows the English text there" }
                }

                dot_stale_view = <LegendDot> {
                    dot = { draw_bg: { color: #e6c46e } }
                    lbl = { text: "English changed since it was translated" }
                }

                <View> { width: Fill, height: Fit }

                lbl_footer_notes = <Label> {
                    width: Fill, height: Fit,
                    padding: 0.0,
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    // A narrow window wraps the notes instead of cutting them.
                    text: "The English column is edited in the Designer · a key never changes when you rename a control · MCP: strings.list / strings.set"
                }
            }
        }

        // Add Language Card Backdrop Overlay
        add_card_backdrop = <View> {
            visible: false,
            width: Fill, height: Fill,
            flow: Overlay,
            align: { x: 0.5, y: 0.5 },

            btn_dismiss_add_card = <Button> {
                width: Fill, height: Fill,
                draw_bg: {
                    color: #0000,
                    color_hover: #0000,
                    color_down: #0000,
                    border_size: 0.0,
                }
                text: ""
            }

            add_card = <RoundedView> {
                width: 320.0, height: Fit,
                flow: Down,
                padding: 16.0,
                spacing: 10.0,
                show_bg: true,
                draw_bg: {
                    color: #2a2d30,
                    border_color: #4a4f54,
                    border_size: 1.0,
                    border_radius: 3.0,
                }

                lbl_card_title = <Label> {
                    padding: 0.0,
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                    }
                    text: "Add Language"
                }

                lbl_card_prompt = <Label> {
                    padding: 0.0,
                    draw_text: {
                        color: #8d9093,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
                    }
                    text: "BCP 47 language code (e.g. pt-BR, de, fr, es):"
                }

                input_code = <TextInput> {
                    width: Fill, height: 28.0,
                    empty_text: "pt-BR"
                    padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 4.0 },
                    draw_bg: {
                        color: #1e2022,
                        border_color_1: #4a4f54,
                        border_color_2: #4a4f54,
                        border_size: 1.0,
                        border_radius: 2.0,
                    }
                    draw_text: {
                        color: #e6e7e8,
                        text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                    }
                }

                add_error_box = <View> {
                    visible: false,
                    width: Fill, height: Fit,
                    lbl_add_error = <Label> {
                        padding: 0.0,
                        draw_text: { color: #f0a39c, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                        text: ""
                    }
                }

                card_buttons = <View> {
                    width: Fill, height: 28.0,
                    flow: Right, align: { y: 0.5 },
                    spacing: 8.0,
                    margin: { top: 4.0 },

                    <View> { width: Fill, height: Fit }

                    btn_card_cancel = <Button> {
                        width: 70.0, height: 26.0,
                        draw_text: { color: #a9abae, text_style: <THEME_FONT_REGULAR> { font_size: 9.5 } }
                        text: "Cancel"
                    }
                    btn_card_add = <Button> {
                        width: 70.0, height: 26.0,
                        draw_bg: {
                            color: #f3b27a, color_hover: #f7c396, color_down: #e09f65,
                            border_size: 0.0,
                        }
                        draw_text: { color: #1b1d1f, text_style: <THEME_FONT_BOLD> { font_size: 9.5 } }
                        text: "Add"
                    }
                }
            }
        }
    }
}

/// A target language chip in the toolbar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LangChip {
    pub name: String,
    pub done: usize,
    pub total: usize,
}

/// Formats the display label for a language chip: `"{name} {done}/{total}"`.
pub fn chip_label(chip: &LangChip) -> String {
    format!("{} {}/{}", chip.name, chip.done, chip.total)
}

/// The display state of a single translation cell in the grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellView {
    Done(String),
    Stale(String),
    Missing,
}

/// Returns the text content of a cell view (empty for missing).
pub fn cell_text(cell: &CellView) -> &str {
    match cell {
        CellView::Done(text) | CellView::Stale(text) => text.as_str(),
        CellView::Missing => "",
    }
}

/// A row in the translation grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridRow {
    pub key: String,
    pub label: String,
    pub source: String,
    pub cells: Vec<CellView>,
}

/// A group of grid rows belonging to one form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridGroup {
    pub form: String,
    pub rows: Vec<GridRow>,
}

/// Actions emitted by the translations view.
#[derive(Clone, Debug, PartialEq, Eq, DefaultNone)]
pub enum TranslationsAction {
    None,
    ToggleMissingOnly,
    AddLanguage,
    Commit {
        key: String,
        lang: usize,
        text: String,
    },
}

#[derive(Clone, Debug)]
enum FlatItem {
    GroupHeader {
        form: String,
        count: usize,
    },
    DataRow {
        group_idx: usize,
        row_idx: usize,
        key: String,
    },
}

#[derive(Live, LiveHook, Widget)]
pub struct TranslationsView {
    #[deref]
    pub view: View,
    #[rust]
    langs: Vec<LangChip>,
    #[rust]
    groups: Vec<GridGroup>,
    #[rust]
    flat_items: Vec<FlatItem>,
    #[rust]
    selected_cell: Option<(String, usize)>,
    /// The text a just-opened cell editor starts with: applied, with the key focus, when the
    /// selected row is drawn (its template is new, so the click's item is not the drawn one).
    #[rust]
    edit_seed: Option<String>,
}

impl Widget for TranslationsView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            let portal = step.as_portal_list();
            let Some(mut list) = portal.borrow_mut() else {
                continue;
            };
            list.set_item_range(cx, 0, self.flat_items.len());
            while let Some(item_idx) = list.next_visible_item(cx) {
                let Some(flat) = self.flat_items.get(item_idx).cloned() else {
                    continue;
                };
                match flat {
                    FlatItem::GroupHeader { form, count } => {
                        let item = list.item(cx, item_idx, live_id!(GroupHeaderRow));
                        item.label(id!(lbl_form)).set_text(cx, &form);
                        item.label(id!(lbl_count))
                            .set_text(cx, &format!("{count} texts"));
                        item.draw_all(cx, scope);
                    }
                    FlatItem::DataRow {
                        group_idx,
                        row_idx,
                        key,
                    } => {
                        let Some(row) =
                            self.groups.get(group_idx).and_then(|g| g.rows.get(row_idx))
                        else {
                            continue;
                        };
                        let editing = self
                            .selected_cell
                            .as_ref()
                            .filter(|(k, _)| *k == key)
                            .map(|(_, l)| *l);
                        let template = if editing.is_some() {
                            live_id!(DataRowSelected)
                        } else {
                            live_id!(DataRow)
                        };
                        let item = list.item(cx, item_idx, template);
                        item.label(id!(lbl_key)).set_text(cx, &row.label);
                        item.label(id!(lbl_source)).set_text(cx, &row.source);
                        // The field can take the key focus only once drawn.
                        let mut focus = None;
                        let cell_ids = [id!(cell_0), id!(cell_1), id!(cell_2), id!(cell_3)];
                        for (lang, cell_id) in cell_ids.iter().enumerate() {
                            let slot = item.view(*cell_id);
                            let Some(cell_data) = row.cells.get(lang) else {
                                slot.set_visible(cx, false);
                                continue;
                            };
                            slot.set_visible(cx, true);
                            let is_editing = editing == Some(lang);
                            slot.view(id!(display_view)).set_visible(cx, !is_editing);
                            slot.view(id!(edit_view)).set_visible(cx, is_editing);
                            if is_editing && let Some(seed) = self.edit_seed.take() {
                                let input = slot.text_input(id!(input_text));
                                input.set_text(cx, &seed);
                                focus = Some(input);
                            }
                            let disp = slot.view(id!(display_view));
                            let (shown, text) = match cell_data {
                                CellView::Done(text) => (live_id!(done_view), text.as_str()),
                                CellView::Stale(text) => (live_id!(stale_view), text.as_str()),
                                CellView::Missing => (live_id!(missing_view), "missing"),
                            };
                            for state in [
                                live_id!(done_view),
                                live_id!(stale_view),
                                live_id!(missing_view),
                            ] {
                                let view = disp.view(&[state]);
                                view.set_visible(cx, state == shown);
                                if state == shown {
                                    view.label(id!(lbl_text)).set_text(cx, text);
                                }
                            }
                        }
                        item.draw_all(cx, scope);
                        if let Some(input) = focus {
                            input.set_key_focus(cx);
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl TranslationsView {
    /// Updates the view state from plain data.
    pub fn set_view(
        &mut self,
        cx: &mut Cx,
        langs: &[LangChip],
        groups: &[GridGroup],
        missing_only: bool,
        missing: usize,
        files: &str,
    ) {
        self.langs = langs.to_vec();
        self.groups = groups.to_vec();

        // 1. Header files line
        self.view.label(id!(lbl_files)).set_text(cx, files);

        // 2. Language chips
        let chip_ids = [
            id!(chip_lang_0),
            id!(chip_lang_1),
            id!(chip_lang_2),
            id!(chip_lang_3),
        ];
        for (i, &chip_id) in chip_ids.iter().enumerate() {
            let chip_view = self.view.view(chip_id);
            if let Some(chip) = langs.get(i) {
                chip_view.set_visible(cx, true);
                chip_view
                    .label(id!(lbl_name))
                    .set_text(cx, &format!("{} ", chip.name));
                let count_lbl = chip_view.label(id!(lbl_count));
                count_lbl.set_text(cx, &format!("{}/{}", chip.done, chip.total));
                if chip.done < chip.total {
                    count_lbl.apply_over(
                        cx,
                        live! { draw_text: { color: (vec4(0.910, 0.569, 0.290, 1.0)) } },
                    );
                } else {
                    count_lbl.apply_over(
                        cx,
                        live! { draw_text: { color: (vec4(0.561, 0.749, 0.498, 1.0)) } },
                    );
                }
            } else {
                chip_view.set_visible(cx, false);
            }
        }

        // 3. Missing only toggle
        self.view
            .label(id!(lbl_missing))
            .set_text(cx, &format!("Missing only ({missing})"));
        let checked_f = if missing_only { 1.0 } else { 0.0 };
        self.view
            .view(id!(tick_box))
            .apply_over(cx, live! { draw_bg: { checked: (checked_f) } });

        // 4. Column header row
        let header_ids = [
            id!(header_lang_0),
            id!(header_lang_1),
            id!(header_lang_2),
            id!(header_lang_3),
        ];
        for (i, &header_id) in header_ids.iter().enumerate() {
            let col_view = self.view.view(header_id);
            if let Some(chip) = langs.get(i) {
                col_view.set_visible(cx, true);
                col_view.apply_over(cx, live! { width: Fill });
                col_view.label(id!(lbl)).set_text(cx, &chip.name);
            } else {
                col_view.set_visible(cx, false);
                col_view.apply_over(cx, live! { width: 0.0 });
            }
        }

        // 5. Flatten groups for PortalList
        self.flat_items.clear();
        for (g_idx, group) in groups.iter().enumerate() {
            self.flat_items.push(FlatItem::GroupHeader {
                form: group.form.clone(),
                count: group.rows.len(),
            });
            for (r_idx, row) in group.rows.iter().enumerate() {
                self.flat_items.push(FlatItem::DataRow {
                    group_idx: g_idx,
                    row_idx: r_idx,
                    key: row.key.clone(),
                });
            }
        }

        self.view.portal_list(id!(grid_list)).redraw(cx);
        self.view.redraw(cx);
    }

    /// Inspects UI gestures and returns any triggered action.
    pub fn action(&mut self, cx: &mut Cx, actions: &Actions) -> Option<TranslationsAction> {
        if self.view.button(id!(btn_missing_only)).clicked(actions) {
            return Some(TranslationsAction::ToggleMissingOnly);
        }

        if self.view.button(id!(btn_add_lang)).clicked(actions) {
            return Some(TranslationsAction::AddLanguage);
        }

        let cell_ids = [id!(cell_0), id!(cell_1), id!(cell_2), id!(cell_3)];
        let list = self.view.portal_list(id!(grid_list));
        for (item_idx, item) in list.items_with_actions(actions) {
            let Some(FlatItem::DataRow {
                group_idx,
                row_idx,
                key,
            }) = self.flat_items.get(item_idx).cloned()
            else {
                continue;
            };
            let Some(row) = self.groups.get(group_idx).and_then(|g| g.rows.get(row_idx)) else {
                continue;
            };

            for (l, cell_id) in cell_ids.iter().enumerate().take(self.langs.len()) {
                let slot = item.view(*cell_id);
                if self.selected_cell.as_ref() == Some(&(key.clone(), l)) {
                    let input = slot.text_input(id!(input_text));
                    if input.returned(actions).is_some() {
                        let text = input.text();
                        self.selected_cell = None;
                        self.view.redraw(cx);
                        return Some(TranslationsAction::Commit {
                            key: key.clone(),
                            lang: l,
                            text,
                        });
                    }
                    if input.escaped(actions) {
                        self.selected_cell = None;
                        self.view.redraw(cx);
                        return None;
                    }
                } else if slot.button(id!(btn_cell)).clicked(actions) {
                    let initial_text = row.cells.get(l).map(cell_text).unwrap_or_default();
                    self.selected_cell = Some((key.clone(), l));
                    self.edit_seed = Some(initial_text.to_string());
                    self.view.redraw(cx);
                    return None;
                }
            }
        }

        None
    }

    /// Toggles the add language card visibility.
    pub fn toggle_add_card(&mut self, cx: &mut Cx) {
        let is_visible = self.view.view(id!(add_card_backdrop)).visible();
        self.set_add_card_visible(cx, !is_visible);
    }

    /// Sets the add language card visibility.
    pub fn set_add_card_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.view
            .view(id!(add_card_backdrop))
            .set_visible(cx, visible);
        if visible {
            self.clear_add_error(cx);
            self.view.text_input(id!(input_code)).set_text(cx, "");
        }
        self.view.redraw(cx);
    }

    /// Displays an error message inside the add language card.
    pub fn set_add_error(&mut self, cx: &mut Cx, error: &str) {
        self.view.view(id!(add_error_box)).set_visible(cx, true);
        self.view.label(id!(lbl_add_error)).set_text(cx, error);
        self.view.redraw(cx);
    }

    /// Clears any error message inside the add language card.
    pub fn clear_add_error(&mut self, cx: &mut Cx) {
        self.view.view(id!(add_error_box)).set_visible(cx, false);
        self.view.label(id!(lbl_add_error)).set_text(cx, "");
        self.view.redraw(cx);
    }

    /// Reads the code typed inside the add language card.
    pub fn add_card_code(&self) -> String {
        self.view
            .text_input(id!(input_code))
            .text()
            .trim()
            .to_string()
    }

    /// Checks if the Add button was clicked inside the add language card.
    pub fn add_card_add_clicked(&self, actions: &Actions) -> bool {
        self.view.button(id!(btn_card_add)).clicked(actions)
            || self
                .view
                .text_input(id!(input_code))
                .returned(actions)
                .is_some()
    }

    /// Checks if the Cancel button was clicked inside the add language card.
    pub fn add_card_cancel_clicked(&self, actions: &Actions) -> bool {
        self.view.button(id!(btn_card_cancel)).clicked(actions)
            || self.view.button(id!(btn_dismiss_add_card)).clicked(actions)
            || self.view.text_input(id!(input_code)).escaped(actions)
    }
}

impl TranslationsViewRef {
    pub fn action(&self, cx: &mut Cx, actions: &Actions) -> Option<TranslationsAction> {
        self.borrow_mut()
            .and_then(|mut inner| inner.action(cx, actions))
    }

    pub fn toggle_add_card(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle_add_card(cx);
        }
    }

    pub fn set_add_card_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_add_card_visible(cx, visible);
        }
    }

    pub fn set_add_error(&self, cx: &mut Cx, error: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_add_error(cx, error);
        }
    }

    pub fn clear_add_error(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_add_error(cx);
        }
    }

    pub fn add_card_code(&self) -> String {
        self.borrow()
            .map(|inner| inner.add_card_code())
            .unwrap_or_default()
    }

    pub fn add_card_add_clicked(&self, actions: &Actions) -> bool {
        self.borrow()
            .is_some_and(|inner| inner.add_card_add_clicked(actions))
    }

    pub fn add_card_cancel_clicked(&self, actions: &Actions) -> bool {
        self.borrow()
            .is_some_and(|inner| inner.add_card_cancel_clicked(actions))
    }

    pub fn set_view(
        &self,
        cx: &mut Cx,
        langs: &[LangChip],
        groups: &[GridGroup],
        missing_only: bool,
        missing: usize,
        files: &str,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_view(cx, langs, groups, missing_only, missing, files);
        }
    }
}

#[cfg(test)]
#[path = "test/translations_view/test.rs"]
mod tests;
