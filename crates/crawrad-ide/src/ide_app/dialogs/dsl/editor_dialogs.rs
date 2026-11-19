use makepad_widgets::*;

live_design! {
    link crawrad_dialogs;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    use link::crawrad::*;
            pub CrawradItemsDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    // `Fit` HEIGHT: the body carries it — see `CrawradDialogBody` in the theme.
                    width: (CRAWRAD_ITEMS_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        items_title = <CrawradDialogTitle> { text: "" }
                    }
                    <CrawradDialogBody> {
                        height: (CRAWRAD_ITEMS_CORPO),
                        // Who owns the collection — `form_main.tree_view1`.
                        items_owner = <CrawradLabelMuted> { text: "" }
                        <View> {
                            width: Fill, height: Fill,
                            flow: Right,
                            spacing: (CRAWRAD_ESPACO_L)
                            // THE TREE.
                            <View> {
                                width: (CRAWRAD_ITEMS_COLUNA), height: Fill,
                                flow: Down,
                                items_label = <CrawradGroupBoxTitle> { text: "Items" }
                                <CrawradGroupBoxFrame> {
                                    items_list = <PortalList> {
                                        width: Fill, height: Fill,
                                        scroll_bar: <CrawradScrollBar> {}
                                        ItemRow = <CrawradItemsRow> {}
                                    }
                                }
                            }
                            // THE STRUCTURE COLUMN.
                            <View> {
                                width: Fit, height: Fill,
                                flow: Down,
                                spacing: (CRAWRAD_ESPACO_M)
                                margin: { top: (CRAWRAD_ITEMS_BOTOES_TOPO) }
                                btn_new_item = <CrawradItemsButton> { text: "" }
                                btn_new_subitem = <CrawradItemsButton> { text: "" }
                                btn_delete = <CrawradItemsButton> { text: "" }
                                <View> { width: Fill, height: (CRAWRAD_ESPACO_L) }
                                btn_move_up = <CrawradItemsButton> { text: "" }
                                btn_move_down = <CrawradItemsButton> { text: "" }
                            }
                            // THE ITEM PROPERTIES.
                            <View> {
                                width: Fill, height: Fill,
                                flow: Down,
                                props_label = <CrawradGroupBoxTitle> { text: "Item Properties" }
                                <CrawradGroupBoxFrame> {
                                    spacing: (CRAWRAD_ESPACO_M)
                                    <CrawradFieldRow> {
                                        text_label = <CrawradLabel> { width: 84, text: "Text:" }
                                        item_text = <CrawradCampoDenso> {
                                            width: Fill, height: (CRAWRAD_CAMPO_ALTURA),
                                        }
                                    }
                                    <CrawradFieldRow> {
                                        image_label = <CrawradLabel> { width: 84, text: "Image Index:" }
                                        item_image = <CrawradCampoDenso> {
                                            width: 60, height: (CRAWRAD_CAMPO_ALTURA),
                                        }
                                    }
                                    item_expanded = <CrawradOptionCheck> { text: "Expanded" }
                                    // What is NOT here: Selected/State Index and Enabled per item.
                                    props_note = <CrawradLabelMuted> {
                                        width: Fill, height: Fit,
                                        text: ""
                                    }
                                }
                            }
                        }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        items_ok = <CrawradDialogPrimaryButton> { text: "" }
                        items_cancel = <CrawradDialogButton> { text: "" }
                        items_apply = <CrawradDialogButton> { text: "" }
                    }
                }
            }
            pub CrawradLinesDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_LINES_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        lines_title = <CrawradDialogTitle> { text: "" }
                    }
                    <CrawradDialogBody> {
                        height: (CRAWRAD_LINES_CORPO),
                        lines_owner = <CrawradLabelMuted> { text: "" }
                        lines_text = <CrawradLinesText> {}
                    }
                    <CrawradDialogFooter> {
                        lines_count = <CrawradLabelMuted> { text: "" }
                        <CrawradFiller> {}
                        lines_ok = <CrawradDialogPrimaryButton> { text: "" }
                        lines_cancel = <CrawradDialogButton> { text: "" }
                        lines_apply = <CrawradDialogButton> { text: "" }
                    }
                }
            }

            pub CrawradColorSwatchBtn = <CrawradColorPatch> { width: 22.0, height: 22.0, pick = <CrawradPatchPick> {} }
            pub CrawradColorStrip = <View> { width: Fill, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_XS) }
            pub CrawradColorField = <View> { width: Fit, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_XS) }
            pub CrawradColorInput = <CrawradPropInput> { width: 55.0 }

            pub CrawradColorDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 520.0, height: Fit,
                    <CrawradDialogTitleBar> { color_title = <CrawradDialogTitle> { text: "" } }
                    <CrawradDialogBody> {
                        height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_L),
                        <View> {
                            width: Fit, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_S),
                            <View> { width: Fit, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_S),
                                color_square = <CrawradColorSquare> {} color_hue = <CrawradHueStrip> {} }
                            <View> { width: Fit, height: Fit, flow: Right, spacing: (CRAWRAD_ESPACO_S),
                                color_current = <CrawradColorPatch> { width: 120.0, height: 30.0 }
                                color_new = <CrawradColorPatch> { width: 120.0, height: 30.0 } }
                        }
                        <View> {
                            width: Fill, height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_S),
                            <CrawradLabelMuted> { text: "Basic colours" }
                            <CrawradColorStrip> { color_basic_0 = <CrawradColorSwatchBtn> {} color_basic_1 = <CrawradColorSwatchBtn> {} color_basic_2 = <CrawradColorSwatchBtn> {} color_basic_3 = <CrawradColorSwatchBtn> {} color_basic_4 = <CrawradColorSwatchBtn> {} color_basic_5 = <CrawradColorSwatchBtn> {} color_basic_6 = <CrawradColorSwatchBtn> {} color_basic_7 = <CrawradColorSwatchBtn> {} }
                            <CrawradColorStrip> { color_basic_8 = <CrawradColorSwatchBtn> {} color_basic_9 = <CrawradColorSwatchBtn> {} color_basic_10 = <CrawradColorSwatchBtn> {} color_basic_11 = <CrawradColorSwatchBtn> {} color_basic_12 = <CrawradColorSwatchBtn> {} color_basic_13 = <CrawradColorSwatchBtn> {} color_basic_14 = <CrawradColorSwatchBtn> {} color_basic_15 = <CrawradColorSwatchBtn> {} }
                            <CrawradLabelMuted> { text: "Recent" }
                            <CrawradColorStrip> { color_recent_0 = <CrawradColorSwatchBtn> {} color_recent_1 = <CrawradColorSwatchBtn> {} color_recent_2 = <CrawradColorSwatchBtn> {} color_recent_3 = <CrawradColorSwatchBtn> {} color_recent_4 = <CrawradColorSwatchBtn> {} color_recent_5 = <CrawradColorSwatchBtn> {} color_recent_6 = <CrawradColorSwatchBtn> {} color_recent_7 = <CrawradColorSwatchBtn> {} }
                            <CrawradColorStrip> {
                                <CrawradColorField> { <CrawradLabelMuted> { text: "Hex" } color_hex = <CrawradPropInput> { width: 110.0 } }
                                <CrawradColorField> { <CrawradLabelMuted> { text: "Alpha" } color_alpha = <CrawradColorInput> {} }
                            }
                            <CrawradColorStrip> {
                                <CrawradColorField> { <CrawradLabelMuted> { text: "Red" } color_red = <CrawradColorInput> {} }
                                <CrawradColorField> { <CrawradLabelMuted> { text: "Green" } color_green = <CrawradColorInput> {} }
                                <CrawradColorField> { <CrawradLabelMuted> { text: "Blue" } color_blue = <CrawradColorInput> {} }
                            }
                            color_error = <CrawradLabelMuted> { width: Fill, height: Fit, draw_text: { color: (CRAWRAD_ERRO) }, text: "" }
                        }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        color_ok = <CrawradDialogPrimaryButton> { text: "OK" }
                        color_cancel = <CrawradDialogButton> { text: "Cancel" }
                    }
                }
            }
            pub CrawradRenameDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_LINES_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        rename_title = <CrawradDialogTitle> { text: "" }
                    }
                    <CrawradDialogBody> {
                        height: Fit,
                        rename_what = <CrawradLabel> { text: "" }
                        rename_input = <CrawradCampoDenso> { width: Fill }
                        rename_preview = <CrawradLabelMuted> { width: Fill, text: "" }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        rename_ok = <CrawradDialogPrimaryButton> { text: "" }
                        rename_cancel = <CrawradDialogButton> { text: "Cancel" }
                    }
                }
            }
            // "Rename everywhere" confirm card.
            pub CrawradComponentRenameDialog = <CrawradDialogVeil> {
                // A cursor makes the veil claim the pointer: a click beside
                // the card must not select on the Designer behind it.
                cursor: Default,
                // ...but not the window's title bar: its close button must
                // still answer while the card is open.
                margin: { top: 34.0 },
                dlg = <CrawradDialog> {
                    width: 580.0, height: Fit,
                    <CrawradDialogTitleBar> {
                        comp_rename_title = <CrawradDialogTitle> { text: "" }
                    }
                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_M),

                        check_comp = <CrawradOptionCheck> {
                            text: "The component and the designer code"
                            draw_bg: {
                                color_active: (CRAWRAD_TEXTO_DESABILITADO),
                                mark_color_active: (CRAWRAD_GROUND),
                                border_color_1_active: (CRAWRAD_TEXTO_DESABILITADO),
                                border_color_2_active: (CRAWRAD_TEXTO_DESABILITADO),
                            }
                        }
                        detail_comp = <CrawradMonoLabelMuted> { margin: { left: 24.0 }, text: "" }
                        row_handlers = <View> {
                            width: Fill, height: Fit, flow: Down,
                            check_handlers = <CrawradOptionCheck> { text: "Handler with the default name" }
                            detail_handlers = <CrawradMonoLabelMuted> { margin: { left: 24.0 }, text: "" }
                        }
                        check_refs = <CrawradOptionCheck> { text: "References in your code" }
                        detail_refs = <CrawradMonoLabelMuted> { margin: { left: 24.0 }, text: "" }
                    }
                    <CrawradDialogFooter> {
                        comp_rename_footer_note = <CrawradLabelMuted> {
                            width: Fit,
                            text: "Code edits stay unsaved until Save All"
                        }
                        <CrawradFiller> {}
                        comp_rename_all = <CrawradDialogPrimaryButton> {
                            width: 90.0,
                            // The label is the action's (`component.rename`), set by the draw.
                            text: ""
                        }
                        comp_rename_cancel = <CrawradDialogButton> {
                            width: 80.0,
                            text: "Cancel"
                        }
                    }
                }
            }
            pub CrawradActionsDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_ACTIONS_DLG_L), height: Fit,
                    <CrawradDialogTitleBar> {
                        // One level down, `‹` goes back; the levels above sit dim before the title.
                        act_back = <CrawradButton> { visible: false, width: 24.0, height: 20.0, margin: { right: (CRAWRAD_ESPACO_S) }, text: "‹" }
                        crumb_prev = <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) }, text: "" }
                        actions_title = <CrawradDialogTitle> { text: "" }
                    }
                    <CrawradDialogBody> {
                        height: (CRAWRAD_ACTIONS_CORPO),

                        // THE COMMAND STRIP. The five are those of egui, and the
                        // labels come from Rust as in everything else.
                        <View> {
                            width: Fill, height: Fit,
                            flow: Right,
                            spacing: (CRAWRAD_ESPACO_S)
                            act_new = <CrawradButton> { text: "" }
                            act_standard = <CrawradButton> { text: "" }
                            act_delete = <CrawradButton> { text: "" }
                            act_up = <CrawradButton> { text: "" }
                            act_down = <CrawradButton> { text: "" }
                            <CrawradFiller> {}
                            act_fields = <CrawradButton> { text: "" }
                        }

                        <View> {
                            width: Fill, height: Fill,
                            flow: Right,
                            spacing: (CRAWRAD_ESPACO_L)

                            cat_column = <View> {
                                width: (CRAWRAD_ACTIONS_COLUNA), height: Fill,
                                flow: Down,
                                cat_label = <CrawradGroupBoxTitle> { text: "Categories:" }
                                <CrawradGroupBoxFrame> {
                                    cat_list = <PortalList> {
                                        width: Fill, height: Fill,
                                        scroll_bar: <CrawradScrollBar> {}
                                        CatRow = <CrawradItemsRow> {}
                                    }
                                }
                            }

                            <View> {
                                width: Fill, height: Fill,
                                flow: Down,
                                act_label = <CrawradGroupBoxTitle> { text: "Actions:" }
                                <CrawradGroupBoxFrame> {
                                    act_list = <PortalList> {
                                        width: Fill, height: Fill,
                                        scroll_bar: <CrawradScrollBar> {}
                                        ActRow = <CrawradItemsRow> {}
                                    }
                                }
                            }
                        }
                    }

                    <CrawradDialogFooter> {
                        act_footer_note = <CrawradLabelMuted> { width: Fill, text: "", draw_text: { wrap: Word, text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                        act_close = <CrawradDialogPrimaryButton> { text: "" }
                    }
                }
            }

            pub CrawradStandardDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_STANDARD_DLG_L), height: Fit,

                    <CrawradDialogTitleBar> {
                        std_title = <CrawradDialogTitle> { text: "Standard Action Classes" }
                    }

                    <CrawradDialogBody> {
                        height: (CRAWRAD_STANDARD_CORPO),
                        std_label = <CrawradGroupBoxTitle> { text: "Available Action Classes:" }
                        <CrawradGroupBoxFrame> {
                            std_list = <PortalList> {
                                width: Fill, height: Fill,
                                scroll_bar: <CrawradScrollBar> {}
                                StdGrupo = <CrawradCategoryRow> {}
                                // A class, with the chosen checkmark.
                                StdClasse = <CrawradInspectorRow> {
                                    marca = <CrawradStdCheck> {}
                                    nome = <CrawradLabel> { text: "" }
                                }
                            }
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        std_ok = <CrawradDialogPrimaryButton> { text: "" }
                        std_cancel = <CrawradDialogButton> { text: "" }
                    }
                }
            }

            pub CrawradImagelistDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_IML_DLG_L), height: Fit,

                    <CrawradDialogTitleBar> {
                        iml_title = <CrawradDialogTitle> { text: "" }
                    }

                    <CrawradDialogBody> {
                        height: (CRAWRAD_IML_CORPO),

                        <CrawradFieldRow> {
                            iml_add = <CrawradButton> { text: "" }
                            iml_replace = <CrawradButton> { text: "" }
                            iml_delete = <CrawradButton> { text: "" }
                            iml_clear = <CrawradButton> { text: "" }
                            iml_size_label = <CrawradLabel> {
                                margin: { left: (CRAWRAD_ESPACO_M) }
                                text: ""
                            }
                            iml_size = <CrawradPropDrop> {}
                            <CrawradFiller> {}
                            iml_count = <CrawradLabelMuted> { text: "" }
                        }

                        <View> {
                            width: Fill, height: Fill,
                            flow: Right,
                            spacing: (CRAWRAD_ESPACO_L)

                            <View> {
                                width: Fill, height: Fill,
                                flow: Down,
                                iml_grid_label = <CrawradGroupBoxTitle> { text: "Images:" }
                                <CrawradGroupBoxFrame> {
                                    iml_grid = <PortalList> {
                                        width: Fill, height: Fill,
                                        scroll_bar: <CrawradScrollBar> {}
                                        ImgRow = <View> {
                                            width: Fill, height: (CRAWRAD_IML_CEL_A),
                                            flow: Right,
                                            spacing: (CRAWRAD_ESPACO_M)
                                            c0 = <CrawradIconCell> {}
                                            c1 = <CrawradIconCell> {}
                                            c2 = <CrawradIconCell> {}
                                            c3 = <CrawradIconCell> {}
                                            c4 = <CrawradIconCell> {}
                                            c5 = <CrawradIconCell> {}
                                        }
                                    }
                                }
                            }

                            <View> {
                                width: (CRAWRAD_IML_COLUNA), height: Fill,
                                flow: Down,
                                iml_sel_label = <CrawradGroupBoxTitle> { text: "Selected Image" }
                                <CrawradGroupBoxFrame> {
                                    align: { x: 0.5 }
                                    spacing: (CRAWRAD_ESPACO_S)
                                    iml_index = <CrawradLabel> { width: Fill, text: "" }
                                    iml_name = <CrawradLabel> { width: Fill, text: "" }
                                    iml_size_of = <CrawradLabel> { width: Fill, text: "" }
                                    <View> {
                                        width: (CRAWRAD_IML_PREVIA), height: (CRAWRAD_IML_PREVIA),
                                        margin: { top: (CRAWRAD_ESPACO_M) }
                                        flow: Overlay,
                                        <CrawradXadrez> {
                                            width: Fill, height: Fill,
                                            draw_bg: { lado: 8.0 }
                                        }
                                        iml_previa = <CrawradLabel> {
                                            width: Fill, height: Fill,
                                            align: { x: 0.5, y: 0.5 }
                                            draw_text: {
                                                color: (CRAWRAD_TINTA_NO_XADREZ)
                                                text_style: { font_size: (CRAWRAD_FONTE_PREVIA) }
                                            }
                                            text: ""
                                        }
                                    }
                                    iml_nota = <CrawradLabelMuted> {
                                        width: Fill, height: Fit,
                                        margin: { top: (CRAWRAD_ESPACO_M) }
                                        text: ""
                                    }
                                }
                            }
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        iml_ok = <CrawradDialogPrimaryButton> { text: "" }
                        iml_cancel = <CrawradDialogButton> { text: "" }
                        iml_apply = <CrawradDialogButton> { text: "" }
                    }
                }
            }

            pub CrawradAddiconDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_ADDICON_DLG_L), height: Fit,

                    <CrawradDialogTitleBar> {
                        add_title = <CrawradDialogTitle> { text: "Add Icons from Catalog" }
                    }

                    <CrawradDialogBody> {
                        height: (CRAWRAD_ADDICON_CORPO),
                        add_label = <CrawradGroupBoxTitle> { text: "Select an icon to add:" }
                        <CrawradGroupBoxFrame> {
                            add_list = <PortalList> {
                                width: Fill, height: Fill,
                                scroll_bar: <CrawradScrollBar> {}
                                PresetRow = <CrawradItemsRow> {}
                            }
                        }

                        novo_label = <CrawradGroupBoxTitle> { text: "New icon in the gallery:" }
                        <CrawradFieldRow> {
                            novo_nome = <CrawradCampoDenso> {
                                width: Fill, height: (CRAWRAD_CAMPO_ALTURA),
                            }
                            novo_simbolo = <CrawradCampoDenso> {
                                width: (CRAWRAD_NOVO_ICONE_SIMBOLO), height: (CRAWRAD_CAMPO_ALTURA),
                            }
                            novo_categoria = <CrawradCampoDenso> {
                                width: Fill, height: (CRAWRAD_CAMPO_ALTURA),
                            }
                            novo_add = <CrawradButton> { text: "Add to gallery" }
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        add_ok = <CrawradDialogPrimaryButton> { text: "Add" }
                        add_close = <CrawradDialogButton> { text: "" }
                    }
                }
            }

            pub CrawradRecoverDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 560.0, height: Fit,
                    <CrawradDialogTitleBar> { <CrawradDialogTitle> { text: "Recover unsaved work" } }
                    <CrawradDialogBody> {
                        height: Fit, flow: Down, spacing: (CRAWRAD_ESPACO_S)
                        <CrawradLabel> {
                            width: Fill, height: Fit, text: "The IDE did not close normally last time."
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CABECALHO) } }
                        }
                        <CrawradLabelMuted> { width: Fill, height: Fit, text: "These recovery copies are newer than the files on disk. Pick the ones to bring back; each opens as an unsaved change, so the file on disk stays as it is until you save." }
                        <CrawradInspectorRow> {
                            <View> { width: (CRAWRAD_MARCA_LARGURA), height: 1.0 }
                            <CrawradLabelMuted> { width: Fill, text: "File" }
                            <CrawradLabelMuted> { width: 90.0, text: "Copied at" }
                            <CrawradLabelMuted> { width: 90.0, text: "On disk" }
                        }
                        <CrawradGroupBoxFrame> { height: 150.0
                            rec_list = <PortalList> { width: Fill, height: Fill, scroll_bar: <CrawradScrollBar> {}
                                RecRow = <CrawradInspectorRow> {
                                    marca = <CrawradStdCheck> {}
                                    file = <CrawradLabel> { width: Fill, text: "" }
                                    copy = <CrawradLabel> { width: 90.0, text: "" }
                                    disk = <CrawradLabel> { width: 90.0, text: "" }
                                }
                            }
                        }
                        <CrawradLabelMuted> { width: Fill, height: Fit, text: "Discarded copies are deleted. Later keeps them and asks again on the next start." }
                    }
                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        rec_recover = <CrawradDialogPrimaryButton> { width: 140.0, text: "Recover selected" }
                        rec_discard = <CrawradDialogButton> { text: "Discard" }
                        rec_later = <CrawradDialogButton> { text: "Later" }
                    }
                }
            }

            pub CrawradQuitDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: (CRAWRAD_QUIT_DLG_L), height: Fit,

                    <CrawradDialogTitleBar> {
                        quit_title = <CrawradDialogTitle> { text: "Unsaved changes" }
                    }

                    <CrawradDialogBody> {
                        height: (CRAWRAD_QUIT_CORPO),
                        flow: Right,
                        spacing: (CRAWRAD_ESPACO_L)
                        // THE WARNING SIGN, in the body of the dialog icon.
                        quit_icone = <CrawradLabel> {
                            width: Fit, height: Fit,
                            draw_text: {
                                color: (CRAWRAD_AVISO)
                                text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_AVISO) }
                            }
                            text: ""
                        }
                        <View> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: (CRAWRAD_ESPACO_S)
                            quit_pergunta = <CrawradLabel> {
                                width: Fill, height: Fit,
                                draw_text: {
                                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CABECALHO) }
                                }
                                text: ""
                            }
                            quit_arquivo = <CrawradLabelMuted> { width: Fill, height: Fit, text: "" }
                            quit_aviso = <CrawradLabelMuted> { width: Fill, height: Fit, text: "Your changes will be lost if you discard them." }
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        quit_save = <CrawradDialogPrimaryButton> { text: "" }
                        quit_discard = <CrawradDialogButton> { text: "Discard" }
                        quit_cancel = <CrawradDialogButton> { text: "" }
                    }
                }
            }

            pub CrawradPermissionDialog = <CrawradDialogVeil> {
                dlg = <CrawradDialog> {
                    width: 480.0, height: Fit,

                    <CrawradDialogTitleBar> {
                        perm_title = <CrawradDialogTitle> { text: "" }
                    }

                    <CrawradDialogBody> {
                        height: Fit,
                        flow: Down,
                        spacing: (CRAWRAD_ESPACO_M),

                        <View> {
                            width: Fill, height: Fit,
                            flow: Right,
                            spacing: (CRAWRAD_ESPACO_M),
                            align: { y: 0.5 },

                            perm_icone = <CrawradLabel> {
                                width: Fit, height: Fit,
                                draw_text: {
                                    color: (CRAWRAD_AVISO),
                                    text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_AVISO) }
                                }
                                text: ""
                            }
                            perm_pergunta = <CrawradLabel> {
                                width: Fill, height: Fit,
                                draw_text: {
                                    color: (CRAWRAD_TEXTO_PRIMARIO),
                                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CABECALHO) }
                                }
                                text: ""
                            }
                        }

                        perm_badge = <CrawradLabelMuted> {
                            width: Fill, height: Fit,
                            draw_text: {
                                color: (CRAWRAD_ACENTO)
                            }
                            text: ""
                        }

                        perm_detalhes = <CrawradLabel> {
                            width: Fill, height: Fit,
                            draw_text: {
                                color: (CRAWRAD_TEXTO_SECUNDARIO),
                                text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                            }
                            text: ""
                        }

                        perm_alvo = <CrawradLabelMuted> {
                            width: Fill, height: Fit,
                            text: ""
                        }
                    }

                    <CrawradDialogFooter> {
                        <CrawradFiller> {}
                        perm_deny = <CrawradDialogButton> { text: "" }
                        perm_allow_session = <CrawradDialogButton> { text: "" }
                        perm_allow_once = <CrawradDialogPrimaryButton> { text: "" }
                    }
                }
            }


}
