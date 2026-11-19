use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use makepad_code_editor::modal_veil::ModalVeil;
    pub CRAWRAD_DIALOGO_BOTAO_L = 80.0
    pub CRAWRAD_DIALOGO_BOTAO_A = 26.0
    pub CRAWRAD_DIALOGO_TITULO = 30.0
    pub CRAWRAD_DIALOGO_RODAPE = 36.0
    pub CRAWRAD_VEU = #0000009c
    pub CrawradDialogVeil = <ModalVeil> {
        visible: false,
        abs_pos: vec2(0.0, 0.0),
        width: Fill, height: Fill,
        align: { x: 0.5, y: 0.5 }
        show_bg: true,
        draw_bg: { color: (CRAWRAD_VEU) }
    }
    // THE FRAME — floating background, hairline border, theme corner.
    pub CrawradDialog = <RoundedView> {
        width: Fit, height: Fit,
        flow: Down,
        draw_bg: {
            color: (CRAWRAD_FLUTUANTE)
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO)
            border_color: (CRAWRAD_BORDA_CONTROLE)
        }
    }
    // THE TITLE BAR of the dialog.
    pub CrawradDialogTitleBar = <RoundedView> {
        width: Fill, height: (CRAWRAD_DIALOGO_TITULO),
        flow: Right,
        align: { y: 0.5 }
        padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_S), top: 0., bottom: 0. }
        draw_bg: {
            color: (CRAWRAD_ELEVADO)
            border_size: 0.0
            border_radius: (CRAWRAD_RAIO)
        }
    }
    pub CrawradDialogTitle = <CrawradLabel> {
        draw_text: {
            color: (CRAWRAD_TEXTO_ENFASE)
            text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }
    pub CrawradDialogBody = <View> {
        width: Fill, height: Fill,
        flow: Down,
        spacing: (CRAWRAD_ESPACO_M)
        padding: (CRAWRAD_ESPACO_L)
    }
    pub CrawradDialogFooter = <View> {
        width: Fill, height: (CRAWRAD_DIALOGO_RODAPE),
        flow: Right,
        align: { y: 0.5 }
        spacing: (CRAWRAD_ESPACO_S)
        padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_L), top: 0., bottom: (CRAWRAD_ESPACO_L) }
    }
    pub CrawradDialogButton = <CrawradButton> {
        width: (CRAWRAD_DIALOGO_BOTAO_L), height: (CRAWRAD_DIALOGO_BOTAO_A),
    }
    pub CrawradDialogPrimaryButton = <CrawradPrimaryButton> {
        width: (CRAWRAD_DIALOGO_BOTAO_L), height: (CRAWRAD_DIALOGO_BOTAO_A),
    }
    pub CrawradGroupBoxTitle = <CrawradLabel> {
        margin: { bottom: (CRAWRAD_ESPACO_XS) }
        draw_text: {
            color: (CRAWRAD_TEXTO_SECUNDARIO)
            text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }
    pub CrawradGroupBoxFrame = <RoundedView> {
        width: Fill, height: Fill,
        flow: Down,
        padding: (CRAWRAD_ESPACO_S)
        draw_bg: {
            color: (CRAWRAD_ELEVADO)
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO)
            border_color: (CRAWRAD_DIVISOR)
        }
    }

    pub CRAWRAD_ITEMS_DLG_L = 620.0
    pub CRAWRAD_ITEMS_CORPO = 314.0
    pub CRAWRAD_ITEMS_COLUNA = 230.0
    pub CRAWRAD_ITEMS_BOTAO_L = 110.0
    pub CRAWRAD_ITEMS_BOTOES_TOPO = 20.0

    pub CrawradItemsButton = <CrawradButton> {
        width: (CRAWRAD_ITEMS_BOTAO_L), height: (CRAWRAD_DIALOGO_BOTAO_A),
    }

    pub CrawradItemsRow = <View> {
        width: Fill, height: (CRAWRAD_LINHA_ALTURA), margin: { right: (CRAWRAD_FOLGA_DA_BARRA) },
        flow: Overlay,
        show_bg: true,
        draw_bg: {
            color: (CRAWRAD_ELEVADO)
            instance selecionada: 0.0
            uniform cor_selecao: (CRAWRAD_SELECAO_FORTE)
            fn pixel(self) -> vec4 {
                let c = mix(self.color, self.cor_selecao, self.selecionada);
                return vec4(c.rgb * c.a, c.a);
            }
        }
        rotulo = <CrawradLabel> {
            width: Fill, height: Fill,
            align: { y: 0.5 }, padding: { left: (CRAWRAD_ESPACO_M) }
            text: ""
        }
        alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    pub CRAWRAD_LINES_DLG_L = 520.0
    // 400 - 30 (title) - 36 (footer) — see `CrawradDialogBody`.
    pub CRAWRAD_LINES_CORPO = 334.0

    pub CrawradLinesText = <CrawradTextInput> {
        width: Fill, height: Fill,
        label_align: { x: 0.0, y: 0.0 },
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S), bottom: (CRAWRAD_ESPACO_S) }
        draw_text: {
            text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) }
        }
    }

    pub CRAWRAD_ACTIONS_DLG_L = 460.0
    pub CRAWRAD_ACTIONS_CORPO = 254.0
    pub CRAWRAD_ACTIONS_COLUNA = 150.0
    pub CRAWRAD_STANDARD_DLG_L = 340.0
    pub CRAWRAD_STANDARD_CORPO = 374.0

    pub CRAWRAD_MARCA_LARGURA = 18.0

    pub CrawradStdCheck = <CrawradPropCheck> {
        width: (CRAWRAD_MARCA_LARGURA), height: (CRAWRAD_CAMPO_ALTURA),
        margin: { right: (CRAWRAD_ESPACO_S) }
    }

    pub CRAWRAD_IML_DLG_L = 640.0
    pub CRAWRAD_IML_CORPO = 354.0
    pub CRAWRAD_IML_CEL_L = 48.0
    pub CRAWRAD_IML_CEL_A = 56.0
    pub CRAWRAD_IML_MINI = 32.0
    pub CRAWRAD_IML_PREVIA = 100.0
    pub CRAWRAD_IML_COLUNA = 200.0
    // The enlarged preview draws the symbol in a large size — it is 54 in egui.
    pub CRAWRAD_FONTE_PREVIA = 40.5
    // The `Add Icons from Catalog` window, from within the editor: 320x360 in egui.
    pub CRAWRAD_ADDICON_DLG_L = 320.0
    pub CRAWRAD_ADDICON_CORPO = 340.0
    pub CRAWRAD_NOVO_ICONE_SIMBOLO = 66.0

    pub CRAWRAD_QUIT_DLG_L = 470.0
    pub CRAWRAD_QUIT_CORPO = 139.0
    // The dialog warning sign, in the size that egui uses (`ICONE_DIALOGO`).
    pub CRAWRAD_FONTE_AVISO = 21.0

    pub CRAWRAD_OPTIONS_DLG_L = 840.0
    pub CRAWRAD_OPTIONS_CORPO = 600.0
    pub CRAWRAD_OPTIONS_ARVORE = 210.0
    // The PAGE title — 22 px in egui, which in points is 16.5.
    pub CRAWRAD_FONTE_TITULO = 16.5

    pub CRAWRAD_NOVO_DLG_L = 1024.0
    pub CRAWRAD_NOVO_CORPO = 610.0
    pub CRAWRAD_NOVO_BOTAO = 130.0
    pub CRAWRAD_NOVO_AZUL = #1d5cc6
    pub CRAWRAD_NOVO_AZUL_BORDA = #2a6beb
    pub CRAWRAD_NOVO_AZUL_ESCURO = #16428c
    pub CRAWRAD_NOVO_CARD_BG = #182232
    pub CRAWRAD_NOVO_CARD_SEL = #202c42
    pub CRAWRAD_NOVO_CARD_BORDA = #232f42
    pub CRAWRAD_NOVO_ICONE_BG = #152438
    pub CRAWRAD_NOVO_ICONE_AZUL = #38bdf8
    pub CRAWRAD_NOVO_TREE_BG = #0d121c
    pub CRAWRAD_NOVO_BTN_AZUL = #1d68e1
    pub CRAWRAD_NOVO_ESTRELA = #eab308
    pub CRAWRAD_NOVO_ESTRELA_MUTED = #64748b
    pub CRAWRAD_NOVO_CAT_TEXTO = #94a3b9
    pub CRAWRAD_CRIANDO_DLG_L = 460.0
    pub CRAWRAD_CRIANDO_CORPO = 120.0

    // Theme tokens for the ACP Registry (ACP Agent Catalog)
    pub CRAWRAD_ACP_CARD_BG = #181a1b
    pub CRAWRAD_ACP_CARD_BORDA = #36393b
    pub CRAWRAD_ACP_CARD_ACTIVE_BORDA = #f59e0b
    pub CRAWRAD_ACP_INSTALLED_BG = #142a1e
    pub CRAWRAD_ACP_INSTALLED_BORDA = #1e6640
    pub CRAWRAD_ACP_INSTALLED_TEXTO = #4ade80
    pub CRAWRAD_ACP_ACTIVE_BG = #2d2006
    pub CRAWRAD_ACP_ACTIVE_BORDA = #92620a
    pub CRAWRAD_ACP_ACTIVE_TEXTO = #facc15
    pub CRAWRAD_ACP_ACTIVE_CARD_BG = #1c1810
    pub CRAWRAD_ACP_TAG_BG = #202326
    pub CRAWRAD_ACP_TERM_BG = #0d1712
    pub CRAWRAD_ACP_TERM_TEXTO_MUTED = #8cf0ba
    pub CRAWRAD_ACP_OAUTH_BG = #0f2238
    pub CRAWRAD_ACP_OAUTH_BORDA = #1d5cc6
    pub CRAWRAD_ACP_OAUTH_TEXTO = #38bdf8

    pub CRAWRAD_ICONES_DLG_L = 620.0
    pub CRAWRAD_ICONES_CORPO = 464.0
    pub CRAWRAD_ICONES_COLUNA = 150.0
    pub CRAWRAD_BUILD_DLG_L = 840.0
    pub CRAWRAD_BUILD_CORPO = 520.0

    // The three sizes, side by side. In POINTS, like every `font_size` in the DSL.
    pub CRAWRAD_ICONE_P = 12.0
    pub CRAWRAD_ICONE_M = 18.0
    pub CRAWRAD_ICONE_G = 24.0

    // ONE ROW of the sheet: the role, the glyph in three sizes, and the font name.
    pub CrawradIconSheetRow = <View> {
        width: Fill, height: (CRAWRAD_IML_CEL_A),
        flow: Right,
        align: { y: 0.5 }
        spacing: (CRAWRAD_ESPACO_M)
        padding: { right: (CRAWRAD_FOLGA_DA_BARRA) }
        papel = <CrawradLabel> { width: (CRAWRAD_ICONES_COLUNA), text: "" }
        g16 = <CrawradLabel> {
            width: Fit,
            draw_text: { text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_ICONE_P) } }
            text: ""
        }
        g24 = <CrawradLabel> {
            width: Fit,
            draw_text: { text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_ICONE_M) } }
            text: ""
        }
        g32 = <CrawradLabel> {
            width: Fit,
            draw_text: { text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_ICONE_G) } }
            text: ""
        }
        <CrawradFiller> {}
        nome = <CrawradLabelMuted> { width: Fit, text: "" }
    }

    pub CrawradOptionRadio = <RadioButton> {
        width: Fill, height: (CRAWRAD_CAMPO_ALTURA),
        align: { x: 0.0, y: 0.5 }
        label_walk: { width: Fit, height: Fit, margin: { left: (CRAWRAD_MARCA_LARGURA) } }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_PRIMARIO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }

    // A label and its field on one line, vertically centred.
    pub CrawradFieldRow = <View> {
        width: Fill, height: Fit,
        flow: Right,
        align: { y: 0.5 }
        spacing: (CRAWRAD_ESPACO_S)
    }

    pub CrawradOptionCheck = <CrawradPropCheck> {
        width: Fill, height: (CRAWRAD_CAMPO_ALTURA),
        label_walk: { width: Fit, height: Fit, margin: { left: (CRAWRAD_MARCA_LARGURA) } }
    }

    pub CRAWRAD_XADREZ_A = #f0f0f0
    pub CRAWRAD_XADREZ_B = #d8d8d8
    pub CRAWRAD_TINTA_NO_XADREZ = #000000

    pub CrawradXadrez = <View> {
        show_bg: true,
        draw_bg: {
            uniform cor_a: (CRAWRAD_XADREZ_A)
            uniform cor_b: (CRAWRAD_XADREZ_B)
            uniform lado: 4.0
            uniform cor_borda: (CRAWRAD_BORDA_CONTROLE)
            fn pixel(self) -> vec4 {
                let p = self.pos * self.rect_size;
                if p.x < 1.0 || p.y < 1.0 || p.x > self.rect_size.x - 1.0 || p.y > self.rect_size.y - 1.0 {
                    return self.cor_borda;
                }
                let c = floor(p / self.lado);
                return mix(self.cor_a, self.cor_b, mod(c.x + c.y, 2.0));
            }
        }
    }

    pub CrawradIconCell = <View> {
        width: (CRAWRAD_IML_CEL_L), height: (CRAWRAD_IML_CEL_A),
        flow: Overlay,
        cel_fundo = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: {
                color: (CRAWRAD_ELEVADO)
                instance selecionada: 0.0
                uniform cor_selecao: (CRAWRAD_SELECAO_FORTE)
                uniform raio: (CRAWRAD_RAIO)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.raio);
                    sdf.fill(mix(self.color, self.cor_selecao, self.selecionada));
                    return sdf.result;
                }
            }
        }
        cel_face = <View> {
            width: Fill, height: Fill,
            flow: Down,
            align: { x: 0.5 }
            padding: { top: (CRAWRAD_ESPACO_S) }
            spacing: (CRAWRAD_ESPACO_XS)
            <View> {
                width: (CRAWRAD_IML_MINI), height: (CRAWRAD_IML_MINI),
                flow: Overlay,
                <CrawradXadrez> { width: Fill, height: Fill }
                cel_simbolo = <CrawradLabel> {
                    width: Fill, height: Fill,
                    align: { x: 0.5, y: 0.5 }
                    draw_text: {
                        color: (CRAWRAD_TINTA_NO_XADREZ)
                        text_style: { font_size: (CRAWRAD_FONTE_CABECALHO) }
                    }
                    text: ""
                }
            }
            cel_indice = <CrawradLabelMuted> {
                width: Fill, height: Fit,
                align: { x: 0.5 }
                text: ""
            }
        }
        alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }


}
