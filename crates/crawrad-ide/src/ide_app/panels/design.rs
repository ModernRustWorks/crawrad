use makepad_widgets::*;

live_design! {
    link crawrad_panels;

    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use link::crawrad_minimap::*;
    use link::crawrad::*;
    use makepad_code_editor::action_bulb_view::*;
    use makepad_code_editor::code_view::CodeView;
    // The central Form / Designer / Code Editor panel as an independent floating card.
    pub CrawradPanelDesign = <CrawradPanel> {
        width: Fill, height: Fill,
        flow: Down,

        // Rulers are drawn directly by the canvas; open file tabs placed on top.
        editor_tabs_bar = <View> {
            visible: false,
            width: Fill, height: (CRAWRAD_ABA_ALTURA),
            flow: Right, align: { y: 0.5 },
            margin: { left: 1.0, right: 1.0, top: 1.0 },
            padding: { left: 0.0, right: 4.0 },
            show_bg: true,
            draw_bg: {
                color: (CRAWRAD_ELEVADO)
                uniform border_radius: 5.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    // Double height pushes the bottom rounded corners out of view.
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y * 2.0, self.border_radius);
                    sdf.fill(self.color);
                    return sdf.result;
                }
            }
            editor_tabs = <PortalList> {
                width: Fill, height: Fill,
                flow: Right,
                scroll_bar: <CrawradTabsScrollBar> {}
                EditorTab = <CrawradEditorTab> {}
            }
            editor_tabs_actions = <View> {
                width: Fit, height: Fill,
                flow: Right, align: { y: 0.5 },
                spacing: 2.0,

                btn_editor_ai = <CrawradActionIcon> {}
                btn_editor_split = <CrawradActionIcon> {}
                btn_editor_more = <CrawradActionIcon> {}
            }
        }
        sep_editor_tabs = <CrawradSeparatorH> {
            visible: false,
        }

        // THE DESIGNER TOP BAR — rounded top frame and designer options.
        designer_options_bar = <View> {
            visible: true,
            width: Fill, height: (CRAWRAD_ABA_ALTURA),
            flow: Right, align: { y: 0.5 },
            padding: { left: 12.0, right: 12.0 },
            spacing: (CRAWRAD_ESPACO_S),

            lbl_designer_form = <CrawradLabel> {
                draw_text: {
                    color: (CRAWRAD_ACENTO_TEXTO)
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
                text: "📐 Form: form_main"
            }
            lbl_web_page = <RoundedView> {
                visible: false, width: Fit, height: 18.0,
                padding: { left: 6.0, right: 6.0 }, align: { y: 0.5 },
                draw_bg: { color: (CRAWRAD_WEB_BADGE_FILL), border_radius: 3.0 }
                <Label> { text: "WEB PAGE", draw_text: { color: (CRAWRAD_WEB_BADGE_TEXT), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
            }
            des_top_sep1 = <CrawradSeparatorV> { height: 14.0, margin: { left: 4.0, right: 4.0 } }
            btn_designer_grid = <CrawradButton> {
                height: 20.0,
                padding: { left: 8.0, right: 8.0 },
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Grid: ON"
            }
            btn_designer_snap = <CrawradButton> {
                height: 20.0,
                padding: { left: 8.0, right: 8.0 },
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Snap: ON"
            }
            btn_designer_tab_order = <CrawradButton> {
                height: 20.0,
                padding: { left: 8.0, right: 8.0 },
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Tab order: ON"
            }
            lbl_grid_title = <CrawradLabelMuted> {
                margin: { left: 6.0 },
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Grid size:"
            }
            btn_grid_out = <CrawradButton> {
                width: 22.0, height: 20.0, padding: 0.,
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 11.0 } },
                text: "-"
            }
            lbl_grid_val = <CrawradLabel> {
                width: 26.0, align: { x: 0.5 },
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "8"
            }
            btn_grid_in = <CrawradButton> {
                width: 22.0, height: 20.0, padding: 0.,
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 11.0 } },
                text: "+"
            }
            des_top_spacer = <CrawradFiller> {}
            lbl_zoom_title = <CrawradLabelMuted> {
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Zoom:"
            }
            btn_zoom_out = <CrawradButton> {
                width: 22.0, height: 20.0,
                padding: 0.,
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 11.0 } },
                text: "-"
            }
            btn_zoom_val = <CrawradButton> {
                height: 20.0,
                padding: { left: 6.0, right: 6.0 },
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "100%"
            }
            btn_zoom_in = <CrawradButton> {
                width: 22.0, height: 20.0,
                padding: 0.,
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 11.0 } },
                text: "+"
            }
            btn_zoom_fit = <CrawradButton> {
                width: 26.0, height: 20.0,
                padding: 0.,
                draw_text: { text_style: { font_size: 10.0 } },
                text: "Fit"
            }
        }
        sep_designer_options = <CrawradSeparatorH> {
            visible: true,
        }
        banner_language = <View> {
            visible: false,
            width: Fill,
            height: 26.0,
            flow: Right,
            align: { y: 0.5 },
            padding: { left: 12.0, right: 12.0 },
            show_bg: true,
            draw_bg: { color: (CRAWRAD_ELEVADO) }

            lbl_banner_language = <CrawradLabel> {
                width: Fill,
                height: Fit,
                text: "",
                draw_text: {
                    color: (CRAWRAD_AVISO),
                    text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
            }
        }
        designer_row = <View> {
            width: Fill, height: Fill,
            flow: Right,

            // Outer view toggled by toggle_view to hide/show the designer canvas.
            designer_canvas = <View> {
                width: Fill, height: Fill,
                flow: Overlay,

                canvas = <DesignCanvas> {}
            }

            // Code editor container for F12 / Code tab
            code_container = <View> {
                visible: false,
                width: Fill, height: Fill,
                flow: Overlay,
                padding: 0.0,

                editor_stack = <View> {
                    width: Fill, height: Fill,
                    flow: Down,

                    diff_review_bar = <View> {
                        visible: false,
                        width: Fill, height: 32.0,
                        flow: Right, align: { y: 0.5 },
                        padding: { left: 12.0, right: 12.0 },
                        spacing: 8.0,
                        show_bg: true,
                        draw_bg: { color: (CRAWRAD_ELEVADO) }
                        lbl_diff_badge = <CrawradLabel> {
                            width: Fit, height: Fit,
                            text: "DIFF REVIEW"
                            draw_text: {
                                color: (CRAWRAD_AVISO)
                                text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                            }
                        }
                        lbl_diff_info = <CrawradLabel> {
                            width: Fit, height: Fit,
                            text: "",
                            draw_text: {
                                color: (CRAWRAD_TEXTO_PRIMARIO)
                                text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                            }
                        }
                        lbl_diff_added = <CrawradLabel> { width: Fit, height: Fit, text: "", draw_text: { color: (CRAWRAD_OK), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                        lbl_diff_removed = <CrawradLabel> { width: Fit, height: Fit, text: "", draw_text: { color: (CRAWRAD_ERRO), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                        lbl_diff_changes = <CrawradLabel> { width: Fit, height: Fit, text: "", draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                        <View> { width: Fill, height: Fit }
                        btn_diff_prev = <CrawradButton> { width: 28.0, height: 24.0, text: "↑" }
                        lbl_diff_count = <CrawradLabel> { width: Fit, height: Fit, text: "", draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                        btn_diff_next = <CrawradButton> { width: 28.0, height: 24.0, text: "↓" }
                        btn_diff_accept = <CrawradPrimaryButton> { width: Fit, height: 24.0, padding: { left: 10.0, right: 10.0 }, text: "Accept all (Tab)" }
                        btn_diff_reject = <CrawradButton> { width: Fit, height: 24.0, padding: { left: 10.0, right: 10.0 }, text: "Reject all (Esc)" }
                    }
                    disk_change_bar = <View> {
                        visible: false,
                        width: Fill, height: 32.0, flow: Right, align: { y: 0.5 },
                        padding: { left: 12.0, right: 12.0 }, spacing: 8.0,
                        show_bg: true, draw_bg: { color: (CRAWRAD_ELEVADO) }
                        lbl_disk_change = <CrawradLabel> { width: Fit, height: Fit, text: "", draw_text: { color: (CRAWRAD_AVISO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                        <View> { width: Fill, height: Fit }
                        btn_disk_keep = <CrawradPrimaryButton> { width: 110.0, height: 24.0, text: "Keep mine" }
                        btn_disk_reload = <CrawradButton> { width: 130.0, height: 24.0, text: "Reload from disk" }
                        btn_disk_compare = <CrawradButton> { width: 90.0, height: 24.0, text: "Compare" }
                    }
                    editor_body = <View> {
                        width: Fill, height: Fill,
                        flow: Right,

                        code_view = <CodeView> {
                            editor: {
                                width: Fill, height: Fill,
                                pad_left_top: vec2(0.0, 0.0),
                                glyph_margin: 28.0,
                                gutter_pad: 4,
                                show_gutter: true,
                                draw_bg: { color: (CRAWRAD_PAINEL) }
                            }
                        }
                        editor_minimap = <CrawradMinimap> { draw_bg: { color: (CRAWRAD_PAINEL) } draw_border: { color: (CRAWRAD_DIVISOR) } }
                    }

                }

                // Floating Find & Replace (Ctrl+F / Ctrl+H)
                find_box = <View> {
                    visible: false,
                    width: Fit, height: Fit,
                    flow: Down,
                    align: { x: 1.0, y: 0.0 },
                    margin: { right: 24.0, top: 8.0 },
                    padding: 6.0,
                    show_bg: true,
                    draw_bg: {
                        color: (CRAWRAD_FLUTUANTE)
                    }

                    find_row = <View> {
                        width: Fit, height: Fit,
                        flow: Right, align: { y: 0.5 },
                        spacing: 4.0,

                        btn_toggle_replace = <CrawradButton> {
                            width: 22.0, height: 22.0,
                            padding: 0.,
                            text: ">"
                        }
                        find_input = <CrawradTextInput> {
                            width: 160.0, height: 22.0,
                            empty_text: "Find..."
                        }
                        find_count = <CrawradLabelMuted> {
                            width: 60.0,
                            text: "0/0"
                        }
                        btn_find_prev = <CrawradButton> {
                            width: 24.0, height: 22.0,
                            padding: 0.,
                            text: "<"
                        }
                        btn_find_next = <CrawradButton> {
                            width: 24.0, height: 22.0,
                            padding: 0.,
                            text: ">"
                        }
                        btn_find_close = <CrawradButton> {
                            width: 22.0, height: 22.0,
                            padding: 0.,
                            text: "X"
                        }
                    }

                    replace_row = <View> {
                        visible: false,
                        width: Fit, height: Fit,
                        flow: Right, align: { y: 0.5 },
                        spacing: 4.0,
                        margin: { top: 4.0 },

                        <View> { width: 22.0, height: 22.0 }
                        replace_input = <CrawradTextInput> {
                            width: 160.0, height: 22.0,
                            empty_text: "Replace..."
                        }
                        btn_replace = <CrawradButton> {
                            height: 22.0,
                            text: "Replace"
                        }
                        btn_replace_all = <CrawradButton> {
                            height: 22.0,
                            text: "All"
                        }
                    }
                }

                // Floating Autocomplete / IntelliSense in classic Code Insight / VS Code style
                completion_box = <CrawradCompletionBox> {
                    margin: { left: 40.0, top: 40.0 },
                    flow: Right,
                    comp_rows = <View> {
                        width: Fill, height: Fit, flow: Down,
                        comp_item_0 = <CrawradCompletionItem> {}
                        comp_item_1 = <CrawradCompletionItem> {}
                        comp_item_2 = <CrawradCompletionItem> {}
                        comp_item_3 = <CrawradCompletionItem> {}
                        comp_item_4 = <CrawradCompletionItem> {}
                        comp_item_5 = <CrawradCompletionItem> {}
                        comp_item_6 = <CrawradCompletionItem> {}
                        comp_item_7 = <CrawradCompletionItem> {}
                        comp_item_8 = <CrawradCompletionItem> {}
                        comp_item_9 = <CrawradCompletionItem> {}
                    }
                    comp_track = <View> { width: 6.0, height: Fit, margin: { left: 4.0 },
                        comp_thumb = <RoundedView> { width: Fill, height: 0.0, draw_bg: { color: (CRAWRAD_BORDA_CONTROLE), border_radius: 3.0 } }
                    }
                }

                // The hover card: rust-analyzer's answer, selectable; over a
                // noted item, the note as rustdoc shows it and where to open it.
                hover_box = <CrawradCompletionBox> {
                    width: 520.0,
                    hover_plain = <View> { width: Fill, height: Fit,
                        hover_text = <CrawradTextInput> {
                            height: Fit, flow: RightWrap, is_read_only: true,
                            draw_text: { text_style: <THEME_FONT_CODE> {} }
                        }
                    }
                    hover_rich = <ScrollYView> { visible: false, width: Fill, height: 200.0,
                        hover_md = <Markdown> { width: Fill, height: Fit, body: "" }
                    }
                    hover_note_row = <View> { visible: false, width: Fill, height: Fit, flow: Right, align: { y: 0.5 },
                        hover_note_kind = <CrawradLabelMuted> { text: "" }
                        <View> { width: Fill, height: Fit }
                        hover_open_note = <CrawradButton> { text: "" }
                    }
                }

                // The selected completion item's documentation.
                completion_doc = <CrawradCompletionBox> {
                    width: 420.0,
                    doc_text = <CrawradTextInput> {
                        height: Fit, flow: RightWrap, is_read_only: true,
                        draw_text: { text_style: <THEME_FONT_CODE> {} }
                    }
                }

                code_bulb = <ActionBulb> {
                    visible: false, width: 20.0, height: 17.0,
                    align: { x: 0.5, y: 0.5 }, padding: 0.0,
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let c = self.rect_size * 0.5;
                            let d = length(self.pos * self.rect_size - c) / (self.rect_size.x * 0.5);
                            let strength = mix(0.18, 0.40, mix(self.pulse, 1.0, self.hover));
                            let a = strength * clamp(1.0 - d, 0.0, 1.0) * mix(0.6, 1.0, self.preferred);
                            return vec4(0.94, 0.48, 0.16, 1.0) * a;
                        }
                    }
                    crab = <Label> {
                        padding: 0.0, margin: 0.0,
                        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.5 } }
                        text: "🦀"
                    }
                }

                // The parameter card: the current parameter in Rust orange.
                signature_box = <CrawradCompletionBox> {
                    width: Fit, flow: Right, padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 4.0 },
                    sig_before = <CrawradLabel> { draw_text: { text_style: <THEME_FONT_CODE> {} } }
                    sig_active = <CrawradLabel> { draw_text: { color: (CRAWRAD_BOTAO_PRIMARIO_HOVER), text_style: <THEME_FONT_CODE> {} } }
                    sig_after = <CrawradLabel> { draw_text: { text_style: <THEME_FONT_CODE> {} } }
                }
            }

            history_container = <View> {
                visible: false,
                width: Fill, height: Fill,
                history_panel = <CrawradHistoryPanel> {}
            }

            welcome_container = <View> {
                visible: false,
                width: Fill, height: Fill,
                welcome_page = <CrawradWelcomePage> {}
            }

            sizes_container = <View> {
                visible: false,
                width: Fill, height: Fill,
                sizes_panel = <SizePreviewView> {}
            }

            translations_container = <View> {
                visible: false,
                width: Fill, height: Fill,
                translations_panel = <TranslationsView> {}
            }
        }

        // Rule at the editor foot: splits the content from the [Design] [Code] strip.
        sep_center_bottom = <CrawradSeparatorH> {}

        center_tabs_bar = <View> {
            width: Fill, height: 26.0,
            flow: Right, align: { y: 0.5 },
            padding: { top: 2.0, bottom: 2.0 },
            spacing: -1.0,
            center_tabs_spacer = <CrawradFiller> {}
            editor_breadcrumbs_bar = <View> {
                visible: false,
                width: Fill, height: Fill,
                margin: { right: (CRAWRAD_ESPACO_L) },
                editor_breadcrumb = <CrawradBreadcrumb> {
                    width: Fill, height: Fill,
                    style: "bsChevron",
                    segment_color: (CRAWRAD_CRUMB_SEGMENT),
                    current_color: (CRAWRAD_CRUMB_CURRENT),
                    chevron_text: (CRAWRAD_CRUMB_TEXT), chevron_current_text: (CRAWRAD_CRUMB_CURRENT_TEXT),
                    draw_text: { color: (CRAWRAD_CRUMB_TEXT), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                    draw_hover: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                    draw_current: { color: (CRAWRAD_CRUMB_CURRENT_TEXT), text_style: <CRAWRAD_FONT_BOLD_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                    draw_separator: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                }
            }
            tab_design = <CrawradCenterTab> { text: "" }
            tab_code = <CrawradCenterTab> { text: "" }
            tab_history = <CrawradCenterTab> { text: "History" }
            tab_sizes = <CrawradCenterTab> { text: "Sizes" }
            tab_translations = <CrawradCenterTab> { text: "Translations" }
        }
        // The line below: the caret's cells centred, both ends kept free (owner, 21/10).
        editor_footer_row = <View> {
            width: Fill, height: 22.0,
            flow: Right,
            align: { x: 0.5, y: 0.5 },
            margin: { bottom: 4.0 },
            editor_status_bar = <View> {
                visible: false,
                width: Fit, height: Fill,
                flow: Right, align: { y: 0.5 },
                show_bg: false,
                lbl_release = <CrawradStatusText> {
                    text: "",
                    margin: { right: (CRAWRAD_ESPACO_M) }
                }
                lbl_caret = <CrawradStatusText> {
                    text: "",
                    margin: { left: (CRAWRAD_ESPACO_M) },
                    // Monospace: a proportional font makes `1: 1` jitter at each key.
                    draw_text: { text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } }
                }
                lbl_mode = <CrawradStatusText> {
                    text: "",
                    margin: { left: (CRAWRAD_ESPACO_L) }
                }
                lbl_modified = <CrawradStatusText> {
                    text: "",
                    margin: { left: (CRAWRAD_ESPACO_L) }
                }
                lbl_spaces = <CrawradStatusText> {
                    text: "",
                    margin: { left: (CRAWRAD_ESPACO_L) }
                }
                lbl_encoding = <CrawradStatusText> {
                    text: "",
                    margin: { left: (CRAWRAD_ESPACO_L) }
                }
                lbl_language = <CrawradStatusText> {
                    text: "",
                    margin: { left: (CRAWRAD_ESPACO_L) }
                }
            }
        }
    }

}
