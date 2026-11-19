//! THE MAIN DSL of the IDE — the visual tree of windows, bars, docks, and dialogs (ADR 0043).

use makepad_code_editor::*;
use makepad_widgets::*;

live_design! {
link crawrad_dsl_main;
use link::widgets::*;
use link::theme::*;
use link::crawrad_theme::*;
use link::crawrad_dialogs::*;
use link::crawrad_minimap::*;
use link::crawrad_panels::*;
use link::crawrad::*;
use makepad_code_editor::form_check_view::*;
use makepad_code_editor::release_size_view::*;
use makepad_code_editor::environment_view::*;
use makepad_code_editor::tests_panel_view::*;
use makepad_code_editor::code_view::CodeView;

pub CrawradMainUi = <Window> {
        window: { inner_size: vec2(1440.0, 900.0), title: "CrawRAD Studio" },
        pass: { clear_color: (CRAWRAD_GROUND) },
        body = <CrawradGround> {
            // The body is Flow::Overlay so that children with abs_pos retain their geometry.
            flow: Overlay,
            width: Fill, height: Fill,

            // The IDE COLUMN: bar, workspace, and footer.
            content = <View> {
                flow: Down,
                width: Fill, height: Fill,

                // --- 1. TOP TITLEBAR & MENU BAR ---
                // Menu titles and Codicon icons are populated at runtime by montar_cromo.
                top_titlebar = <CrawradBand> {
                    width: Fill, height: (CRAWRAD_BARRA_ALTURA),
                    flow: Right, align: { y: 0.5 },
                    padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_CARD_GAP) },
                    spacing: 0.0,

                    // The app's own icon before the menus, as other IDEs do (owner, 13/11).
                    app_icon = <Image> { width: 24.0, height: 24.0, margin: { left: 2.0, right: (CRAWRAD_ESPACO_S) } }
                    menu_file = <CrawradMenuButton> { text: "" }
                    menu_edit = <CrawradMenuButton> { text: "" }
                    menu_view = <CrawradMenuButton> { text: "" }
                    menu_go = <CrawradMenuButton> { text: "" }
                    menu_refactor = <CrawradMenuButton> { text: "" }
                    menu_project = <CrawradMenuButton> { text: "" }
                    menu_run = <CrawradMenuButton> { text: "" }
                    menu_component = <CrawradMenuButton> { text: "" }
                    menu_tools = <CrawradMenuButton> { text: "" }
                    menu_help = <CrawradMenuButton> { text: "" }

                    sep_menu = <CrawradBarSeparator> {}
                    btn_undo = <CrawradIconButton> { text: "" }
                    btn_redo = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "" }
                    btn_save = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "" }
                    btn_save_all = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "" }
                    btn_preview = <CrawradSplitButton> { width: (CRAWRAD_ALVO_ICONE), padding: 0., margin: { left: (CRAWRAD_ESPACO_XS) }, text: "",
                        draw_bg: { color: (CRAWRAD_CONTROLE), color_hover: (CRAWRAD_BORDA_CONTROLE), color_down: (CRAWRAD_SELECAO_FORTE) }
                        draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_CABECALHO) } } }
                    btn_preview_menu = <CrawradSplitButton> { width: 18.0, padding: 0., margin: 0., text: "",
                        draw_bg: { radius_left: 0.0, radius_right: (CRAWRAD_RAIO_BOTAO), color: (CRAWRAD_CONTROLE), color_hover: (CRAWRAD_BORDA_CONTROLE), color_down: (CRAWRAD_SELECAO_FORTE) }
                        draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                    btn_build = <CrawradCaptionButton> { margin: { left: (CRAWRAD_GRUPO_BARRA) } }
                    btn_play = <CrawradCaptionButton> {
                        margin: { left: (CRAWRAD_ESPACO_S) }
                        button = <CrawradSplitButton> { padding: { left: 31., right: 12., top: 0., bottom: 0. } }
                        icon_slot = { glyph = { draw_text: { color: (CRAWRAD_GROUND) } } }
                    }
                    btn_play_menu = <CrawradSplitButton> {
                        width: 22.0, padding: 0.,
                        draw_bg: {
                            radius_left: 0.0, radius_right: (CRAWRAD_RAIO_BOTAO)
                            color: (CRAWRAD_BOTAO_PRIMARIO_DOWN), color_hover: (CRAWRAD_BOTAO_PRIMARIO)
                        }
                        draw_text: { text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                        text: ""
                    }
                    sep_debug = <CrawradBarSeparator> {}
                    tb_debug_pause = <CrawradDebugBarButton> { margin: 0. }
                    tb_debug_stop = <CrawradDebugBarButton> { draw_text: { color: (CRAWRAD_ERRO) } }
                    tb_step_over = <CrawradDebugBarButton> {}
                    tb_step_into = <CrawradDebugBarButton> {}
                    tb_step_out = <CrawradDebugBarButton> {}

                    // Centered project title
                    title_container = <View> {
                        width: Fill, height: Fill,
                        align: { x: 0.5, y: 0.5 },
                        title_lbl = <CrawradLabel> {
                            draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) },
                            text: ""
                        }
                    }

                    form_check_chip = <FormCheckChip> {}
                    btn_min = <CrawradMinButton> {}
                    btn_max = <CrawradMaxButton> {}
                    btn_restore = <CrawradRestoreButton> {}
                    btn_close = <CrawradCloseButton> {}
                }

                // --- 2. MAIN HORIZONTAL WORKSPACE ---
                main_workspace = <View> {
                    width: Fill, height: Fill,
                    flow: Right,
                    show_bg: true,
                    draw_bg: { color: (CRAWRAD_GROUND) },
                    padding: { top: 0.0, bottom: 0.0, left: 0.0, right: (CRAWRAD_CARD_GAP) },

                    activity_rail = <CrawradCardRail> {
                        width: (CRAWRAD_RAIL_LARGURA), height: Fill,
                        flow: Down,

                        rail_list = <PortalList> {
                            width: Fill, height: Fill,
                            // The rail scrollbar has zero width: the rail has
                            // fixed destinations and does not scroll.
                            scroll_bar: <CrawradScrollBar> { bar_size: 0.0 }

                            // The TEMPLATE of a destination: see `rail.rs`.
                            Destino = <CrawradRailButton> { text: "" }
                        }

                        rail_settings = <CrawradRailButtonMuted> {
                            margin: { left: 6.0, right: 6.0, top: 4.0, bottom: 6.0 },
                            text: ""
                        }
                    }

                    // The options bar and left panel are joined
                    // without an internal divider (modern-editor style, ADR 0014 / Amendment 10).
                    sep_rail = <View> {
                        width: 0.0, height: 0.0,
                        visible: false
                    }

                    dock = <CrawradDock> {
                        width: Fill, height: Fill,

                        // ------------------------------------------ the templates

                        PanelComponents = <CrawradPanelComponents> {}
                        PanelInspector = <CrawradPanelInspector> {}
                        // The EXPLORER: who owns each file (ADR 0016).
                        // Line count comes from the generator plan (ADR 0027).
                        PanelExplorer = <CrawradPanelExplorer> {}
                        // The PACKAGE MANAGER: crates.io store (ADR 0014, Amendment 8).
                        PanelPackages = <CrawradPanelPackages> {}
                        // RUN AND DEBUG: the controls docked on
                        // top, the sections below — `crawrad_host::debug_panel`.
                        PanelDebug = <CrawradPanelDebug> {}
                        // FIND ALL REFERENCES: `crawrad_host::references`.
                        PanelReferences = <CrawradPanelReferences> {}
                        PanelProblems = <CrawradPanelProblems> {}
                        // EXTENSION VIEWS: `crawrad_extension_host::views`.
                        PanelExtensions = <CrawradPanelExtensions> {}
                        PanelNotes = <CrawradPanelNotes> {}
                        PanelTests = <CrawradPanelLeft> {
                            tests_view = <TestsPanelView> {}
                        }
                        PanelTestoutput = <CrawradPanel> {
                            output_view = <TestOutputView> {}
                        }
                        PanelFormcheck = <CrawradPanel> {
                            check_view = <FormCheckView> {}
                        }
                        PanelDesign = <CrawradPanelDesign> {}
                        PanelChat = <CrawradPanelChat> {}
                        PanelTerminal = <CrawradPanelTerminal> {}
                        root = Splitter {
                            axis: Horizontal,
                            align: FromA((CRAWRAD_DOCA_LARGURA)),
                            a: dock_left,
                            b: dock_rest
                        }
                        dock_rest = Splitter {
                            axis: Horizontal,
                            align: FromB((CRAWRAD_CHAT_LARGURA)),
                            a: dock_center,
                            b: dock_right
                        }
                        dock_left = Tabs { tabs: [tab_components], selected: 0, closable: true }
                        dock_center = Tabs { tabs: [tab_design], selected: 0, closable: true }
                        dock_right = Tabs { tabs: [tab_chat], selected: 0, closable: true }
                        tab_components = Tab { name: "", template: PermanentTab, kind: PanelComponents }
                        tab_design = Tab { name: "", template: PermanentTab, kind: PanelDesign }
                        tab_chat = Tab { name: "", template: PermanentTab, kind: PanelChat }
                        tab_terminal = Tab { name: "", template: PermanentTab, kind: PanelTerminal }
                    }
                }

                // --- 3. BOTTOM STATUS BAR ---
                bottom_status_bar = <CrawradStatusBar> {
                    width: Fill, height: (CRAWRAD_RODAPE_ALTURA),
                    flow: Right, align: { y: 0.5 },

                    cell_target = <CrawradStatusCell> {
                        ic_target = <CrawradStatusIcon> { text: "" }
                        lbl_target = <CrawradStatusText> { text: "" }
                    }
                    cell_project = <CrawradStatusCell> {
                        lbl_project = <CrawradStatusText> { text: "" }
                    }
                    cell_problems = <CrawradStatusCell> {
                        ic_errors = <CrawradStatusIcon> {
                            text: "",
                            draw_text: { color: (CRAWRAD_ERRO) }
                        }
                        lbl_errors = <CrawradStatusText> { text: "" }
                        ic_warnings = <CrawradStatusIcon> {
                            text: "",
                            margin: { left: (CRAWRAD_ESPACO_S) },
                            draw_text: { color: (CRAWRAD_AVISO) }
                        }
                        lbl_warnings = <CrawradStatusText> { text: "" }
                    }
                    // Hidden until a server is asked for: no Rust file open, nothing to say.
                    cell_lsp = <CrawradStatusCell> { visible: false, cursor: Hand, lsp_icon = <CrawradLspIcon> {}, lbl_lsp = <CrawradStatusText> { text: "" } }
                    cell_lsp_progress = <CrawradStatusCell> { visible: false, width: 230.0, lbl_lsp_progress = <CrawradStatusText> { width: Fill, draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) }, text: "" } }

                    cell_message = <CrawradStatusCell> {
                        width: Fill,
                        opened_file_label = <CrawradStatusText> { text: "" }
                    }

                    cell_selection = <CrawradStatusCell> {
                        // Hidden: the Object Inspector header names the selection (MCP still reads it).
                        visible: false,
                        status_nodes_label = <CrawradStatusText> { text: "" }
                    }
                    cell_nodes = <CrawradStatusCell> {
                        lbl_nodes = <CrawradStatusText> { text: "" }
                    }
                    cell_rev = <CrawradStatusCell> {
                        lbl_rev = <CrawradStatusText> { text: "" }
                    }
                    cell_zoom = <CrawradStatusCell> {
                        ic_zoom = <CrawradStatusIcon> { text: "" }
                        lbl_zoom = <CrawradStatusText> { text: "" }
                    }
                    // The framework's credit keeps its accent pill: moved down, not dimmed.
                    cell_framework = <CrawradStatusCell> {
                        pill = <View> {
                            width: Fit, height: 16.0, align: { y: 0.5 }, show_bg: true,
                            padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M) },
                            draw_bg: { color: (CRAWRAD_SELECAO) },
                            lbl_framework = <CrawradStatusText> { draw_text: { color: (CRAWRAD_ACENTO_TEXTO) }, text: "" }
                        }
                    }
                }

            }

            menu_popup = <CrawradMenuPopup> {}
            // The submenu beside it: the same panel, its own list.
            menu_sub_popup = <CrawradMenuPopup> {}


            align_pad = <CrawradInspectorPad> {
                visible: false,
                abs_pos: vec2(0.0, 0.0),

                align_paper = <CrawradAlignPaper> {
                    zone_top = <CrawradAlignZone> {
                        width: Fill, height: (CRAWRAD_ALIGN_FAIXA)
                    }
                    align_middle = <View> {
                        width: Fill, height: Fill,
                        flow: Right,
                        spacing: (CRAWRAD_ESPACO_XS)
                        zone_left = <CrawradAlignZone> {
                            width: (CRAWRAD_ALIGN_FAIXA), height: Fill
                        }
                        zone_client = <CrawradAlignZone> {
                            width: Fill, height: Fill
                        }
                        zone_right = <CrawradAlignZone> {
                            width: (CRAWRAD_ALIGN_FAIXA), height: Fill
                        }
                    }
                    zone_bottom = <CrawradAlignZone> {
                        width: Fill, height: (CRAWRAD_ALIGN_FAIXA)
                    }
                }
                zone_none = <CrawradButton> { width: Fill, text: "" }
            }

            // CORNER CARD: live progress and lingering verdict of background builds
            <View> {
                width: Fill, height: Fill,
                align: { x: 1.0, y: 1.0 },
                padding: { right: 16.0, bottom: 36.0 },
                build_card = <CrawradProgressCard> { visible: false }
            }

            items_dialog = <CrawradItemsDialog> {}
            lines_dialog = <CrawradLinesDialog> {}
            // RENAME SYMBOL: the new name, then the preview.
            rename_dialog = <CrawradRenameDialog> {}
            component_rename_dialog = <CrawradComponentRenameDialog> {}
            actions_dialog = <CrawradActionsDialog> {}
            standard_dialog = <CrawradStandardDialog> {}
            imagelist_dialog = <CrawradImagelistDialog> {}
            addicon_dialog = <CrawradAddiconDialog> {}
            quit_dialog = <CrawradQuitDialog> {}
            // The IDE's one tooltip, styled as the generated app's.
            ide_tooltip = <Tooltip> {
                content: <View> {
                    flow: Overlay, width: Fit, height: Fit,
                    <RoundedView> {
                        width: Fit, height: Fit,
                        padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 4.0 },
                        draw_bg: { color: (CRAWRAD_FLUTUANTE), border_size: 1.0, border_color: (CRAWRAD_BORDA_CONTROLE), border_radius: 3.0 }
                        tooltip_label = <Label> {
                            width: Fit,
                            draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }, color: (CRAWRAD_TEXTO_PRIMARIO) }
                        }
                    }
                }
            }
            form_check_tip = <FormCheckTip> {}
            recover_dialog = <CrawradRecoverDialog> {}
            options_dialog = <CrawradOptionsDialog> {}
            project_options_dialog = <CrawradProjectOptionsDialog> {}
            color_dialog = <CrawradColorDialog> {}
            newproject_dialog = <CrawradNewprojectDialog> {}
            creating_dialog = <CrawradCreatingDialog> {}
            icons_dialog = <CrawradIconsDialog> {}
            memory_dialog = <CrawradMemoryDialog> {}
            environment_dialog = <CrawradEnvironmentDialog> {}
            mcp_dialog = <CrawradMcpDialog> {}
            note_dialog = <CrawradNoteDialog> {}
            goto_dialog = <CrawradGotoDialog> {}
            quick_open_dialog = <CrawradQuickOpenDialog> {}
            inherit_form_dialog = <CrawradInheritFormDialog> {}
            new_component_dialog = <CrawradNewComponentDialog> {}
            build_dialog = <CrawradBuildDialog> {}
            shrink_dialog = <CrawradDialogVeil> {
                dlg = <ShrinkDialogView> {
                    visible: true,
                    show_bg: false,
                }
            }
            permission_dialog = <CrawradPermissionDialog> {}
            templates_dialog = <CrawradTemplatesDialog> {}
        }
    }
}
