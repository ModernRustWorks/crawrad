use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    pub CrawradInspectorRow = <View> {
        width: Fill, height: (CRAWRAD_LINHA_INSPECTOR),
        flow: Right, align: { y: 0.5 },
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: 0., bottom: 0. }
        spacing: (CRAWRAD_ESPACO_S)
        show_bg: true,
        draw_bg: {
            color: (CRAWRAD_PAINEL)
            instance listra: 0.0
            instance corrente: 0.0
            uniform cor_listra: (CRAWRAD_LISTRA)
            uniform cor_corrente: (CRAWRAD_SELECAO)
            fn pixel(self) -> vec4 {
                let c = mix(self.color, self.cor_listra, self.listra);
                let c2 = mix(c, self.cor_corrente, self.corrente);
                return vec4(c2.rgb * c2.a, c2.a);
            }
        }
    }
    pub CrawradPropName = <CrawradLabel> {
        width: Fit, height: Fit, flow: Right,
        draw_text: { color: (CRAWRAD_ACENTO_TEXTO) }
    }
    // The name column, cut by the turtle's clip; the shell writes the splitter's width.
    pub CrawradPropNameBox = <View> { width: (CRAWRAD_NAME_COLUMN), height: Fill, flow: Right, align: { y: 0.5 } }
    pub CrawradPropValueBox = <View> {
        width: Fill, height: Fill,
        flow: Right,
        align: { y: 0.5 }
    }
    // The same box holding several cells side by side: swatch, value, `…`.
    pub CrawradPropBar = <CrawradPropValueBox> { spacing: (CRAWRAD_ESPACO_XS) }
    pub CrawradPropValue = <CrawradLabel> {
        width: Fit, height: Fit,
        draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) }
    }
    pub CrawradPropNote = <CrawradLabelMuted> {
        width: Fit, height: Fit,
        draw_text: { color: (CRAWRAD_TEXTO_DESABILITADO) }
    }
    pub CrawradCampoDenso = <CrawradTextInput> {
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_TEXTO_NO_CAMPO), bottom: 0. }
    }
    pub CrawradPropInput = <CrawradCampoDenso> {
        width: (CRAWRAD_COLUNA_VALOR), height: (CRAWRAD_CAMPO_ALTURA),
    }
    // The `When` of a StyleRule: code font; red border when it is wrong.
    pub CrawradWhenInput = <CrawradPropInput> { width: Fill, draw_text: { text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } } }
    pub CrawradWhenInputBad = <CrawradWhenInput> { draw_bg: { border_color_1: (CRAWRAD_ERRO), border_color_2: (CRAWRAD_ERRO), border_color_1_hover: (CRAWRAD_ERRO), border_color_2_hover: (CRAWRAD_ERRO), border_color_1_focus: (CRAWRAD_ERRO), border_color_2_focus: (CRAWRAD_ERRO), border_color_1_empty: (CRAWRAD_ERRO), border_color_2_empty: (CRAWRAD_ERRO) } }
    // One colour row of Options > Color Scheme: its label and its hex.
    pub CrawradOptionColorRow = <View> { width: Fill, height: 26.0, flow: Right, align: { y: 0.5 }, spacing: 8.0 }
    pub CrawradOptionColorLabel = <CrawradLabelMuted> { width: 110.0, height: Fit }
    pub CrawradOptionColorInput = <CrawradPropInput> { width: Fill, height: 24.0 }
    pub CrawradEventInput = <CrawradPropInput> {
        width: Fill,
        draw_text: {
            text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) }
        }
    }
    // An inherited event's handler, drawn and not edited — dashed, dim,
    // italic; the base runs it, and a double click makes the override.
    pub CrawradInheritedHandler = <View> {
        width: Fill, height: (CRAWRAD_CAMPO_ALTURA),
        align: { y: 0.5 }
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: 0., bottom: 0. }
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                let p = self.pos * self.rect_size;
                let sdf = Sdf2d::viewport(p);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 1.5);
                sdf.fill_keep((CRAWRAD_GROUND));
                let dash = step(0.5, fract((p.x + p.y) / 6.0));
                let c = (CRAWRAD_BORDA_CONTROLE);
                sdf.stroke(vec4(c.rgb, c.a * dash), 1.0);
                return sdf.result;
            }
        }
        // Fit, clipped by the box: a Fill label breaks the name in two lines.
        clip_x: true,
        handler = <CrawradLabel> {
            width: Fit, height: Fit,
            draw_text: {
                color: (CRAWRAD_TEXTO_DESABILITADO)
                text_style: <THEME_FONT_ITALIC> { font_size: (CRAWRAD_FONTE_MONO) }
            }
        }
    }
    // The pill after the handler: `inherited` outlined, `override` filled.
    // `sdf.box` draws twice the radius it is given: 4 makes the 16 px pill.
    pub CrawradEventTag = <View> {
        width: Fit, height: 16.0,
        align: { y: 0.5 }
        padding: { left: 7., right: 7., top: 0., bottom: 0. }
        show_bg: true,
        draw_bg: {
            uniform fill: (CRAWRAD_TRANSPARENTE)
            uniform border: (CRAWRAD_BORDA_CONTROLE)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                sdf.fill_keep(self.fill);
                sdf.stroke(self.border, 1.0);
                return sdf.result;
            }
        }
        label = <CrawradLabel> {
            width: Fit, height: Fit,
            draw_text: {
                color: (CRAWRAD_TEXTO_SECUNDARIO)
                text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
            }
            text: "inherited"
        }
    }
    pub CrawradOverrideTag = <CrawradEventTag> {
        draw_bg: { fill: (CRAWRAD_ACENTO), border: (CRAWRAD_ACENTO) }
        label = {
            draw_text: {
                color: (CRAWRAD_GROUND)
                text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) }
            }
            text: "override"
        }
    }
    // The handler of an override: the event field, with the accent border.
    pub CrawradOverrideInput = <CrawradEventInput> {
        draw_bg: {
            border_color_1: (CRAWRAD_ACENTO)
            border_color_2: (CRAWRAD_ACENTO)
            border_color_1_hover: (CRAWRAD_ACENTO)
            border_color_2_hover: (CRAWRAD_ACENTO)
            border_color_1_empty: (CRAWRAD_ACENTO)
            border_color_2_empty: (CRAWRAD_ACENTO)
        }
    }
    // A `Bool`'s check box. No label: the `CrawradPropName` on the left names the row,
    // and `color_dither: 0.0` is the same cleanup as the buttons (`button.rs:118`).
    pub CrawradPropCheck = <CheckBox> {
        width: (CRAWRAD_COLUNA_VALOR), height: (CRAWRAD_CAMPO_ALTURA),
        padding: 0.,
        align: { x: 0.0, y: 0.5 }
        label_walk: { width: Fit, height: Fit, margin: 0. }
        draw_bg: {
            color_dither: 0.0
            size: 13.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO_GRADE)
            color: (CRAWRAD_GROUND)
            color_hover: (CRAWRAD_GROUND)
            color_down: (CRAWRAD_GROUND)
            color_active: (CRAWRAD_SELECAO)
            color_focus: (CRAWRAD_GROUND)
            color_disabled: (CRAWRAD_PAINEL)
            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_BORDA_FOCO)
            border_color_1_down: (CRAWRAD_BORDA_FOCO)
            border_color_1_active: (CRAWRAD_ACENTO)
            // The box on screen IS its border colour; an accent focus read as checked
            border_color_1_focus: (CRAWRAD_BORDA_FOCO)
            border_color_1_disabled: (CRAWRAD_DIVISOR)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_hover: (CRAWRAD_BORDA_FOCO)
            border_color_2_down: (CRAWRAD_BORDA_FOCO)
            border_color_2_active: (CRAWRAD_ACENTO)
            border_color_2_focus: (CRAWRAD_BORDA_FOCO)
            border_color_2_disabled: (CRAWRAD_DIVISOR)
            mark_color_active: (CRAWRAD_ACENTO_FOCO)
            mark_color_active_hover: (CRAWRAD_ACENTO_FOCO)
            mark_color_focus: (CRAWRAD_ACENTO_FOCO)
            mark_color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
        }
        text: ""
    }
    pub CrawradPropDrop = <DropDown> {
        width: (CRAWRAD_COLUNA_VALOR), height: (CRAWRAD_CAMPO_ALTURA),
        margin: 0.,
        // O `top` acompanha o `CrawradPropInput`: sem ele este molde RESETA o
        // respiro medido e o valor volta a sair alto na caixa.
        padding: { left: (CRAWRAD_ESPACO_M), right: 20.0, top: (CRAWRAD_TEXTO_NO_CAMPO), bottom: 0. }
        align: { x: 0.0, y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO_GRADE)
            color: (CRAWRAD_GROUND)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_CONTROLE)
            color_focus: (CRAWRAD_GROUND)
            color_disabled: (CRAWRAD_PAINEL)
            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_BORDA_FOCO)
            border_color_1_down: (CRAWRAD_BORDA_FOCO)
            border_color_1_focus: (CRAWRAD_ACENTO_FOCO)
            border_color_1_disabled: (CRAWRAD_DIVISOR)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_hover: (CRAWRAD_BORDA_FOCO)
            border_color_2_down: (CRAWRAD_BORDA_FOCO)
            border_color_2_focus: (CRAWRAD_ACENTO_FOCO)
            border_color_2_disabled: (CRAWRAD_DIVISOR)
            arrow_color: (CRAWRAD_TEXTO_TERCIARIO)
            arrow_color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            arrow_color_down: (CRAWRAD_TEXTO_PRIMARIO)
            arrow_color_focus: (CRAWRAD_ACENTO_FOCO)
            arrow_color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_ENFASE)
            color_down: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_ENFASE)
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
        popup_menu_position: BelowInput,
        popup_menu: <PopupMenu> {
            // Not the stock 150 px (`popup_menu.rs:446`), which ran off the
            // panel over the canvas; it does not follow the splitter.
            width: (CRAWRAD_COLUNA_VALOR),
            draw_bg: {
                color_dither: 0.0
                color: (CRAWRAD_FLUTUANTE)
                border_size: 1.0
                border_color_1: (CRAWRAD_DIVISOR)
                border_color_2: (CRAWRAD_DIVISOR)
            }
            menu_item: <PopupMenuItem> {
                height: (CRAWRAD_LINHA_INSPECTOR),
                padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M) }
                draw_bg: {
                    color_dither: 0.0
                    color: (CRAWRAD_FLUTUANTE)
                    color_hover: (CRAWRAD_SELECAO)
                    color_active: (CRAWRAD_SELECAO_FORTE)
                    mark_color_active: (CRAWRAD_ACENTO_FOCO)
                }
                draw_text: {
                    color: (CRAWRAD_TEXTO_SECUNDARIO)
                    color_hover: (CRAWRAD_TEXTO_ENFASE)
                    color_active: (CRAWRAD_ACENTO_TEXTO)
                    text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
            }
        }
    }
    pub CrawradOptionDrop = <CrawradPropDrop> {
        width: (CRAWRAD_OPTIONS_COMBO), height: (CRAWRAD_FORM_CAMPO_ALTURA),
        popup_menu: { width: (CRAWRAD_OPTIONS_COMBO) }
    }


    pub CrawradSubName = <CrawradPropName> {
        draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) }
    }

    // A row of the Inspector with its name in front; a sub-row's name is the
    // indented one. Every template of the list starts from one of the two.
    pub CrawradPropRow = <CrawradInspectorRow> { name_box = <CrawradPropNameBox> { name = <CrawradPropName> {} } }
    pub CrawradSubRow = <CrawradInspectorRow> { name_box = <CrawradPropNameBox> { name = <CrawradSubName> {} } }

    pub CrawradPropSwatch = <RoundedView> {
        width: (CRAWRAD_AMOSTRA), height: (CRAWRAD_AMOSTRA),
        margin: { right: (CRAWRAD_ESPACO_XS) }
        show_bg: true,
        draw_bg: { color: (CRAWRAD_CONTROLE), border_radius: (CRAWRAD_RAIO_GRADE), border_size: 1.0, border_color: (CRAWRAD_BORDA_CONTROLE) }
    }

    pub CrawradPropOpen = <CrawradButton> {
        width: (CRAWRAD_ALVO_LINHA), height: (CRAWRAD_ALVO_LINHA),
        margin: 0., padding: 0.,
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO_GRADE)
            color: (CRAWRAD_CONTROLE)
            color_hover: (CRAWRAD_SELECAO)
            color_down: (CRAWRAD_SELECAO_FORTE)
            color_focus: (CRAWRAD_CONTROLE)
            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_ACENTO)
            border_color_2_hover: (CRAWRAD_ACENTO)
            border_color_1_down: (CRAWRAD_ACENTO)
            border_color_2_down: (CRAWRAD_ACENTO)
            border_color_1_focus: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_focus: (CRAWRAD_BORDA_CONTROLE)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO)
            color_hover: (CRAWRAD_TEXTO_ENFASE)
            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) }
        }
    }
    // A Color Scheme field: the row, then its error under it — no
    // height while the field is valid, so the page does not grow a line per field.
    pub CrawradSchemeField = <View> { width: Fill, height: Fit, flow: Down }
    pub CrawradSchemeError = <CrawradLabelMuted> { width: Fill, height: 0.0, draw_text: { color: (CRAWRAD_ERRO) }, text: "" }
    pub CrawradColorPatch = <RoundedView> {
        width: 16.0, height: 16.0, show_bg: true,
        draw_bg: { color: (CRAWRAD_CONTROLE), border_radius: 2.0, border_size: 1.0, border_color: (CRAWRAD_BORDA_CONTROLE) }
    }
    // The click over a patch: no surface of its own, so the patch shows through.
    pub CrawradPatchPick = <Button> {
        width: Fill, height: Fill, margin: 0., padding: 0., text: ""
        draw_bg: { color: #0000, color_hover: #ffffff22, color_down: #ffffff44, color_focus: #0000, border_size: 0.0 }
    }

    pub CrawradPropFold = <CrawradPropOpen> {
        draw_bg: {
            border_size: 0.0
            color: (CRAWRAD_PAINEL)
            color_hover: (CRAWRAD_SELECAO)
            color_focus: (CRAWRAD_PAINEL)
            border_color_1: #0000
            border_color_2: #0000
            border_color_1_focus: #0000
            border_color_2_focus: #0000
        }
    }

    pub CrawradPropPick = <CrawradButton> {
        width: (CRAWRAD_COLUNA_VALOR), height: (CRAWRAD_CAMPO_ALTURA),
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M), top: 0., bottom: 0. }
        align: { x: 0.0, y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO_GRADE)
            color: (CRAWRAD_GROUND)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_CONTROLE)
            color_focus: (CRAWRAD_GROUND)
            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_BORDA_FOCO)
            border_color_2_hover: (CRAWRAD_BORDA_FOCO)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_ENFASE)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
        }
    }



    // A moldura das duas.
    pub CrawradInspectorPad = <CrawradOutlined> {
        width: Fit, height: Fit,
        flow: Down,
        padding: (CRAWRAD_ESPACO_M)
        spacing: (CRAWRAD_ESPACO_S)
        draw_bg: {
            uniform border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_FLUTUANTE)
        }
    }


    pub CrawradAlignZone = <Button> {
        margin: 0., padding: 0.,
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: 1.0
            instance ativo: 0.0
            uniform cor_ativo: (CRAWRAD_SELECAO_FORTE)
            color: (CRAWRAD_CONTROLE)
            color_hover: (CRAWRAD_SELECAO)
            color_down: (CRAWRAD_SELECAO_FORTE)
            color_focus: (CRAWRAD_CONTROLE)
            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_ACENTO)
            border_color_2_hover: (CRAWRAD_ACENTO)
        }
        draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } }
        text: ""
    }

    // O PAPEL do formulario, atras das cinco regioes.
    pub CrawradAlignPaper = <View> {
        width: (CRAWRAD_ALIGN_LARGURA), height: (CRAWRAD_ALIGN_ALTURA),
        flow: Down,
        padding: (CRAWRAD_ESPACO_S)
        spacing: (CRAWRAD_ESPACO_XS)
        show_bg: true,
        draw_bg: {
            uniform border_color: (CRAWRAD_BORDA_CONTROLE)
            color: (CRAWRAD_GROUND)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 2.0);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }


}
