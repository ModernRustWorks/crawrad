use makepad_widgets::*;

live_design! {
    link crawrad_dialogs;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use link::crawrad::*;
    use makepad_code_editor::note_icon::NoteKindIcon;
    use makepad_code_editor::release_size_view::*;
    use makepad_code_editor::environment_view::*;
            pub CrawradIconsDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_ICONES_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        ic_title = <CrawradDialogTitle> { text: "Icon sheet" }
                    }
                    <CrawradDialogBody> {
                        height: (CRAWRAD_ICONES_CORPO),
                        ic_nota = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        <CrawradGroupBoxFrame> {
                            ic_list = <PortalList> {
                                width: Fill, height: Fill,
                                scroll_bar: <CrawradScrollBar> {}
                                IconRow = <CrawradIconSheetRow> {}
                            }
                        }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        ic_fechar = <CrawradDialogPrimaryButton> { text: "" }
                    }
                }
            }
            pub CrawradMemoryDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_OPTIONS_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        mem_title = <CrawradDialogTitle> { text: "Aefos Memory" }
                    }
                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_M),
                        mem_subtitle = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Global instructions and context shared with all AI tools and sessions." }
                        mem_input = <CrawradTextInput> {
                            width: Fill, height: 160.0,
                            empty_text: "Enter global instructions for AI..."
                        }
                        mem_counter = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        mem_cancel = <CrawradDialogButton> { text: "Cancel" }
                        mem_save = <CrawradDialogPrimaryButton> { text: "" }
                    }
                }
            }
            // Why: Environment check dialog (approved mockup, 820 px wide)
            pub CrawradEnvironmentDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> { view = <EnvironmentCheckView> {} }
            }
            pub CrawradMcpDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_OPTIONS_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        mcp_title = <CrawradDialogTitle> { text: "MCP Servers" }
                    }
                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_M),
                        mcp_subtitle = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Connected Model Context Protocol (MCP) servers and tools." }
                        mcp_servers_info = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        mcp_close = <CrawradDialogPrimaryButton> { text: "" }
                    }
                }
            }
            // Why: ACP Agent diagnostic dialog
            pub CrawradAcpDiagnosticDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 720.0, height: Fit,
                    <CrawradDialogTitleBar> {
                        diag_title = <CrawradDialogTitle> { text: "ACP Agent Diagnostic" }
                    }
                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_M),
                        diag_status = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        diag_checks_box = <CrawradGroupBoxFrame> {
                            height: Fit,
                            flow: Down,
                            spacing: (CRAWRAD_ESPACO_S),
                            diag_checks_title = <CrawradGroupBoxTitle> { text: "Diagnostic Checks" }
                            diag_check_0 = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                            diag_check_1 = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                            diag_check_2 = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                            diag_check_3 = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                        }
                        diag_info_box = <CrawradGroupBoxFrame> {
                            height: Fit,
                            flow: Down,
                            spacing: (CRAWRAD_ESPACO_XS),
                            diag_info_title = <CrawradGroupBoxTitle> { text: "Configuration & Environment" }
                            diag_config_path = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                            diag_detected_agents = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        }
                    }
                    <CrawradDialogFooter> {
                        diag_act_0 = <CrawradDialogPrimaryButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_1 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_2 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_3 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_4 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_5 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_6 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        diag_act_7 = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 10.0, right: 10.0 }, visible: false, text: "" }
                        <CrawradFiller> {}
                        diag_close = <CrawradDialogButton> { width: Fit, height: (CRAWRAD_DIALOGO_BOTAO_A), padding: { left: 12.0, right: 12.0 }, text: "" }
                    }
                }
            }
            pub CrawradNoteDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_OPTIONS_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        spacing: (CRAWRAD_ESPACO_S),
                        note_icon = <NoteKindIcon> {}
                        note_title = <CrawradDialogTitle> { text: "" }
                        note_subtitle = <CrawradLabelMuted> { text: "" }
                        note_badge = <CrawradLabel> { text: "" }
                        <CrawradFiller> {}
                        note_location_right = <CrawradLabelMuted> { text: "" }
                    }
                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_S),
                        // A label draws its height even empty: the row is what hides.
                        note_item_row = <View> { width: Fill, height: Fit,
                            note_item = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        }
                        note_code_box = <CrawradGroupBoxFrame> {
                            visible: false, height: Fit,
                            draw_bg: { color: (CRAWRAD_GROUND) }
                            note_code = <CrawradMonoLabel> { margin: 0 }
                        }
                        note_insert_row = <View> {
                            width: Fill, height: Fit,
                            flow: Right, align: { y: 0.5 },
                            spacing: (CRAWRAD_ESPACO_S),
                            note_sec_summary = <CrawradButton> { text: "Summary" }
                            note_sec_examples = <CrawradButton> { text: "# Examples" }
                            note_sec_errors = <CrawradButton> { text: "# Errors" }
                            note_sec_panics = <CrawradButton> { text: "# Panics" }
                            note_sec_safety = <CrawradButton> { text: "# Safety" }
                            note_sec_link = <CrawradButton> { text: "Link item…" }
                            note_suggest = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        }
                        note_id_row = <View> {
                            width: Fill, height: Fit,
                            flow: Right, align: { y: 0.5 },
                            spacing: (CRAWRAD_ESPACO_S),
                            note_title_input = <CrawradPropInput> { width: Fill }
                            note_tag_input = <CrawradPropInput> { width: 180.0 }
                        }
                        note_tab_row = <View> {
                            width: Fill, height: Fit,
                            flow: Right, align: { y: 0.5 },
                            spacing: (CRAWRAD_ESPACO_S),
                            note_tab_edit = <CrawradButton> { text: "Edit" }
                            note_tab_preview = <CrawradButton> { text: "" }
                            note_tab_hint = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        }
                        note_editor = <View> { width: Fill, height: Fit,
                            note_content_input = <CrawradTextInput> {
                                width: Fill, height: 220.0,
                                empty_text: "Note content in Markdown (architecture, context, reasoning)..."
                            }
                        }
                        // The Preview tab: the markdown `cargo doc` would show.
                        note_preview = <ScrollYView> {
                            visible: false, width: Fill, height: 220.0,
                            note_preview_md = <Markdown> { width: Fill, height: Fit, body: "" }
                        }
                        note_warning_row = <View> { width: Fill, height: Fit,
                            note_warning = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        }
                        note_status = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                    }

                    <CrawradDialogFooter> {
                        note_resolve = <CrawradButton> { width: Fit, padding: { left: 12., right: 12. }, text: "", draw_text: { color: (CRAWRAD_NOTE_COMMENT) } }
                        note_test_result = <CrawradLabelMuted> { width: Fit, height: Fit, text: "" }
                        <CrawradFiller> {}
                        note_test = <CrawradDialogButton> { width: Fit, padding: { left: 12., right: 12. }, text: "" }
                        note_cancel = <CrawradDialogButton> { text: "" }
                        note_save = <CrawradDialogPrimaryButton> { text: "" }
                    }
                }
            }

            pub CrawradGotoDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 380.0, height: Fit,

                    <CrawradDialogTitleBar> {
                        goto_title = <CrawradDialogTitle> { text: "Go to Line / Column" }
                    }

                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_S),
                        goto_help = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        goto_input = <CrawradTextInput> {
                            width: Fill, height: 26.0,
                            empty_text: ":1"
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        goto_confirm = <CrawradDialogPrimaryButton> { text: "Go" }
                        goto_cancel = <CrawradDialogButton> { text: "Cancel" }
                    }
                }
            }

            pub CrawradQuickOpenDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 520.0, height: Fit,

                    <CrawradDialogTitleBar> {
                        quick_title = <CrawradDialogTitle> { text: "" }
                    }

                    <CrawradDialogBody> {
                        height: 320.0,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_S),
                        quick_input = <CrawradTextInput> {
                            width: Fill, height: 26.0,
                            empty_text: "Type to search files..."
                        }
                        quick_list = <PortalList> {
                            width: Fill, height: Fill,
                            scroll_bar: <CrawradScrollBar> {}
                            FileRow = <CrawradItemsRow> {}
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        quick_cancel = <CrawradDialogButton> { text: "Fechar" }
                    }
                }
            }

            pub CrawradBuildRunDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 440.0, height: Fit,
                    <CrawradDialogTitleBar> { run_title = <CrawradDialogTitle> { text: "Building" } }
                    <CrawradDialogBody> {
                        height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_M),
                        run_action = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                        <View> {
                            width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_S),
                            <View> { width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, <CrawradFiller> {} run_counter = <CrawradLabelMuted> { width: Fit, height: Fit, text: "" } }
                            run_bar = <View> { width: Fill, height: 6.0, show_bg: true, draw_bg: { color: (CRAWRAD_BORDA_CONTROLE) } bar = <View> { width: 0.0, height: Fill, show_bg: true, draw_bg: { color: (CRAWRAD_ACENTO) } } }
                        }
                        <View> {
                            width: Fill, height: Fit, flow: Down, spacing: 4.0,
                            <View> { width: Fill, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_M), <CrawradLabelMuted> { width: 56.0, height: Fit, text: "Output" } run_output = <CrawradLabel> { width: Fill, height: Fit, text: "" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_M), <CrawradLabelMuted> { width: 56.0, height: Fit, text: "Elapsed" } run_elapsed = <CrawradLabel> { width: Fill, height: Fit, text: "" } }
                        }
                    }
                    <CrawradDialogFooter> { <CrawradFiller> {} run_cancel = <CrawradDialogButton> { text: "Cancel" } }
                }
            }

            pub CrawradBuildDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_BUILD_DLG_L), height: Fit,

                    <CrawradDialogTitleBar> {
                        bld_title = <CrawradDialogTitle> { text: "Build - my_app" }
                        <CrawradFiller> {}
                        bld_close = <CrawradCloseButton> {}
                    }

                    <CrawradDialogBody> {
                        height: (CRAWRAD_BUILD_CORPO),
                        padding: { left: 12.0, right: 12.0, top: 10.0, bottom: 8.0 },
                        flow: Right,
                        spacing: 10.0,

                        // LEFT RAIL (Actions: Build, Run, Check, Clean, Settings and Help)
                        <View> {
                            width: 106.0, height: Fill,
                            flow: Down,
                            bld_rail = <CrawradGroupBoxFrame> {
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: 6.0,
                                padding: { top: 8.0, bottom: 8.0, left: 6.0, right: 6.0 },

                                bld_page_opts = <CrawradButton> {
                                    width: Fill, height: 30.0,
                                    text: "⚙ Options"
                                }
                                <View> {
                                    width: Fill, height: 1.0,
                                    show_bg: true,
                                    draw_bg: { color: (CRAWRAD_BORDA_CONTROLE) }
                                }
                                bld_mode_build = <CrawradButton> {
                                    width: Fill, height: 30.0,
                                    text: ""
                                }
                                bld_btn_check = <CrawradButton> {
                                    width: Fill, height: 30.0,
                                    text: ""
                                }
                                bld_btn_clean = <CrawradButton> {
                                    width: Fill, height: 30.0,
                                    text: ""
                                }
                                bld_btn_size = <CrawradButton> {
                                    width: Fill, height: 30.0,
                                    text: "📏 Size"
                                }

                                <CrawradFiller> {}

                                bld_btn_help = <CrawradButton> {
                                    width: Fill, height: 28.0,
                                    text: "❓ Help"
                                }
                            }
                        }

                        // MAIN AREA
                        bld_main = <View> {
                            width: Fill, height: Fill,
                            flow: Down,
                            padding: 0.,
                            spacing: 6.0,

                            bld_page_options = <View> {
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: 6.0,

                                // TOP CONFIGURATION ROW (Profile)
                                bld_top_row = <View> {
                                    width: Fill, height: 28.0,
                                    flow: Right, align: { y: 0.5 },
                                    spacing: 8.0,

                                    <CrawradLabel> { text: "Profile:" }
                                    bld_profile_btn = <CrawradButton> { width: 110.0, height: 26.0, text: "Debug ▾" }
                                }

                                // MIDDLE AREA: 2 COLUMNS
                                bld_columns = <View> {
                                    width: Fill, height: Fill,
                                    flow: Right,
                                    spacing: 10.0,

                                    // LEFT COLUMN
                                    bld_col_left = <View> {
                                        width: Fill, height: Fill,
                                        flow: Down,
                                        spacing: 8.0,

                                        // CARD: TARGET PLATFORMS
                                        <CrawradGroupBoxFrame> {
                                            width: Fill, height: 150.0,
                                            flow: Down,
                                            padding: 8.0,
                                            spacing: 3.0,

                                            <CrawradGroupBoxTitle> { text: "Target Platforms" }
                                            // The palette's own CheckListBox, the triple at each row's end
                                            bld_targets = <CrawradCheckListBox> {
                                                width: Fill, height: Fill,
                                                draw_bg: { color: (CRAWRAD_TRANSPARENTE) }
                                                draw_detail: { color: (CRAWRAD_TEXTO_TERCIARIO) }
                                                list = {
                                                    row = <CrawradOptionCheck> {}
                                                }
                                            }
                                        }

                                        // CARD: BUILD OPTIONS
                                        <CrawradGroupBoxFrame> {
                                            width: Fill, height: Fit,
                                            flow: Down,
                                            padding: 8.0,
                                            spacing: 4.0,

                                            <CrawradGroupBoxTitle> { text: "Build Options" }
                                            // One under the other: side by side, the longer
                                            // labels were cut at the column's edge.
                                            <View> {
                                                width: Fill, height: Fit,
                                                flow: Down,
                                                spacing: 8.0,

                                                // Options Subcolumn
                                                <View> {
                                                    width: Fill, height: Fit,
                                                    flow: Down,
                                                    spacing: 2.0,

                                                    bld_opt_clean = <CrawradOptionCheck> { text: "Clean before build" }
                                                    bld_opt_workspace = <CrawradOptionCheck> { text: "Build all workspace members" }
                                                    bld_opt_verbose = <CrawradOptionCheck> { text: "Verbose output (-v)" }
                                                    bld_opt_messages = <CrawradOptionCheck> { text: "Compiler messages" }
                                                }

                                                // Features Subcolumn
                                                <View> {
                                                    width: Fill, height: Fit,
                                                    flow: Down,
                                                    spacing: 4.0,

                                                    <CrawradLabelMuted> { text: "Features:" }
                                                    bld_features_input = <CrawradPropInput> {
                                                        width: Fill, height: 24.0,
                                                        empty_text: "e.g. feature1,feature2"
                                                    }
                                                    bld_opt_all_features = <CrawradOptionCheck> { text: "All features (--all-features)" }
                                                    bld_opt_no_default = <CrawradOptionCheck> { text: "No default features (--no-default-features)" }
                                                }
                                            }
                                        }
                                    }

                                    // RIGHT COLUMN
                                    bld_col_right = <View> {
                                        width: Fill, height: Fill,
                                        flow: Down,
                                        spacing: 8.0,

                                        // CARD: SELECTED TARGETS (tall enough for all five buttons)
                                        <CrawradGroupBoxFrame> {
                                            width: Fill, height: 182.0,
                                            flow: Down,
                                            padding: 8.0,
                                            spacing: 4.0,

                                            <CrawradGroupBoxTitle> { text: "Selected Targets" }
                                            <View> {
                                                width: Fill, height: Fill,
                                                flow: Right,
                                                spacing: 8.0,

                                                // The selected targets are the palette's own Chips
                                                bld_sel_targets_view = <View> {
                                                    width: Fill, height: Fill, flow: Down, spacing: 4.0,
                                                    bld_chips = <CrawradChips> { width: Fill, height: Fill }
                                                    bld_chip_empty = <CrawradLabelMuted> { text: "" }
                                                }

                                                // Preset quick buttons
                                                <View> {
                                                    width: 120.0, height: Fill,
                                                    flow: Down,
                                                    spacing: 3.0,

                                                    bld_btn_select_all = <CrawradButton> { width: Fill, text: "Select All" }
                                                    bld_btn_deselect_all = <CrawradButton> { width: Fill, text: "Deselect All" }
                                                    bld_btn_preset_desktop = <CrawradButton> { width: Fill, text: "Common Desktop" }
                                                    bld_btn_preset_mobile = <CrawradButton> { width: Fill, text: "Mobile" }
                                                    bld_btn_preset_web = <CrawradButton> { width: Fill, text: "Web" }
                                                }
                                            }
                                        }

                                        // CARD: ADDITIONAL ARGUMENTS
                                        <View> {
                                            width: Fill, height: Fit,
                                            flow: Down,
                                            spacing: 3.0,

                                            <CrawradLabelMuted> { text: "Additional Arguments (passed to cargo)" }
                                            bld_args_input = <CrawradPropInput> {
                                                width: Fill, height: 26.0,
                                                empty_text: "--release, --offline, -j 4, --timings..."
                                            }
                                        }

                                        // CARD: ENVIRONMENT VARIABLES
                                        <CrawradGroupBoxFrame> {
                                            width: Fill, height: 85.0,
                                            flow: Down,
                                            padding: 6.0,
                                            spacing: 3.0,

                                            <View> {
                                                width: Fill, height: Fit,
                                                flow: Right, align: { y: 0.5 },
                                                <CrawradGroupBoxTitle> { text: "Environment Variables (optional)" }
                                                <CrawradFiller> {}
                                                bld_env_add = <CrawradButton> { width: 22.0, height: 20.0, padding: 0., text: "+" }
                                            }
                                            bld_env_table = <ScrollYView> {
                                                width: Fill, height: Fill,
                                                flow: Down,
                                                spacing: 2.0,
                                                bld_env_row_0 = <View> {
                                                    width: Fill, height: 24.0,
                                                    flow: Right, align: { y: 0.5 }, spacing: 4.0,
                                                    bld_env_k_0 = <CrawradPropInput> { width: 140.0, height: 22.0, empty_text: "Name" }
                                                    bld_env_v_0 = <CrawradPropInput> { width: Fill, height: 22.0, empty_text: "Value" }
                                                    bld_env_del_btn_0 = <CrawradButton> { width: 22.0, height: 22.0, padding: 0., text: "×" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            bld_page_command = <View> {
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: 6.0,

                                // HEADER ROW
                                <View> {
                                    width: Fill, height: 28.0,
                                    flow: Right, align: { y: 0.5 },
                                    spacing: 8.0,

                                    bld_cmd_title = <CrawradLabel> {
                                        draw_text: {
                                            text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CABECALHO) }
                                            color: (CRAWRAD_TEXTO_PRIMARIO)
                                        }
                                        text: "Build"
                                    }
                                    bld_cmd_meta = <CrawradLabelMuted> { text: "" }
                                    <CrawradFiller> {}
                                    bld_cancel = <CrawradButton> { width: Fit, height: 24.0, text: "" }
                                }

                                // PROGRESS BOX
                                <CrawradGroupBoxFrame> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    padding: 8.0,
                                    spacing: 6.0,

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right, align: { y: 0.5 },
                                        spacing: 8.0,

                                        bld_run_headline = <CrawradLabel> { text: "" }
                                        <CrawradFiller> {}
                                        bld_run_counter = <CrawradLabelMuted> { text: "" }
                                        bld_run_clock = <CrawradLabelMuted> { text: "" }
                                    }

                                    bld_run_bar = <View> {
                                        width: Fill, height: 4.0,
                                        show_bg: true,
                                        draw_bg: { color: (CRAWRAD_BORDA_CONTROLE) }
                                        bar = <View> {
                                            width: 0.0, height: Fill,
                                            show_bg: true,
                                            draw_bg: { color: (CRAWRAD_ACENTO) }
                                        }
                                    }
                                }

                                // OUTPUT / TERMINAL PANEL
                                bld_terminal_panel = <CrawradGroupBoxFrame> {
                                    width: Fill, height: Fill,
                                    flow: Down,
                                    padding: 6.0,
                                    spacing: 4.0,

                                    // Terminal Tabs Header
                                    <View> {
                                        width: Fill, height: 24.0,
                                        flow: Right, align: { y: 0.5 },
                                        spacing: 8.0,

                                        bld_tab_output = <CrawradButton> { width: Fit, height: 22.0, text: ">_ Output" }
                                        bld_tab_warnings = <CrawradButton> { width: Fit, height: 22.0, text: "⚠ Warnings (0)" }
                                        bld_tab_errors = <CrawradButton> { width: Fit, height: 22.0, text: "🚫 Errors (0)" }
                                        <CrawradFiller> {}
                                        bld_out_copy = <CrawradButton> { width: Fit, height: 22.0, text: "📋 Copy" }
                                        bld_out_clear = <CrawradButton> { width: Fit, height: 22.0, text: "🗑 Clear" }
                                    }

                                    // Selectable and Copyable Console Viewer with Scroll Bar
                                    bld_console_scroll = <ScrollYView> {
                                        width: Fill, height: Fill,
                                        flow: Down,
                                        scroll_bars: <CrawradScrollBars> {}

                                        bld_console_text = <CrawradLinesText> {
                                            width: Fill, height: Fit,
                                        }
                                    }
                                }
                            }

                            bld_page_size = <ReleaseSizeView> { visible: false }
                        }
                    }

                    // FOOTER
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        bld_btn_close = <CrawradDialogButton> { width: 100.0, text: "" }
                    }
                }
            }

            // 20. CODE TEMPLATES MANAGER: the shipped ones read-only, the user's editable.
            pub CrawradTemplatesDialog = <CrawradDialogVeil> { dlg = <CrawradDialog> { width: 1000.0, height: Fit,
                <CrawradDialogTitleBar> { tpl_title = <CrawradDialogTitle> { text: "Code templates" } }
                <CrawradDialogBody> { height: 554.0, flow: Right, spacing: 16.0,
                    <View> { width: 300.0, height: Fill, flow: Down, spacing: 8.0,
                        <View> { width: Fill, height: 26.0, flow: Right, spacing: 4.0,
                            tpl_btn_new = <CrawradButton> { width: Fit, height: 24.0, text: "New" }
                            tpl_btn_duplicate = <CrawradButton> { width: Fit, height: 24.0, text: "" }
                            tpl_btn_import = <CrawradButton> { width: Fit, height: 24.0, text: "Import\u{2026}" }
                            tpl_btn_export = <CrawradButton> { width: Fit, height: 24.0, text: "Export\u{2026}" }
                        }
                        <CrawradGroupBoxFrame> { width: Fill, height: Fill, tpl_list = <PortalList> { width: Fill, height: Fill, scroll_bar: <CrawradScrollBar> {}
                        TplSection = <View> { width: Fill, height: 24.0, align: { y: 0.5 }, padding: { left: 8.0, right: 8.0 },
                            sec_label = <CrawradLabelMuted> { width: Fill, text: "", draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_BOLD> { font_size: 9.0 } } }
                        }
                        TplRow = <View> { width: Fill, height: 26.0, flow: Overlay, show_bg: true, draw_bg: { color: (CRAWRAD_ELEVADO) }
                            <View> { width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, padding: { left: 8.0, right: 8.0 }, spacing: 8.0,
                                trigger = <CrawradLabel> { width: Fit, text: "", draw_text: { color: (CRAWRAD_ACENTO), text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } } }
                                description = <CrawradLabelMuted> { width: Fill, text: "", draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) } } }
                            }
                            alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
                        }
                        TplRowSelected = <View> { width: Fill, height: 26.0, flow: Overlay, show_bg: true, draw_bg: { color: (CRAWRAD_SELECAO_FORTE) }
                            <View> { width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, padding: { left: 8.0, right: 8.0 }, spacing: 8.0,
                                trigger = <CrawradLabel> { width: Fit, text: "", draw_text: { color: (CRAWRAD_ACENTO_TEXTO), text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } } }
                                description = <CrawradLabelMuted> { width: Fill, text: "", draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) } } }
                            }
                            alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
                        }
                        } }
                        <CrawradLabelMuted> { width: Fill, height: Fit, text: "Saved per user \u{00B7} templates.toml" }
                    }
                    <View> { width: Fill, height: Fill, flow: Down, spacing: 8.0,
                        <View> { width: Fill, height: Fit, flow: Down, spacing: 2.0, <CrawradLabelMuted> { text: "Trigger" }
                            tpl_trigger = <CrawradPropInput> { width: Fill, height: 26.0, draw_text: { text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } } }
                        }
                        <View> { width: Fill, height: Fit, flow: Down, spacing: 2.0, <CrawradLabelMuted> { text: "Description" }
                            tpl_desc = <CrawradPropInput> { width: Fill, height: 26.0 }
                        }
                        <View> { width: Fill, height: Fill, flow: Down, spacing: 2.0,
                            <View> { width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, <CrawradLabelMuted> { text: "Body" } <CrawradFiller> {}
                                <CrawradLabelMuted> { draw_text: { text_style: { font_size: 9.0 } } text: "${1:name} is a tab stop, $0 is where the caret ends" }
                            }
                            tpl_body = <CrawradLinesText> { width: Fill, height: Fill }
                        }
                        <View> { width: Fill, height: 110.0, flow: Down, spacing: 2.0, tpl_preview_head = <CrawradLabelMuted> { text: "" }
                            <CrawradGroupBoxFrame> { width: Fill, height: Fill, padding: 6.0,
                                tpl_preview = <CrawradLabelMuted> { width: Fill, height: Fill, text: "", draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } } }
                            }
                        }
                    }
                }
                <CrawradDialogFooter> {
                    tpl_delete = <CrawradButton> { width: 80.0, height: (CRAWRAD_DIALOGO_BOTAO_A), text: "", draw_text: { color: (CRAWRAD_ERROR_CHIP_TEXT) }
                        draw_bg: { color: (CRAWRAD_ERROR_CHIP), color_hover: (CRAWRAD_ERROR_CHIP_HOVER), border_color_1: (CRAWRAD_ERRO), border_color_2: (CRAWRAD_ERRO) }
                    }
                    tpl_problem = <CrawradLabel> { margin: { left: 12.0 }, text: "", draw_text: { color: (CRAWRAD_ERRO), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } } }
                    <CrawradFiller> {}
                    tpl_cancel = <CrawradDialogButton> { text: "Cancel" }
                    tpl_save = <CrawradDialogPrimaryButton> { text: "" }
                }
            }
        }
}
