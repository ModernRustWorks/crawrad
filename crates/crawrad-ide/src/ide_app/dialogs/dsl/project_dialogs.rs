use makepad_widgets::*;

live_design! {
    link crawrad_dialogs;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use link::crawrad::*;

    // New Inherited Form: the base on the left, the new
    // form's name on the right, under a preview of the base.
    InhFormRow = <View> {
        width: Fill, height: (CRAWRAD_LINHA_ALTURA),
        flow: Overlay,
        show_bg: true,
        draw_bg: {
            color: (CRAWRAD_ELEVADO)
            instance picked: 0.0
            uniform picked_color: (CRAWRAD_SELECAO_FORTE)
            uniform mark_color: (CRAWRAD_INHERITED)
            fn pixel(self) -> vec4 {
                let c = mix(self.color, self.picked_color, self.picked);
                let mark = step(self.pos.x * self.rect_size.x, 2.0) * self.picked;
                let o = mix(c, self.mark_color, mark);
                return vec4(o.rgb * o.a, o.a);
            }
        }
        face = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 10.0, right: 10.0 },
            name = <CrawradLabel> { text: "" }
            <CrawradFiller> {}
            note = <CrawradLabelMuted> { text: "" }
            inherits = <CrawradLabel> {
                draw_text: {
                    color: (CRAWRAD_INHERITED)
                    text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
                text: ""
            }
        }
        alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    NpCard = <CrawradFormCard> {
        width: Fill, height: Fit,
        flow: Right, align: { y: 0.5 },
        padding: { left: 10.0, right: 10.0, top: 8.0, bottom: 8.0 },
        spacing: 10.0, cursor: Hand,
        header = { visible: false }
        draw_bg: {
            color: (CRAWRAD_NOVO_CARD_BG),
            border_radius: (CRAWRAD_RAIO),
            border_color: (CRAWRAD_NOVO_CARD_BORDA),
            shadow_color: (CRAWRAD_TRANSPARENTE),
        }
    }
    NpCardIconBox = <RoundedView> {
        width: 38.0, height: 38.0,
        align: { x: 0.5, y: 0.5 },
        draw_bg: { color: (CRAWRAD_NOVO_ICONE_BG), border_radius: (CRAWRAD_RAIO) }
    }
    NpCardIcon = <CrawradLabel> {
        draw_text: { color: (CRAWRAD_NOVO_ICONE_AZUL), text_style: <THEME_FONT_BOLD> { font_size: 14.0 } }
    }
    NpCardTitle = <CrawradLabel> {
        draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: 11.0 } }
    }
    NpCardFav = <CrawradButton> {
        width: 20.0, height: 18.0,
        padding: 0.,
        draw_bg: { color: (CRAWRAD_TRANSPARENTE), border_size: 0.0 }
        draw_text: { color: (CRAWRAD_NOVO_ESTRELA), text_style: <THEME_FONT_BOLD> { font_size: 10.0 } }
    }
    NpCardDesc = <CrawradLabel> {
        width: Fill, height: Fit,
        draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }
    }
    NpCardPlat = <CrawradLabel> {
        width: Fit, height: Fit,
        draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: 11.0 } }
    }
    NpFwName = <CrawradLabel> {
        draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: 10.0 } }
    }
    NpFwDesc = <CrawradLabel> {
        width: Fill, height: Fit,
        draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO), text_style: <THEME_FONT_REGULAR> { font_size: 8.0 } }
    }
            // 14. NEW PROJECT — reached by the catalogue action `project.new`. No
            // project type nor framework is written here: the catalogues declare them.
            pub CrawradNewprojectDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_NOVO_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        height: 38.0,
                        <View> {
                            width: Fit, height: Fit,
                            flow: Right, align: { y: 0.5 },
                            spacing: (CRAWRAD_ESPACO_M),
                            np_logo = <CrawradLabel> {
                                draw_text: {
                                    color: (CRAWRAD_ERRO)
                                    text_style: <THEME_FONT_BOLD> { font_size: 13.0 }
                                }
                                text: "🦀"
                            }
                            np_title = <CrawradDialogTitle> { text: "New Project" }
                        }
                        <CrawradFiller> {}
                        np_busca = <CrawradCampoDenso> {
                            width: 220.0, height: 26.0,
                        }
                        <View> { width: (CRAWRAD_ESPACO_M), height: 1.0 }
                        np_close = <CrawradCloseButton> {}
                    }
                    <CrawradDialogBody> {
                        height: (CRAWRAD_NOVO_CORPO),
                        padding: { left: 12.0, right: 12.0, top: 10.0, bottom: 10.0 },
                        spacing: (CRAWRAD_ESPACO_M),
                        // MAIN BODY: 3 COLUMNS
                        <View> {
                            width: Fill, height: 475.0,
                            flow: Right,
                            spacing: 14.0,
                            // COLUMN 1: Categories (175px)
                            <View> {
                                width: 175.0, height: Fill,
                                flow: Down,
                                spacing: 2.0,
                                // The palette's own ListBox, its count at each row's end
                                np_cat_list = <CrawradListBox> {
                                    width: Fill, height: Fill,
                                    draw_bg: { color: (CRAWRAD_TRANSPARENTE) }
                                    draw_detail: { color: (CRAWRAD_NOVO_ESTRELA_MUTED) }
                                    detail_color_selected: (CRAWRAD_TEXTO_ENFASE)
                                    list = {
                                        row = { height: 32.0, padding: { left: 10.0, right: 10.0 }, draw_bg: { color: (CRAWRAD_TRANSPARENTE), color_hover: (CRAWRAD_NOVO_CARD_BG), color_focus: (CRAWRAD_TRANSPARENTE), border_radius: (CRAWRAD_RAIO) }, draw_text: { color: (CRAWRAD_NOVO_CAT_TEXTO), color_hover: (CRAWRAD_TEXTO_ENFASE), color_focus: (CRAWRAD_NOVO_CAT_TEXTO) } }
                                        row_selected = { height: 32.0, padding: { left: 10.0, right: 10.0 }, draw_bg: { color: (CRAWRAD_ACENTO), color_hover: (CRAWRAD_ACENTO), color_down: (CRAWRAD_ACENTO), color_focus: (CRAWRAD_ACENTO), border_radius: (CRAWRAD_RAIO) } }
                                    }
                                }
                            }
                            // COLUMN 2: Template cards (460px with scroll)
                            np_templates_scroll = <ScrollYView> {
                                width: 460.0, height: Fill,
                                flow: Down,
                                spacing: 6.0,
                                // Card 0: GUI Application
                                np_card_0 = <NpCard> {
                                    <NpCardIconBox> { np_c0_icon = <NpCardIcon> { text: "💻" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c0_title = <NpCardTitle> { text: "GUI Application" }
                                            np_c0_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c0_desc = <NpCardDesc> { text: "A desktop application with a window, built with Makepad and Rust." }
                                    }
                                    np_c0_plat = <NpCardPlat> { text: "⊞ 🐧 🍎" }
                                }
                                // Card 1: Console Application
                                np_card_1 = <NpCard> {
                                    <NpCardIconBox> { np_c1_icon = <NpCardIcon> { text: ">_" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c1_title = <NpCardTitle> { text: "Console Application" }
                                            np_c1_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c1_desc = <NpCardDesc> { text: "A command-line application. No UI, fast and lightweight." }
                                    }
                                    np_c1_plat = <NpCardPlat> { text: "⊞ 🐧 🍎" }
                                }
                                // Card 2: Library Crate
                                np_card_2 = <NpCard> {
                                    <NpCardIconBox> { np_c2_icon = <NpCardIcon> { text: "📦" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c2_title = <NpCardTitle> { text: "Library Crate" }
                                            np_c2_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c2_desc = <NpCardDesc> { text: "A reusable library crate to share code with other projects." }
                                    }
                                    np_c2_plat = <NpCardPlat> { text: "⊞ 🐧 🍎" }
                                }
                                // Card 3: Web App (WASM)
                                np_card_3 = <NpCard> {
                                    <NpCardIconBox> { np_c3_icon = <NpCardIcon> { text: "🌐" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c3_title = <NpCardTitle> { text: "Web App (WASM)" }
                                            np_c3_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c3_desc = <NpCardDesc> { text: "A web application compiled to WebAssembly, running in browser." }
                                    }
                                    np_c3_plat = <NpCardPlat> { text: "WA 🌐" }
                                }
                                // Card 4: Mobile App
                                np_card_4 = <NpCard> {
                                    <NpCardIconBox> { np_c4_icon = <NpCardIcon> { text: "📱" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c4_title = <NpCardTitle> { text: "Mobile App" }
                                            np_c4_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c4_desc = <NpCardDesc> { text: "A mobile application for Android and iOS using Makepad." }
                                    }
                                    np_c4_plat = <NpCardPlat> { text: "🤖 🍎" }
                                }
                                // Card 5: Multi-window Desktop
                                np_card_5 = <NpCard> {
                                    <NpCardIconBox> { np_c5_icon = <NpCardIcon> { text: "🖥" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c5_title = <NpCardTitle> { text: "Multi-window Desktop" }
                                            np_c5_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c5_desc = <NpCardDesc> { text: "A desktop app with multiple windows and advanced window management." }
                                    }
                                    np_c5_plat = <NpCardPlat> { text: "⊞ 🐧 🍎" }
                                }

                                // Card 6: Package / Plugin
                                np_card_6 = <NpCard> {
                                    <NpCardIconBox> { np_c6_icon = <NpCardIcon> { text: "🧩" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c6_title = <NpCardTitle> { text: "Package / Plugin" }
                                            np_c6_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c6_desc = <NpCardDesc> { text: "A package, plugin or add-on for CrawRAD or other applications." }
                                    }
                                    np_c6_plat = <NpCardPlat> { text: "⊞ 🐧 🍎" }
                                }

                                // Card 7: Custom Template
                                np_card_7 = <NpCard> {
                                    <NpCardIconBox> { np_c7_icon = <NpCardIcon> { text: "📄" } }
                                    <View> {
                                        width: Fill, height: Fit, flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_c7_title = <NpCardTitle> { text: "Custom Template" }
                                            np_c7_fav = <NpCardFav> { text: "☆" }
                                        }
                                        np_c7_desc = <NpCardDesc> { text: "Start from a custom template or example." }
                                    }
                                    np_c7_plat = <NpCardPlat> { text: "⚙" }
                                }
                            }

                            // COLUMN 3: Details panel (Fill with scroll)
                            np_det_scroll = <ScrollYView> {
                                width: Fill, height: Fill,
                                flow: Down,
                                spacing: 6.0,
                                padding: { right: 6.0 },

                                // Top card
                                <RoundedView> {
                                    width: Fill, height: Fit,
                                    flow: Right, align: { y: 0.5 },
                                    padding: { left: 10.0, right: 10.0, top: 6.0, bottom: 6.0 },
                                    spacing: 10.0,
                                    draw_bg: {
                                        color: (CRAWRAD_NOVO_CARD_BG),
                                        border_radius: (CRAWRAD_RAIO),
                                        border_size: 1.0,
                                        border_color: (CRAWRAD_NOVO_CARD_BORDA),
                                    }
                                    <RoundedView> {
                                        width: 38.0, height: 38.0,
                                        align: { x: 0.5, y: 0.5 },
                                        draw_bg: { color: (CRAWRAD_NOVO_ICONE_BG), border_radius: (CRAWRAD_RAIO) }
                                        np_det_icon = <CrawradLabel> {
                                            draw_text: {
                                                color: (CRAWRAD_NOVO_ICONE_AZUL)
                                                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                                            }
                                            text: "💻"
                                        }
                                    }
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down, spacing: 2.0,
                                        <View> {
                                            width: Fill, height: Fit,
                                            flow: Right, align: { y: 0.5 }, spacing: 6.0,
                                            np_det_title = <CrawradLabel> {
                                                draw_text: {
                                                    color: (CRAWRAD_TEXTO_ENFASE)
                                                    text_style: <THEME_FONT_BOLD> { font_size: 11.5 }
                                                }
                                                text: "GUI Application"
                                            }
                                            np_det_star = <CrawradLabel> {
                                                draw_text: {
                                                    color: (CRAWRAD_NOVO_ESTRELA)
                                                    text_style: <THEME_FONT_BOLD> { font_size: 10.5 }
                                                }
                                                text: "☆"
                                            }
                                        }
                                        np_det_sub = <CrawradLabel> {
                                            width: Fill, height: Fit,
                                            draw_text: {
                                                color: (CRAWRAD_TEXTO_TERCIARIO)
                                                text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                            }
                                            text: "A desktop application with a window, built with Makepad and Rust."
                                        }
                                    }
                                }

                                np_det_desc = <CrawradLabel> {
                                    width: Fill, height: Fit,
                                    draw_text: {
                                        color: (CRAWRAD_TEXTO_SECUNDARIO)
                                        text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                    }
                                    text: ""
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down, spacing: 3.0,
                                    <CrawradLabel> {
                                        text: "Platforms"
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_ENFASE)
                                            text_style: <THEME_FONT_BOLD> { font_size: 9.5 }
                                        }
                                    }
                                    np_det_plat = <CrawradLabel> {
                                        width: Fill, height: Fit,
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_SECUNDARIO)
                                            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                                        }
                                        text: "⊞ Windows      🐧 Linux      🍎 macOS"
                                    }
                                }

                                // THE UI FRAMEWORK — only for a template with
                                // a window, chosen once, kept for the project's life
                                np_fw_section = <View> {
                                    width: Fill, height: Fit,
                                    flow: Down, spacing: 4.0,
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right, align: { y: 0.5 }, spacing: 8.0,
                                        <CrawradLabel> {
                                            text: "UI framework"
                                            draw_text: {
                                                color: (CRAWRAD_TEXTO_ENFASE)
                                                text_style: <THEME_FONT_BOLD> { font_size: 9.5 }
                                            }
                                        }
                                        <CrawradLabel> {
                                            text: "chosen once — the project keeps it"
                                            draw_text: {
                                                color: (CRAWRAD_NOVO_ESTRELA)
                                                text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
                                            }
                                        }
                                    }
                                    // A list that grows with `crawrad_project::frameworks`, and
                                    // the detail of the selected entry — rows filled by the code
                                    <View> {
                                        width: Fill, height: Fit, flow: Right, spacing: 8.0,
                                        // The palette's own ListBox, themed by the IDE's tokens
                                        np_fw_list = <CrawradListBox> {
                                            width: 140, height: 52.0,
                                            draw_bg: { color: (CRAWRAD_TRANSPARENTE) }
                                            list = {
                                                row = { height: 26.0, draw_bg: { color: (CRAWRAD_TRANSPARENTE), color_hover: (CRAWRAD_NOVO_CARD_BG), color_focus: (CRAWRAD_TRANSPARENTE) }, draw_text: { color: (CRAWRAD_TEXTO_ENFASE) } }
                                                row_selected = { height: 26.0, draw_bg: { color: (CRAWRAD_NOVO_CARD_SEL), color_hover: (CRAWRAD_NOVO_CARD_SEL), color_focus: (CRAWRAD_NOVO_CARD_SEL) } }
                                            }
                                        }
                                        <View> {
                                            width: Fill, height: Fit, flow: Down, spacing: 3.0,
                                            np_fw_det_summary = <NpFwDesc> {}
                                            np_fw_det_platforms = <NpFwDesc> {}
                                            np_fw_det_crate = <NpFwDesc> {}
                                        }
                                    }
                                }
                                np_fw_none = <View> {
                                    width: Fill, height: Fit,
                                    <CrawradLabel> {
                                        width: Fill, height: Fit,
                                        text: "No UI framework: this template has no window."
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_TERCIARIO)
                                            text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                        }
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down, spacing: 3.0,
                                    <CrawradLabel> {
                                        text: "Project structure"
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_ENFASE)
                                            text_style: <THEME_FONT_BOLD> { font_size: 9.5 }
                                        }
                                    }
                                    <RoundedView> {
                                        width: Fill, height: Fit,
                                        padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 4.0 },
                                        draw_bg: {
                                            color: (CRAWRAD_NOVO_TREE_BG),
                                            border_radius: (CRAWRAD_RAIO),
                                            border_size: 1.0,
                                            border_color: (CRAWRAD_NOVO_CARD_BORDA),
                                        }
                                        np_det_tree = <CrawradLabel> {
                                            width: Fill, height: Fit,
                                            draw_text: {
                                                color: (CRAWRAD_TEXTO_SECUNDARIO)
                                                text_style: <THEME_FONT_CODE> { font_size: 8.0 }
                                            }
                                            text: ""
                                        }
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down, spacing: 3.0,
                                    <CrawradLabel> {
                                        text: "Template options"
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_ENFASE)
                                            text_style: <THEME_FONT_BOLD> { font_size: 9.5 }
                                        }
                                    }
                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down, spacing: 3.0,
                                        np_opt_example = <CrawradOptionCheck> { text: "Include example UI (counter app)" }
                                        np_opt_livereload = <CrawradOptionCheck> { text: "Use Makepad live-reload (dev mode)" }
                                        np_opt_menu = <CrawradOptionCheck> { text: "Add basic application menu" }
                                        np_opt_tracing = <CrawradOptionCheck> { text: "Enable tracing (logging)" }
                                    }
                                }
                            }
                        }

                        // DIVIDER LINE
                        <View> { width: Fill, height: 1.0, draw_bg: { color: (CRAWRAD_NOVO_CARD_BORDA) } }

                        // CONFIGURATION PANEL (INTEGRATED FOOTER)
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8.0,

                            // Input row 1: Name, Location + Browse
                            <View> {
                                width: Fill, height: Fit,
                                flow: Right,
                                spacing: 12.0,

                                <View> {
                                    width: 320.0, height: Fit,
                                    flow: Down, spacing: 3.0,
                                    <CrawradLabel> {
                                        text: "Project name"
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_SECUNDARIO)
                                            text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                        }
                                    }
                                    np_nome = <CrawradCampoDenso> {
                                        width: Fill, height: 26.0,
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down, spacing: 3.0,
                                    <CrawradLabel> {
                                        text: "Location"
                                        draw_text: {
                                            color: (CRAWRAD_TEXTO_SECUNDARIO)
                                            text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                        }
                                    }
                                    <View> {
                                        width: Fill, height: 26.0,
                                        flow: Right, spacing: 6.0,
                                        np_loc = <CrawradCampoDenso> {
                                            width: Fill, height: 26.0,
                                        }
                                        np_browse = <CrawradButton> {
                                            width: 34.0, height: 26.0,
                                            padding: 0.,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                                            }
                                            text: "📁"
                                        }
                                    }
                                }
                            }

                            // Control row 2: Git Init, Template Source, Action buttons
                            <View> {
                                width: Fill, height: 32.0,
                                flow: Right, align: { y: 0.5 },
                                spacing: 12.0,

                                np_git_init = <CrawradOptionCheck> { width: Fit, text: "Create git repository" }

                                <View> { width: 1.0, height: 16.0, draw_bg: { color: (CRAWRAD_DIVISOR) } }

                                <CrawradLabel> {
                                    text: "Template source:"
                                    draw_text: {
                                        color: (CRAWRAD_TEXTO_TERCIARIO)
                                        text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                    }
                                }
                                np_tpl_source = <CrawradButton> {
                                    width: 165.0, height: 26.0,
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
                                    }
                                    text: "Built-in Templates ▾"
                                }

                                <CrawradFiller> {}

                                np_escolher = <CrawradPrimaryButton> {
                                    width: Fit, height: 30.0, padding: { left: 14.0, right: 14.0 },
                                    text: "Create Project"
                                }

                                np_cancel = <CrawradButton> {
                                    width: 85.0, height: 30.0,
                                    text: "Cancel"
                                }
                            }

                            np_recusa = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        }
                    }
                }
            }

            pub CrawradCreatingDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_CRIANDO_DLG_L), height: Fit,

                    <CrawradDialogTitleBar> {
                        cr_title = <CrawradDialogTitle> { text: "Creating project" }
                    }

                    <CrawradDialogBody> {
                        height: (CRAWRAD_CRIANDO_CORPO),
                        cr_onde = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        cr_passo = <CrawradLabel> {
                            width: Fill, height: Fit,
                            draw_text: {
                                text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CABECALHO) }
                            }
                            text: ""
                        }
                        cr_tempo = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                        cr_desfecho = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        cr_fechar = <CrawradDialogButton> { text: "Hide" }
                    }
                }
            }

            pub CrawradInheritFormDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 700.0, height: Fit,

                    <CrawradDialogTitleBar> {
                        <CrawradDialogTitle> { text: "New Inherited Form" }
                        <CrawradFiller> {}
                        inh_close = <CrawradCloseButton> {}
                    }

                    <CrawradDialogBody> {
                        height: 440.0,
                        <CrawradLabelMuted> {
                            width: Fill, height: Fit,
                            text: "Pick the base form. The new form gets its components and handlers, and saves only what you change."
                        }
                        <View> {
                            width: Fill, height: Fill,
                            flow: Right, spacing: 14.0,
                            <View> {
                                width: 300.0, height: Fill,
                                flow: Down, spacing: (CRAWRAD_ESPACO_S),
                                inh_filter = <CrawradTextInput> {
                                    width: Fill, height: 30.0,
                                    empty_text: "Filter forms"
                                }
                                <RoundedView> {
                                    width: Fill, height: Fill,
                                    flow: Down,
                                    padding: { top: 4.0, bottom: 4.0 },
                                    draw_bg: { color: (CRAWRAD_ELEVADO), border_radius: 4.0 }
                                    <CrawradLabelMuted> {
                                        padding: { left: 10.0, top: 6.0, bottom: 6.0 },
                                        draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                                        text: "FORMS IN THIS PROJECT"
                                    }
                                    inh_list = <PortalList> {
                                        width: Fill, height: Fill,
                                        scroll_bar: <CrawradScrollBar> {}
                                        FormRow = <InhFormRow> {}
                                    }
                                }
                            }
                            <View> {
                                width: Fill, height: Fill,
                                flow: Down, spacing: 8.0,
                                inh_preview = <DesignCanvas> { width: Fill, height: 170.0 }
                                inh_summary = <CrawradLabel> { width: Fill, height: Fit, text: "" }
                                inh_parts = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                                <CrawradLabelMuted> { margin: { top: 4.0 }, text: "Form name" }
                                inh_name = <CrawradTextInput> { width: Fill, height: 30.0 }
                                inh_problem = <CrawradLabel> {
                                    width: Fill, height: Fit,
                                    draw_text: { color: (CRAWRAD_AVISO) }
                                    text: ""
                                }
                                inh_files = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                                inh_struct = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                            }
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        inh_cancel = <CrawradDialogButton> { text: "Cancel" }
                        inh_create = <CrawradDialogPrimaryButton> { text: "Create" }
                    }
                }
            }

            NcPropRow = <View> {
                width: Fill, height: 28.0,
                flow: Right, spacing: 6.0, align: { y: 0.5 },
                nc_pname = <CrawradTextInput> { width: 140.0, height: 24.0, empty_text: "name" }
                nc_pty = <CrawradTextInput> { width: 90.0, height: 24.0, empty_text: "i64" }
                nc_pdefault = <CrawradTextInput> { width: 100.0, height: 24.0, empty_text: "default" }
                nc_pcat = <CrawradTextInput> { width: 100.0, height: 24.0, empty_text: "behavior" }
                nc_phint = <CrawradTextInput> { width: 180.0, height: 24.0, empty_text: "hint" }
                nc_pdel = <CrawradCloseButton> { margin: { left: 4.0 } }
            }

            pub CrawradNewComponentDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 760.0, height: Fit,

                    <CrawradDialogTitleBar> {
                        nc_title = <CrawradDialogTitle> { text: "New Component" }
                        <CrawradFiller> {}
                        nc_step_label = <CrawradLabelMuted> { text: "Step 1 of 2" }
                        nc_close = <CrawradCloseButton> { margin: { left: 10.0 } }
                    }

                    <CrawradDialogBody> {
                        height: 440.0,
                        flow: Overlay,

                        step1_view = <View> {
                            width: Fill, height: Fill,
                            flow: Down, spacing: 14.0,

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down, spacing: 4.0,
                                <CrawradLabelMuted> { text: "Name" }
                                nc_name = <CrawradTextInput> { width: 280.0, height: 30.0 }
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Right, spacing: 12.0,
                                <RoundedView> {
                                    width: 330.0, height: 110.0,
                                    flow: Down, spacing: 6.0, padding: 12.0,
                                    draw_bg: { color: (CRAWRAD_ELEVADO), border_size: 1.0, border_color: (CRAWRAD_SELECAO_FORTE), border_radius: 4.0 }
                                    <CrawradLabel> {
                                        draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }
                                        text: "Inherit a component"
                                    }
                                    <CrawradLabelMuted> {
                                        width: Fill, height: Fit,
                                        text: "Starts as a copy of one from the palette and keeps all of its properties and events."
                                    }
                                    nc_base = <CrawradOptionDrop> { width: Fill }
                                }

                                <RoundedView> {
                                    width: 330.0, height: 110.0,
                                    flow: Down, spacing: 6.0, padding: 12.0,
                                    draw_bg: { color: (CRAWRAD_GROUND), border_size: 1.0, border_color: (CRAWRAD_BORDA_CONTROLE), border_radius: 4.0 }
                                    <CrawradLabelMuted> {
                                        draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }
                                        text: "Compose several (Coming later)"
                                    }
                                    <CrawradLabelMuted> {
                                        width: Fill, height: Fit,
                                        text: "A panel you lay out in the Designer, like a form. Dropped on a form, it moves as one piece."
                                    }
                                }
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down, spacing: 6.0,
                                <CrawradLabelMuted> { text: "Available in" }
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right, spacing: 16.0,
                                    nc_scope_project = <CrawradOptionRadio> { text: "This project" }
                                    nc_scope_mine = <CrawradOptionRadio> { text: "All my projects" }
                                }
                                nc_scope_hint = <CrawradMonoLabelMuted> { text: "" }
                            }
                        }

                        step2_view = <View> {
                            width: Fill, height: Fill,
                            flow: Down, spacing: 10.0, visible: false,

                            <View> {
                                width: Fill, height: Fit,
                                flow: Right, align: { y: 0.5 },
                                <CrawradLabel> {
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }
                                    text: "Properties"
                                }
                                <CrawradFiller> {}
                                nc_add_prop = <CrawradButton> { text: "+ Property" }
                            }

                            <View> {
                                width: Fill, height: 200.0,
                                flow: Down,
                                <View> {
                                    width: Fill, height: 24.0,
                                    flow: Right, spacing: 6.0,
                                    <CrawradLabelMuted> { width: 140.0, text: "Name" }
                                    <CrawradLabelMuted> { width: 90.0, text: "Type" }
                                    <CrawradLabelMuted> { width: 100.0, text: "Default" }
                                    <CrawradLabelMuted> { width: 100.0, text: "Category" }
                                    <CrawradLabelMuted> { width: 180.0, text: "Hint" }
                                }
                                nc_props_list = <PortalList> {
                                    width: Fill, height: Fill,
                                    scroll_bar: <CrawradScrollBar> {}
                                    PropRow = <NcPropRow> {}
                                }
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down, spacing: 4.0,
                                <CrawradLabelMuted> { text: "What the project gets" }
                                <RoundedView> {
                                    width: Fill, height: Fit,
                                    padding: 8.0,
                                    draw_bg: { color: (CRAWRAD_GROUND), border_size: 1.0, border_color: (CRAWRAD_BORDA_CONTROLE), border_radius: 4.0 }
                                    nc_gets = <CrawradMonoLabelMuted> {
                                        text: "Cargo.toml\nsrc/lib.rs\nmanifest.json"
                                    }
                                }
                            }
                        }
                    }

                    <CrawradDialogFooter> {
                        nc_error = <CrawradLabel> {
                            width: 380.0, height: Fit,
                            draw_text: { color: (CRAWRAD_AVISO) }
                            text: ""
                        }
                        <CrawradFiller> {}
                        nc_cancel = <CrawradDialogButton> { text: "Cancel" }
                        nc_back = <CrawradDialogButton> { text: "Back" }
                        nc_next = <CrawradDialogPrimaryButton> { text: "Next" }
                        nc_create = <CrawradDialogPrimaryButton> { text: "Create" }
                    }
                }
            }

    // Project › Options… (approved mockup): pages on the left, the page on the right.
    pub CrawradPoName = <CrawradLabel> { width: 140.0, text: "" }
    pub CrawradPoFrame = <RoundedView> { width: Fit, height: Fit, padding: 12.0, flow: Right, spacing: 16.0, align: { y: 1.0 }, show_bg: true,
        draw_bg: { color: (CRAWRAD_GROUND), border_size: 1.0, border_color: (CRAWRAD_BORDA_CONTROLE), border_radius: 4.0 } }
    pub CrawradPoHead = <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }, text: "" }
    pub CrawradPoChipButton = <CrawradButton> { padding: { left: 6.0, right: 6.0, top: 3.0, bottom: 3.0 }, draw_bg: { color: (CRAWRAD_LOG_FILL), color_hover: (CRAWRAD_LOG_FILL), color_focus: (CRAWRAD_LOG_FILL), border_color_1: (CRAWRAD_LOG_BORDER), border_color_2: (CRAWRAD_LOG_BORDER), border_color_1_hover: (CRAWRAD_LOG_TEXT), border_color_2_hover: (CRAWRAD_LOG_TEXT), border_color_1_focus: (CRAWRAD_LOG_BORDER), border_color_2_focus: (CRAWRAD_LOG_BORDER) }, draw_text: { color: (CRAWRAD_LOG_TEXT), color_hover: (CRAWRAD_LOG_TEXT), color_focus: (CRAWRAD_LOG_TEXT), text_style: <THEME_FONT_CODE> { font_size: 8.5 } } }
    pub CrawradPoStep = <CrawradFieldRow> { n = <CrawradLabelMuted> { width: 20.0, text: "" } what = <CrawradMonoLabel> { width: 250.0, text: "" } why = <CrawradLabelMuted> { text: "" } }
    pub CrawradProjectOptionsDialog = <CrawradDialogVeil> { dlg = <CrawradDialog> { width: (CRAWRAD_OPTIONS_DLG_L), height: Fit,
        <CrawradDialogTitleBar> { popt_title = <CrawradDialogTitle> { text: "Project Options" } <CrawradFiller> {} popt_close = <CrawradCloseButton> {} }
        <CrawradDialogBody> { height: 560.0, flow: Right, spacing: (CRAWRAD_ESPACO_L)
            <View> { width: (CRAWRAD_OPTIONS_ARVORE), height: Fill, flow: Down
                popt_page_app = <CrawradItemsRow> { rotulo = { text: "Application" } }
                popt_page_log = <CrawradItemsRow> { rotulo = { text: "Log" } }
                <CrawradLabelMuted> { padding: { left: (CRAWRAD_ESPACO_M), top: 6.0 }, text: "… later pages" } }
            <View> { width: Fill, height: Fill, flow: Down, spacing: 8.0 <View> { width: Fill, height: Fill, flow: Overlay
                popt_view_app = <View> { width: Fill, height: Fill, flow: Down, spacing: 12.0
                    <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_ENFASE), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_TITULO) } }, text: "Application" }
                    <View> { width: Fill, height: Fit, align: { y: 0.5 } <CrawradPoName> { text: "Title" } popt_title_input = <CrawradPropInput> { width: Fill, height: 28.0 } }
                    <View> { width: Fill, height: Fit, spacing: 16.0 <CrawradPoName> { text: "Icon" }
                        <CrawradPoFrame> { popt_icon_256 = <Image> { width: 132.0, height: 132.0 } }
                        <View> { width: Fill, height: Fit, flow: Down, spacing: 8.0
                            <View> { width: Fit, height: Fit, spacing: 8.0 popt_load_btn = <CrawradButton> { text: "Load icon…" } popt_default_btn = <CrawradButton> { text: "Default" } }
                            popt_caption_lbl = <CrawradLabelMuted> { text: "CrawRAD crab (default)" }
                            <CrawradLabelMuted> { width: Fill, text: "Accepts .ico, or a .png (CrawRAD makes the 16·32·48·256 .ico from it). Saved in the project folder as icon.ico." }
                            <CrawradPoFrame> { popt_icon_48 = <Image> { width: 48.0, height: 48.0 } popt_icon_32 = <Image> { width: 32.0, height: 32.0 }
                                popt_icon_16 = <Image> { width: 16.0, height: 16.0 } <CrawradLabelMuted> { text: "48 · 32 · 16" } } } }
                    <CrawradLabel> { width: Fill, text: "•  Windows: the icon goes into the .exe (Explorer, taskbar, Alt+Tab).\n•  Every OS: the window shows it in its title bar / dock while the app runs (Makepad and egui).\n•  A new project starts with the crab; \"Default\" brings it back." } }
                popt_view_log = <View> { width: Fill, height: Fill, flow: Down, spacing: 8.0, visible: false
                    plog_enabled = <CrawradOptionCheck> { text: "Write the log points to a file" }
                    <CrawradPoHead> { text: "File name" }
                    plog_name = <CrawradPropInput> { width: Fill, height: 28.0 }
                    <CrawradFieldRow> { <CrawradLabelMuted> { text: "Insert:" } plog_m0 = <CrawradPoChipButton> { text: "{app}" } plog_m1 = <CrawradPoChipButton> { text: "{date}" }
                        plog_m2 = <CrawradPoChipButton> { text: "{time}" } plog_m3 = <CrawradPoChipButton> { text: "{pid}" } plog_m4 = <CrawradPoChipButton> { text: "{user}" } <CrawradFiller> {} plog_today = <CrawradMonoLabelMuted> { width: Fit, text: "" } }
                    <CrawradPoHead> { text: "Folder" }
                    plog_f0 = <CrawradOptionRadio> { width: Fit, text: "Next to the executable" }
                    <CrawradFieldRow> { plog_f1 = <CrawradOptionRadio> { width: Fit, text: "The user's data folder" } plog_user_dir = <CrawradMonoLabelMuted> { text: "" } }
                    <CrawradFieldRow> { plog_f2 = <CrawradOptionRadio> { width: Fit, text: "Custom" } plog_custom = <CrawradPropInput> { width: Fill, height: 28.0 } plog_browse = <CrawradButton> { text: "Browse…" } }
                    <View> { width: Fill, height: Fit, spacing: 24.0
                        <View> { width: Fill, height: Fit, flow: Down, spacing: 6.0 <CrawradPoHead> { text: "Minimum level" }
                            <CrawradFieldRow> { <CrawradLabelMuted> { width: 64.0, text: "Debug" } plog_debug = <CrawradOptionDrop> { width: 120.0 } }
                            <CrawradFieldRow> { <CrawradLabelMuted> { width: 64.0, text: "Release" } plog_release = <CrawradOptionDrop> { width: 120.0 } } }
                        <View> { width: Fill, height: Fit, flow: Down, spacing: 6.0 <CrawradPoHead> { text: "Files" }
                            <CrawradFieldRow> { <CrawradLabelMuted> { width: 84.0, text: "New file" } plog_new = <CrawradOptionDrop> { width: 120.0 } }
                            <CrawradFieldRow> { <CrawradLabelMuted> { width: 84.0, text: "Keep the last" } plog_keep = <CrawradPropInput> { width: 60.0, height: 28.0 } <CrawradLabelMuted> { text: "files" } } } }
                    <CrawradPoFrame> { width: Fill, flow: Down, spacing: 6.0, align: { y: 0.0 }
                        <CrawradPoHead> { text: "Change it at the customer, without a new build — the first one found wins" }
                        plog_p1 = <CrawradPoStep> { n = { text: "1" } why = { text: "also warn, error or off" } }
                        plog_p2 = <CrawradPoStep> { n = { text: "2" } why = { text: "environment variable" } }
                        plog_p3 = <CrawradPoStep> { n = { text: "3" } why = { text: "beside the executable: level, folder, file name" } }
                        <CrawradPoStep> { n = { text: "4" } what = { text: "This page", draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) } } } why = { text: "built into the executable" } }
                        plog_sample = <CrawradButton> { text: "" } } } }
                popt_error_wrap = <View> { width: Fill, height: Fit, visible: false
                    popt_error_lbl = <CrawradLabel> { width: Fill, draw_text: { color: (CRAWRAD_ERRO) } } } } }
        <CrawradDialogFooter> { <CrawradFiller> {} popt_ok = <CrawradDialogPrimaryButton> { text: "OK" } popt_cancel = <CrawradDialogButton> { text: "Cancel" } } } }
}
