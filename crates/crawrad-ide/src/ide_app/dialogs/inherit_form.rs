//! `File > New Inherited Form…`: paints `NewInheritedForm` and turns
//! clicks into changes of it. What is listed, checked and written is decided
//! in `crawrad_project::inherited_form`.

use super::super::*;
use crawrad_note::note;
use crawrad_project::form_file::{self, is_form};
use crawrad_project::inherited_form::NewInheritedForm;

impl IdeApp {
    #[note("ide-app-dialogs-inherit-form-ideapp-open-inherit-form")]
    pub(in super::super) fn open_inherit_form(&mut self, cx: &mut Cx) {
        let form = self.host.active_file().filter(|f| is_form(f));
        let dir = match form.and_then(std::path::Path::parent) {
            Some(d) => d.to_path_buf(),
            None => self.explorer_target_folder(None),
        };
        let base = self.host.doc().map(|d| d.ir().root.clone());
        self.inherit_form = Some(NewInheritedForm::new(&dir, base.as_deref()));
        (self.inherit_form_fresh, self.inherit_name_focus) = (true, true);
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-inherit-form-ideapp-draw-inherit-form")]
    pub(in super::super) fn draw_inherit_form(&mut self, cx: &mut Cx) {
        let veil = self.ui.widget(id!(inherit_form_dialog));
        veil.set_visible(cx, self.inherit_form.is_some());
        let Some(m) = &self.inherit_form else {
            return;
        };
        let dlg = self.ui.view(id!(inherit_form_dialog.dlg));
        let (summary, parts) = m.summary().unwrap_or_default();
        let (files, ty) = m.files();
        let problem = m.name_problem().unwrap_or_default();
        for (label, text) in [
            (id!(inh_summary), summary.as_str()),
            (id!(inh_parts), parts.as_str()),
            (id!(inh_problem), problem.as_str()),
            (id!(inh_files), files.as_str()),
            (id!(inh_struct), ty.as_str()),
        ] {
            dlg.label(label).set_text(cx, text);
        }
        dlg.button(id!(inh_create)).set_enabled(cx, m.can_create());
        let name = dlg.text_input(id!(inh_name));
        if self.inherit_form_fresh {
            name.set_text(cx, &m.name);
        }
        let base = m
            .base()
            .and_then(|b| crawrad_ir::Document::load(&b.path).ok());
        let base = base
            .zip(m.base())
            .map(|(own, b)| form_file::design_doc(&b.path, own));
        if self.inherit_form_fresh
            && let Some(mut c) = dlg.widget(id!(inh_preview)).borrow_mut::<DesignCanvas>()
        {
            let rt = c.runtime_mut();
            let (w, h) = (VIEWPORT.0, VIEWPORT.1);
            rt.set_viewport(ViewportPx { x: 0, y: 0, w, h }, Scale(1.0));
            rt.set_mode(crawrad_runtime::Mode::Live);
            rt.set_grid(0.0);
            match &base {
                Some(d) => {
                    let _ = rt.open(d.ir());
                    let size = d.ir().node(&d.ir().root).and_then(|n| n.rect());
                    let fit = size.map_or(1.0, |r| (300.0 / r.w).min(150.0 / r.h).min(1.0));
                    rt.set_zoom(fit);
                }
                None => rt.close(),
            }
        }
        self.inherit_form_fresh = false;
        dlg.widget(id!(inh_preview)).redraw(cx);
        self.focus_inherit_form(cx);
    }

    #[note("ide-app-dialogs-inherit-form-ideapp-focus-inherit-form")]
    pub(in super::super) fn focus_inherit_form(&mut self, cx: &mut Cx) {
        if !self.inherit_name_focus || self.inherit_form.is_none() {
            return;
        }
        let name = (self.ui.view(id!(inherit_form_dialog.dlg))).text_input(id!(inh_name));
        if name.area().is_valid(cx) {
            cx.set_key_focus(name.area());
            if let Some(mut n) = name.borrow_mut() {
                n.select_all(cx);
            }
            self.inherit_name_focus = false;
        } else {
            cx.new_next_frame();
        }
    }

    #[note("ide-app-dialogs-inherit-form-ideapp-atender-inherit-form")]
    pub(in super::super) fn atender_inherit_form(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some(m) = self.inherit_form.as_mut() else {
            return;
        };
        let dlg = self.ui.view(id!(inherit_form_dialog.dlg));
        if let Some(text) = dlg.text_input(id!(inh_name)).changed(actions) {
            m.name = text;
        }
        if let Some(text) = dlg.text_input(id!(inh_filter)).changed(actions) {
            m.filter = text;
        }
        let list = dlg.portal_list(id!(inh_list));
        let rows = m.rows();
        for (i, item) in list.items_with_actions(actions) {
            if item.button(id!(alvo)).clicked(actions)
                && let Some(&form) = rows.get(i)
            {
                m.selected = form;
                (self.inherit_form_fresh, self.inherit_name_focus) = (true, true);
            }
        }
        let escaped = dlg.text_input(id!(inh_name)).escaped(actions);
        let closed = dlg.button(id!(inh_cancel)).clicked(actions)
            || dlg.button(id!(inh_close)).clicked(actions);
        let create = dlg.button(id!(inh_create)).clicked(actions)
            || dlg.text_input(id!(inh_name)).returned(actions).is_some();
        if create && m.can_create() {
            match m.create() {
                Ok(path) => {
                    self.inherit_form = None;
                    self.status = Some(format!("Created form {}", path.display()));
                    self.abrir_formulario(cx, &path);
                    self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
                }
                Err(why) => self.status = Some(why),
            }
        } else if escaped || closed {
            self.inherit_form = None;
        }
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-inherit-form-ideapp-draw-inherit-form-list")]
    pub(in super::super) fn draw_inherit_form_list(&self, cx: &mut Cx2d, lista: &mut PortalList) {
        let Some(m) = &self.inherit_form else {
            return;
        };
        let rows = m.rows();
        lista.set_item_range(cx, 0, rows.len());
        while let Some(i) = lista.next_visible_item(cx) {
            let Some(f) = rows.get(i).and_then(|&k| m.forms.get(k).map(|f| (k, f))) else {
                continue;
            };
            let (k, form) = f;
            let item = lista.item(cx, i, live_id!(FormRow));
            let picked = f64::from(u8::from(k == m.selected));
            item.apply_over(cx, live! { draw_bg: { picked: (picked) } });
            let indent = 10.0 + 16.0 * form.depth as f64;
            item.widget(id!(face))
                .apply_over(cx, live! { padding: { left: (indent) } });
            item.label(id!(face.name)).set_text(cx, &form.name);
            let place = (form.path.parent().and_then(|p| p.file_name()))
                .map(|d| format!("{}/{}", d.to_string_lossy(), form.name));
            let (note, inherits) = match &form.base {
                Some(b) => (String::new(), format!("inherits {b}")),
                None => (place.unwrap_or_default(), String::new()),
            };
            item.label(id!(face.note)).set_text(cx, &note);
            item.label(id!(face.inherits)).set_text(cx, &inherits);
            item.draw_all(cx, &mut Scope::empty());
        }
    }
}
