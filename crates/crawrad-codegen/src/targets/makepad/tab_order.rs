//! **Tab walks the form's controls in `TabOrder`** in the Makepad target
//! the call that hands every key to
//! `crawrad_runtime_makepad::tab_order::tab_key` before the widgets see it,
//! with the stops sorted here, once, at generation.
//!
//! A stop is a control with a place in the Tab sequence
//! (`targets::tab_places`) whose own widget can hold the keyboard. A
//! container is left out: its place only orders the controls inside it, and
//! focusing its view would leave the keyboard on no control.

use super::components;
use crate::targets::tab_places;
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;
use crawrad_registry::Children;

#[note("targets-makepad-tab-order-tab-key-call")]
pub fn tab_key_call(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<String, Erro> {
    let mut stops: Vec<(Vec<i64>, usize, &Node)> = Vec::new();
    for (i, n) in emitted.iter().enumerate() {
        let Some(desc) = reg.get(&n.class) else {
            continue;
        };
        if !desc.visual {
            continue;
        }
        // a container is no stop unless it says which of its widgets
        // takes the keyboard (a page control's open tab)
        let holds_focus = || {
            identificador(n.name())
                .ok()
                .and_then(|f| components::for_class(&n.class)?.focus_path(doc, n, &f))
                .is_some()
        };
        if desc.children == Children::Area && !holds_focus() {
            continue;
        }
        if let Some(place) = tab_places::place(reg, doc, n) {
            stops.push((place, i, n));
        }
    }
    if stops.is_empty() {
        return Ok(String::new());
    }
    // ties keep the document order, as classic RAD does
    stops.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut list = String::new();
    for (place, _, n) in &stops {
        let f = identificador(n.name())?;
        let has = |key: &str| {
            reg.get(&n.class)
                .is_some_and(|d| d.props.iter().any(|p| p.key == key))
        };
        let mut on = Vec::new();
        for key in ["visible", "enabled"] {
            if has(key) {
                on.push(format!("self.form.{f}.{key}"));
            }
        }
        let on = if on.is_empty() {
            "true".to_string()
        } else {
            on.join(" && ")
        };
        // a Memo with WantTabs types the Tab instead of leaving
        let keeps = if n.class == "Memo" {
            format!("self.form.{f}.want_tabs")
        } else {
            "false".to_string()
        };
        let widget_expr = components::for_class(&n.class)
            .and_then(|c| c.focus_path(doc, n, &f))
            .unwrap_or_else(|| format!("self.ui.widget(id!({}))", components::text_path(n, &f)));
        list.push_str(&format!(
            "            // TabOrder {}\n\
             \x20           ({on}, {keeps}, {widget_expr}),\n",
            tab_places::literal(place)
        ));
    }
    Ok(format!(
        "        // Tab walks the controls in TabOrder; the control that\n\
         \x20       // takes the focus does not take the key as well.\n\
         \x20       let crawrad_tab = [\n{list}\x20       ];\n\
         \x20       if crawrad_runtime_makepad::tab_order::tab_key(cx, &crawrad_tab, event) {{\n\
         \x20           return;\n\
         \x20       }}\n"
    ))
}

#[cfg(test)]
#[path = "../../test/makepad_tab_order/test.rs"]
mod tests;
