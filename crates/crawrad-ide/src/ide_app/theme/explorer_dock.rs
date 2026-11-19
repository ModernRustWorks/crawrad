use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    pub CrawradRowGlyph = <CrawradLabel> {
        width: (CRAWRAD_GOTEIRA), height: Fit,
        align: { x: 0.5, y: 0.5 }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO)
            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_ICONE_LINHA) }
        }
        text: ""
    }
    pub CrawradCategoryRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        margin: { top: (CRAWRAD_ESPACO_M), bottom: (CRAWRAD_ESPACO_XS) }
        face = <View> {
        width: Fill, height: Fit,
        flow: Right,
        align: { y: 0.5 }
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA) }
        marca = <CrawradRowGlyph> {}
        nome = <CrawradGroupLabel> {
            margin: { top: 0., bottom: 0. }
        }
        }
        alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }
    // The CHEVRON, which is dimmer than the type drawing: it is an affordance, not information.
    pub CrawradRowChevron = <CrawradRowGlyph> {
        draw_text: { color: (CRAWRAD_TEXTO_DESABILITADO) }
    }
    pub CrawradBadgeRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        // Why `flow: Right` with the name in `Fill`: the shell layout notes
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
            spacing: (CRAWRAD_ESPACO_S)
            marca = <CrawradRowChevron> {}
            icone = <CrawradRowGlyph> {}
            nome = <CrawradLabel> {
                width: Fill, height: Fit,
                margin: { left: (CRAWRAD_ESPACO_XS) }
                draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) }
                text: ""
            }
            selo = <CrawradLabel> {
                margin: { left: (CRAWRAD_ESPACO_M) }
                draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) }
                text: ""
            }
        }
        alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }
    pub CrawradBadgeRowMuted = <CrawradBadgeRow> {
        face = { nome = { draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) } } }
    }
    pub CrawradBadgeRowOwned = <CrawradBadgeRow> {
        face = { icone = { draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) } } }
    }
    // The LEFTOVER: name and badge in warning color. It is the only case in this list where
    // someone needs to take action.
    pub CrawradBadgeRowWarn = <CrawradBadgeRow> {
        face = {
            icone = { draw_text: { color: (CRAWRAD_AVISO) } }
            nome = { draw_text: { color: (CRAWRAD_AVISO) } }
            selo = { draw_text: { color: (CRAWRAD_AVISO) } }
        }
    }
    // The OPEN FORM, in bold — like the active project of the `.groupproj`
    // in a classic RAD IDE: what separates it is weight, not color.
    pub CrawradBadgeRowStrong = <CrawradBadgeRow> {
        face = {
            nome = {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
            }
        }
    }
    // The project's actions on its own line, over the row's hit area (owner, 13/11).
    pub CrawradProjectActions = <View> {
        width: Fill, height: Fill,
        flow: Right, align: { x: 1.0, y: 0.5 }, spacing: 3.0,
        padding: { right: (CRAWRAD_FOLGA_DA_BARRA) }
        btn_exp_new_file = <CrawradIconButton> { width: 20.0, height: 20.0, padding: 0., text: "" }
        btn_exp_new_form = <CrawradIconButton> { width: 20.0, height: 20.0, padding: 0., text: "" }
        btn_exp_new_folder = <CrawradIconButton> { width: 20.0, height: 20.0, padding: 0., text: "" }
        btn_exp_refresh = <CrawradIconButton> { width: 20.0, height: 20.0, padding: 0., text: "" }
        btn_exp_collapse = <CrawradIconButton> { width: 20.0, height: 20.0, padding: 0., text: "" }
    }
    // Every project sits in a discreet frame, icon and name together; the
    // active one turns orange, like the Designer's "Snap: ON" (owner, 13/11).
    pub CrawradFoldRow = <CrawradBadgeRow> {
        face = {
            align: { y: 0.5 }
            spacing: 0.0
            icone = { width: 0.0 }
            nome = { width: Fit, margin: 0.0 }
            selo = { width: Fit, margin: 0.0 }
            pilula = <RoundedView> {
                width: Fit, height: 20.0,
                margin: { left: (CRAWRAD_ESPACO_XS) }
                flow: Right, align: { y: 0.5 }, spacing: (CRAWRAD_ESPACO_XS),
                padding: { left: 4.0, right: 6.0 }
                draw_bg: {
                    color: #0000
                    border_size: 1.0
                    border_radius: 2.5
                    border_color: (CRAWRAD_DIVISOR)
                }
                glifo = <CrawradRowGlyph> { draw_text: { color: (CRAWRAD_ACENTO_TEXTO) } }
                texto = <CrawradLabel> {
                    width: Fit, height: Fit,
                    draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) }
                    text: ""
                }
            }
        }
    }
    pub CrawradFoldRowActive = <CrawradFoldRow> {
        face = {
            pilula = {
                draw_bg: { color: #3B2A22, border_color: (CRAWRAD_ACENTO) }
                texto = {
                    draw_text: {
                        color: #FFB38A
                        text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
                    }
                }
            }
        }
        // Only the active project carries the actions; the others have them
        // on the right-click (owner, 13/11).
        acoes = <CrawradProjectActions> {}
    }
    // A file with errors: the name and its count in the error colour,
    // as the reference does; a warning reuses the amber row above.
    pub CrawradBadgeRowError = <CrawradBadgeRow> {
        face = {
            icone = { draw_text: { color: (CRAWRAD_ERRO) } }
            nome = { draw_text: { color: (CRAWRAD_ERRO) } }
            selo = { draw_text: { color: (CRAWRAD_ERRO) } }
        }
    }
    // A member whose files have problems: its name takes their colour.
    pub CrawradFoldRowError = <CrawradFoldRow> {
        face = { pilula = { texto = { draw_text: { color: (CRAWRAD_ERRO) } } } }
    }
    pub CrawradFoldRowWarn = <CrawradFoldRow> {
        face = { pilula = { texto = { draw_text: { color: (CRAWRAD_AVISO) } } } }
    }
    pub CrawradSectionRow = <CrawradBadgeRow> {
        face = {
            padding: { top: (CRAWRAD_ESPACO_S), bottom: (CRAWRAD_ESPACO_XS) }
            nome = {
                draw_text: {
                    color: (CRAWRAD_TEXTO_SECUNDARIO)
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
            }
        }
    }
    pub CrawradGroupNameRow = <CrawradBadgeRow> {
        face = {
            icone = { draw_text: { color: (CRAWRAD_ACENTO_TEXTO) } }
            nome = {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
            }
        }
        acoes = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { x: 1.0, y: 0.5 },
            padding: { right: (CRAWRAD_FOLGA_DA_BARRA) }
            btn_ws_more = <CrawradIconButton> { width: 20.0, height: 20.0, padding: 0., text: "" }
        }
    }
    pub CrawradLegendRow = <CrawradBadgeRow> {
        face = <View> {
            width: Fill, height: Fit,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
            nome = <CrawradLabel> {
                draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) }
                text: ""
            }
            selo = <CrawradLabelMuted> {
                width: Fill, height: Fit,
                margin: { left: (CRAWRAD_ESPACO_M) }
                text: ""
            }
        }
    }
    pub CrawradLegendRowWarn = <CrawradLegendRow> {
        face = {
            nome = { draw_text: { color: (CRAWRAD_AVISO) } }
            selo = { draw_text: { color: (CRAWRAD_AVISO) } }
        }
    }
    pub CrawradMonoLabel = <CrawradLabel> {
        width: Fill, height: Fit,
        margin: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA) }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) }
        }
        text: ""
    }
    pub CrawradMonoLabelMuted = <CrawradMonoLabel> {
        draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) }
    }


    pub CrawradCaptionRow = <View> {
        width: Fill, height: Fit,
        flow: Down,
        face = <View> {
            width: Fill, height: Fit,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA) }
            nome = <CrawradGroupLabel> {}
        }
    }

    pub CrawradNoteRow = <View> {
        width: Fill, height: Fit,
        flow: Down,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
            spacing: (CRAWRAD_ESPACO_XS)
            icone = <CrawradRowGlyph> {
                align: { x: 0.5, y: 0.0 }
            }
            nome = <CrawradLabelMuted> {
                width: Fill, height: Fit,
                text: ""
            }
        }
    }

    pub CrawradNoteRowWarn = <CrawradNoteRow> {
        face = {
            icone = { draw_text: { color: (CRAWRAD_AVISO) } }
            nome = { draw_text: { color: (CRAWRAD_AVISO) } }
        }
    }

    pub CrawradSummaryRow = <CrawradNoteRow> {
        face = { icone = { width: (CRAWRAD_GOTEIRA) } }
    }

    pub CrawradPathRow = <View> {
        width: Fill, height: Fit,
        flow: Down,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), bottom: (CRAWRAD_ESPACO_XS) }
            marca = <CrawradRowChevron> {}
            icone = <CrawradRowGlyph> {}
            nome = <CrawradMonoLabelMuted> { margin: 0. }
        }
    }

    pub CrawradWrapRow = <View> {
        width: Fill, height: Fit,
        flow: Down,
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
        texto = <CrawradLabelMuted> {
            width: Fill, height: Fit,
            text: ""
        }
    }

    pub CrawradWrapRowWarn = <CrawradWrapRow> {
        texto = { draw_text: { color: (CRAWRAD_AVISO) } }
    }

    pub CrawradWrapRowError = <CrawradWrapRow> {
        texto = { draw_text: { color: (CRAWRAD_ERRO) } }
    }

    // The second line of a warning — what to do about it.
    pub CrawradWrapRowHint = <CrawradWrapRow> {
        texto = {
            draw_text: {
                color: (CRAWRAD_TEXTO_TERCIARIO)
                text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
            }
        }
    }

    pub CrawradPacoteStat = <View> {
        width: Fit, height: Fit,
        visible: false,
        texto = <CrawradLabelMuted> { width: Fit, text: "" }
    }

    pub CrawradPackageRow = <View> {
        width: Fill, height: Fit,
        flow: Right,
        padding: (CRAWRAD_ESPACO_S),
        spacing: (CRAWRAD_ESPACO_M)
        show_bg: true,
        draw_bg: { color: (CRAWRAD_ELEVADO) }
        margin: { bottom: (CRAWRAD_ESPACO_S) }

        esquerda = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: (CRAWRAD_ESPACO_XS)

            topo = <View> {
                width: Fill, height: Fit,
                flow: RightWrap,
                spacing: (CRAWRAD_ESPACO_M)
                nome = <CrawradLabel> {
                    draw_text: {
                        color: (CRAWRAD_TEXTO_PRIMARIO)
                        text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
                    }
                    text: ""
                }
                versao = <CrawradLabel> {
                    draw_text: { color: (CRAWRAD_ACENTO_TEXTO) }
                    text: ""
                }
            }
            descricao = <CrawradWrapRow> {
                padding: 0.
            }
        }

        direita = <View> {
            width: (CRAWRAD_PACOTE_COLUNA), height: Fit,
            flow: Down,
            align: { x: 1.0 }
            spacing: (CRAWRAD_ESPACO_XS)

            stat_1 = <CrawradPacoteStat> {}
            stat_2 = <CrawradPacoteStat> {}
            stat_3 = <CrawradPacoteStat> {}

            acao = <CrawradButton> { text: "Add" }
        }
    }

    pub CrawradPackageRowInstalled = <CrawradPackageRow> {
        direita = {
            acao = <CrawradLabel> {
                width: Fit,
                draw_text: { color: (CRAWRAD_ACENTO_TEXTO) }
                text: "Installed"
            }
        }
    }

    pub CrawradPackageRowUpdatable = <CrawradPackageRow> {
        direita = {
            acao = <CrawradButton> {
                text: "Update"
            }
        }
    }

    // A DEPENDENCY ROW declared in the manifest (2-column layout with right-aligned button).
    pub CrawradDependencyRow = <View> {
        width: Fill, height: 32.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: (CRAWRAD_ESPACO_S), right: 12.0, top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
        spacing: (CRAWRAD_ESPACO_S)
        nome = <CrawradLabel> {
            draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) }
            text: ""
        }
        requisito = <CrawradMonoLabelMuted> { width: Fit, margin: 0. }
        <View> { width: Fill, height: 1.0 }
        acao = <CrawradButton> { text: "Remove" }
    }
    pub CrawradDock = <Dock> {
        width: Fill, height: Fill,
        padding: 0.,

        tab_bar: <TabBar> {
            height: 0.0,
            draw_fill: {
                color_dither: 0.0
                color: (CRAWRAD_ELEVADO)
            }
            draw_bg: {
                color_dither: 0.0
                color: (CRAWRAD_ELEVADO)
            }
            draw_drag: { color: (CRAWRAD_ACENTO) }

            PermanentTab = <Tab> {
                closeable: false,
                padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M) }
                draw_bg: {
                    color_dither: 0.0
                    border_size: 0.0
                    color: (CRAWRAD_ELEVADO)
                    color_hover: (CRAWRAD_CONTROLE)
                    color_active: (CRAWRAD_PAINEL)
                }
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                    color: (CRAWRAD_TEXTO_TERCIARIO)
                    color_hover: (CRAWRAD_TEXTO_PRIMARIO)
                    color_active: (CRAWRAD_TEXTO_ENFASE)
                }
            }
            CloseableTab = <Tab> {
                closeable: true,
                padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M) }
                draw_bg: {
                    color_dither: 0.0
                    border_size: 0.0
                    color: (CRAWRAD_ELEVADO)
                    color_hover: (CRAWRAD_CONTROLE)
                    color_active: (CRAWRAD_PAINEL)
                }
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                    color: (CRAWRAD_TEXTO_TERCIARIO)
                    color_hover: (CRAWRAD_TEXTO_PRIMARIO)
                    color_active: (CRAWRAD_TEXTO_ENFASE)
                }
            }
        }

        splitter: <Splitter> {
            size: 4.0,
            draw_bg: {
                color: (CRAWRAD_GROUND)
                color_hover: (CRAWRAD_SPLITTER_HOVER)
                color_drag: (CRAWRAD_SPLITTER_DRAG)
                uniform dot_color: (CRAWRAD_TEXTO_TERCIARIO)
                uniform dot_color_active: (CRAWRAD_SPLITTER_HOVER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.clear(self.color);

                    let mid_x = self.rect_size.x * 0.5;
                    let mid_y = self.rect_size.y * 0.5;
                    let active = max(self.hover, self.drag);
                    let dt_color = mix(self.dot_color, self.dot_color_active, active);

                    // Three discreet dots in the middle of the splitter (VS Code-style resize grip)
                    if self.is_vertical > 0.5 {
                        // Vertical splitter between columns (is_vertical = 1.0 in Makepad): 3 dots aligned vertically
                        sdf.circle(mid_x, mid_y - 8.0, 1.8);
                        sdf.circle(mid_x, mid_y, 1.8);
                        sdf.circle(mid_x, mid_y + 8.0, 1.8);
                    } else {
                        // Horizontal splitter between rows (is_vertical = 0.0 in Makepad): 3 dots aligned horizontally
                        sdf.circle(mid_x - 8.0, mid_y, 1.8);
                        sdf.circle(mid_x, mid_y, 1.8);
                        sdf.circle(mid_x + 8.0, mid_y, 1.8);
                    }
                    sdf.fill(dt_color);

                    // Rust orange visual highlight on mouse hover or drag (entire continuous bar)
                    if self.drag > 0.0 {
                        if self.is_vertical > 0.5 {
                            sdf.rect(mid_x - 1.0, 0.0, 2.0, self.rect_size.y);
                        } else {
                            sdf.rect(0.0, mid_y - 1.0, self.rect_size.x, 2.0);
                        }
                        sdf.fill(self.color_drag);
                    } else if self.hover > 0.0 {
                        if self.is_vertical > 0.5 {
                            sdf.rect(mid_x - 1.0, 0.0, 2.0, self.rect_size.y);
                        } else {
                            sdf.rect(0.0, mid_y - 1.0, self.rect_size.x, 2.0);
                        }
                        sdf.fill(self.color_hover);
                    }

                    return sdf.result;
                }
            }
        }

        // The rounded corner that `Dock` draws over edges: disabled with zero border_radius
        // to keep the entire IDE with 100% straight and flat corners (VS Code / a classic RAD IDE style).
        round_corner: {
            border_radius: 0.0
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }

        // TARGET PREVIEW — the rectangle that appears while dragging a
        // tab, indicating where it will drop (`dock.rs:429`).
        drag_target_preview: { color: (CRAWRAD_SELECAO_FORTE) }
    }


}
