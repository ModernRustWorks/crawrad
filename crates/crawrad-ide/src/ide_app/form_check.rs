//! The Form check glue: the checker and the panel's state live in
//! `crawrad_designer::form_check*`, the drawing in `form_check_view` and the
//! canvas; here only the wires.

use crawrad_actions::Panel;
use crawrad_designer::form_check_panel::{CheckEffect, CheckInput as I, tab_badges};
use crawrad_runtime::Runtime;
use crawrad_runtime_makepad::canvas::DesignCanvas;
use makepad_code_editor::form_check_view::FormCheckAction as A;
use makepad_code_editor::form_check_view::*;
use makepad_widgets::*;

use super::{IdeApp, layout};

/// What a control with no colour of its own shows: `ROLE_TEXT` on `MAKEPAD_PAPER`.
const CHECK_THEME: crawrad_designer::form_check::Theme = crawrad_designer::form_check::Theme {
    text: crawrad_ir::Color::rgb(0xCD, 0xD6, 0xF4),
    surface: crawrad_ir::Color::rgb(0x25, 0x25, 0x38),
};

impl IdeApp {
    /// A web project's form is a white page, badged in the bar.
    pub(super) fn sync_page_look(&mut self, cx: &mut Cx) {
        let (light, web) = crawrad_project::page_look::of_form(
            self.membro().as_deref(),
            self.host.doc().map(|d| d.ir()),
        );
        let look = crawrad_runtime_makepad::canvas::PageLook { light, web };
        if self
            .canvas()
            .borrow_mut::<DesignCanvas>()
            .is_some_and(|mut c| c.set_page_look(look))
        {
            self.painel(Panel::Design)
                .view(id!(designer_options_bar.lbl_web_page))
                .set_visible(cx, web);
            self.canvas().redraw(cx);
        }
    }

    /// Checks the form again when it changed, and repaints what shows it.
    pub(super) fn check_form(&mut self, cx: &mut Cx) {
        let reg = self.designer.registry.clone();
        let changed = match (self.host.doc(), reg) {
            (Some(d), Some(reg)) => {
                let form = d.ir().root.clone();
                let doc_localized;
                let ir_to_check = if let Some(ref loc) = self.design_locale {
                    doc_localized =
                        crawrad_designer::translations::localized(d.ir(), &form, &reg, loc);
                    &doc_localized
                } else {
                    d.ir()
                };
                let mut rev = d.revision();
                if let Some(ref lang) = self.design_language {
                    use std::hash::{Hash, Hasher};
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    lang.hash(&mut hasher);
                    rev ^= hasher.finish();
                }
                self.form_check
                    .refresh(ir_to_check, &reg, CHECK_THEME, &form, rev)
            }
            _ => !std::mem::take(&mut self.form_check).findings.is_empty(),
        };
        if changed {
            self.paint_form_check(cx);
        }
    }

    pub(super) fn paint_form_check(&mut self, cx: &mut Cx) {
        let open = self.painel_a_vista(&self.arranjo(), Panel::FormCheck);
        let badges = match (open && self.show_tab_order, self.host.doc()) {
            (true, Some(d)) => tab_badges(d.ir()),
            _ => Vec::new(),
        };
        if let Some(mut c) = self.canvas().borrow_mut::<DesignCanvas>() {
            c.runtime_mut().set_findings(&self.form_check.pins());
            c.runtime_mut().set_badges(&badges);
        }
        self.canvas().redraw(cx);
        let v = self.form_check.view();
        let rows = v.rows.into_iter();
        let rows = rows.map(|r| (r.number, r.error, r.rule.into(), r.text, r.fix, r.selected));
        let tab = self.dock().item(layout::tab_of(Panel::FormCheck));
        tab.form_check_view(id!(check_view)).set_view(
            cx,
            (v.chips, v.note, v.fix_all),
            rows.collect(),
        );
        let chip = self.ui.button(id!(form_check_chip));
        chip.set_text(
            cx,
            &format!("Form check \u{00b7} {}", self.form_check.summary()),
        );
        chip.set_visible(cx, self.host.doc().is_some());
    }

    pub(super) fn form_check_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(form_check_chip)).clicked(actions) {
            return self.show_panel(cx, Panel::FormCheck);
        }
        let tab = self.dock().item(layout::tab_of(Panel::FormCheck));
        let input = match tab.form_check_view(id!(check_view)).action(actions) {
            A::Chip(i) => I::Chip(i),
            A::Row(i) => I::Row(i),
            A::Fix(i) => I::Fix(i),
            A::FixAll => I::FixAll,
            A::None => return,
        };
        let Some(doc) = self.host.doc().map(|d| d.ir().clone()) else {
            return;
        };
        match self.form_check.input(input, &doc) {
            CheckEffect::Paint => {}
            CheckEffect::Select(node) => self.select_node(cx, &node),
            CheckEffect::Apply(tx) => {
                let _ = self.apply_effect(cx, crawrad_actions::Effect::tx(tx));
                self.sincronizar(cx);
            }
            CheckEffect::Bind {
                node,
                event,
                handler,
            } => {
                self.bind_event(cx, node, &event, &handler);
                self.abrir_no_codigo(cx, &handler);
            }
        }
        self.paint_form_check(cx);
    }

    /// Hover tooltip for the pin under the pointer.
    pub(super) fn hint_form_check_pin(&mut self, cx: &mut Cx, at: DVec2) {
        let canvas = self.canvas();
        let pin = (self.open_menu.is_none() && !self.host.center().is_text())
            .then(|| canvas.borrow::<DesignCanvas>()?.pin_at(at))
            .flatten();
        if pin == self.form_check_tip {
            return;
        }
        self.form_check_tip = pin;
        let tip = self.ui.form_check_tip(id!(form_check_tip));
        match pin.and_then(|number| self.form_check.tip(number)) {
            Some((text, error, fix)) => {
                tip.show(cx, at + dvec2(12.0, 20.0), &text, error, fix.as_deref());
            }
            None => tip.hide(cx),
        }
    }

    /// `form.check` over MCP.
    pub(super) fn form_check_json(&self) -> serde_json::Value {
        match self.host.doc() {
            Some(d) => self.form_check.to_json(d.ir()),
            None => serde_json::json!({ "summary": "No form open", "findings": [] }),
        }
    }
}
