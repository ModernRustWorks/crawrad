use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    pub CrawradIconRow = <View> {
        width: Fill, height: (CRAWRAD_LINHA_ALTURA),
        flow: Overlay,
        alvo = <CrawradListRow> { width: Fill, height: Fill }
        face = <View> {
            width: Fill, height: Fill,
            flow: Right,
            align: { y: 0.5 }
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) }
            spacing: (CRAWRAD_ESPACO_M)
            visto = <CrawradLabel> {
                width: (CRAWRAD_VISTO_LARGURA),
                draw_text: {
                    color: (CRAWRAD_ACENTO_TEXTO)
                    text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_ICONE_LINHA) }
                }
                text: ""
            }
            icone = <CrawradClassIcon> {}
            nome = <CrawradLabel> {
                draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) }
                text: ""
            }
            classe = <CrawradLabel> {
                draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) }
                text: ""
            }
            // "inherited" on a component of the base form.
            gap = <CrawradFiller> {}
            inherited = <CrawradLabel> {
                draw_text: {
                    color: (CRAWRAD_INHERITED)
                    text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) }
                }
                text: ""
            }
        }
    }
    pub CrawradIconRowRoot = <CrawradIconRow> {
        face = {
            nome = {
                draw_text: {
                    color: (CRAWRAD_TEXTO_PRIMARIO)
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
            }
        }
    }
    pub CrawradTab = <CrawradListRow> {
        width: Fit, height: (CRAWRAD_ABA_ALTURA),
        padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_L), top: 0., bottom: 0. }
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            color: (CRAWRAD_ELEVADO)
            color_hover: (CRAWRAD_CONTROLE)
            border_color_1: (CRAWRAD_ELEVADO)
            border_color_2: (CRAWRAD_ELEVADO)
            border_color_1_hover: (CRAWRAD_CONTROLE)
            border_color_2_hover: (CRAWRAD_CONTROLE)
        }
    }
    // The ACTIVE tab. It is the only place in the strip with a selection surface.
    pub CrawradTabActive = <CrawradTab> {
        draw_bg: {
            color: (CRAWRAD_SELECAO)
            color_hover: (CRAWRAD_SELECAO)
            border_color_1: (CRAWRAD_ACENTO)
            border_color_2: (CRAWRAD_ACENTO)
            border_color_1_hover: (CRAWRAD_ACENTO)
            border_color_2_hover: (CRAWRAD_ACENTO)
        }
        draw_text: { color: (CRAWRAD_ACENTO_TEXTO), color_hover: (CRAWRAD_ACENTO_FOCO) }
    }
    pub CrawradInspectorTab = <RadioButtonTab> {
        width: Fit, height: (CRAWRAD_LINHA_INSPECTOR),
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: 0., bottom: 0. }
        label_walk: { width: Fit, height: Fit, margin: 0. }
        label_align: { y: 0.5 }
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_ELEVADO)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_CONTROLE)
            color_active: (CRAWRAD_SELECAO)
            color_focus: (CRAWRAD_ELEVADO)
            color_disabled: (CRAWRAD_PAINEL)
            border_color_1: (CRAWRAD_ELEVADO)
            border_color_1_hover: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_down: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_active: (CRAWRAD_ACENTO)
            border_color_1_focus: (CRAWRAD_ELEVADO)
            border_color_1_disabled: (CRAWRAD_DIVISOR)
            border_color_2: (CRAWRAD_ELEVADO)
            border_color_2_hover: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_down: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_active: (CRAWRAD_ACENTO)
            border_color_2_focus: (CRAWRAD_ELEVADO)
            border_color_2_disabled: (CRAWRAD_DIVISOR)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_TEXTO_PRIMARIO)
            color_active: (CRAWRAD_ACENTO_TEXTO)
            color_focus: (CRAWRAD_TEXTO_PRIMARIO)
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
        }
    }
    pub CrawradCenterTab = <CrawradInspectorTab> {
        height: 22.0,
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_L), top: 0., bottom: 0. }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: 0.0
            color: (CRAWRAD_PAINEL)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_CONTROLE)
            color_active: (CRAWRAD_CONTROLE)
            color_focus: (CRAWRAD_CONTROLE)
            color_disabled: (CRAWRAD_PAINEL)
            border_color_1: (CRAWRAD_DIVISOR)
            border_color_1_hover: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_down: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_active: (CRAWRAD_TEXTO_PRIMARIO)
            border_color_1_focus: (CRAWRAD_TEXTO_PRIMARIO)
            border_color_1_disabled: (CRAWRAD_DIVISOR)
            border_color_2: (CRAWRAD_DIVISOR)
            border_color_2_hover: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_down: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_active: (CRAWRAD_TEXTO_PRIMARIO)
            border_color_2_focus: (CRAWRAD_TEXTO_PRIMARIO)
            border_color_2_disabled: (CRAWRAD_DIVISOR)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_TEXTO_PRIMARIO)
            color_active: (CRAWRAD_TEXTO_PRIMARIO)
            color_focus: (CRAWRAD_TEXTO_PRIMARIO)
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
        }
    }
    pub CrawradActionIcon = <Button> {
        width: 26.0, height: (CRAWRAD_ABA_ALTURA),
        margin: 0., padding: 0.,
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_radius: 0.0
            color: #0000
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_SELECAO_FORTE)
            color_focus: #0000
            border_size: 0.0
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_TERCIARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_TERCIARIO)
            text_style: <CRAWRAD_FONT_ICONS> { font_size: 14.0 }
        }
    }
    // The code editor TAB — one per open file. The ACTIVE tab has the color of the
    // editor (CRAWRAD_PAINEL) and blends into code; the inactive tab, CRAWRAD_ELEVADO.
    pub CrawradEditorTab = <View> {
        width: Fit, height: (CRAWRAD_ABA_ALTURA),
        flow: Right, align: { y: 0.5 },
        padding: { left: 10.0, right: 6.0, top: 0., bottom: 0. }
        spacing: (CRAWRAD_ESPACO_S),
        show_bg: true,
        draw_bg: {
            instance active: 0.0
            uniform color_idle: (CRAWRAD_ELEVADO)
            uniform color_on: (CRAWRAD_PAINEL)
            uniform border_color: (CRAWRAD_DIVISOR)
            uniform color_mark: (CRAWRAD_ACENTO) // the active tab's line, at its base
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Subtle radius on top corners to follow the outer card;
                // doubled height pushes rounded bottom corners outward.
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y * 2.0, 3.0);
                sdf.fill(mix(self.color_idle, self.color_on, self.active));
                // above `CrawradTabsScrollBar` (4 px over the base); 3 px keep a full row
                sdf.rect(0.0, self.rect_size.y - 7.0, self.rect_size.x - 1.0, 3.0);
                sdf.fill(vec4(self.color_mark.rgb, self.color_mark.a * self.active));
                // Subtle vertical separator on the right of the tab
                sdf.rect(self.rect_size.x - 1.0, 0.0, 1.0, self.rect_size.y);
                sdf.fill(self.border_color);
                return sdf.result;
            }
        }
        tab_label = <Button> {
            width: Fit, height: Fill,
            margin: 0., padding: 0.,
            align: { x: 0.0, y: 0.5 }
            draw_bg: {
                // TRANSPARENT in all states: background comes from the `View` above,
                // and a second rectangle here would split the tab in half.
                color: #0000
                color_hover: #0000
                color_down: #0000
                color_focus: #0000
                border_size: 0.0
            }
            draw_text: {
                color: (CRAWRAD_TEXTO_TERCIARIO)
                color_hover: (CRAWRAD_TEXTO_PRIMARIO)
                color_down: (CRAWRAD_TEXTO_PRIMARIO)
                color_focus: (CRAWRAD_TEXTO_TERCIARIO)
                text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
            }
        }
        tab_close = <Button> {
            width: (CRAWRAD_ABA_FECHAR), height: (CRAWRAD_ABA_FECHAR),
            margin: 0., padding: 0.,
            align: { x: 0.5, y: 0.5 }
            draw_bg: {
                color_dither: 0.0
                border_radius: 0.0
                color: #0000
                color_hover: (CRAWRAD_CONTROLE)
                color_down: (CRAWRAD_CONTROLE)
                color_focus: #0000
                border_size: 0.0
                instance dirty: 0.0
                uniform color_dot: (CRAWRAD_ACENTO)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 2.0);
                    sdf.fill(mix(self.color, self.color_hover, max(self.hover, self.down)));
                    sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 3.5);
                    sdf.fill(vec4(self.color_dot.rgb, self.color_dot.a * self.dirty * (1.0 - self.hover)));
                    return sdf.result;
                }
            }
            draw_text: {
                // The X hides while the tab is dirty, unless the mouse is on it.
                instance dirty: 0.0
                fn get_color(self) -> vec4 {
                    let c = mix(mix(self.color, self.color_hover, self.hover), self.color_down, self.down);
                    return vec4(c.rgb, c.a * mix(1.0, self.hover, self.dirty));
                }
                color: (CRAWRAD_TEXTO_TERCIARIO)
                color_hover: (CRAWRAD_TEXTO_PRIMARIO)
                color_down: (CRAWRAD_TEXTO_PRIMARIO)
                color_focus: (CRAWRAD_TEXTO_TERCIARIO)
                text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_ABA_FECHAR) }
            }
        }
    }
    // A text field. The background is `ground` — the same `extreme_bg_color`
    // that egui uses for input fields (`tema.rs:557`).
    pub CrawradTextInput = <TextInput> {
        width: Fill, height: 24.0,
        margin: 0.,
        empty_text: ""
        label_align: { x: 0.0, y: 0.5 },
        padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_ESPACO_M), top: (CRAWRAD_TEXTO_NO_CAMPO), bottom: 0. }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_GROUND)
            color_hover: (CRAWRAD_GROUND)
            color_focus: (CRAWRAD_GROUND)
            color_down: (CRAWRAD_GROUND)
            color_empty: (CRAWRAD_GROUND)
            border_color_1: (CRAWRAD_BORDA_CONTROLE)
            border_color_2: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_hover: (CRAWRAD_BORDA_FOCO)
            border_color_2_hover: (CRAWRAD_BORDA_FOCO)
            border_color_1_focus: (CRAWRAD_ACENTO_FOCO)
            border_color_2_focus: (CRAWRAD_ACENTO_FOCO)
            border_color_1_empty: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_empty: (CRAWRAD_BORDA_CONTROLE)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_focus: (CRAWRAD_TEXTO_ENFASE)
            color_empty: (CRAWRAD_TEXTO_DESABILITADO)
            color_empty_hover: (CRAWRAD_TEXTO_TERCIARIO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO), line_spacing: 1.0 }
        }
        draw_selection: {
            color: (CRAWRAD_SELECAO)
            color_hover: (CRAWRAD_SELECAO)
            color_focus: (CRAWRAD_SELECAO_FORTE)
        }
        draw_cursor: { color: (CRAWRAD_ACENTO_FOCO) }
    }
    pub CrawradScrollBar = <ScrollBar> {
        draw_bg: {
            color: (CRAWRAD_BORDA_CONTROLE)
            color_hover: (CRAWRAD_BORDA_FOCO)
            color_drag: (CRAWRAD_ACENTO)
        }
    }
    // Horizontal scrollbar for the editor file tabs.
    // Thin (3px) and positioned at the base of the tab bar, with breathing room identical to a modern editor.
    pub CrawradTabsScrollBar = <ScrollBar> {
        bar_size: 4.0,
        bar_side_margin: 0.0,
        min_handle_size: 20.0,
        draw_bg: {
            uniform size: 3.0,
            uniform border_size: 0.0,
            uniform border_radius: 1.5,
            color: (CRAWRAD_BORDA_CONTROLE)
            color_hover: (CRAWRAD_BORDA_FOCO)
            color_drag: (CRAWRAD_ACENTO)
        }
    }
    pub CrawradScrollBars = <ScrollBars> {
        show_scroll_x: false,
        show_scroll_y: true,
        scroll_bar_x: <CrawradScrollBar> {}
        scroll_bar_y: <CrawradScrollBar> {}
    }
    pub CrawradCard = <CrawradOutlined> {
        width: Fill, height: Fit,
        flow: Down,
        padding: { left: 14.0, right: 14.0, top: 12.0, bottom: 12.0 }
    }

    // The USER prompt card. It uses the orange bubble look of CrawRAD (#de5a2b),
    // rounded corners and a larger left margin to produce the characteristic right-aligned send effect.
    pub CrawradCardUser = <CrawradCard> {
        margin: { left: 44.0, right: 0.0, top: 4.0, bottom: 4.0 },
        draw_bg: {
            uniform border_color: (CRAWRAD_ACENTO_BORDA)
            uniform border_radius: 8.0
            color: (CRAWRAD_ACENTO)
            // The bottom-right corner is the bubble's tail: the side it speaks from.
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;
                sdf.box_all(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, r, r, 3.0, r);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
        padding: { left: 13.0, right: 11.0, top: 6.0, bottom: 6.0 }
    }

    // AI chat input field, integrated into the floating card.
    // Transparent so that the card provides a unified outline and background.
    pub CrawradPromptInput = <TextInput> {
        width: Fill, height: 38.0,
        margin: 0.,
        empty_text: "Ask anything, @ to mention, / for actions",
        label_align: { x: 0.0, y: 0.5 },
        padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_TEXTO_NO_CAMPO), bottom: 0. }
        draw_bg: {
            color_dither: 0.0
            border_size: 0.0
            border_radius: 0.0
            color: #0000
            color_hover: #0000
            color_focus: #0000
            color_down: #0000
            color_empty: #0000
            border_color_1: #0000
            border_color_2: #0000
            border_color_1_hover: #0000
            border_color_2_hover: #0000
            border_color_1_focus: #0000
            border_color_2_focus: #0000
            border_color_1_empty: #0000
            border_color_2_empty: #0000
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_focus: (CRAWRAD_TEXTO_ENFASE)
            color_empty: (CRAWRAD_TEXTO_TERCIARIO)
            color_empty_hover: (CRAWRAD_TEXTO_SECUNDARIO)
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO), line_spacing: 1.0 }
        }
        draw_selection: {
            color: (CRAWRAD_SELECAO)
            color_hover: (CRAWRAD_SELECAO)
            color_focus: (CRAWRAD_SELECAO_FORTE)
        }
        draw_cursor: { color: (CRAWRAD_ACENTO_FOCO) }
    }

    // Circular button to submit AI prompts (26x26 perfect circle with white arrow).
    pub CrawradCircleButton = <Button> {
        width: 26.0, height: 26.0,
        margin: 0., padding: 0.,
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            instance disabled: 0.0
            color: (CRAWRAD_ACENTO)
            color_hover: (CRAWRAD_ACENTO_FOCO)
            color_down: (CRAWRAD_ACENTO_TEXTO)
            uniform color_disabled: (CRAWRAD_BORDA_CONTROLE)

            border_size: 0.0
            border_radius: 13.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let base = mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                );
                let c = mix(base, self.color_disabled, self.disabled);
                let r = min(self.rect_size.x, self.rect_size.y) * 0.5;
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, r - 0.5);
                sdf.fill(c);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff
            color_hover: #ffffff
            color_down: #ffffff
            color_focus: #ffffff
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
        }
    }

    // Microphone button for audio input (Speech-to-Text).
    // SDF vector drawing identical to reference (capsule + U-cradle + stem + base).
    pub CrawradMicButton = <Button> {
        width: 24.0, height: 26.0,
        margin: { right: (CRAWRAD_ESPACO_XS) },
        padding: 0.,
        align: { x: 0.5, y: 0.5 }
        draw_bg: {
            instance recording: 0.0
            color: (CRAWRAD_TEXTO_SECUNDARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let ic = mix(
                    mix(self.color, self.color_hover, self.hover),
                    (CRAWRAD_ERRO),
                    self.recording
                );
                let mid_x = self.rect_size.x * 0.5;
                let mid_y = self.rect_size.y * 0.5;

                // 1. Central microphone capsule
                sdf.box(mid_x - 3.0, mid_y - 7.0, 6.0, 10.0, 3.0);
                sdf.fill(ic);

                // 2. U-cradle
                sdf.move_to(mid_x - 5.5, mid_y - 2.5);
                sdf.line_to(mid_x - 5.5, mid_y + 0.5);
                sdf.line_to(mid_x - 3.0, mid_y + 4.5);
                sdf.line_to(mid_x + 3.0, mid_y + 4.5);
                sdf.line_to(mid_x + 5.5, mid_y + 0.5);
                sdf.line_to(mid_x + 5.5, mid_y - 2.5);
                sdf.stroke(ic, 1.3);

                // 3. Vertical stem
                sdf.move_to(mid_x, mid_y + 4.5);
                sdf.line_to(mid_x, mid_y + 8.5);
                sdf.stroke(ic, 1.3);

                // 4. Horizontal base
                sdf.move_to(mid_x - 3.5, mid_y + 8.5);
                sdf.line_to(mid_x + 3.5, mid_y + 8.5);
                sdf.stroke(ic, 1.3);

                return sdf.result;
            }
        }
    }

    // Floating autocomplete container (classic Code Insight / a modern editor).
    pub CrawradCompletionBox = <View> {
        visible: false,
        width: 480.0, height: Fit,
        flow: Down,
        align: { x: 0.0, y: 0.0 },
        padding: 4.0,
        show_bg: true,
        draw_bg: {
            uniform border_color: (CRAWRAD_DIVISOR)
            color: (CRAWRAD_FLUTUANTE)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, (CRAWRAD_CARD_RADIUS));
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }

    // One column of a completion row: the shell colours it per row.
    pub CrawradCompletionCell = <Label> {
        draw_text: { text_style: <CRAWRAD_FONT_CODE_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
    }

    // Individual autocomplete / Code Insight list item (classic RAD IDE / modern-editor style).
    // The button under the columns takes the click and the hover.
    pub CrawradCompletionItem = <View> {
        width: Fill, height: 22.0, flow: Overlay,
        bg = <Button> {
            width: Fill, height: Fill, margin: 0.0, text: "",
            draw_bg: {
                color_dither: 0.0
                border_size: 0.0
                border_radius: 2.0
                color: (CRAWRAD_FLUTUANTE)
                color_hover: (CRAWRAD_SELECAO_FORTE)
                // A deprecated label struck through, from x to x.
                instance strike_from: 0.0, instance strike_to: 0.0
                uniform strike_color: (CRAWRAD_TEXTO_PRIMARIO)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                    sdf.fill(mix(self.color, self.color_hover, self.hover));
                    if self.strike_to > self.strike_from {
                        sdf.move_to(self.strike_from, self.rect_size.y * 0.5);
                        sdf.line_to(self.strike_to, self.rect_size.y * 0.5);
                        sdf.stroke(self.strike_color, 1.0);
                    }
                    return sdf.result;
                }
            }
        }
        cols = <View> {
            width: Fill, height: Fill, flow: Right, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0 }, spacing: 8.0,
            icon = <CrawradCompletionCell> {}
            label = <CrawradCompletionCell> {}
            origin = <CrawradCompletionCell> {}
            <View> { width: Fill, height: Fill }
            signature = <CrawradCompletionCell> {}
        }
    }

    // Pill-shaped button for model or provider selection.
    pub CrawradPillButton = <Button> {
        height: 22.0,
        margin: 0.,
        padding: { left: (CRAWRAD_ESPACO_L), right: (CRAWRAD_ESPACO_L), top: 0., bottom: 0. }
        align: { y: 0.5 }
        draw_bg: {
            color_dither: 0.0
            border_size: 1.0
            border_radius: (CRAWRAD_RAIO_BOTAO)

            color: (CRAWRAD_LISTRA)
            color_hover: (CRAWRAD_CONTROLE)
            color_down: (CRAWRAD_PAINEL)
            color_focus: (CRAWRAD_LISTRA)
            color_disabled: (CRAWRAD_PAINEL)

            border_color_1: (CRAWRAD_DIVISOR)
            border_color_2: (CRAWRAD_DIVISOR)
            border_color_1_hover: (CRAWRAD_BORDA_CONTROLE)
            border_color_2_hover: (CRAWRAD_BORDA_CONTROLE)
            border_color_1_down: (CRAWRAD_ACENTO)
            border_color_2_down: (CRAWRAD_ACENTO)
            border_color_1_focus: (CRAWRAD_DIVISOR)
            border_color_2_focus: (CRAWRAD_DIVISOR)
            border_color_1_disabled: (CRAWRAD_DIVISOR)
            border_color_2_disabled: (CRAWRAD_DIVISOR)
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_SECUNDARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_TEXTO_ENFASE)
            color_focus: (CRAWRAD_TEXTO_SECUNDARIO)
            color_disabled: (CRAWRAD_TEXTO_DESABILITADO)
            text_style: <THEME_FONT_REGULAR> { font_size: 9.5 }
        }
    }


}
