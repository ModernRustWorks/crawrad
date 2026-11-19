use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-imagelist-ideapp-open-imagelist")]
    pub(in super::super) fn open_imagelist(&mut self, cx: &mut Cx, lista: NodeId) {
        let novo = self
            .host
            .doc()
            .and_then(|d| d.ir().node(&lista))
            .map(|n| ImageListEditorState::from_node(&lista, n));
        let Some(estado) = novo else { return };
        self.imagelist = Some(estado);
        // The GALLERY is read here, once per opening — see [`IdeApp::galeria`].
        self.galeria = image_list_editor::gallery(&match Self::caminho_da_galeria() {
            Some(c) => image_list_editor::load_gallery(&c),
            None => Vec::new(),
        });
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-imagelist-ideapp-caminho-da-galeria")]
    pub(in super::super) fn caminho_da_galeria() -> Option<std::path::PathBuf> {
        Some(crawrad_project::workspace::config_folder()?.join(image_list_editor::GALLERY_FILE))
    }

    #[note("ide-app-dialogs-imagelist-ideapp-fileiras-das-imagens")]
    pub(in super::super) fn fileiras_das_imagens(&self) -> usize {
        let n = self
            .imagelist
            .as_ref()
            .map(|e| e.entries.len())
            .unwrap_or(0);
        n.div_ceil(IML_COLUNAS)
    }

    #[note("ide-app-dialogs-imagelist-ideapp-draw-imagelist")]
    pub(in super::super) fn draw_imagelist(&mut self, cx: &mut Cx) {
        let aberto = self.imagelist.is_some();
        self.show_veil(cx, id!(imagelist_dialog), aberto);
        let Some(estado) = self.imagelist.as_ref() else {
            return;
        };
        let raiz = self.form_name();
        let titulo = format!("{EDITANDO} {raiz}.{}", estado.list_name);
        let conta = format!("{IML_CONTA}{}", estado.entries.len());
        let medida = format!("{}x{}", estado.width, estado.height);
        let tem_sel = estado.has_selection();
        let vazia = estado.entries.is_empty();
        let escolhida = estado
            .selected
            .and_then(|i| estado.entries.get(i).map(|e| (i, e.clone())));

        let dlg = self.ui.view(id!(imagelist_dialog.dlg));
        dlg.label(id!(iml_title)).set_text(cx, &titulo);
        dlg.label(id!(iml_size_label)).set_text(cx, IML_TAMANHO);
        dlg.label(id!(iml_count)).set_text(cx, &conta);
        for (caminho, rotulo) in IML_BOTOES {
            dlg.button(caminho()).set_text(cx, rotulo);
        }
        dlg.button(id!(iml_ok)).set_text(cx, ITEMS_OK);
        dlg.button(id!(iml_cancel)).set_text(cx, ITEMS_CANCEL);
        dlg.button(id!(iml_apply)).set_text(cx, ITEMS_APPLY);
        Self::habilitar(cx, &dlg.widget(id!(iml_replace)), tem_sel);
        Self::habilitar(cx, &dlg.widget(id!(iml_delete)), tem_sel);
        Self::habilitar(cx, &dlg.widget(id!(iml_clear)), !vazia);
        let combo = dlg.drop_down(id!(iml_size));
        let opcoes: Vec<String> = image_list_editor::SIZES
            .iter()
            .map(|s| format!("{s}x{s}"))
            .collect();
        combo.set_labels(cx, opcoes.clone());
        if let Some(i) = opcoes.iter().position(|o| *o == medida) {
            combo.set_selected_item(cx, i);
        }

        // The PREVIEW of the selected image — and what it says when there is none.
        match escolhida {
            Some((i, e)) => {
                dlg.label(id!(iml_index))
                    .set_text(cx, &format!("Index: {i}"));
                dlg.label(id!(iml_name))
                    .set_text(cx, &format!("Name: {}", e.name));
                dlg.label(id!(iml_size_of))
                    .set_text(cx, &format!("Size: {medida}"));
                dlg.label(id!(iml_previa)).set_text(cx, &e.symbol);
                dlg.label(id!(iml_nota)).set_text(cx, IML_NOTA);
            }
            None => {
                for caminho in [id!(iml_index), id!(iml_name), id!(iml_size_of)] {
                    dlg.label(caminho).set_text(cx, "");
                }
                dlg.label(id!(iml_previa)).set_text(cx, "");
                dlg.label(id!(iml_nota)).set_text(cx, IML_SEM_IMAGEM);
            }
        }

        // And the inner window.
        let add = self.imagelist.as_ref().is_some_and(|e| e.add_dialog_open);
        self.show_veil(cx, id!(addicon_dialog), add);
        if !add {
            return;
        }
        let pode = self
            .imagelist
            .as_ref()
            .is_some_and(|e| e.add_selected_preset.is_some());
        let janela = self.ui.view(id!(addicon_dialog.dlg));
        janela.button(id!(add_close)).set_text(cx, ACT_CLOSE);
        Self::habilitar(cx, &janela.widget(id!(add_ok)), pode);
        for (caminho, espera) in NOVO_CAMPOS {
            janela
                .text_input(caminho())
                .apply_over(cx, live! { empty_text: (espera) });
        }
    }

    #[note("ide-app-dialogs-imagelist-ideapp-atender-imagelist")]
    pub(in super::super) fn atender_imagelist(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.imagelist.is_none() {
            return;
        }
        if self.imagelist.as_ref().is_some_and(|e| e.add_dialog_open) {
            self.atender_addicon(cx, actions);
            return;
        }
        let dlg = self.ui.view(id!(imagelist_dialog.dlg));

        // The CLICK on a grid cell.
        let grade = dlg.portal_list(id!(iml_grid));
        if grade.any_items_with_actions(actions) {
            let mut escolhida = None;
            for (fileira, item) in grade.items_with_actions(actions) {
                for (coluna, caminho) in IML_CELULAS.iter().enumerate() {
                    if item.view(caminho()).button(id!(alvo)).clicked(actions) {
                        escolhida = Some(fileira * IML_COLUNAS + coluna);
                    }
                }
            }
            if let Some(i) = escolhida {
                if let Some(e) = self.imagelist.as_mut()
                    && i < e.entries.len()
                {
                    e.selected = Some(i);
                }
                self.redraw_imagelist(cx);
            }
        }

        if dlg.button(id!(iml_add)).clicked(actions)
            || dlg.button(id!(iml_replace)).clicked(actions)
        {
            if let Some(e) = self.imagelist.as_mut() {
                e.add_dialog_open = true;
                e.add_selected_preset = None;
            }
            self.redraw_imagelist(cx);
        }
        if dlg.button(id!(iml_delete)).clicked(actions) {
            if let Some(e) = self.imagelist.as_mut() {
                e.delete();
            }
            self.redraw_imagelist(cx);
        }
        if dlg.button(id!(iml_clear)).clicked(actions) {
            if let Some(e) = self.imagelist.as_mut() {
                e.entries.clear();
                e.selected = None;
            }
            self.redraw_imagelist(cx);
        }
        if let Some(i) = dlg.drop_down(id!(iml_size)).selected(actions)
            && let Some(lado) = image_list_editor::SIZES.get(i).copied()
        {
            if let Some(e) = self.imagelist.as_mut() {
                e.width = lado;
                e.height = lado;
            }
            self.redraw_imagelist(cx);
        }

        let aplicar = dlg.button(id!(iml_apply)).clicked(actions);
        let ok = dlg.button(id!(iml_ok)).clicked(actions);
        let cancelar = dlg.button(id!(iml_cancel)).clicked(actions);
        if aplicar || ok {
            self.gravar_imagelist(cx);
        }
        if ok || cancelar {
            self.imagelist = None;
            self.refresh(cx);
        }
    }

    #[note("ide-app-dialogs-imagelist-ideapp-atender-addicon")]
    pub(in super::super) fn atender_addicon(&mut self, cx: &mut Cx, actions: &Actions) {
        let janela = self.ui.view(id!(addicon_dialog.dlg));
        let lista = janela.portal_list(id!(add_list));
        if let Some(i) = clicked_row(&lista, actions) {
            if let Some(e) = self.imagelist.as_mut() {
                e.add_selected_preset = Some(i);
            }
            self.redraw_imagelist(cx);
        }
        if janela.button(id!(novo_add)).clicked(actions) {
            self.acrescentar_a_galeria(cx);
        }

        if janela.button(id!(add_ok)).clicked(actions) {
            let escolhido = self
                .imagelist
                .as_ref()
                .and_then(|e| e.add_selected_preset)
                .and_then(|i| self.galeria.get(i).cloned());
            if let Some(e) = self.imagelist.as_mut() {
                if let Some(icone) = escolhido {
                    e.add_from_gallery(&icone);
                }
                e.add_dialog_open = false;
            }
            self.redraw_imagelist(cx);
        }
        if janela.button(id!(add_close)).clicked(actions) {
            if let Some(e) = self.imagelist.as_mut() {
                e.add_dialog_open = false;
            }
            self.redraw_imagelist(cx);
        }
    }

    #[note("ide-app-dialogs-imagelist-ideapp-acrescentar-a-galeria")]
    pub(in super::super) fn acrescentar_a_galeria(&mut self, cx: &mut Cx) {
        let janela = self.ui.view(id!(addicon_dialog.dlg));
        let nome = janela.text_input(id!(novo_nome)).text();
        let simbolo = janela.text_input(id!(novo_simbolo)).text();
        let categoria = janela.text_input(id!(novo_categoria)).text();
        let existe = |n: &str| self.galeria.iter().any(|i| i.name == n);
        if let Some(porque) = image_list_editor::icon_refusal(&nome, &simbolo, &existe) {
            self.status = Some(porque.to_string());
            self.sincronizar(cx);
            return;
        }
        let categoria = match categoria.trim() {
            "" => image_list_editor::NO_CATEGORY.to_string(),
            c => c.to_string(),
        };
        let nome = nome.trim().to_string();
        self.galeria.push(image_list_editor::GalleryIcon {
            name: nome.clone(),
            symbol: simbolo.trim().to_string(),
            category: categoria,
            builtin: false,
        });
        self.status = Some(match Self::caminho_da_galeria() {
            Some(c) => match image_list_editor::save_gallery(&c, &self.galeria) {
                Some(e) => format!("{nome} is in this session only — {e}"),
                None => format!("{nome} is in the gallery"),
            },
            None => format!("{nome} is in this session only — no config folder on this machine"),
        });
        for (caminho, _) in NOVO_CAMPOS {
            janela.text_input(caminho()).set_text(cx, "");
        }
        self.redraw_imagelist(cx);
    }

    #[note("ide-app-dialogs-imagelist-ideapp-gravar-imagelist")]
    pub(in super::super) fn gravar_imagelist(&mut self, cx: &mut Cx) {
        let Some(estado) = self.imagelist.as_ref() else {
            return;
        };
        let no = estado.list_id.clone();
        let valor = estado.value();
        let (l, a) = (estado.width, estado.height);
        let antes = self
            .host
            .doc()
            .and_then(|d| d.ir().node(&no))
            .map(|n| ImageListEditorState::from_node(&no, n));
        let alvos = self.alvos_da_edicao(&no);
        self.write_property(cx, alvos.clone(), IML_KEY, valor);
        if antes.is_some_and(|b| b.width != l || b.height != a) {
            self.write_property(cx, alvos.clone(), IML_KEY_L, Value::Int(l.into()));
            self.write_property(cx, alvos, IML_KEY_A, Value::Int(a.into()));
        }
    }

    pub(in super::super) fn redraw_imagelist(&mut self, cx: &mut Cx) {
        self.sincronizar(cx);
        self.ui
            .view(id!(imagelist_dialog.dlg))
            .portal_list(id!(iml_grid))
            .redraw(cx);
        self.ui
            .view(id!(addicon_dialog.dlg))
            .portal_list(id!(add_list))
            .redraw(cx);
        self.ui.redraw(cx);
    }

    // -----------------------------------------------------------------
    // The ACTIONLIST EDITOR and STANDARD CLASSES — the two windows of
    // `TActionList`
    // -----------------------------------------------------------------
}
