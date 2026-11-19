use makepad_widgets::*;

live_design! {
    link crawrad_dialogs;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use link::crawrad::*;
    // One setting of the Options: "Scope: **Name**", its hint, its editor.
    OptSetting = <View> { width: Fill, height: Fit, flow: Down, spacing: 2.0 }
    OptTitle = <View> {
        width: Fill, height: Fit, flow: Right, spacing: 4.0,
        scope = <CrawradLabel> {}
        name = <CrawradLabel> {
            draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: 9.75 } }
        }
    }
            pub CrawradOptionsDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_OPTIONS_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        opt_title = <CrawradDialogTitle> { text: "Options" }
                        <CrawradFiller> {}
                        opt_close = <CrawradCloseButton> {}
                    }
                    // THE TOP SEARCH BAR AND SCOPE (VS Code / Antigravity style)
                    opt_search_bar = <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        padding: { left: 16.0, right: 16.0, top: 10.0, bottom: 8.0 },
                        spacing: 8.0,
                        // Search field with clear button
                        <View> {
                            width: Fill, height: 30.0,
                            flow: Right, align: { y: 0.5 },
                            spacing: 6.0,
                            opt_search_input = <CrawradPropInput> {
                                width: Fill, height: 28.0,
                                empty_text: "Search settings (e.g. font, save, grid, snap, ai, cargo)"
                            }
                            opt_search_clear = <CrawradButton> {
                                width: 28.0, height: 28.0,
                                text: "×"
                            }
                        }
                        // Scope tabs: User | Workspace
                        opt_scope_row = <View> {
                            width: Fill, height: Fit,
                            flow: Right, align: { y: 0.5 },
                            spacing: 8.0,
                            opt_scope_user = <CrawradInspectorTab> { text: "User" }
                            opt_scope_workspace = <CrawradInspectorTab> { text: "Workspace" }
                            <CrawradFiller> {}
                            opt_sync_status = <CrawradLabelMuted> {
                                draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
                                text: "Preferences synced"
                            }
                        }
                    }
                    sep_search = <CrawradSeparatorH> {}
                    <CrawradDialogBody> {
                        height: (CRAWRAD_OPTIONS_CORPO),
                        flow: Right,
                        spacing: (CRAWRAD_ESPACO_L)
                        <View> {
                            width: (CRAWRAD_OPTIONS_ARVORE), height: Fill,
                            flow: Down,
                            <CrawradGroupBoxFrame> {
                                opt_tree = <PortalList> {
                                    width: Fill, height: Fill,
                                    scroll_bar: <CrawradScrollBar> {}
                                    OptGrupo = <CrawradCategoryRow> {}
                                    OptItem = <CrawradItemsRow> {}
                                }
                            }
                        }
                        <View> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: (CRAWRAD_ESPACO_M)
                            opt_pagina = <CrawradLabel> {
                                width: Fill, height: Fit,
                                draw_text: {
                                    color: (CRAWRAD_TEXTO_ENFASE)
                                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_TITULO) }
                                }
                                text: ""
                            }
                            // THE COMMONLY USED PAGE (VS CODE STYLE)
                            opt_commonly_used = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_M),
                                // Editor: Font Size
                                <OptSetting> {
                                    <OptTitle> { scope = { text: "Editor: " } name = { text: "Font Size" } }
                                    <CrawradLabelMuted> { text: "Controls the font size in pixels." }
                                    opt_cu_font_size = <CrawradOptionDrop> { width: 160.0 }
                                }
                                // Editor: Line Numbers
                                <OptSetting> {
                                    <OptTitle> { scope = { text: "Editor: " } name = { text: "Line Numbers" } }
                                    <CrawradLabelMuted> { text: "Controls the display of line numbers in the editor margin." }
                                    opt_cu_line_numbers = <CrawradOptionCheck> {}
                                }
                                // Files: Save before Build
                                <OptSetting> {
                                    <OptTitle> { scope = { text: "Files: " } name = { text: "Save before Build" } }
                                    <CrawradLabelMuted> { text: "Saves every open editor file before Build or Run." }
                                    opt_cu_save_build = <CrawradOptionCheck> {}
                                }
                                // Designer: Show Grid & Snap
                                <OptSetting> {
                                    <OptTitle> { scope = { text: "Designer: " } name = { text: "Grid & Snap" } }
                                    <CrawradLabelMuted> { text: "Show dot grid and snap components to grid." }
                                    <View> {
                                        width: Fill, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_L),
                                        opt_cu_ver_grade = <CrawradOptionCheck> {}
                                        opt_cu_snap = <CrawradOptionCheck> {}
                                    }
                                }
                                // AI Assistant: Provider
                                <OptSetting> {
                                    <OptTitle> { scope = { text: "AI Assistant: " } name = { text: "Provider" } }
                                    <CrawradLabelMuted> { text: "AI Agent Client Protocol provider service." }
                                    opt_cu_ai_provider = <CrawradOptionDrop> {}
                                }
                            }
                            // THE SEARCH RESULTS PAGE
                            opt_search_results = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_M),
                                opt_sr_empty_wrap = <View> {
                                    visible: false,
                                    width: Fill, height: Fit,
                                    opt_sr_empty = <CrawradLabelMuted> {
                                        width: Fill, height: Fit,
                                        text: "No settings found matching your search query."
                                    }
                                }
                                opt_sr_editor_font = <View> {
                                    visible: false, width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                    <CrawradLabel> { text: "Editor: Font Size" }
                                    <CrawradLabelMuted> { text: "Controls the font size in pixels." }
                                    opt_sr_font_size = <CrawradOptionDrop> { width: 160.0 }
                                }
                                opt_sr_editor_line_numbers = <View> {
                                    visible: false, width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                    <CrawradLabel> { text: "Editor: Line Numbers" }
                                    <CrawradLabelMuted> { text: "Controls display of line numbers in editor margin." }
                                    opt_sr_line_numbers = <CrawradOptionCheck> {}
                                }
                                opt_sr_saving = <View> {
                                    visible: false, width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                    <CrawradLabel> { text: "Files: Save before Build" }
                                    <CrawradLabelMuted> { text: "Saves every open editor file before Build or Run." }
                                    opt_sr_save_build = <CrawradOptionCheck> {}
                                }
                                opt_sr_grid = <View> {
                                    visible: false, width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                    <CrawradLabel> { text: "Designer: Grid & Snap" }
                                    <CrawradLabelMuted> { text: "Controls grid visibility and snapping on designer." }
                                    <View> {
                                        width: Fill, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_L),
                                        opt_sr_ver_grade = <CrawradOptionCheck> {}
                                        opt_sr_snap = <CrawradOptionCheck> {}
                                    }
                                }

                                opt_sr_ai = <View> {
                                    visible: false, width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                    <CrawradLabel> { text: "AI Assistant: Provider & ACP" }
                                    <CrawradLabelMuted> { text: "AI Assistant agent protocol provider." }
                                    opt_sr_ai_provider = <CrawradOptionDrop> {}
                                }

                                opt_sr_keymap = <View> {
                                    visible: false, width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                    <CrawradLabel> { text: "Keyboard: Scheme" }
                                    <CrawradLabelMuted> { text: "Keyboard shortcut scheme (Modern or Classic)." }
                                    opt_sr_keymap_scheme = <CrawradOptionDrop> {}
                                }
                            }

                            opt_designer = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_M)
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: (CRAWRAD_ESPACO_L)
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: (CRAWRAD_ESPACO_S)
                                        opt_grade_label = <CrawradGroupBoxTitle> { text: "Grid options" }
                                        opt_ver_grade = <CrawradOptionCheck> {}
                                        opt_snap = <CrawradOptionCheck> {}
                                    }
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: (CRAWRAD_ESPACO_S)
                                        opt_tamanho_label = <CrawradGroupBoxTitle> { text: "Grid size / snap tolerance" }
                                        <CrawradFieldRow> {
                                            opt_grade_x = <CrawradPropInput> {}
                                            opt_grade_menos = <CrawradButton> { text: "-" }
                                            opt_grade_mais = <CrawradButton> { text: "+" }
                                        }
                                        opt_grade_nota = <CrawradLabelMuted> {
                                            width: Fill, height: Fit,
                                            text: ""
                                        }
                                    }
                                }
                            }

                            // THE APPEARANCE PAGE — both dimmed.
                            opt_appearance = <View> {
                                visible: false,
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_S)
                                opt_zoom_label = <CrawradGroupBoxTitle> { text: "Zoom" }
                                opt_zoom_nota = <CrawradLabelMuted> {
                                    width: Fill, height: Fit,
                                    text: ""
                                }
                                opt_tema_label = <CrawradGroupBoxTitle> { text: "Theme" }
                                opt_tema_nota = <CrawradLabelMuted> {
                                    width: Fill, height: Fit,
                                    text: ""
                                }
                                opt_icones = <CrawradButton> {
                                    width: Fit,
                                    margin: { top: (CRAWRAD_ESPACO_M) }
                                    text: ""
                                }
                            }

                            // THE KEYBOARD SCHEME AND SHORTCUTS PAGE.
                            opt_keyboard = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_S),

                                opt_keymap_label = <CrawradGroupBoxTitle> { text: "Keyboard Shortcuts & Keymap Scheme" }
                                opt_keymap_nota = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Select active keybindings preset. CrawRAD defaults to Modern; Classic is the traditional RAD keymap." }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: (CRAWRAD_ESPACO_XS),
                                    opt_keymap_scheme_lbl = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Keymap Scheme Preset:" }
                                    opt_keymap_scheme = <CrawradOptionDrop> {}
                                }

                                opt_keymap_summary = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }

                                <CrawradGroupBoxFrame> {
                                    width: Fill, height: Fill,
                                    opt_keymap_list = <PortalList> {
                                        width: Fill, height: Fill,
                                        scroll_bar: <CrawradScrollBar> {}
                                        KeyRow = <View> {
                                            width: Fill, height: 24.0,
                                            flow: Right, align: { y: 0.5 },
                                            padding: { left: 8.0, right: (CRAWRAD_ESPACO_L) },
                                            key_cmd = <CrawradLabel> {
                                                width: Fill, height: Fit,
                                                draw_text: {
                                                    color: (CRAWRAD_TEXTO_PRIMARIO),
                                                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                                                }
                                                text: ""
                                            }
                                            key_binding = <CrawradLabel> {
                                                width: Fit, height: Fit,
                                                draw_text: {
                                                    color: (CRAWRAD_ACENTO_TEXTO),
                                                    text_style: <THEME_FONT_BOLD> { font_size: 9.0 }
                                                }
                                                text: ""
                                            }
                                        }
                                    }
                                }
                            }

                            // THE AI ASSISTANT & ACP (Agent Client Protocol) PAGE
                            opt_ai = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: 8.0,

                                opt_ai_main = <View> { visible: true, width: Fill, height: Fit, flow: Down, spacing: 8.0,
                                    <CrawradGroupBoxTitle> { text: "Agent in use" }
                                    <RoundedView> { width: Fill, height: Fit, flow: Down, padding: 12.0, spacing: 8.0, draw_bg: { color: (CRAWRAD_ACP_CARD_BG), border_size: 1.0, border_radius: 6.0, border_color: (CRAWRAD_ACP_CARD_BORDA) }
                                        <View> { width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                            opt_ai_active_name = <CrawradLabel> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: 10.5 } }, text: "" }
                                            opt_ai_active_ver = <CrawradLabelMuted> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }, text: "" }
                                            <CrawradFiller> {}
                                            opt_ai_dot_ready = <RoundedView> { visible: false, width: 9.0, height: 9.0, draw_bg: { color: (CRAWRAD_CHIP_ACP), border_radius: 4.5 } }
                                            opt_ai_dot_signed_out = <RoundedView> { visible: false, width: 9.0, height: 9.0, draw_bg: { color: (CRAWRAD_AVISO), border_radius: 4.5 } }
                                            opt_ai_dot_missing = <RoundedView> { visible: false, width: 9.0, height: 9.0, draw_bg: { color: (CRAWRAD_ERRO), border_radius: 4.5 } }
                                            opt_ai_status_text = <CrawradLabel> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_REGULAR> { font_size: 9.5 } }, text: "" }
                                            opt_ai_fix_btn = <CrawradPrimaryButton> { visible: false, width: Fit, height: 24.0, text: "" }
                                        }
                                        <View> { width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                            opt_ai_status_hint = <CrawradLabelMuted> { width: Fill, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
                                            opt_ai_change_btn = <CrawradButton> { width: Fit, height: 24.0, text: "Change agent…" }
                                        }
                                        opt_ai_adv_btn = <CrawradButton> { width: Fit, height: 22.0, text: "Show advanced" }
                                        opt_ai_adv = <View> { visible: false, width: Fill, height: Fit, flow: Down, spacing: 6.0,
                                            <View> { width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                                <CrawradLabelMuted> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "Command" }
                                                opt_ai_active_cmd = <CrawradLabelMuted> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_CODE> { font_size: 8.5 } }, text: "" }
                                            }
                                            <View> { width: Fill, height: 26.0, flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                                opt_ai_test_btn = <CrawradButton> { width: Fit, height: 26.0, text: "Test connection" }
                                                opt_ai_term_btn = <CrawradButton> { width: Fit, height: 26.0, text: "Open terminal" }
                                            }
                                            opt_ai_status_detail = <CrawradLabelMuted> { width: Fill, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_CODE> { font_size: 8.5 } }, text: "" }
                                        }
                                    }
                                    <CrawradGroupBoxTitle> { text: "Defaults for new chats" }
                                    <View> { width: Fill, height: 26.0, flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                        <CrawradLabel> { width: 120.0, height: Fit, text: "Model" }
                                        opt_ai_model = <CrawradOptionDrop> { width: Fill }
                                    }
                                    opt_ai_effort_row = <View> { visible: false, width: Fill, height: 26.0, flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                        <CrawradLabel> { width: 120.0, height: Fit, text: "Effort" }
                                        opt_ai_effort = <CrawradOptionDrop> { width: Fill }
                                    }
                                    <CrawradLabelMuted> { width: Fill, height: Fit, text: "The lists come from what the agent offers, the same as the chat buttons." }
                                    <CrawradGroupBoxTitle> { text: "Permissions" }
                                    opt_ai_auto_approve = <CrawradOptionCheck> { width: Fit }
                                    opt_ai_native_tools = <CrawradOptionCheck> { width: Fit }
                                }

                                opt_ai_catalogue = <View> {
                                    visible: false,
                                    width: Fill, height: Fill,
                                    flow: Down,
                                    spacing: 8.0,

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                        opt_ai_back_btn = <CrawradButton> { width: Fit, height: 26.0, text: "‹ AI Assistant" }
                                        <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: 13.0 } }, text: "Change agent" }
                                        <CrawradFiller> {}
                                        opt_reg_link = <CrawradButton> { width: Fit, height: 24.0, text: "" }
                                    }

                                    <View> {
                                        width: Fill, height: 30.0,
                                        flow: Right, align: { y: 0.5 },
                                        spacing: 6.0,
                                        opt_reg_search = <CrawradPropInput> { width: Fill, height: 28.0, empty_text: "Search agents" }
                                        opt_reg_clear = <CrawradButton> { width: 60.0, height: 28.0, text: "Clear" }
                                    }

                                    opt_reg_feedback = <CrawradLabelMuted> { width: Fill, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) }, text: "" }

                                    <CrawradGroupBoxFrame> {
                                        width: Fill, height: Fill,
                                        opt_reg_list = <PortalList> {
                                            width: Fill, height: Fill,
                                            scroll_bar: <CrawradScrollBar> {}
                                            RegRow = <View> {
                                                width: Fill, height: 114.0,
                                                flow: Down,
                                                padding: { left: 2.0, right: 6.0, top: 4.0, bottom: 4.0 },
                                                reg_card = <View> {
                                                    width: Fill, height: Fill,
                                                    flow: Down,
                                                    padding: { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 },
                                                    spacing: 5.0,
                                                    show_bg: true,
                                                    draw_bg: {
                                                        instance is_active: 0.0
                                                        color: (CRAWRAD_ACP_CARD_BG)
                                                        uniform border_default: (CRAWRAD_ACP_CARD_BORDA)
                                                        uniform border_active: (CRAWRAD_ACP_CARD_ACTIVE_BORDA)

                                                        fn pixel(self) -> vec4 {
                                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 6.0);
                                                            sdf.fill_keep(self.color);
                                                            let border_c = mix(self.border_default, self.border_active, self.is_active);
                                                            let border_w = mix(1.0, 1.5, self.is_active);
                                                            sdf.stroke(border_c, border_w);
                                                            return sdf.result;
                                                        }
                                                    }
                                                    <View> {
                                                        width: Fill, height: Fit,
                                                        flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                                        reg_agent_name = <CrawradLabel> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: 11.5 } }, text: "" }
                                                        reg_badge_box = <RoundedView> { width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_FLUTUANTE), border_size: 1.0, border_radius: 4.0, border_color: (CRAWRAD_DIVISOR) }, reg_agent_badge = <CrawradLabelMuted> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }, text: "" } }
                                                        reg_installed_box = <RoundedView> { visible: false, width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_ACP_INSTALLED_BG), border_size: 1.0, border_radius: 4.0, border_color: (CRAWRAD_ACP_INSTALLED_BORDA) }, reg_installed_lbl = <CrawradLabel> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_ACP_INSTALLED_TEXTO), text_style: <THEME_FONT_BOLD> { font_size: 8.5 } }, text: "Installed" } }
                                                        reg_active_box = <RoundedView> { visible: false, width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_ACP_ACTIVE_BG), border_size: 1.0, border_radius: 4.0, border_color: (CRAWRAD_ACP_ACTIVE_BORDA) }, reg_active_lbl = <CrawradLabel> { width: Fit, height: Fit, draw_text: { color: (CRAWRAD_ACP_ACTIVE_TEXTO), text_style: <THEME_FONT_BOLD> { font_size: 8.5 } }, text: "In use" } }
                                                        <CrawradFiller> {}
                                                        reg_agent_action = <CrawradPrimaryButton> { width: Fit, height: 24.0, text: "" }
                                                    }
                                                    reg_agent_desc = <CrawradLabelMuted> { width: Fill, height: Fit, draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
                                                    <CrawradFiller> {}
                                                    <View> {
                                                        width: Fill, height: Fit,
                                                        flow: Right, align: { y: 0.5 }, spacing: 5.0,
                                                        reg_tag_0 = <RoundedView> { visible: false, width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_ACP_TAG_BG), border_size: 1.0, border_radius: 3.0, border_color: (CRAWRAD_DIVISOR) }, reg_tag_0_lbl = <CrawradLabelMuted> { draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }, color: (CRAWRAD_TEXTO_TERCIARIO) }, text: "" } }
                                                        reg_tag_1 = <RoundedView> { visible: false, width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_ACP_TAG_BG), border_size: 1.0, border_radius: 3.0, border_color: (CRAWRAD_DIVISOR) }, reg_tag_1_lbl = <CrawradLabelMuted> { draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }, color: (CRAWRAD_TEXTO_TERCIARIO) }, text: "" } }
                                                        reg_tag_2 = <RoundedView> { visible: false, width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_ACP_TAG_BG), border_size: 1.0, border_radius: 3.0, border_color: (CRAWRAD_DIVISOR) }, reg_tag_2_lbl = <CrawradLabelMuted> { draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }, color: (CRAWRAD_TEXTO_TERCIARIO) }, text: "" } }
                                                        reg_tag_3 = <RoundedView> { visible: false, width: Fit, height: Fit, padding: { left: 6.0, right: 6.0, top: 1.0, bottom: 2.0 }, draw_bg: { color: (CRAWRAD_ACP_TAG_BG), border_size: 1.0, border_radius: 3.0, border_color: (CRAWRAD_DIVISOR) }, reg_tag_3_lbl = <CrawradLabelMuted> { draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }, color: (CRAWRAD_TEXTO_TERCIARIO) }, text: "" } }
                                                        <CrawradFiller> {}
                                                        reg_agent_repo = <CrawradButton> { width: Fit, height: 20.0, text: "" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                <View> {
                                    visible: false, width: 0.0, height: 0.0,
                                    opt_ai_provider = <CrawradOptionDrop> {}
                                    opt_ai_endpoint = <CrawradPropInput> {}
                                }
                            }

                            // INVISIBLE ALIAS FOR THE ACP AGENTS REGISTRY
                            opt_registry = <View> {
                                visible: false,
                                width: 0.0, height: 0.0,
                            }

                            // THE CODE EDITOR PAGE
                            opt_editor = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_S),

                                opt_editor_label = <CrawradGroupBoxTitle> { text: "Editor Settings" }
                                opt_editor_nota = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Configure typography, indentation, and margins for the code editor." }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: (CRAWRAD_ESPACO_XS),
                                    opt_editor_font_lbl = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Editor Font Size:" }
                                    opt_editor_font_size = <CrawradOptionDrop> { width: 160.0 }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: (CRAWRAD_ESPACO_XS),
                                    opt_editor_tab_lbl = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Tab Indentation Size:" }
                                    opt_editor_tab_size = <CrawradOptionDrop> { width: 160.0 }
                                }

                                opt_editor_line_numbers = <CrawradOptionCheck> {}
                                opt_editor_inlay_hints = <CrawradOptionCheck> {}
                                opt_editor_current_line = <CrawradOptionCheck> {}
                                opt_editor_breadcrumb_chevron = <CrawradOptionCheck> {}
                            }

                            // THE EDITOR COLOR SCHEME PAGE
                            opt_color_scheme = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_S),

                                opt_cs_label = <CrawradGroupBoxTitle> { text: "Editor Color Scheme & Syntax Highlighting" }
                                opt_cs_nota = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Select a modern syntax palette preset or customize individual token colors (#RRGGBB)." }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right, align: { y: 0.5 },
                                    spacing: (CRAWRAD_ESPACO_S),
                                    opt_cs_preset_lbl = <CrawradLabelMuted> { width: Fit, height: Fit, text: "Color Theme Preset:" }
                                    opt_cs_preset = <CrawradOptionDrop> { width: 220.0 }
                                    opt_cs_reset_btn = <CrawradButton> { width: 110.0, height: 26.0, text: "Reset Default" }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 6.0,

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: (CRAWRAD_ESPACO_M),

                                        // Column 1
                                        <View> {
                                            width: Fill, height: Fit,
                                            flow: Down,
                                            spacing: 6.0,

                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_kw_lbl = <CrawradOptionColorLabel> { text: "Keywords:" }
                                                opt_cs_kw_sw = <CrawradColorPatch> {} opt_cs_kw_val = <CrawradOptionColorInput> {} opt_cs_kw_more = <CrawradPropOpen> {} }
                                                opt_cs_kw_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_type_lbl = <CrawradOptionColorLabel> { text: "Types:" }
                                                opt_cs_type_sw = <CrawradColorPatch> {} opt_cs_type_val = <CrawradOptionColorInput> {} opt_cs_type_more = <CrawradPropOpen> {} }
                                                opt_cs_type_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_fn_lbl = <CrawradOptionColorLabel> { text: "Functions:" }
                                                opt_cs_fn_sw = <CrawradColorPatch> {} opt_cs_fn_val = <CrawradOptionColorInput> {} opt_cs_fn_more = <CrawradPropOpen> {} }
                                                opt_cs_fn_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_id_lbl = <CrawradOptionColorLabel> { text: "Identifiers:" }
                                                opt_cs_id_sw = <CrawradColorPatch> {} opt_cs_id_val = <CrawradOptionColorInput> {} opt_cs_id_more = <CrawradPropOpen> {} }
                                                opt_cs_id_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_num_lbl = <CrawradOptionColorLabel> { text: "Numbers:" }
                                                opt_cs_num_sw = <CrawradColorPatch> {} opt_cs_num_val = <CrawradOptionColorInput> {} opt_cs_num_more = <CrawradPropOpen> {} }
                                                opt_cs_num_err = <CrawradSchemeError> {} }
                                        }

                                        // Column 2
                                        <View> {
                                            width: Fill, height: Fit,
                                            flow: Down,
                                            spacing: 6.0,

                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_str_lbl = <CrawradOptionColorLabel> { text: "Strings:" }
                                                opt_cs_str_sw = <CrawradColorPatch> {} opt_cs_str_val = <CrawradOptionColorInput> {} opt_cs_str_more = <CrawradPropOpen> {} }
                                                opt_cs_str_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_com_lbl = <CrawradOptionColorLabel> { text: "Comments:" }
                                                opt_cs_com_sw = <CrawradColorPatch> {} opt_cs_com_val = <CrawradOptionColorInput> {} opt_cs_com_more = <CrawradPropOpen> {} }
                                                opt_cs_com_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_delim_lbl = <CrawradOptionColorLabel> { text: "Delimiters:" }
                                                opt_cs_delim_sw = <CrawradColorPatch> {} opt_cs_delim_val = <CrawradOptionColorInput> {} opt_cs_delim_more = <CrawradPropOpen> {} }
                                                opt_cs_delim_err = <CrawradSchemeError> {} }
                                            <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_bg_lbl = <CrawradOptionColorLabel> { text: "Background:" }
                                                opt_cs_bg_sw = <CrawradColorPatch> {} opt_cs_bg_val = <CrawradOptionColorInput> {} opt_cs_bg_more = <CrawradPropOpen> {} }
                                                opt_cs_bg_err = <CrawradSchemeError> {} }
                                        }
                                    }
                                }

                                // Diagnostics: the message and line number (text) and the band
                                // behind the line, per severity.
                                opt_cs_diag_title = <CrawradLabelMuted> { width: Fill, height: Fit, margin: { top: 8.0 }, text: "Diagnostics (message and line number · line band)" }
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: (CRAWRAD_ESPACO_M),
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 6.0,

                                        <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_diag_err_lbl = <CrawradOptionColorLabel> { text: "Error:" }
                                            opt_cs_diag_err_sw = <CrawradColorPatch> {} opt_cs_diag_err_val = <CrawradOptionColorInput> {} opt_cs_diag_err_more = <CrawradPropOpen> {} }
                                            opt_cs_diag_err_err = <CrawradSchemeError> {} }
                                        <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_diag_warn_lbl = <CrawradOptionColorLabel> { text: "Warning:" }
                                            opt_cs_diag_warn_sw = <CrawradColorPatch> {} opt_cs_diag_warn_val = <CrawradOptionColorInput> {} opt_cs_diag_warn_more = <CrawradPropOpen> {} }
                                            opt_cs_diag_warn_err = <CrawradSchemeError> {} }
                                        <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_diag_info_lbl = <CrawradOptionColorLabel> { text: "Info:" }
                                            opt_cs_diag_info_sw = <CrawradColorPatch> {} opt_cs_diag_info_val = <CrawradOptionColorInput> {} opt_cs_diag_info_more = <CrawradPropOpen> {} }
                                            opt_cs_diag_info_err = <CrawradSchemeError> {} }
                                    }
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 6.0,

                                        <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_diag_err_band_lbl = <CrawradOptionColorLabel> { text: "Error band:" }
                                            opt_cs_diag_err_band_sw = <CrawradColorPatch> {} opt_cs_diag_err_band_val = <CrawradOptionColorInput> {} opt_cs_diag_err_band_more = <CrawradPropOpen> {} }
                                            opt_cs_diag_err_band_err = <CrawradSchemeError> {} }
                                        <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_diag_warn_band_lbl = <CrawradOptionColorLabel> { text: "Warning band:" }
                                            opt_cs_diag_warn_band_sw = <CrawradColorPatch> {} opt_cs_diag_warn_band_val = <CrawradOptionColorInput> {} opt_cs_diag_warn_band_more = <CrawradPropOpen> {} }
                                            opt_cs_diag_warn_band_err = <CrawradSchemeError> {} }
                                        <CrawradSchemeField> { <CrawradOptionColorRow> { opt_cs_diag_info_band_lbl = <CrawradOptionColorLabel> { text: "Info band:" }
                                            opt_cs_diag_info_band_sw = <CrawradColorPatch> {} opt_cs_diag_info_band_val = <CrawradOptionColorInput> {} opt_cs_diag_info_band_more = <CrawradPropOpen> {} }
                                            opt_cs_diag_info_band_err = <CrawradSchemeError> {} }
                                    }
                                }

                                opt_cs_preview_lbl = <CrawradLabelMuted> { width: Fill, height: Fit, margin: { top: 8.0 }, text: "Sample" }
                                opt_cs_preview = <CrawradSchemePreview> {}
                                opt_cs_preview_note = <CrawradLabelMuted> { width: Fill, height: Fit, text: "The sample is painted with the colours above." }
                            }

                            // THE DEFAULT FOLDERS PAGE
                            opt_folders = <View> {
                                visible: false,
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_S),

                                opt_folders_label = <CrawradGroupBoxTitle> { text: "Default Workspace Folders" }
                                opt_folders_nota = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Specify default directories for creating and searching Cargo projects." }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: (CRAWRAD_ESPACO_XS),
                                    opt_default_dir_lbl = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Default Projects Directory:" }
                                    <View> {
                                        width: 520.0, height: 26.0,
                                        flow: Right, align: { y: 0.5 },
                                        spacing: (CRAWRAD_ESPACO_S),
                                        opt_default_dir_input = <CrawradPropInput> { width: Fill, height: 26.0 }
                                        opt_default_dir_btn = <CrawradButton> { width: 90.0, height: 26.0, text: "Browse..." }
                                    }
                                }
                            }

                            // The File History & Backups page
                            opt_history = <View> {
                                visible: false, width: Fill, height: Fill, flow: Down, spacing: (CRAWRAD_ESPACO_S),
                                <CrawradGroupBoxTitle> { text: "File History & Backups" }
                                <CrawradLabelMuted> { width: Fill, height: Fit, text: "A backup copy of the file is kept in .crawrad/history on every save." }
                                opt_history_enabled = <CrawradOptionCheck> { text: "Keep a backup copy of a file on every save" }
                                <View> { width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_XS),
                                    <CrawradLabelMuted> { width: Fill, height: Fit, text: "File backup limit:" }
                                    opt_history_limit = <CrawradOptionDrop> { width: 160.0 } }
                                <View> { width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_XS),
                                    <CrawradLabelMuted> { width: Fill, height: Fit, text: "Merge saves closer than:" }
                                    opt_history_merge = <CrawradOptionDrop> { width: 160.0 } }
                                <View> { width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_XS),
                                    <CrawradLabelMuted> { width: Fill, height: Fit, text: "Skip files larger than:" }
                                    opt_history_max = <CrawradOptionDrop> { width: 160.0 } }
                            }

                            // The Saving and Recovering page
                            opt_saving = <View> {
                                visible: false, width: Fill, height: Fill, flow: Down, spacing: (CRAWRAD_ESPACO_S),
                                <CrawradGroupBoxTitle> { text: "Autosave" }
                                opt_reopen_last = <CrawradOptionCheck> {}
                                opt_save_build = <CrawradOptionCheck> {}
                                opt_save_desktop = <CrawradOptionCheck> {}
                                <CrawradLabelMuted> { width: Fill, height: Fit, margin: {left: 18.0}, text: "Docks, splitters, window, open tabs and the caret of each, kept per project." }
                                <View> { width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_XS),
                                    <CrawradLabelMuted> { width: Fill, height: Fit, text: "Auto save editor files:" }
                                    opt_auto_save = <CrawradOptionDrop> { width: 240.0 } }
                                <CrawradLabelMuted> { width: Fill, height: Fit, text: "Editor files only: the form saves with Save, because saving it regenerates the designer code." }
                                <CrawradGroupBoxTitle> { text: "Autorecover" }
                                opt_recover = <CrawradOptionCheck> {}
                                <View> { width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_XS),
                                    <CrawradLabelMuted> { width: Fill, height: Fit, text: "Every:" }
                                    opt_recover_minutes = <CrawradOptionDrop> { width: 160.0 } }
                                <CrawradLabelMuted> { width: Fill, height: Fit, text: "Copies go to .crawrad/recover, never over the file. After a crash the IDE offers the copies newer than the file on disk." }
                                <CrawradGroupBoxTitle> { text: "Desktop profiles" }
                                <CrawradLabelMuted> { width: Fill, height: Fit, text: "Active profile:" }
                                <View> { width: Fill, height: 26.0, flow: Right, align: { y: 0.5 }, spacing: (CRAWRAD_ESPACO_S),
                                    opt_profile = <CrawradOptionDrop> { width: 240.0 }
                                    opt_profile_save_as = <CrawradButton> { width: 90.0, height: 26.0, text: "Save as..." }
                                    opt_profile_delete = <CrawradButton> { width: 76.0, height: 26.0, text: "Remove" }
                                    opt_profile_default = <CrawradButton> { width: 120.0, height: 26.0, text: "Restore default" } }
                                opt_profile_name_wrap = <View> { visible: false, width: Fill, height: 26.0, flow: Right, align: { y: 0.5 }, spacing: (CRAWRAD_ESPACO_S),
                                    opt_profile_name = <CrawradPropInput> { width: 240.0, height: 26.0, empty_text: "Profile name" }
                                    opt_profile_name_ok = <CrawradPrimaryButton> { width: 100.0, height: 26.0, text: "Save profile" }
                                    opt_profile_name_cancel = <CrawradButton> { width: 70.0, height: 26.0, text: "Cancel" } }
                                <CrawradLabelMuted> { width: Fill, height: Fit, text: "A profile holds the window, each dock's size and visibility, the Inspector split, fonts and zoom. A bad or old profile falls back to the defaults." }
                            }

                            // And the phrase for pages that only INFORM.
                            opt_nota = <CrawradLabelMuted> {
                                width: Fill, height: Fit,
                                text: ""
                            }

                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        opt_save = <CrawradDialogPrimaryButton> { text: "" }
                        opt_cancel = <CrawradDialogButton> { text: "" }
                    }
                }
            }


}
