//! Layout utility functions, property extraction, and hierarchy resolution for Makepad.

use crate::identificador;
use crawrad_ir::{Document, Node, Value};
use crawrad_note::note;

#[note("targets-makepad-layout-posicao")]
pub fn posicao(x: f32, y: f32) -> String {
    format!("margin: {{left: {x}, top: {y}}}")
}

#[note("targets-makepad-layout-text")]
pub fn text(n: &Node) -> String {
    crate::sem_acelerador(n.text().unwrap_or(n.name()))
}

pub fn texto(n: &Node) -> String {
    text(n)
}

#[note("targets-makepad-layout-str-val")]
pub fn str_val<'a>(n: &'a Node, key: &str) -> Option<&'a str> {
    match n.props.get(key) {
        Some(Value::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

pub fn valor_str<'a>(n: &'a Node, key: &str) -> Option<&'a str> {
    str_val(n, key)
}

#[note("targets-makepad-layout-event-handler")]
pub fn event_handler(n: &Node, event: &str) -> Option<String> {
    n.events
        .get(event)
        .filter(|h| !h.trim().is_empty())
        .cloned()
}

pub fn handler(n: &Node, event: &str) -> Option<String> {
    event_handler(n, event)
}

#[note("targets-makepad-layout-active-page")]
pub fn active_page(n: &Node) -> usize {
    match n.props.get("active_page") {
        Some(Value::Int(i)) if *i >= 0 => *i as usize,
        _ => 0,
    }
}

pub fn pagina_ativa(n: &Node) -> usize {
    active_page(n)
}

#[note("targets-makepad-layout-visibility-expression")]
pub fn visibility_expression(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    n: &Node,
) -> Result<String, crate::Erro> {
    let field = identificador(n.name())?;
    let mut conditions = vec![format!("self.form.{field}.visible")];
    if n.class == "Drawer" {
        conditions.push(format!("self.form.{field}.opened"));
    }

    let mut current_id = n.id.clone();
    while let Some(parent_id) = doc.parent_of(&current_id) {
        if let Some(parent) = doc.node(parent_id) {
            if parent.class == "TabSheet" {
                if let Some(pgc_id) = doc.parent_of(parent_id)
                    && let Some(pgc) = doc.node(pgc_id)
                    && pgc.class == "PageControl"
                {
                    let tab_index = pgc
                        .children
                        .iter()
                        .filter_map(|c_id| doc.node(c_id))
                        .filter(|c| c.class == "TabSheet")
                        .position(|c| c.id == *parent_id)
                        .unwrap_or(0);
                    let pgc_field = identificador(pgc.name())?;
                    conditions.push(format!("self.form.{pgc_field}.active_page == {tab_index}"));
                }
            } else if let Some(parent_desc) = reg.get(&parent.class)
                && parent.class != "Form"
                && parent.class != "PageControl"
                && parent_desc.children_area()
            {
                let parent_field = identificador(parent.name())?;
                conditions.push(format!("self.form.{parent_field}.visible"));
                if parent.class == "Drawer" {
                    conditions.push(format!("self.form.{parent_field}.opened"));
                }
            }
            current_id = parent.id.clone();
        } else {
            break;
        }
    }

    Ok(conditions.join(" && "))
}

#[note("targets-makepad-layout-is-inside-container")]
pub fn is_inside_container(doc: &Document, reg: &crawrad_registry::Registry, id: &str) -> bool {
    for a in doc.ancestors(id) {
        if a == doc.root {
            continue;
        }
        if let Some(parent) = doc.node(&a) {
            if parent.class == "TabSheet" {
                return true;
            }
            if let Some(desc) = reg.get(&parent.class)
                && desc.children_area()
            {
                return true;
            }
        }
    }
    false
}

#[note("targets-makepad-layout-is-inside-scroll-box")]
pub fn is_inside_scroll_box(doc: &Document, id: &str) -> bool {
    for a in doc.ancestors(id) {
        if a == doc.root {
            continue;
        }
        if let Some(parent) = doc.node(&a)
            && parent.class == "ScrollBox"
        {
            return true;
        }
    }
    false
}

pub fn is_visible(n: &Node) -> bool {
    !matches!(n.props.get("visible"), Some(Value::Bool(false)))
}

pub fn is_enabled(n: &Node) -> bool {
    !matches!(n.props.get("enabled"), Some(Value::Bool(false)))
}

pub fn is_checked(n: &Node) -> bool {
    matches!(n.props.get("checked"), Some(Value::Bool(true)))
}

pub fn has_widget(reg: &crawrad_registry::Registry, class: &str) -> bool {
    if let Some(desc) = reg.get(class) {
        desc.visual && !desc.derived_geometry
    } else {
        false
    }
}

pub fn expressao_visibilidade(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    n: &Node,
) -> Result<String, crate::Erro> {
    visibility_expression(doc, reg, n)
}

#[cfg(test)]
#[path = "../../test/makepad_position/test.rs"]
mod test;
