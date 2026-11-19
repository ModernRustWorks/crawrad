//! Code templates manager dialog.
//!
//! Visual shell glue: draws the Manage Code Templates modal dialog,
//! dispatches button clicks, imports and exports files via `rfd::FileDialog`.
//! State and validation logic live in `crawrad_insight::templates::TemplateManager`.
//! File I/O lives in `crawrad_project::code_templates`.

use super::super::*;
use crawrad_insight::templates::{TemplateListItem, TemplateManager};

impl IdeApp {
    pub(in super::super) fn open_manage_templates(&mut self, cx: &mut Cx) {
        let user = crawrad_project::code_templates::read_user();
        self.templates_dialog = Some(TemplateManager::new(user));
        self.templates_selected_seen = None;
        // Dialog-local words, not catalogue actions: set here so the DSL holds no action label.
        let dlg = self.ui.view(id!(templates_dialog.dlg));
        for (id, text) in [
            (id!(tpl_save), "Save"),
            (id!(tpl_delete), "Delete"),
            (id!(tpl_btn_duplicate), "Duplicate"),
        ] {
            dlg.button(id).set_text(cx, text);
        }
        dlg.label(id!(tpl_preview_head)).set_text(cx, "Preview");
        self.refresh(cx);
    }

    pub(in super::super) fn close_templates(&mut self, cx: &mut Cx) {
        self.templates_dialog = None;
        self.templates_selected_seen = None;
        self.refresh(cx);
    }

    pub(in super::super) fn draw_templates(&mut self, cx: &mut Cx) {
        let is_open = self.templates_dialog.is_some();
        self.show_veil(cx, id!(templates_dialog), is_open);
        let Some(mgr) = self.templates_dialog.as_ref() else {
            return;
        };

        let dlg = self.ui.view(id!(templates_dialog.dlg));
        let (trigger, desc, body, read_only) = mgr.fields();

        let problem_display = mgr.problem.as_deref().unwrap_or("");
        dlg.label(id!(tpl_problem)).set_text(cx, problem_display);

        Self::habilitar(cx, &dlg.widget(id!(tpl_delete)), !read_only);
        let preview = crawrad_insight::templates::preview(body);
        dlg.label(id!(tpl_preview)).set_text(cx, &preview);

        if self.templates_selected_seen == Some(mgr.selected) {
            return;
        }
        self.templates_selected_seen = Some(mgr.selected);

        dlg.text_input(id!(tpl_trigger)).set_text(cx, trigger);
        dlg.text_input(id!(tpl_desc)).set_text(cx, desc);
        dlg.text_input(id!(tpl_body)).set_text(cx, body);
    }

    pub(in super::super) fn draw_templates_list(&self, cx: &mut Cx2d, list: &mut PortalList) {
        let Some(mgr) = &self.templates_dialog else {
            return;
        };
        let items = mgr.list_items();
        list.set_item_range(cx, 0, items.len());
        while let Some(i) = list.next_visible_item(cx) {
            let Some(row) = items.get(i) else {
                continue;
            };
            match row {
                TemplateListItem::Section(title) => {
                    let item = list.item(cx, i, live_id!(TplSection));
                    item.label(id!(sec_label)).set_text(cx, title);
                    item.draw_all(cx, &mut Scope::empty());
                }
                TemplateListItem::Item(tpl) => {
                    let template_kind = if tpl.selected {
                        live_id!(TplRowSelected)
                    } else {
                        live_id!(TplRow)
                    };
                    let item = list.item(cx, i, template_kind);
                    item.label(id!(trigger)).set_text(cx, &tpl.trigger);
                    item.label(id!(description)).set_text(cx, &tpl.description);
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
    }

    pub(in super::super) fn handle_templates(&mut self, cx: &mut Cx, actions: &Actions) {
        let dlg = self.ui.view(id!(templates_dialog.dlg));
        let Some(mgr) = self.templates_dialog.as_mut() else {
            return;
        };
        let step = Self::templates_step(&dlg, mgr, actions);
        if step == TemplatesStep::Close {
            self.close_templates(cx);
            return;
        }
        if step == TemplatesStep::Reload {
            self.templates_selected_seen = None;
        }
        if step != TemplatesStep::Same {
            self.refresh(cx);
        }
    }

    /// One pass of the dialog's events over its state; the caller redraws.
    fn templates_step(
        dlg: &ViewRef,
        mgr: &mut TemplateManager,
        actions: &Actions,
    ) -> TemplatesStep {
        use crawrad_project::code_templates as file;
        let input = |name| dlg.text_input(name);
        if !mgr.fields().3 {
            let edits = [
                input(id!(tpl_trigger))
                    .changed(actions)
                    .map(|v| mgr.set_trigger(v)),
                input(id!(tpl_desc))
                    .changed(actions)
                    .map(|v| mgr.set_description(v)),
                input(id!(tpl_body))
                    .changed(actions)
                    .map(|v| mgr.set_body(v)),
            ];
            if edits.iter().any(Option::is_some) {
                mgr.problem = None;
                return TemplatesStep::Edited;
            }
        }
        let items = mgr.list_items();
        for (i, item) in dlg.portal_list(id!(tpl_list)).items_with_actions(actions) {
            if item.button(id!(alvo)).clicked(actions)
                && let Some(TemplateListItem::Item(row)) = items.get(i)
            {
                mgr.select(row.target);
                return TemplatesStep::Reload;
            }
        }
        let clicked = |name| dlg.button(name).clicked(actions);
        let picker = |title| {
            rfd::FileDialog::new()
                .set_title(title)
                .add_filter("TOML Templates", &["toml"])
        };
        if clicked(id!(tpl_btn_new)) {
            mgr.add_new();
        } else if clicked(id!(tpl_btn_duplicate)) {
            mgr.duplicate();
        } else if clicked(id!(tpl_delete)) {
            mgr.delete();
        } else if clicked(id!(tpl_btn_import)) {
            let Some(path) = picker("Import Code Templates").pick_file() else {
                return TemplatesStep::Same;
            };
            match file::read_file(&path) {
                Ok(entries) => mgr.import(entries),
                Err(why) => mgr.problem = Some(why),
            }
        } else if clicked(id!(tpl_btn_export)) {
            let save = picker("Export Code Templates").set_file_name("templates.toml");
            let Some(path) = save.save_file() else {
                return TemplatesStep::Same;
            };
            mgr.problem = file::write_file(&path, &mgr.mine).err();
            return TemplatesStep::Edited;
        } else if clicked(id!(tpl_save)) {
            match mgr.validate().and_then(|()| file::write_user(&mgr.mine)) {
                Ok(()) => return TemplatesStep::Close,
                Err(why) => mgr.problem = Some(why),
            }
            return TemplatesStep::Edited;
        } else if clicked(id!(tpl_cancel))
            || [id!(tpl_trigger), id!(tpl_desc), id!(tpl_body)]
                .into_iter()
                .any(|name| input(name).escaped(actions))
        {
            return TemplatesStep::Close;
        } else {
            return TemplatesStep::Same;
        }
        TemplatesStep::Reload
    }
}

/// What one pass of the dialog's events asks of the shell.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TemplatesStep {
    Same,
    Edited,
    Reload,
    Close,
}
