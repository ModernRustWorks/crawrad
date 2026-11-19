//! Thin shell implementation of the New Component wizard dialog.
//! Logic and validation live in `crawrad_project::new_component`.

use super::super::*;
use crawrad_project::new_component::NewComponentWizard;

impl IdeApp {
    pub(in super::super) fn open_new_component(&mut self, cx: &mut Cx) {
        let root = self.desktop_root.clone().or_else(|| self.raiz());
        self.new_component = Some(NewComponentWizard::new(root.as_deref()));
        self.new_component_fresh = true;
        self.refresh(cx);
    }

    pub(in super::super) fn draw_new_component(&mut self, cx: &mut Cx) {
        let veil = self.ui.widget(id!(new_component_dialog));
        veil.set_visible(cx, self.new_component.is_some());
        let Some(m) = &self.new_component else {
            return;
        };
        let dlg = self.ui.view(id!(new_component_dialog.dlg));
        let step1 = dlg.view(id!(step1_view));
        let step2 = dlg.view(id!(step2_view));
        let is_step1 = m.step == 1;

        step1.set_visible(cx, is_step1);
        step2.set_visible(cx, !is_step1);

        dlg.button(id!(nc_back)).set_visible(cx, !is_step1);
        dlg.button(id!(nc_next)).set_visible(cx, is_step1);
        dlg.button(id!(nc_create)).set_visible(cx, !is_step1);

        let step_text = if is_step1 {
            "Step 1 of 2 · Base"
        } else {
            "Step 2 of 2 · Properties"
        };
        dlg.label(id!(nc_step_label)).set_text(cx, step_text);

        let err_text = m.error.as_deref().unwrap_or("");
        dlg.label(id!(nc_error)).set_text(cx, err_text);

        dlg.label(id!(nc_scope_hint))
            .set_text(cx, &crawrad_project::new_component::scope_hint(&m.draft));
        if self.new_component_fresh {
            if is_step1 {
                dlg.text_input(id!(nc_name)).set_text(cx, &m.draft.name);
                let combo = dlg.drop_down(id!(nc_base));
                combo.set_labels(cx, m.native_classes.clone());
                combo.set_selected_item(cx, m.base_index);
                let mine = m.draft.scope == crawrad_project::installed::ComponentScope::Mine;
                let radio = if mine {
                    id!(nc_scope_mine)
                } else {
                    id!(nc_scope_project)
                };
                dlg.radio_button(radio).select(cx, &mut Scope::empty());
            }
            self.new_component_fresh = false;
        }
    }

    pub(in super::super) fn atender_new_component(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some(m) = self.new_component.as_mut() else {
            return;
        };
        let dlg = self.ui.view(id!(new_component_dialog.dlg));

        if dlg.button(id!(nc_cancel)).clicked(actions) || dlg.button(id!(nc_close)).clicked(actions)
        {
            self.new_component = None;
            self.refresh(cx);
            return;
        }

        if m.step == 1 {
            if let Some(text) = dlg.text_input(id!(nc_name)).changed(actions) {
                m.draft.name = text;
            }
            if let Some(i) = dlg.drop_down(id!(nc_base)).selected(actions) {
                m.base_index = i;
            }
            let mut scope_set = WidgetSet::empty();
            scope_set.push(dlg.widget(id!(nc_scope_project)));
            scope_set.push(dlg.widget(id!(nc_scope_mine)));
            if let Some(i) = scope_set.as_radio_button_set().selected(cx, actions) {
                m.draft.scope = if i == 0 {
                    crawrad_project::installed::ComponentScope::Project
                } else {
                    crawrad_project::installed::ComponentScope::Mine
                };
                self.ui.redraw(cx);
            }
            if dlg.button(id!(nc_next)).clicked(actions)
                || dlg.text_input(id!(nc_name)).returned(actions).is_some()
            {
                let existing = Self::existing_classes();
                if m.next_step(&existing) {
                    self.new_component_fresh = true;
                }
                self.refresh(cx);
            }
        } else {
            if dlg.button(id!(nc_back)).clicked(actions) {
                m.prev_step();
                self.new_component_fresh = true;
                self.refresh(cx);
                return;
            }
            if dlg.button(id!(nc_add_prop)).clicked(actions) {
                m.add_prop();
                self.refresh(cx);
                return;
            }
            let list = dlg.portal_list(id!(nc_props_list));
            let mut remove_idx = None;
            for (i, item) in list.items_with_actions(actions) {
                if item.button(id!(nc_pdel)).clicked(actions) {
                    remove_idx = Some(i);
                }
                if let Some(val) = item.text_input(id!(nc_pname)).changed(actions)
                    && let Some(p) = m.draft.props.get_mut(i)
                {
                    p.name = val;
                }
                if let Some(val) = item.text_input(id!(nc_pty)).changed(actions)
                    && let Some(p) = m.draft.props.get_mut(i)
                {
                    p.ty = val;
                }
                if let Some(val) = item.text_input(id!(nc_pdefault)).changed(actions)
                    && let Some(p) = m.draft.props.get_mut(i)
                {
                    p.default = val;
                }
                if let Some(val) = item.text_input(id!(nc_pcat)).changed(actions)
                    && let Some(p) = m.draft.props.get_mut(i)
                {
                    p.category = val;
                }
                if let Some(val) = item.text_input(id!(nc_phint)).changed(actions)
                    && let Some(p) = m.draft.props.get_mut(i)
                {
                    p.hint = val;
                }
            }
            if let Some(idx) = remove_idx {
                m.remove_prop(idx);
                self.refresh(cx);
                return;
            }
            if dlg.button(id!(nc_create)).clicked(actions) {
                let existing = Self::existing_classes();
                match m.finish(&existing) {
                    Ok(lib_path) => {
                        self.new_component = None;
                        crawrad_project::installed::reload_ide_catalog();
                        self.status = Some(format!("Created component in {}", lib_path.display()));
                        self.reload_registry(cx);
                        self.abrir_no_editor(cx, lib_path);
                    }
                    Err(err) => {
                        m.error = Some(err);
                        self.refresh(cx);
                    }
                }
            }
        }
    }

    pub(in super::super) fn draw_new_component_props_list(
        &self,
        cx: &mut Cx2d,
        lista: &mut PortalList,
    ) {
        let Some(m) = &self.new_component else {
            return;
        };
        lista.set_item_range(cx, 0, m.draft.props.len());
        while let Some(i) = lista.next_visible_item(cx) {
            let Some(p) = m.draft.props.get(i) else {
                continue;
            };
            let item = lista.item(cx, i, live_id!(PropRow));
            item.text_input(id!(nc_pname)).set_text(cx, &p.name);
            item.text_input(id!(nc_pty)).set_text(cx, &p.ty);
            item.text_input(id!(nc_pdefault)).set_text(cx, &p.default);
            item.text_input(id!(nc_pcat)).set_text(cx, &p.category);
            item.text_input(id!(nc_phint)).set_text(cx, &p.hint);
            item.draw_all(cx, &mut Scope::empty());
        }
    }

    fn existing_classes() -> Vec<String> {
        crawrad_project::installed::ide_catalog()
            .catalog
            .all()
            .map(|d| d.class.to_string())
            .collect()
    }
}
