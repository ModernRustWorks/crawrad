use makepad_widgets::*;

live_design! {
    link crawrad_panels;

    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    pub CrawradPanelTerminal = <CrawradPanel> {
        width: Fill, height: Fill,
        flow: Down,

        // Terminal Top Toolbar (Ref 17, 19, 20)
        term_header = <CrawradPanelHeader> {
            btn_term_new = <CrawradButton> { height: 22.0, text: "+ ▾" }
            btn_term_ai = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "💬" }
            btn_term_actions = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "⚡" }
            btn_term_snippets = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "{}" }

            term_hdr_spacer = <CrawradFiller> {}

            btn_term_split_h = <CrawradIconButton> { text: "⊟" }
            btn_term_split_v = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "⊞" }
            btn_term_search = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "🔍" }
            btn_term_trash = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "🗑" }
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        // Terminal Search Bar (Ref 26)
        term_search_bar = <CrawradBand> {
            width: Fill, height: Fit,
            visible: false,
            flow: Right,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },
            align: { y: 0.5 },
            search_input = <CrawradTextInput> { width: Fill, height: 22.0, empty_text: "Find in terminal..." }
            btn_search_prev = <CrawradIconButton> { text: "↑" }
            btn_search_next = <CrawradIconButton> { text: "↓" }
            btn_search_close = <CrawradIconButton> { text: "✕" }
        }

        // Terminal Body: Left Sidebar + Center Shell Area + Right Tabs Sidebar
        term_body = <View> {
            width: Fill, height: Fill,
            flow: Right,

            // Left Actions & Snippets Sidebar (Ref 22, 23)
            term_sidebar_left = <View> {
                width: 180.0, height: Fill,
                visible: false,
                flow: Down,
                padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S) },
                spacing: (CRAWRAD_ESPACO_XS),
                lbl_sidebar_title = <CrawradLabel> { text: "Actions" }
                sidebar_filter = <CrawradTextInput> { height: 22.0, empty_text: "Filter..." }
                btn_act_pull = <CrawradButton> { width: Fill, height: 24.0, text: "Git Pull ▶" }
                btn_act_push = <CrawradButton> { width: Fill, height: 24.0, text: "Git Push ▶" }
                btn_act_build = <CrawradButton> { width: Fill, height: 24.0, text: "Build (Debug) ▶" }
                btn_act_tasks = <CrawradButton> { width: Fill, height: 24.0, text: "List Tasks ▶" }
                btn_act_cargo_test = <CrawradButton> { width: Fill, height: 24.0, text: "Cargo Test ▶" }
            }

            // Center Area
            term_main_area = <View> {
                width: Fill, height: Fill,
                flow: Down,

                // Welcome View (Ref 17)
                term_welcome_view = <View> {
                    width: Fill, height: Fill,
                    flow: Down,
                    align: { x: 0.5, y: 0.2 },
                    spacing: (CRAWRAD_ESPACO_M),

                    term_welcome_logo = <CrawradLabel> {
                        draw_text: { text_style: { font_size: (CRAWRAD_FONTE_CABECALHO) } },
                        text: ">_ Aefos AI Terminal"
                    }
                    term_welcome_sub = <CrawradLabelMuted> {
                        text: "Start a new shell or jump back into a session."
                    }

                    term_cards_row1 = <View> {
                        flow: Right, width: Fit, height: Fit,
                        spacing: (CRAWRAD_ESPACO_M),
                        btn_card_cmd = <CrawradButton> { width: 160.0, height: 48.0, text: "⌘  New cmd\n    cmd.exe" }
                        btn_card_pwsh = <CrawradButton> { width: 160.0, height: 48.0, text: "⚡  New pwsh\n    powershell.exe" }
                    }
                    term_cards_row2 = <View> {
                        flow: Right, width: Fit, height: Fit,
                        spacing: (CRAWRAD_ESPACO_M),
                        btn_card_wsl = <CrawradButton> { width: 160.0, height: 48.0, text: "🐧  New wsl\n    wsl.exe" }
                        btn_card_bash = <CrawradButton> { width: 160.0, height: 48.0, text: "🌿  New git-bash\n    bash.exe -l" }
                    }
                }

                // Active Shell View (Ref 18)
                term_active_view = <View> {
                    width: Fill, height: Fill,
                    visible: false,
                    flow: Down,

                    // Breadcrumb Powerline Bar
                    term_breadcrumb = <CrawradBand> {
                        width: Fill, height: 24.0,
                        flow: Right, align: { y: 0.5 },
                        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },
                        spacing: (CRAWRAD_ESPACO_S),
                        lbl_pl_date = <CrawradLabelMuted> { text: "📅 09-20" }
                        lbl_pl_time = <CrawradLabelMuted> { text: "⏰ 18:09" }
                        lbl_pl_cwd = <CrawradLabel> { text: "📁 D:\\CrawRAD" }
                    }

                    // Terminal Monospace Output Viewport
                    term_output_view = <View> {
                        width: Fill, height: Fill,
                        flow: Down,
                        padding: (CRAWRAD_ESPACO_M),
                        lbl_term_buffer = <CrawradLabel> {
                            width: Fill, height: Fit,
                            text: "Aefos Terminal Ready.\nPS D:\\CrawRAD> "
                        }
                    }

                    // CLI Input Bar
                    term_cli_bar = <CrawradBand> {
                        width: Fill, height: 30.0,
                        flow: Right, align: { y: 0.5 },
                        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },
                        lbl_prompt_sign = <CrawradLabel> { text: "PS > " }
                        cli_prompt_input = <CrawradTextInput> { width: Fill, height: 22.0, empty_text: "Type command..." }
                        btn_cli_run = <CrawradButton> { width: 30.0, height: 22.0, text: "↵" }
                    }
                }

                // Retractable AI Drawer at Bottom (Ref 20)
                term_ai_drawer = <CrawradBand> {
                    width: Fill, height: Fit,
                    visible: false,
                    flow: Right,
                    padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M), top: 6.0, bottom: 6.0 },
                    align: { y: 0.5 },
                    spacing: (CRAWRAD_ESPACO_S),
                    term_ai_input = <CrawradTextInput> {
                        width: Fill, height: 26.0,
                        empty_text: "Message Aefos... (type / for a command)"
                    }
                    btn_term_ai_send = <CrawradButton> { width: 32.0, height: 26.0, text: "↑" }
                }
            }

            // Right Sessions Sidebar (Ref 17, 18, 19)
            term_sessions_sidebar = <View> {
                width: 130.0, height: Fill,
                flow: Down,
                padding: (CRAWRAD_ESPACO_S),
                spacing: (CRAWRAD_ESPACO_XS),
                lbl_sess_header = <CrawradLabelMuted> { text: "SESSIONS" }
                btn_sess_welcome = <CrawradButton> { width: Fill, height: 22.0, text: "● Welcome" }
                btn_sess_pwsh = <CrawradButton> { width: Fill, height: 22.0, text: "● pwsh" }
                btn_sess_cmd = <CrawradButton> { width: Fill, height: 22.0, text: "● cmd" }
                btn_sess_wsl = <CrawradButton> { width: Fill, height: 22.0, text: "● wsl" }
                btn_sess_bash = <CrawradButton> { width: Fill, height: 22.0, text: "● git-bash" }
            }
        }
    }

}
