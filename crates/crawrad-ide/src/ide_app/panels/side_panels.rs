use makepad_widgets::*;

live_design! {
    link crawrad_panels;

    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use link::crawrad::*;
    use makepad_code_editor::notes_panel_view::NotesPanelView;
    pub CrawradPanelComponents = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        // Toolbox Header
        toolbox_header = <CrawradPanelHeader> {
            title = { text: "Toolbox" }
            toolbox_spacer = <CrawradFiller> {}
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }
        no_form = <CrawradEmptyState> { visible: false }
        toolbox_filter = <View> {
            width: Fill, height: Fit,
            flow: Down,
            padding: (CRAWRAD_MARGEM_PAINEL),
            filter_input = <CrawradTextInput> { empty_text: "Filter" }
            sub = <CrawradLabelMuted> {
                margin: { top: (CRAWRAD_ESPACO_S) },
                text: "click a component, then click the form"
            }
        }

        // The palette comes from the catalogue (`paleta::linhas`), a PortalList because live_design! is static (ADR 0027); it shares the column with Structure.
        toolbox_palette = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_MARGEM_PAINEL), right: (CRAWRAD_MARGEM_PAINEL), bottom: (CRAWRAD_ESPACO_M) },

            toolbox_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}

                Secao = <CrawradGroupLabel> {}

                SecaoPrimeira = <CrawradGroupLabel> {
                    margin: { top: 0.0, bottom: (CRAWRAD_ESPACO_XS) }
                }

                Item = <CrawradIconRow> {}
                ItemOff = <CrawradIconRow> { face = { nome = { draw_text: { color: (CRAWRAD_TEXTO_DESABILITADO) } } } }

                Vazia = <CrawradLabelMuted> {
                    margin: { top: (CRAWRAD_ESPACO_M) }
                }
            }
        }

        sep_left_mid = <CrawradSeparatorH> {}

        // Structure Object Hierarchy Tree
        structure_header = <CrawradPanelHeader> {
            title = { text: "Structure" }
            preencher = <View> { width: Fill, height: 0. }
            add_item = <CrawradPanelClose> {}
        }
        // The document tree: a PortalList filled by handle_draw_2d, one row per node of the open form (ADR 0027).
        structure_tree = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S) },

            structure_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}

                // Two templates: the root row (bold) and any other; selection is drawn by the row's own mark.
                Linha = <CrawradIconRow> {}
                Raiz = <CrawradIconRowRoot> {}
            }
        }
    }

    pub CrawradPanelInspector = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        inspector_header = <CrawradPanelHeader> {
            // The title is written from Rust (`Panel::label`): with the dock tabs collapsed, the panel needs its own name.
            title = { text: "" }
            inspector_spacer = <CrawradFiller> {}
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        // The component picker (the classic designer's Object Inspector combo): the inspected component's
        // name in bold, its class muted; a click lists every component of the form.
        inspector_picker = <View> {
            width: Fill, height: 28.0, flow: Overlay,
            margin: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), bottom: (CRAWRAD_ESPACO_XS) }
            show_bg: true,
            draw_bg: { color: (CRAWRAD_FLUTUANTE) }
            // The hit area is drawn first, so its hover never covers the text.
            picker_hit = <CrawradButton> { width: Fill, height: Fill, text: "", draw_bg: { color: (CRAWRAD_TRANSPARENTE), color_hover: (CRAWRAD_SELECAO), color_down: (CRAWRAD_SELECAO_FORTE) } }
            <View> {
                width: Fill, height: Fill, flow: Right, align: { y: 0.5 },
                spacing: (CRAWRAD_ESPACO_S),
                padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) }
                picker_name = <Label> {
                    draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }
                    text: ""
                }
                picker_class = <CrawradLabelMuted> { text: "" }
                <CrawradFiller> {}
                picker_arrow = <CrawradLabelMuted> { text: "", draw_text: { text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
            }
        }

        inspector_tabs = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: (CRAWRAD_ESPACO_XS),
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
            tab_props = <CrawradInspectorTab> { text: "" }
            tab_events = <CrawradInspectorTab> { text: "" }
        }

        inspector_panel = <View> {
            width: Fill, height: Fill,
            // Overlay: the name/value splitter lies over the rows.
            flow: Overlay,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S) },

            inspector_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}


                Category = <CrawradCategoryRow> {}

                Notice = <CrawradLabelMuted> {
                    margin: { top: (CRAWRAD_ESPACO_S) }
                }

                RowText = <CrawradPropRow> { value = <CrawradPropInput> { width: Fill } }

                // A StyleRule's `When`: green check, or a red box and the reason under it.
                RowWhenOk = <CrawradPropRow> { value = <CrawradWhenInput> {} note = <CrawradPropNote> { draw_text: { color: (CRAWRAD_OK) } } }
                RowWhenBad = <CrawradInspectorRow> {
                    flow: Down, height: Fit, padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: 0., bottom: (CRAWRAD_ESPACO_XS) }
                    line = <View> { width: Fill, height: (CRAWRAD_LINHA_INSPECTOR), flow: Right, align: { y: 0.5 }, spacing: (CRAWRAD_ESPACO_S)
                        name_box = <CrawradPropNameBox> { name = <CrawradPropName> {} } value = <CrawradWhenInputBad> {} }
                    reason_row = <View> { width: Fill, height: Fit, flow: Right, clip_x: true, lead = <View> { width: (CRAWRAD_NAME_COLUMN), height: 1. }
                        note = <CrawradLabelMuted> { width: Fill, draw_text: { color: (CRAWRAD_ERRO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } } } }
                }

                RowEvent = <CrawradPropRow> { value = <CrawradEventInput> {} }

                // An inherited component's event: still the base's
                // handler, or this form's override of it.
                RowEventInherited = <CrawradPropRow> {
                    value = <CrawradInheritedHandler> {}
                    tag = <CrawradEventTag> {}
                }
                RowEventOverride = <CrawradInspectorRow> {
                    flow: Down, height: Fit,
                    padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: 0., bottom: (CRAWRAD_ESPACO_XS) }
                    line = <View> {
                        width: Fill, height: (CRAWRAD_LINHA_INSPECTOR),
                        flow: Right, align: { y: 0.5 }, spacing: (CRAWRAD_ESPACO_S)
                        name_box = <CrawradPropNameBox> { name = <CrawradPropName> {} }
                        value = <CrawradOverrideInput> {}
                        tag = <CrawradOverrideTag> {}
                    }
                    calls_row = <View> {
                        width: Fill, height: Fit, flow: Right, clip_x: true,
                        lead = <View> { width: (CRAWRAD_NAME_COLUMN), height: 1. }
                        calls = <CrawradLabelMuted> {
                            width: Fit,
                            draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                        }
                    }
                }
                // `Inherited from form_parent`, above the events of one.
                InheritedFrom = <View> {
                    width: Fill, height: (CRAWRAD_LINHA_INSPECTOR),
                    flow: Right, align: { y: 0.5 }, spacing: (CRAWRAD_ESPACO_XS)
                    padding: { left: (CRAWRAD_ESPACO_S), right: 0., top: 0., bottom: 0. }
                    lead = <CrawradLabelMuted> { text: "Inherited from" }
                    form = <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) } }
                }

                RowCheck = <CrawradPropRow> { value = <CrawradPropCheck> { width: Fill } }

                RowChoice = <CrawradPropRow> { value = <CrawradPropDrop> { width: Fill } }

                RowMute = <CrawradPropRow> {
                    caixa = <CrawradPropValueBox> {
                        value = <CrawradPropValue> {}
                    }
                    note = <CrawradPropNote> {}
                }

                // COLOR: the swatch you see, the `#RRGGBB` you
                // type, and the `…` button that opens the Color dialog.
                RowColor = <CrawradPropRow> {
                    caixa = <CrawradPropBar> {
                        swatch = <CrawradPropSwatch> {}
                        value = <CrawradPropInput> { width: Fill }
                        button = <CrawradPropOpen> {}
                    }
                }

                // The SECOND FORM of a row: summary plus opening button. `Items`, `Lines`.
                RowOpen = <CrawradPropRow> {
                    caixa = <CrawradPropBar> {
                        button = <CrawradPropOpen> {}
                        cut = <CrawradPropValueBox> { value = <CrawradPropValue> {} }
                        clear = <CrawradPropOpen> { visible: false } // a picture's `×`
                    }
                }

                // The same form, with the VALUE written on the button —
                // `Align`, which opens the graphical picker.
                RowPick = <CrawradPropRow> { button = <CrawradPropPick> { width: Fill } }

                // The THIRD FORM: summary plus inline expansion. `Margins`, `Anchors`.
                RowFold = <CrawradPropRow> {
                    caixa = <CrawradPropBar> {
                        fold = <CrawradPropFold> {}
                        value = <CrawradPropValue> {}
                    }
                }

                // The same, with the `…` button that opens: `Font`.
                RowFoldOpen = <CrawradPropRow> {
                    caixa = <CrawradPropBar> {
                        fold = <CrawradPropFold> {}
                        cut = <CrawradPropValueBox> { value = <CrawradPropValue> {} }
                        button = <CrawradPropOpen> {}
                    }
                }

                // And the two SUB-ROWS that expansion produces:
                // one side of `Margins`, one anchor of `Anchors`.
                SubText = <CrawradSubRow> { value = <CrawradPropInput> { width: Fill } }

                SubCheck = <CrawradSubRow> { value = <CrawradPropCheck> { width: Fill } }

                // The `Name` of a `Font`: installed families.
                SubChoice = <CrawradSubRow> { value = <CrawradPropDrop> { width: Fill } }

                // The color of a `Font`: the swatch and the `#RRGGBB`.
                SubColor = <CrawradSubRow> {
                    caixa = <CrawradPropBar> {
                        swatch = <CrawradPropSwatch> {}
                        value = <CrawradPropInput> { width: Fill }
                    }
                }
            }

            // THE SPLITTER, placed by `margin.left` from the shell.
            name_splitter = <CrawradColumnSplitter> { draw_line: { color: (CRAWRAD_SPLITTER_DRAG) } }
        }
    }

    pub CrawradPanelExplorer = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        explorer_header = <CrawradPanelHeader> {
            spacing: 3.0,
            title = { text: "EXPLORER" } // only the title: the project line carries the actions
            explorer_spacer = <CrawradFiller> {}
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        explorer_body = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S) },

            explorer_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}

                Caption = <CrawradCaptionRow> {}

                Section = <CrawradSectionRow> {}

                GroupName = <CrawradGroupNameRow> {}
                GroupPath = <CrawradPathRow> {}

                // Open and closed are the SAME template, for the same reason as the section.
                Member = <CrawradFoldRow> {}
                MemberActive = <CrawradFoldRowActive> {}

                Form = <CrawradBadgeRow> {}
                FormCurrent = <CrawradBadgeRowStrong> {}

                File = <CrawradBadgeRow> {}
                FileYours = <CrawradBadgeRowOwned> {}
                FilePending = <CrawradBadgeRowMuted> {}
                FileStale = <CrawradBadgeRowWarn> {}
                // A file or member with problems.
                FileError = <CrawradBadgeRowError> {}
                FileWarn = <CrawradBadgeRowWarn> {}
                MemberError = <CrawradFoldRowError> {}
                MemberWarn = <CrawradFoldRowWarn> {}

                Note = <CrawradNoteRow> {}
                NoteWarn = <CrawradNoteRowWarn> {}
                Summary = <CrawradSummaryRow> {}

                // The LEGEND is not a badged row: its sentence is PROSE, and wraps.
                Legend = <CrawradLegendRow> {}
                LegendWarn = <CrawradLegendRowWarn> {}
            }
        }
    }

    pub CrawradPanelPackages = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        packages_header = <CrawradPanelHeader> {
            // Blank: the dock tab already says `Package Manager`.
            title = { text: "" }
            packages_spacer = <CrawradFiller> {}
            packages_target = <CrawradLabelMuted> { text: "" }
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        packages_tabs = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: (CRAWRAD_ESPACO_XS),
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_LINHA_RESPIRO), bottom: (CRAWRAD_LINHA_RESPIRO) }
            tab_browse = <CrawradInspectorTab> { text: "" }
            tab_installed = <CrawradInspectorTab> { text: "" }
        }

        packages_search = <View> {
            width: Fill, height: Fit,
            flow: Down,
            padding: (CRAWRAD_MARGEM_PAINEL),
            packages_query = <CrawradTextInput> { empty_text: "Search crates.io" }
        }

        packages_body = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },

            packages_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}

                Note = <CrawradWrapRow> {}
                Hint = <CrawradWrapRowHint> {}
                Error = <CrawradWrapRowError> {}
                Found = <CrawradPackageRow> {}
                FoundInstalled = <CrawradPackageRowInstalled> {}
                FoundUpdatable = <CrawradPackageRowUpdatable> {}
                Dependency = <CrawradDependencyRow> {}
            }
        }

        packages_status = <CrawradWrapRowHint> {}
    }

    pub CrawradDebugRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            spacing: (CRAWRAD_ESPACO_XS),
            padding: { left: (CRAWRAD_ESPACO_M), right: (CRAWRAD_FOLGA_DA_BARRA), top: 2., bottom: 2. }
            // The ▸/▾ of a variable that opens; empty for the rest.
            marca = <CrawradRowChevron> {}
            nome = <CrawradMonoLabel> { width: Fit, flow: Right, margin: 0., text: "" }
            valor = <CrawradMonoLabelMuted> { width: Fill, flow: Right, margin: 0., text: "" }
        }
        // A click picks the frame or opens the variable.
        alvo = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    pub CrawradDebugRowCurrent = <CrawradDebugRow> {
        show_bg: true,
        draw_bg: { color: (CRAWRAD_SELECAO) }
        face = { nome = { draw_text: { color: (CRAWRAD_ACENTO_TEXTO) } } }
    }

    pub CrawradPanelDebug = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        debug_header = <CrawradPanelHeader> {
            spacing: 3.0,
            title = { text: "" }
            debug_spacer = <CrawradFiller> {}
            btn_dbg_continue = <CrawradIconButton> { width: 22.0, height: 22.0, padding: 0., text: "" }
            btn_dbg_pause = <CrawradIconButton> { width: 22.0, height: 22.0, padding: 0., text: "" }
            btn_dbg_step_over = <CrawradIconButton> { width: 22.0, height: 22.0, padding: 0., text: "" }
            btn_dbg_step_into = <CrawradIconButton> { width: 22.0, height: 22.0, padding: 0., text: "" }
            btn_dbg_step_out = <CrawradIconButton> { width: 22.0, height: 22.0, padding: 0., text: "" }
            btn_dbg_stop = <CrawradIconButton> { width: 22.0, height: 22.0, padding: 0., text: "" }
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        debug_body = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S) },

            debug_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}
                Section = <View> { width: Fill, height: Fit, margin: { top: 4., right: 12. }, show_bg: true, draw_bg: { color: (CRAWRAD_DIVISOR) }, padding: { left: 8., top: 3., bottom: 3. }, face = <View> { width: Fill, height: Fit, nome = <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO), text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } } } } } // a band per group title (owner, 30/09)
                Note = <CrawradNoteRow> {}
                Variable = <CrawradDebugRow> {}
                Watch = <CrawradDebugRow> { face = { fechar = <CrawradIconButton> { width: 18., height: 18., padding: 0., text: "" } }, alvo = { margin: { right: 34. } } } // the ✕ stays out of the row's click
                Frame = <CrawradDebugRow> {}
                FrameCurrent = <CrawradDebugRowCurrent> {}
                FrameForeign = <CrawradDebugRow> { face = { nome = { draw_text: { color: (CRAWRAD_TEXTO_DESABILITADO) } } } } // not the project's code: dimmed
                Breakpoint = <CrawradDebugRow> {}
            }
        }
    }

    pub CrawradRefFileRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            spacing: (CRAWRAD_ESPACO_XS),
            align: { y: 0.5 },
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: 3., bottom: 3. }
            chevron = <CrawradRowChevron> {}
            name = <CrawradLabel> { width: Fit, text: "" }
            folder = <CrawradLabelMuted> { width: Fit, text: "" }
            // `generated` / `read-only`: the line is not the user's.
            mark = <CrawradLabel> { width: Fit, draw_text: { color: (CRAWRAD_AVISO) }, text: "" }
            spacer = <View> { width: Fill, height: 1. }
            count = <CrawradLabelMuted> { width: Fit, text: "" }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    pub CrawradRefHitRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            padding: { left: 30., right: (CRAWRAD_FOLGA_DA_BARRA), top: 2., bottom: 2. }
            line_no = <CrawradMonoLabelMuted> { width: 34., flow: Right, margin: 0., text: "" }
            // ONE line: the model already cut the text around the name.
            before = <CrawradMonoLabel> { width: Fit, flow: Right, margin: 0., text: "" }
            highlight = <View> {
                width: Fit, height: Fit,
                show_bg: true,
                draw_bg: { color: (CRAWRAD_SELECAO_FORTE) }
                word = <CrawradMonoLabel> { width: Fit, flow: Right, margin: 0., draw_text: { color: (CRAWRAD_ACENTO_TEXTO) }, text: "" }
            }
            after = <CrawradMonoLabel> { width: Fill, flow: Right, margin: 0., text: "" }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    pub CrawradPanelReferences = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        references_header = <CrawradPanelHeader> {
            title = { text: "" }
            references_spacer = <CrawradFiller> {}
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        references_body = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: (CRAWRAD_ESPACO_S) },

            references_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}
                RefFile = <CrawradRefFileRow> {}
                RefHit = <CrawradRefHitRow> {}
            }
        }
    }

    // The Problems panel (approved mockup): chips, the stop reason, a flat list.
    pub CrawradProblemChip = <Button> {
        width: Fit, height: 22.0,
        padding: { left: 10., right: 10., top: 0., bottom: 0. }
        margin: { right: (CRAWRAD_ESPACO_S) }
        // `fill` and `border` are the chip's own look; the active filter gets a light ring.
        draw_bg: {
            instance active: 0.0
            uniform fill: (CRAWRAD_TRANSPARENTE)
            uniform border: (CRAWRAD_BORDA_CONTROLE)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 5.0);
                sdf.fill_keep(self.fill);
                sdf.stroke(mix(self.border, (CRAWRAD_TEXTO_PRIMARIO), self.active), 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            color: (CRAWRAD_TEXTO_PRIMARIO)
            color_hover: (CRAWRAD_TEXTO_PRIMARIO)
            color_down: (CRAWRAD_TEXTO_PRIMARIO)
            color_focus: (CRAWRAD_TEXTO_PRIMARIO)
            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
        }
        text: ""
    }

    pub CrawradProblemRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: 4., bottom: 4. }
            spacing: (CRAWRAD_ESPACO_S),
            icon = <Label> {
                draw_text: {
                    color: (CRAWRAD_ERRO)
                    text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
                text: ""
            }
            message = <CrawradLabel> { width: Fill, text: "" }
            place = <CrawradMonoLabelMuted> { width: 170., text: "" }
            code = <CrawradMonoLabelMuted> { width: 140., text: "" }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    // A panic of the app Run started: the message and its spot, then the thread.
    pub CrawradProblemPanicRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        show_bg: true,
        draw_bg: { color: (CRAWRAD_FLUTUANTE) }
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S), top: 6., bottom: 6. }
            spacing: (CRAWRAD_ESPACO_S),
            icon = <Label> {
                draw_text: {
                    color: (CRAWRAD_ERRO)
                    text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_CORPO) }
                }
                text: ""
            }
            text = <View> {
                width: Fill, height: Fit, flow: Down, spacing: 3.,
                head = <View> {
                    width: Fill, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_S),
                    message = <CrawradLabel> {
                        draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } }
                        text: ""
                    }
                    place = <CrawradMonoLabelMuted> { draw_text: { color: (CRAWRAD_LINK) } text: "" }
                }
                meta = <CrawradLabelMuted> { text: "" }
            }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    // One of the project's own frames under a panic: the function, then where.
    pub CrawradProblemFrameRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            padding: { left: 36., right: (CRAWRAD_ESPACO_S), top: 3., bottom: 3. }
            spacing: (CRAWRAD_ESPACO_M),
            function = <CrawradMonoLabelMuted> { width: Fit, draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) } text: "" }
            place = <CrawradMonoLabelMuted> { draw_text: { color: (CRAWRAD_LINK) } text: "" }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    pub CrawradPanelProblems = <CrawradPanel> {
        width: Fill, height: Fill,
        flow: Down,

        problems_header = <CrawradPanelHeader> {
            title = { text: "" }
            chip_errors = <CrawradProblemChip> {
                draw_bg: { fill: (CRAWRAD_ERROR_CHIP), border: (CRAWRAD_ERROR_CHIP) }
                draw_text: {
                    color: (CRAWRAD_ERROR_CHIP_TEXT), color_hover: (CRAWRAD_ERROR_CHIP_TEXT)
                    color_down: (CRAWRAD_ERROR_CHIP_TEXT), color_focus: (CRAWRAD_ERROR_CHIP_TEXT)
                }
            }
            chip_warnings = <CrawradProblemChip> {}
            chip_panics = <CrawradProblemChip> {
                visible: false
                draw_bg: { fill: (CRAWRAD_ERROR_CHIP), border: (CRAWRAD_ERROR_CHIP) }
                draw_text: {
                    color: (CRAWRAD_ERROR_CHIP_TEXT), color_hover: (CRAWRAD_ERROR_CHIP_TEXT)
                    color_down: (CRAWRAD_ERROR_CHIP_TEXT), color_focus: (CRAWRAD_ERROR_CHIP_TEXT)
                }
            }
            chip_output = <CrawradProblemChip> {
                draw_text: {
                    color: (CRAWRAD_TEXTO_TERCIARIO), color_hover: (CRAWRAD_TEXTO_TERCIARIO)
                    color_down: (CRAWRAD_TEXTO_TERCIARIO), color_focus: (CRAWRAD_TEXTO_TERCIARIO)
                }
            }
            <CrawradFiller> {}
            reason = <CrawradLabel> { draw_text: { color: (CRAWRAD_ERRO) } text: "" }
        }

        problems_body = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },

            problems_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}
                ProblemError = <CrawradProblemRow> {}
                ProblemWarning = <CrawradProblemRow> {
                    face = { icon = { draw_text: { color: (CRAWRAD_AVISO) } } }
                }
                ProblemPanic = <CrawradProblemPanicRow> {}
                ProblemFrame = <CrawradProblemFrameRow> {}
                ProblemHeading = <View> {
                    width: Fill, height: Fit,
                    padding: { left: 36., top: 6., bottom: 2. }
                    line = <CrawradLabelMuted> { width: Fill, text: "" }
                }
                ProblemOutput = <View> {
                    width: Fill, height: Fit,
                    padding: { left: (CRAWRAD_ESPACO_S), top: 1., bottom: 1. }
                    line = <CrawradMonoLabelMuted> { width: Fill, text: "" }
                }
            }
            <CrawradLabelMuted> {
                padding: { top: (CRAWRAD_ESPACO_S), bottom: (CRAWRAD_ESPACO_S) }
                text: "Click a row to go to the line. The panel closes itself on the next Run that builds clean."
            }
        }
    }

    pub CrawradViewHeaderRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            spacing: (CRAWRAD_ESPACO_XS),
            align: { y: 0.5 },
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_FOLGA_DA_BARRA), top: 3., bottom: 3. }
            chevron = <CrawradRowChevron> {}
            title = <CrawradLabel> { width: Fit, text: "" }
            spacer = <View> { width: Fill, height: 1. }
            count = <CrawradLabelMuted> { width: Fit, text: "" }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    pub CrawradViewItemRow = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            spacing: (CRAWRAD_ESPACO_XS),
            align: { y: 0.5 },
            padding: { left: 22., right: (CRAWRAD_FOLGA_DA_BARRA), top: 2., bottom: 2. }
            chevron = <CrawradRowChevron> {}
            dot = <View> {
                width: 8., height: 8.,
                show_bg: true,
                draw_bg: {
                    color: (CRAWRAD_TRANSPARENTE)
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.circle(4., 4., 3.5);
                        sdf.fill(self.color);
                        return sdf.result;
                    }
                }
            }
            label = <CrawradLabel> { width: Fit, text: "" }
            detail = <CrawradLabelMuted> { width: Fill, text: "" }
        }
        target = <CrawradListRowOverlay> { width: Fill, height: Fill }
    }

    // An installed extension: name, version, and its uninstall button.
    pub CrawradViewInstalledRow = <View> {
        width: Fill, height: Fit,
        face = <View> {
            width: Fill, height: Fit,
            flow: Right,
            spacing: (CRAWRAD_ESPACO_XS),
            align: { y: 0.5 },
            padding: { left: 22., right: (CRAWRAD_FOLGA_DA_BARRA), top: 2., bottom: 2. }
            name = <CrawradLabel> { width: Fit, text: "" }
            version = <CrawradLabelMuted> { width: Fill, text: "" }
            uninstall = <CrawradPanelClose> { text: "" }
        }
    }

    pub CrawradPanelNotes = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,
        notes_header = <CrawradPanelHeader> {
            title = { text: "Notes" }
            notes_spacer = <CrawradFiller> {}
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }
        notes_view = <NotesPanelView> {}
    }

    pub CrawradPanelExtensions = <CrawradPanelLeft> {
        width: Fill, height: Fill,
        flow: Down,

        extensions_header = <CrawradPanelHeader> {
            title = { text: "Extensions" }
            extensions_spacer = <CrawradFiller> {}
            // Install from Folder…: the glyph is written from the catalogue.
            btn_install = <CrawradPanelClose> { text: "" }
            btn_panel_close = <CrawradPanelClose> { text: "" }
        }

        extensions_filter = <View> {
            width: Fill, height: Fit,
            padding: (CRAWRAD_MARGEM_PAINEL),
            views_filter = <CrawradTextInput> { empty_text: "Filter" }
        }

        extensions_body = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },

            views_list = <PortalList> {
                width: Fill, height: Fill,
                scroll_bar: <CrawradScrollBar> {}
                ViewHeader = <CrawradViewHeaderRow> {}
                ViewItem = <CrawradViewItemRow> {}
                ViewInstalled = <CrawradViewInstalledRow> {}
            }
        }
    }

}
