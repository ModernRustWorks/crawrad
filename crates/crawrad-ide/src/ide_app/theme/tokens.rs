use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    pub CRAWRAD_GROUND = #101214
    pub CRAWRAD_PAINEL = #1d1f20
    // Chrome strip (top bar, icon rail, header, footer): sits below the panel.
    pub CRAWRAD_ELEVADO = #161819
    // Floating elements: menu, tooltip, window.
    pub CRAWRAD_FLUTUANTE = #26292b
    // Control surface at rest.
    pub CRAWRAD_CONTROLE = #2b2b2d
    // Alternating stripe of a dense grid.
    pub CRAWRAD_LISTRA = #232628
    // Thin divider between regions.
    pub CRAWRAD_DIVISOR = #424244
    // Outline of a control at rest.
    pub CRAWRAD_BORDA_CONTROLE = #4d4f52
    // Outline on hover.
    pub CRAWRAD_BORDA_FOCO = #5d5d60
    pub CRAWRAD_TEXTO_DESABILITADO = #6b6b6e
    pub CRAWRAD_TEXTO_TERCIARIO = #98989b
    pub CRAWRAD_TEXTO_SECUNDARIO = #b7b7ba
    pub CRAWRAD_TEXTO_PRIMARIO = #e7e7ea
    pub CRAWRAD_TEXTO_ENFASE = #f5f5f8
    pub CRAWRAD_SELECAO = #1d2d3d
    pub CRAWRAD_SELECAO_FORTE = #2c455d
    pub CRAWRAD_DEPURANDO = #1d5cc6
    // Log points: the violet their markers wear (approved Project Options › Log mockup).
    pub CRAWRAD_LOG_TEXT = #c9b6ff
    pub CRAWRAD_LOG_FILL = #2a2540
    pub CRAWRAD_LOG_BORDER = #4a3f72
    // The designer bar's WEB PAGE badge (approved web-page mockup).
    pub CRAWRAD_WEB_BADGE_FILL = #2d4a63
    pub CRAWRAD_WEB_BADGE_TEXT = #b3dcff
    pub CRAWRAD_ACENTO_BORDA = #b83d16
    // Official CrawRAD accent: Rust Orange (Ferris / official Rust identity)
    pub CRAWRAD_ACENTO = #de5a2b
    pub CRAWRAD_ACENTO_TEXTO = #ff9c6a
    pub CRAWRAD_TEACH_FILL = #3a2a24
    pub CRAWRAD_TEACH_HOVER = #4a3027
    pub CRAWRAD_BUBBLE_META = #ffe2d3
    // The agent chip's dot when the last answer came through ACP.
    pub CRAWRAD_CHIP_ACP = #3fb950
    pub CRAWRAD_AVATAR_USER = #3a3b40
    pub CRAWRAD_AVATAR_AGENT = #2d2117
    pub CRAWRAD_AVATAR_AGENT_BORDER = #5a3a24
    pub CRAWRAD_CRUMB_SEGMENT = #303136
    pub CRAWRAD_CRUMB_TEXT = #bcbcbc
    pub CRAWRAD_CRUMB_CURRENT = #4a3a30
    pub CRAWRAD_CRUMB_CURRENT_TEXT = #f3c9ad
    pub CRAWRAD_ACENTO_FOCO = #f07b46
    pub CRAWRAD_ERRO = #e2726b
    // The "ready" green of the rust-analyzer pill.
    pub CRAWRAD_OK = #7cb46b
    pub CRAWRAD_AVISO = #d9a441
    // The Problems panel's Errors chip: a dark red pill with light red text.
    pub CRAWRAD_ERROR_CHIP = #3a2224
    pub CRAWRAD_ERROR_CHIP_TEXT = #f2a3a0
    pub CRAWRAD_ERROR_CHIP_HOVER = #4a2a2d
    // A clickable file:line:column in a panel (approved mockup).
    pub CRAWRAD_LINK = #8fb8ff
    pub CRAWRAD_NOTE_COMMENT = #3DC9B0
    pub CRAWRAD_BOTAO_PRIMARIO = #de5a2b
    pub CRAWRAD_BOTAO_PRIMARIO_HOVER = #ea6328
    pub CRAWRAD_BOTAO_PRIMARIO_DOWN = #c34316
    pub CRAWRAD_BOTAO_PRIMARIO_BORDA = #b83d16
    pub CRAWRAD_SPLITTER_HOVER = #de5a2b
    // What comes from the base form: the canvas badge's orange.
    pub CRAWRAD_INHERITED = #de5a2b
    pub CRAWRAD_SPLITTER_DRAG = #f07b46
    pub CRAWRAD_TRANSPARENTE = #00000000
    pub CRAWRAD_BRANCO = #ffffff
    pub CRAWRAD_FECHAR_HOVER = #c42b1c
    pub CRAWRAD_ESPACO_XS = 2.0
    pub CRAWRAD_ESPACO_S = 4.0
    pub CRAWRAD_ESPACO_M = 6.0
    pub CRAWRAD_ESPACO_L = 10.0
    pub CRAWRAD_TEXTO_NO_CAMPO = 2.0
    pub CRAWRAD_MARGEM_PAINEL = 10.0
    pub CRAWRAD_INDENT = 14.0

    pub CRAWRAD_GOTEIRA = 18.0
    // Minimum width of an icon-only button.
    pub CRAWRAD_ALVO_ICONE = 26.0
    // Gap separating a button GROUP from its neighbor on the toolbar.
    pub CRAWRAD_GRUPO_BARRA = 7.0
    // Discrete corners. No pills.
    pub CRAWRAD_RAIO = 2.0
    // Standard rounding constant for buttons across the entire IDE.
    pub CRAWRAD_RAIO_BOTAO = 2.0
    pub CRAWRAD_RAIO_GRADE = 0.0

    pub CRAWRAD_CARD_RADIUS = 6.0
    pub CRAWRAD_CARD_GAP = 3.0

    pub CRAWRAD_BARRA_ALTURA = 34.0
    pub CRAWRAD_RAIL_LARGURA = 48.0
    pub CRAWRAD_ABA_ALTURA = 35.0
    pub CRAWRAD_ABA_FECHAR = 16.0
    pub CRAWRAD_ABA_CENTRO_ALTURA = 22.0
    // The checkmark column in a tree row. See `CrawradIconRow`.
    pub CRAWRAD_VISTO_LARGURA = 12.0
    pub CRAWRAD_RODAPE_ALTURA = 26.0
    pub CRAWRAD_DOCA_LARGURA = 300.0
    pub CRAWRAD_CHAT_LARGURA = 390.0
    pub CRAWRAD_LINHA_ALTURA = 19.0 // list/tree rows on VS Code's 22 px pitch: a fixed-height row...
    pub CRAWRAD_LINHA_RESPIRO = 3.0 // ...or a content-height one, with this above and below its text

    pub CRAWRAD_LINHA_INSPECTOR = 21.0
    pub CRAWRAD_CAMPO_ALTURA = 19.0
    // The splitter's rest = `NAME_WIDTH_DEFAULT`; `dock_fits_widest_catalog_label` checks.
    pub CRAWRAD_NAME_COLUMN = 136.0
    // A combo's POPUP list, which does not follow the splitter.
    pub CRAWRAD_COLUNA_VALOR = 96.0
    // Standard height and width for fields and combos in Options and Dialog screens
    pub CRAWRAD_FORM_CAMPO_ALTURA = 26.0
    pub CRAWRAD_OPTIONS_COMBO = 520.0
    pub CRAWRAD_FOLGA_DA_BARRA = 16.0
    pub CRAWRAD_AMOSTRA = 14.0
    pub CRAWRAD_ALVO_LINHA = 18.0

    pub CRAWRAD_PAD_LARGURA = 320.0
    pub CRAWRAD_PAD_ALTURA = 200.0
    pub CRAWRAD_ALIGN_LARGURA = 168.0
    pub CRAWRAD_ALIGN_ALTURA = 120.0
    pub CRAWRAD_ALIGN_FAIXA = 22.0


    // 12 px — hint, count, footer.
    pub CRAWRAD_FONTE_PEQUENO = 9.0
    // 13 px — normal text.
    pub CRAWRAD_FONTE_CORPO = 9.75
    // 12 px — code.
    pub CRAWRAD_FONTE_MONO = 9.0
    // 14 px — panel header, and toolbar command icon.
    pub CRAWRAD_FONTE_CABECALHO = 10.5
    pub CRAWRAD_FONTE_RAIL = 18.0
    pub CRAWRAD_FONTE_ABA_FECHAR = 8.25
    pub CRAWRAD_FONTE_RODAPE_ICONE = 10.5

    pub CRAWRAD_FONTE_ICONE_LINHA = 12.0

    pub CRAWRAD_FONT_ICONS = {
        font_family: {
            icons = font("crate://self/assets/fontes/codicon.ttf", 0.0, 0.0)
        },
        line_spacing: 1.2,
    }

    pub CRAWRAD_FONT_CODE_ICONS = {
        font_family: {
            latin = font("crate://crawrad_runtime_makepad/resources/LiberationMono-Regular.ttf", 0.0, 0.0)
            icons = font("crate://self/assets/fontes/codicon.ttf", 0.0, 0.0)
        },
        line_spacing: 1.35,
    }

    // The same for the UI text: a menu row's keys column shows a chevron.
    pub CRAWRAD_FONT_TEXT_ICONS = {
        font_family: {
            latin = font("crate://makepad_widgets/resources/IBMPlexSans-Text.ttf", -0.1, 0.0)
            icons = font("crate://self/assets/fontes/codicon.ttf", 0.0, 0.0)
        },
        line_spacing: 1.2,
    }

    // The same for the bold UI text (the breadcrumb's symbol).
    pub CRAWRAD_FONT_BOLD_ICONS = {
        font_family: {
            latin = font("crate://makepad_widgets/resources/IBMPlexSans-SemiBold.ttf", -0.1, 0.0)
            icons = font("crate://self/assets/fontes/codicon.ttf", 0.0, 0.0)
        },
        line_spacing: 1.2,
    }

    pub CRAWRAD_ICONE_GRADE = 16.0

    // Number of icons in the strip, equal to ClassIcon::ALL.len() (a test checks it).
    // Rows override it with the strip's real count (native + third-party icons).
    pub CRAWRAD_ICONE_QUANTIDADE = 54.0

    pub CRAWRAD_ICONE_TRACO = 1.25

    pub CRAWRAD_MENU_LARGURA = 260.0

    pub CRAWRAD_PACOTE_COLUNA = 150.0

    // Menu row height. It is larger than a list row: a menu is a
    // pass-through click target, and 17 px is the pitch of a dense tree.
    pub CRAWRAD_MENU_LINHA = 22.0

    pub CRAWRAD_MENU_MARCA = 16.0

    pub CRAWRAD_MENU_SEPARADOR = 7.0


}
