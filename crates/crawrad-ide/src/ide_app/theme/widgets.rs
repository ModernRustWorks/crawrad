use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    pub CrawradOutlined = <View> {
        show_bg: true,
        draw_bg: {
            uniform border_color: (CRAWRAD_DIVISOR)
            uniform border_radius: (CRAWRAD_CARD_RADIUS)
            color: (CRAWRAD_PAINEL)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }

    // Window background lining.
    pub CrawradGround = <View> {
        show_bg: true,
        draw_bg: { color: (CRAWRAD_GROUND) }
    }
    // Dock panel as a modern independent card (VS Code / Cursor style):
    // 4 rounded corners, perimeter margin, and subtle border.
    pub CrawradCardPanel = <CrawradOutlined> {
        width: Fill, height: Fill,
        margin: (CRAWRAD_CARD_GAP),
    }
    pub CrawradPanel = <CrawradCardPanel> {}
    pub CrawradCardRail = <View> {
        width: (CRAWRAD_RAIL_LARGURA), height: Fill,
        margin: { top: (CRAWRAD_CARD_GAP), bottom: (CRAWRAD_CARD_GAP), left: (CRAWRAD_CARD_GAP), right: 0.0 },
        show_bg: true,
        draw_bg: {
            uniform border_color: (CRAWRAD_DIVISOR)
            uniform border_radius: (CRAWRAD_CARD_RADIUS)
            color: (CRAWRAD_ELEVADO)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;
                sdf.box(0.5, 0.5, self.rect_size.x * 2.0, self.rect_size.y - 1.0, r);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }
    pub CrawradCardPanelLeft = <View> {
        width: Fill, height: Fill,
        margin: { top: (CRAWRAD_CARD_GAP), bottom: (CRAWRAD_CARD_GAP), left: 0.0, right: (CRAWRAD_CARD_GAP) },
        show_bg: true,
        draw_bg: {
            uniform border_color: (CRAWRAD_DIVISOR)
            uniform border_radius: (CRAWRAD_CARD_RADIUS)
            color: (CRAWRAD_PAINEL)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;
                sdf.box(-self.rect_size.x, 0.5, self.rect_size.x * 2.0 - 0.5, self.rect_size.y - 1.0, r);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }
    pub CrawradPanelLeft = <CrawradCardPanelLeft> {}
    // A **chrome band** — top bar, icon rail, panel header,
    // and status bar. It recedes relative to the panel, intentionally.
    pub CrawradBand = <View> {
        show_bg: true,
        draw_bg: { color: (CRAWRAD_ELEVADO) }
    }
    // 1 px line that separates two regions vertically.
    pub CrawradSeparatorV = <View> {
        width: 1.0, height: Fill,
        show_bg: true,
        draw_bg: { color: (CRAWRAD_DIVISOR) }
    }
    // 1 px line that separates two regions horizontally.
    pub CrawradSeparatorH = <View> {
        width: Fill, height: 1.0,
        show_bg: true,
        draw_bg: { color: (CRAWRAD_DIVISOR) }
    }
    // Spacer that consumes remaining space.
    pub CrawradFiller = <View> { width: Fill, height: Fill }
    // ---------------------------------------------------------------- text
    pub CrawradLabel = <Label> {
        padding: 0.,
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO),
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }
    // Text the user can SELECT and copy: a read-only `TextInput` drawn as a label
    // (no box, no cursor); drag selects, Ctrl+C copies (`text_input.rs:1468-1529`, `:1680`).
    pub CrawradSelectableText = <TextInput> {
        width: Fill, height: Fit, margin: 0., padding: 0., is_read_only: true, empty_text: "",
        draw_bg: { fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); } }
        draw_cursor: { fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); } }
        draw_selection: { color: (CRAWRAD_SELECAO_FORTE), color_hover: (CRAWRAD_SELECAO_FORTE), color_focus: (CRAWRAD_SELECAO_FORTE), color_down: (CRAWRAD_SELECAO_FORTE) }
        draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }, fn get_color(self) -> vec4 { return self.color; } }
    }
    // An agent's reply rendered as Markdown (approved 18/11): body in the secondary tone, bold and headings bright, inline code in the accent.
    pub CrawradChatMarkdown = <Markdown> { width: Fill, height: Fit, padding: 0., paragraph_spacing: 8, font_size: (CRAWRAD_FONTE_CORPO), font_color: (CRAWRAD_TEXTO_SECUNDARIO), draw_normal: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: { font_size: (CRAWRAD_FONTE_CORPO) } }, draw_italic: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: { font_size: (CRAWRAD_FONTE_CORPO) } }, draw_bold: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: { font_size: (CRAWRAD_FONTE_CORPO) } }, draw_bold_italic: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: { font_size: (CRAWRAD_FONTE_CORPO) } }, draw_fixed: { color: (CRAWRAD_ACENTO_TEXTO), text_style: { font_size: (CRAWRAD_FONTE_CORPO) } } }
    // Supporting text — hint, count, footer.
    pub CrawradLabelMuted = <CrawradLabel> {
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO),
            text_style: { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }
    pub CrawradStatusBar = <View> {
        show_bg: true,
        // Blue while a program is under the debugger (the approved design).
        draw_bg: { color: (CRAWRAD_ELEVADO), uniform color_debug: (CRAWRAD_DEPURANDO), instance debugging: 0.0, fn pixel(self) -> vec4 { return mix(self.color, self.color_debug, self.debugging); } }
    }
    pub CrawradStatusCell = <View> {
        width: Fit, height: Fill,
        flow: Right, align: { y: 0.5 },
        padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M) }
        spacing: (CRAWRAD_ESPACO_XS),
    }
    // Icon for a status bar cell. COLOR comes from the caller — this is the only
    // place on the bar where it conveys meaning.
    pub CrawradStatusIcon = <CrawradLabel> {
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO),
            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_RODAPE_ICONE) }
        }
    }
    pub CrawradLspIcon = <View> {
        width: 12.0, height: 12.0, show_bg: true,
        draw_bg: {
            instance phase: 1.0, instance turn: 0.0,
            uniform busy_color: (CRAWRAD_ACENTO_TEXTO), uniform ready_color: (CRAWRAD_OK),
            uniform failed_color: (CRAWRAD_ERRO), uniform track_color: (CRAWRAD_DIVISOR),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let colour = mix(self.ready_color, self.failed_color, step(1.5, self.phase));
                sdf.circle(c.x, c.y, 4.5);
                sdf.stroke(mix(self.track_color, colour, step(0.5, self.phase)), 0.9);
                if self.phase < 0.5 {
                    let a = self.turn * 6.2831853;
                    sdf.arc_round_caps(c.x, c.y, 4.5, a, a + 1.5707963, 2.0);
                    sdf.fill(self.busy_color);
                } else if self.phase < 1.5 {
                    sdf.move_to(c.x - 2.2, c.y + 0.2); sdf.line_to(c.x - 0.6, c.y + 1.7);
                    sdf.line_to(c.x + 2.3, c.y - 1.4); sdf.stroke(colour, 0.9);
                } else {
                    sdf.move_to(c.x - 1.8, c.y - 1.8); sdf.line_to(c.x + 1.8, c.y + 1.8);
                    sdf.move_to(c.x + 1.8, c.y - 1.8); sdf.line_to(c.x - 1.8, c.y + 1.8);
                    sdf.stroke(colour, 0.9);
                }
                return sdf.result;
            }
        }
    }
    // Text for a status bar cell.
    pub CrawradStatusText = <CrawradLabel> {
        draw_text: {
            color: (CRAWRAD_TEXTO_SECUNDARIO),
            text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) }
        }
    }

    // Label for a group inside a panel — "Standard", "System".
    // It LABELS the group; does not compete with its items.
    pub CrawradGroupLabel = <CrawradLabel> {
        width: Fill, height: Fit,
        margin: { top: (CRAWRAD_ESPACO_M), bottom: (CRAWRAD_ESPACO_XS) }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO),
            text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }

    pub CrawradPanelHeader = <View> {
        width: Fill, height: Fit,
        flow: Right, align: { y: 0.5 },
        padding: { left: (CRAWRAD_MARGEM_PAINEL), right: (CRAWRAD_MARGEM_PAINEL), top: (CRAWRAD_ESPACO_S), bottom: (CRAWRAD_ESPACO_S) }
        title = <Label> {
            padding: 0.,
            draw_text: {
                color: (CRAWRAD_TEXTO_TERCIARIO),
                text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
            }
            text: ""
        }
    }


    pub CrawradButton = <Button> {
        height: (CRAWRAD_ALVO_ICONE),
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M), top: 0., bottom: 0. }
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO_BOTAO)

            color: (CRAWRAD_CONTROLE)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_SELECAO_FORTE)
            color_focus: (CRAWRAD_CONTROLE)
            color_disabled: (CRAWRAD_PAINEL)

            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_BORDA_FOCO)
            border_color_2_hover: (CRAWRAD_BORDA_FOCO)
            border_color_1_down: (CRAWRAD_ACENTO)
            border_color_2_down: (CRAWRAD_ACENTO)
            border_color_1_focus: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_focus: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_disabled: (CRAWRAD_DIVISOR)
            border_color_2_disabled: (CRAWRAD_DIVISOR)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_ENFASE)
            color_down: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_PRIMARIO)
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }

    pub CrawradIconButton = <CrawradButton> {
        width: (CRAWRAD_ALVO_ICONE), height: (CRAWRAD_ALVO_ICONE),
        padding: 0.,
        draw_text: {
            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_CABECALHO) }
        }
    }

    pub CrawradPanelClose = <CrawradIconButton> {
        width: 20.0, height: 20.0,
        padding: 0.,
        margin: { left: (CRAWRAD_ESPACO_S) }
    }

    pub CrawradPrimaryButton = <CrawradButton> {
        draw_bg: {
            color: (CRAWRAD_BOTAO_PRIMARIO)
            color_hover: (CRAWRAD_BOTAO_PRIMARIO_HOVER)
            color_down: (CRAWRAD_BOTAO_PRIMARIO_DOWN)
            border_radius: (CRAWRAD_RAIO_BOTAO)
            border_size: 1.0
            border_color_1: (CRAWRAD_BOTAO_PRIMARIO_BORDA)
            border_color_2: (CRAWRAD_BOTAO_PRIMARIO_BORDA)
            border_color_1_hover: (CRAWRAD_BOTAO_PRIMARIO_HOVER)
            border_color_2_hover: (CRAWRAD_BOTAO_PRIMARIO_HOVER)
        }
        draw_text: {
            color: (CRAWRAD_BRANCO)
            text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }

    // Primary command when it is only an icon — the toolbar Run button.
    pub CrawradPrimaryIconButton = <CrawradPrimaryButton> {
        width: (CRAWRAD_ALVO_ICONE), height: (CRAWRAD_ALVO_ICONE),
        padding: 0.,
        draw_text: {
            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_CABECALHO) }
        }
    }

    // Empty state shown when an open project has no form.
    pub CrawradEmptyState = <View> {
        width: Fill, height: Fill, flow: Down, align: { x: 0.5, y: 0.5 }, spacing: 10.0,
        padding: { left: 28.0, right: 28.0 }
        icon = <CrawradLabelMuted> { draw_text: { text_style: <CRAWRAD_FONT_ICONS> { font_size: 21.0 } } }
        title = <CrawradLabel> {
            text: "This project has no form"
            draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }
        }
        body = <CrawradLabelMuted> {
            width: Fill, align: { x: 0.5 }, draw_text: { wrap: Word, text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } }
            text: "Components go on a form. Add one and the Toolbox and the Structure fill in."
        }
        new_form = <CrawradPrimaryButton> { margin: { top: 6.0 }, text: "New Form…" }
        hint = <CrawradLabelMuted> { width: Fill, align: { x: 0.5 }, draw_text: { wrap: Word, text_style: { font_size: 8.0 } } }
    }

    // A toolbar command with its name: the icon (own colour) is a label over the button.
    pub CrawradCaptionButton = <View> {
        width: Fit, height: (CRAWRAD_ALVO_ICONE),
        flow: Overlay,
        button = <CrawradButton> {
            padding: { left: 32., right: (CRAWRAD_ESPACO_M), top: 0., bottom: 0. }
            text: ""
        }
        icon_slot = <View> {
            width: Fit, height: Fill,
            align: { y: 0.5 },
            padding: { left: 9. }
            glyph = <Label> {
                draw_text: {
                    color: (CRAWRAD_AVISO)
                    text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_CABECALHO) }
                }
                text: ""
            }
        }
    }

    // One half of a split button: rounded on its outer side only.
    pub CrawradSplitButton = <CrawradPrimaryButton> {
        draw_bg: {
            uniform radius_left: (CRAWRAD_RAIO_BOTAO)
            uniform radius_right: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let size = self.rect_size;
                sdf.box(0., 0., size.x, size.y, max(self.radius_left, self.radius_right));
                if self.radius_right < 0.5 {
                    sdf.rect(size.x * 0.5, 0., size.x * 0.5, size.y);
                }
                if self.radius_left < 0.5 {
                    sdf.rect(0., 0., size.x * 0.5, size.y);
                }
                sdf.fill(mix(mix(self.color, self.color_hover, self.hover), self.color_down, self.down));
                return sdf.result;
            }
        }
        draw_text: { color: (CRAWRAD_GROUND) }
    }

    // A debugger command on the toolbar: lit only while a program is debugged.
    pub CrawradDebugBarButton = <CrawradIconButton> {
        margin: { left: (CRAWRAD_ESPACO_XS) }
        draw_text: { color: #4da3ff }
        text: ""
    }

    pub CrawradMenuButton = <Button> {
        height: (CRAWRAD_ALVO_ICONE),
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M), top: 0., bottom: 0. }
        draw_bg: {
            color_dither: 0.0
            border_size: 0.0
            border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_ELEVADO)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_SELECAO_FORTE)
            color_focus: (CRAWRAD_ELEVADO)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_SECUNDARIO)
            color_hover: (CRAWRAD_TEXTO_ENFASE)
            color_down: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_SECUNDARIO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }

    pub CrawradMenuHit = <Button> {
        width: Fill, height: Fill,
        margin: 0., padding: 0.,
        draw_bg: {
            color_dither: 0.0
            border_size: 0.0
            border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_FLUTUANTE)
            color_hover: (CRAWRAD_SELECAO)
            color_down: (CRAWRAD_SELECAO_FORTE)
            color_focus: (CRAWRAD_FLUTUANTE)
        }
        draw_text: { text_style: { font_size: (CRAWRAD_FONTE_CORPO) } }
    }

    pub CrawradMenuRow = <View> {
        width: Fill, height: (CRAWRAD_MENU_LINHA),
        flow: Overlay,
        alvo = <CrawradMenuHit> {}
        face = <View> {
            width: Fill, height: Fill,
            flow: Right,
            align: { y: 0.5 }
            padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_L) }
            marca = <CrawradLabel> {
                width: (CRAWRAD_MENU_MARCA),
                draw_text: {
                    color: (CRAWRAD_ACENTO_TEXTO)
                    text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
                text: ""
            }
            rotulo = <CrawradLabel> {
                draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) }
                text: ""
            }
            vao = <CrawradFiller> {}
            atalho = <CrawradLabel> {
                draw_text: {
                    color: (CRAWRAD_TEXTO_TERCIARIO)
                    text_style: <CRAWRAD_FONT_TEXT_ICONS> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
                text: ""
            }
        }
    }

    pub CrawradMenuSeparator = <View> {
        width: Fill, height: (CRAWRAD_MENU_SEPARADOR),
        flow: Down,
        align: { y: 0.5 }
        padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_L) }
        filete = <View> {
            width: Fill, height: 1.0,
            show_bg: true,
            draw_bg: { color: (CRAWRAD_DIVISOR) }
        }
    }

    pub CrawradMenuRowActive = <CrawradMenuRow> {
        show_bg: true,
        draw_bg: { color: (CRAWRAD_SELECAO_FORTE) }
    }

    pub CrawradMenuRowDisabled = <CrawradMenuRow> {
        alvo = <CrawradMenuHit> {
            draw_bg: {
                color_hover: (CRAWRAD_SELECAO)
                color_down: (CRAWRAD_SELECAO)
            }
        }
        face = {
            rotulo = { draw_text: { color: (CRAWRAD_TEXTO_DESABILITADO) } }
            atalho = { draw_text: { color: (CRAWRAD_TEXTO_DESABILITADO) } }
        }
    }

    // The small title on top of a context menu: what it speaks of.
    pub CrawradMenuRowTitle = <CrawradMenuRowDisabled> {
        alvo = { draw_bg: { color_hover: (CRAWRAD_FLUTUANTE), color_down: (CRAWRAD_FLUTUANTE) } }
        face = {
            rotulo = {
                draw_text: {
                    color: (CRAWRAD_TEXTO_TERCIARIO)
                    text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
            }
        }
    }

    // A notice under the rows it explains: a lock, then two lines.
    pub CrawradMenuRowNotice = <CrawradMenuRowDisabled> {
        alvo = { draw_bg: { color_hover: (CRAWRAD_FLUTUANTE), color_down: (CRAWRAD_FLUTUANTE) } }
        height: (CRAWRAD_MENU_LINHA * 2.0),
        face = {
            align: { y: 0.0 }
            padding: { top: (CRAWRAD_ESPACO_S) }
            marca = { draw_text: { color: (CRAWRAD_AVISO) } }
            rotulo = {
                width: Fill,
                draw_text: {
                    color: (CRAWRAD_AVISO)
                    text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
            }
            vao = { width: 0.0 }
            atalho = { width: 0.0 }
        }
    }

    pub CrawradMenuPopup = <CrawradOutlined> {
        width: (CRAWRAD_MENU_LARGURA), height: Fit,
        flow: Down,
        padding: (CRAWRAD_ESPACO_S)
        draw_bg: {
            uniform border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_FLUTUANTE)
        }
        visible: false,
        abs_pos: vec2(0.0, 0.0),

        menu_list = <PortalList> {
            width: Fill, height: Fill,
            // Zero width: the panel fits exactly, and the bar theme (which
            // lives in `workspace.rs`, which imports this) would be a cycle.
            scroll_bar: <ScrollBar> { bar_size: 0.0 }

            // The action that CAN run now.
            Comando = <CrawradMenuRow> {}
            // The row the arrow keys moved to.
            ComandoAtivo = <CrawradMenuRowActive> {}
            ComandoCinza = <CrawradMenuRowDisabled> {}
            ComandoTitulo = <CrawradMenuRowTitle> {}
            ComandoAviso = <CrawradMenuRowNotice> {}
            // The SEPARATOR line between two blocks -- see `menu::Row`.
            Separador = <CrawradMenuSeparator> {}
        }
    }

    // Separator line between a command GROUP and its neighbor inside the toolbar.
    pub CrawradBarSeparator = <View> {
        width: 1.0, height: 18.0,
        margin: { left: (CRAWRAD_GRUPO_BARRA), right: (CRAWRAD_GRUPO_BARRA) }
        show_bg: true,
        draw_bg: { color: (CRAWRAD_DIVISOR) }
    }

    pub CrawradWindowButton = <Button> {
        width: 44.0, height: (CRAWRAD_BARRA_ALTURA),
        margin: 0., padding: 0.,
        text: "",
        draw_bg: {
            uniform color_mark: (CRAWRAD_TEXTO_PRIMARIO)
            uniform color_mark_hover: (CRAWRAD_TEXTO_ENFASE)
            color: (CRAWRAD_ELEVADO)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_CONTROLE)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                return sdf.result;
            }
        }
    }

    // Minimize: a dash.
    pub CrawradMinButton = <CrawradWindowButton> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                let c = self.rect_size * 0.5;
                sdf.move_to(c.x - 5.0, c.y);
                sdf.line_to(c.x + 5.0, c.y);
                sdf.stroke(mix(self.color_mark, self.color_mark_hover, self.hover), 1.0);
                return sdf.result;
            }
        }
    }

    // Maximize/restore: a rectangle.
    pub CrawradMaxButton = <CrawradWindowButton> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                let c = self.rect_size * 0.5;
                sdf.rect(c.x - 4.5, c.y - 4.5, 9.0, 9.0);
                sdf.stroke(mix(self.color_mark, self.color_mark_hover, self.hover), 1.0);
                return sdf.result;
            }
        }
    }

    pub CrawradRestoreButton = <CrawradWindowButton> {
        visible: false,
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                let c = self.rect_size * 0.5;
                let marca = mix(self.color_mark, self.color_mark_hover, self.hover);
                // Back rectangle, offset upward and to the right.
                sdf.rect(c.x - 2.5, c.y - 5.5, 8.0, 8.0);
                sdf.stroke(marca, 1.0);
                // And front rectangle, which covers the corner of the other.
                sdf.rect(c.x - 5.5, c.y - 2.5, 8.0, 8.0);
                sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                sdf.rect(c.x - 5.5, c.y - 2.5, 8.0, 8.0);
                sdf.stroke(marca, 1.0);
                return sdf.result;
            }
        }
    }

    // CLOSE: an X. The only red element on the bar, and only under the mouse — missing
    // this button costs unsaved work.
    pub CrawradCloseButton = <CrawradWindowButton> {
        draw_bg: {
            color_hover: (CRAWRAD_FECHAR_HOVER)
            color_down: (CRAWRAD_FECHAR_HOVER)
            color_mark_hover: #fff

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                let c = self.rect_size * 0.5;
                sdf.move_to(c.x - 4.5, c.y - 4.5);
                sdf.line_to(c.x + 4.5, c.y + 4.5);
                sdf.move_to(c.x + 4.5, c.y - 4.5);
                sdf.line_to(c.x - 4.5, c.y + 4.5);
                sdf.stroke(mix(self.color_mark, self.color_mark_hover, self.hover), 1.0);
                return sdf.result;
            }
        }
    }

    pub CrawradRailButton = <Button> {
        width: 36.0, height: 36.0,
        margin: { left: 6.0, right: 6.0, top: 3.0, bottom: 3.0 },
        padding: 0.,
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            instance active: 0.0
            uniform color_bar: (CRAWRAD_ACENTO)
            color: #00000000
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_SELECAO)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let is_hover_down = max(self.hover, self.down);
                let r = (CRAWRAD_RAIO);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, r);
                sdf.fill(mix(self.color, self.color_hover, is_hover_down));

                // Active tab indicator bar on the left (VS Code style):
                if self.active > 0.5 {
                    sdf.box(0.5, 6.0, 2.5, self.rect_size.y - 12.0, 1.0);
                    sdf.fill(self.color_bar);
                }
                return sdf.result;
            }
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_ACENTO_TEXTO)
            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_RAIL) }
        }
    }

    pub CrawradRailButtonMuted = <CrawradRailButton> {
        draw_text: {
            color: (CRAWRAD_TEXTO_DESABILITADO)
            color_hover: (CRAWRAD_TEXTO_TERCIARIO)
            color_down: (CRAWRAD_TEXTO_TERCIARIO)
        }
    }

    pub CrawradListRow = <Button> {
        width: Fill, height: (CRAWRAD_LINHA_ALTURA),
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: 0., bottom: 0. }
        align: { x: 0.0, y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_PAINEL)
            color_hover: (CRAWRAD_LISTRA)
            color_down: (CRAWRAD_SELECAO)
            color_focus: (CRAWRAD_PAINEL)
            border_color_1: (CRAWRAD_PAINEL)
            border_color_2: (CRAWRAD_PAINEL)
            border_color_1_hover: (CRAWRAD_LISTRA)
            border_color_2_hover: (CRAWRAD_LISTRA)
            border_color_1_down: (CRAWRAD_ACENTO)
            border_color_2_down: (CRAWRAD_ACENTO)
            border_color_1_focus: (CRAWRAD_PAINEL)
            border_color_2_focus: (CRAWRAD_PAINEL)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_SECUNDARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_SECUNDARIO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }

    pub CrawradListRowOverlay = <CrawradListRow> {
        draw_bg: {
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            border_color_1: #0000
            border_color_2: #0000
            border_color_1_hover: (CRAWRAD_ACENTO)
            border_color_2_hover: (CRAWRAD_ACENTO)
            border_color_1_down: (CRAWRAD_ACENTO)
            border_color_2_down: (CRAWRAD_ACENTO)
            border_color_1_focus: #0000
            border_color_2_focus: #0000
        }
    }
}
