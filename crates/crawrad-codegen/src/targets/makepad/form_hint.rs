//! **The form's own hint** (`Hint` / `ShowHint` of the form), on
//! Makepad.
//!
//! The classic form shows its `Hint` over its empty body. Here it is one
//! block after `self.ui.handle_event`, beside the pointer's
//! ([`cursor`](super::cursor)): on every move it asks whether the pointer is
//! over a visible control, and over none it shows the form's hint in the
//! form's one tooltip (`crawrad_tooltip`).
//!
//! The controls' own hints show on their `Enter`, in the pointer router that
//! runs before the widgets. So when the pointer comes onto a control the
//! form's hint is hidden only if that control has none to show — hiding it
//! always would take away the hint the router has just put up.

use super::components;
use super::layout::{has_widget, visibility_expression};
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-form-hint-formhint")]
pub struct FormHint {
    pub field: String,
    pub after_widgets: String,
}

pub fn form_hint(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<Option<FormHint>, Erro> {
    let form_hint = doc
        .node(&doc.root)
        .and_then(crate::targets::shared::form_hint_text);
    let mut tab_checks = Vec::new();
    for n in emitted {
        if n.class != "PageControl" {
            continue;
        }
        let pgc_field = identificador(n.name())?;
        let sheets: Vec<(usize, String)> = n
            .children
            .iter()
            .filter_map(|c_id| doc.node(c_id))
            .filter(|c| c.class == "TabSheet")
            .enumerate()
            .filter_map(|(i, c)| identificador(c.name()).ok().map(|name| (i, name)))
            .collect();
        for (i, sheet) in sheets {
            tab_checks.push((pgc_field.clone(), i, sheet));
        }
    }
    if form_hint.is_none() && tab_checks.is_empty() {
        return Ok(None);
    }
    let no_menu_open = match super::menus::none_open(doc, emitted)? {
        Some(c) => format!(" && {c}"),
        None => String::new(),
    };
    let mut checks = String::new();
    // the last drawn is the one on top
    for n in emitted.iter().rev() {
        if !has_widget(reg, &n.class)
            || !components::for_class(&n.class)
                .is_some_and(|c| c.struct_definition().contains("pub hint:"))
        {
            continue;
        }
        let field = identificador(n.name())?;
        let visible = visibility_expression(doc, reg, n)?;
        checks.push_str(&format!(
            "            if over.is_none() && {visible} && self.ui.widget(id!({field})).area().rect(cx).contains(e.abs) {{\n\
             \x20               over = Some(CRAWRAD_FORM_SHOW_HINT && self.form.{field}.show_hint && !self.form.{field}.hint.is_empty());\n\
             \x20           }}\n"
        ));
    }
    let hide = "self.ui.tooltip(id!(crawrad_tooltip)).hide(cx);\n";
    // Over a sheet's tab, that sheet's hint (or none); the tab is the
    // whole decision, so the form's hint below runs only off the tabs
    let mut tabs = String::new();
    for (pgc_field, i, sheet) in &tab_checks {
        let kw = if tabs.is_empty() { "if" } else { "} else if" };
        tabs.push_str(&format!(
            "            {kw} self.ui.widget(id!({pgc_field}.tabs_header.tab_{i})).area().rect(cx).contains(e.abs) {{\n\
             \x20               if CRAWRAD_FORM_SHOW_HINT && self.form.{sheet}.show_hint && !self.form.{sheet}.hint.is_empty(){no_menu_open} {{\n\
             \x20                   self.ui.tooltip(id!(crawrad_tooltip)).show_with_options(cx, e.abs + dvec2(12.0, 20.0), &self.form.{sheet}.hint);\n\
             \x20                   self.crawrad_form_hint = true;\n\
             \x20               }} else if self.crawrad_form_hint {{\n\
             \x20                   {hide}\
             \x20                   self.crawrad_form_hint = false;\n\
             \x20               }}\n"
        ));
    }
    let mut block = String::new();
    match form_hint {
        Some(hint) => {
            let text = crate::literal(hint);
            let show = format!(
                "self.ui.tooltip(id!(crawrad_tooltip)).show_with_options(cx, e.abs + dvec2(12.0, 20.0), {text});\n"
            );
            // when the form's hint shows: over no control, and no menu open
            let mut when = Vec::new();
            if !checks.is_empty() {
                when.push("over.is_none()".to_string());
            }
            if let Some(c) = super::menus::none_open(doc, emitted)? {
                when.push(c);
            }
            if when.is_empty() && tabs.is_empty() {
                // nothing ever covers the body: the hint just follows the pointer
                return Ok(Some(FormHint {
                    field: String::new(),
                    after_widgets: format!(
                        "        // the form's Hint, over its empty body\n\
                         \x20       if let Event::MouseMove(e) = event {{\n\
                         \x20           {show}\
                         \x20       }}\n"
                    ),
                }));
            }
            if !checks.is_empty() {
                block.push_str(
                    "            // over which control, and whether it shows a hint of its own\n\
                     \x20           let mut over: Option<bool> = None;\n",
                );
                block.push_str(&checks);
            }
            // shown on every move, so it follows the pointer as egui's does — shown
            // once, at the edge it came in by, it could open past the window and stay
            // out of sight
            let when = if when.is_empty() {
                "true".to_string()
            } else {
                when.join(" && ")
            };
            block.push_str(&format!(
                "            if {when} {{\n\
                 \x20               {show}\
                 \x20               self.crawrad_form_hint = true;\n"
            ));
            if !checks.is_empty() {
                // onto a control with a hint: the router has just put it up, so the
                // tooltip is the control's now and stays
                block.push_str(
                    "            } else if over == Some(true) {\n\
                     \x20               self.crawrad_form_hint = false;\n",
                );
            }
            block.push_str(&format!(
                "            }} else if self.crawrad_form_hint {{\n\
                 \x20               {hide}\
                 \x20               self.crawrad_form_hint = false;\n\
                 \x20           }}\n"
            ));
        }
        // no form hint: off the tabs, a tab's hint goes
        None => block.push_str(&format!(
            "            if self.crawrad_form_hint {{\n\
             \x20               {hide}\
             \x20               self.crawrad_form_hint = false;\n\
             \x20           }}\n"
        )),
    }
    if !tabs.is_empty() {
        let inner: String = block.lines().map(|l| format!("    {l}\n")).collect();
        block = format!("{tabs}            }} else {{\n{inner}            }}\n");
    }
    Ok(Some(FormHint {
        field: "    #[rust]\n    crawrad_form_hint: bool,\n".to_string(),
        after_widgets: format!(
            "        // the form's Hint, over its empty body\n\
             \x20       if let Event::MouseMove(e) = event {{\n\
             {block}\
             \x20       }}\n"
        ),
    }))
}

#[cfg(test)]
#[path = "../../test/makepad_form_hint/test.rs"]
mod tests;
