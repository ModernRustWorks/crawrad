//! Geometry, anchor, and visibility calculations for components in egui.

use crate::{Erro, identificador, sem_acelerador};
use crawrad_ir::{Document, Node, Value};
use crawrad_note::note;

#[note("targets-egui-layout-e-visual")]
pub fn e_visual(class: &str) -> bool {
    if let Some(desc) = crawrad_registry::Registry::builtin().get(class) {
        return desc.visual;
    }
    !matches!(class, "ActionList" | "Action" | "ImageList" | "Timer")
}

#[note("targets-egui-layout-emitido")]
pub fn emitido(class: &str) -> bool {
    crawrad_registry::Registry::builtin()
        .get(class)
        .is_none_or(|d| d.emits_code)
}

#[note("targets-egui-layout-emitido-no")]
pub fn emitido_no(doc: &Document, n: &Node) -> bool {
    if !emitido(&n.class) {
        return false;
    }
    if n.class != "TabSheet" {
        return true;
    }
    doc.parent_of(&n.id)
        .and_then(|p| doc.node(p))
        .is_some_and(|p| p.class == "PageControl")
}

#[note("targets-egui-layout-desenha-por-si")]
pub fn desenha_por_si(reg: &crawrad_registry::Registry, class: &str) -> bool {
    let effective = reg.effective_class(class);
    match reg.get(effective) {
        Some(d) => d.visual && !d.derived_geometry,
        None => e_visual(effective) && effective != "TabSheet",
    }
}

pub fn visivel(n: &Node) -> bool {
    !matches!(n.props.get("visible"), Some(Value::Bool(false)))
}

#[note("targets-egui-layout-ancoras")]
pub fn ancoras(n: &Node) -> String {
    if let Some(Value::Enum(a)) = n.props.get("align") {
        let quatro = match a.as_str() {
            "Top" => Some((true, true, true, false)),
            "Bottom" => Some((true, false, true, true)),
            "Left" => Some((true, true, false, true)),
            "Right" => Some((false, true, true, true)),
            "Client" => Some((true, true, true, true)),
            _ => None,
        };
        if let Some((e, t, d, b)) = quatro {
            return format!("Anchors {{ left: {e}, top: {t}, right: {d}, bottom: {b} }}");
        }
    }
    let b = |k: &str, padrao: bool| match n.props.get("anchors") {
        Some(Value::Map(m)) => match m.get(k) {
            Some(Value::Bool(x)) => *x,
            _ => padrao,
        },
        _ => padrao,
    };
    format!(
        "Anchors {{ left: {}, top: {}, right: {}, bottom: {} }}",
        b("left", true),
        b("top", true),
        b("right", false),
        b("bottom", false)
    )
}

pub fn marcado(n: &Node) -> bool {
    matches!(n.props.get("checked"), Some(Value::Bool(true)))
}

pub fn intervalo(n: &Node) -> u64 {
    match n.props.get("interval") {
        Some(Value::Int(i)) if *i > 0 => *i as u64,
        _ => 0,
    }
}

pub fn largura_da_borda(n: &Node) -> f32 {
    match n.props.get("border_width") {
        Some(Value::Int(i)) => (*i).max(0) as f32,
        _ => 1.0,
    }
}

pub fn habilitado(n: &Node) -> bool {
    !matches!(n.props.get("enabled"), Some(Value::Bool(false)))
}

pub fn texto(n: &Node) -> String {
    sem_acelerador(n.text().unwrap_or_default())
}

pub fn handler(n: &Node, evento: &str) -> Option<String> {
    n.events
        .get(evento)
        .filter(|h| !h.trim().is_empty())
        .cloned()
}

pub fn pagina_ativa(n: &Node) -> usize {
    match n.props.get("active_page") {
        Some(Value::Int(i)) if *i >= 0 => *i as usize,
        _ => 0,
    }
}

pub struct GuardaDePagina {
    pub abre: String,
    pub fecha: String,
}

pub fn visibility_guard(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    id: &str,
) -> Result<GuardaDePagina, Erro> {
    let mut condicoes: Vec<String> = Vec::new();
    let mut atual = Some(id.to_string());
    while let Some(no) = atual {
        if doc.node(&no).is_some_and(|n| n.class == "TabSheet") {
            let pc = doc
                .parent_of(&no)
                .and_then(|p| doc.node(p))
                .filter(|pc| pc.class == "PageControl");
            if let Some(pc) = pc {
                let indice = pc
                    .children
                    .iter()
                    .filter(|c| doc.node(c).is_some_and(|n| n.class == "TabSheet"))
                    .position(|c| *c == no)
                    .unwrap_or(0);
                let dono = identificador(pc.name())?;
                let pagina = identificador(doc.node(&no).map_or("", |n| n.name()))?;
                // The page open AND visible. Without the second half, hiding a
                // page hid nothing — `TabSheet`'s `visible` was a field
                // nobody read, and `dead_code` knew it.
                condicoes.push(format!(
                    "self.{dono}.active_page == {indice} && self.{pagina}.visible"
                ));
            }
        } else if no != id && no != doc.root {
            // A hidden container hides its children, as in the
            // Makepad target, where a child is nested in its parent's view
            let hides = doc.node(&no).filter(|c| {
                reg.get(&c.class).is_some_and(|desc| {
                    super::state::fields_of(desc)
                        .iter()
                        .any(|(field, _)| field == "visible")
                })
            });
            if let Some(container) = hides {
                condicoes.push(format!("self.{}.visible", identificador(container.name())?));
            }
        }
        atual = doc.parent_of(&no).map(|p| p.to_string());
    }
    if condicoes.is_empty() {
        return Ok(GuardaDePagina {
            abre: String::new(),
            fecha: String::new(),
        });
    }
    condicoes.reverse();
    Ok(GuardaDePagina {
        abre: format!("        if {} {{\n", condicoes.join(" && ")),
        fecha: "        }\n".to_string(),
    })
}

pub fn pai_de(doc: &Document, n: &Node) -> crawrad_ir::NodeId {
    doc.parent_of(&n.id)
        .cloned()
        .unwrap_or_else(|| doc.root.clone())
}

#[note("targets-egui-layout-caption-from-action")]
pub fn caption_from_action(
    doc: &crawrad_ir::Document,
    n: &crawrad_ir::Node,
    nome: &str,
) -> Result<String, crate::Erro> {
    let act = match n.props.get("action") {
        Some(crawrad_ir::Value::Ref(id)) if !id.trim().is_empty() => {
            doc.node(id).filter(|a| a.class == "Action")
        }
        _ => None,
    };
    let Some(act) = act else {
        return Ok(String::new());
    };
    if !matches!(act.props.get("text"), Some(crawrad_ir::Value::Str(s)) if !s.is_empty()) {
        return Ok(String::new());
    }
    let a = crate::identificador(act.name())?;
    Ok(format!(
        "        if self.{nome}.text != self.{a}.text {{\n\
         \x20           self.{nome}.text = self.{a}.text.clone();\n\
         \x20       }}\n"
    ))
}

#[note("targets-egui-layout-linked-action-exec")]
pub fn linked_action_exec(doc: &Document, act: &Node) -> Result<String, Erro> {
    if let Some(h) = handler(act, "on_execute") {
        return Ok(format!("                self.{}();\n", identificador(&h)?));
    }
    if let Some(da) = crate::targets::data_binding::dataset_action_of(doc, act)? {
        let op_call = crate::targets::data_binding::dataset_operation_call(
            &da.operation,
            &format!("self.{}_adapter", da.dataset_name),
            "                ",
        );
        return Ok(op_call);
    }
    Ok(String::new())
}
