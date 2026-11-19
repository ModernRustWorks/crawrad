//! Event and action dispatch for the Egui target.
//!
//! Maps widget interactions to business method calls on the form.

use crate::{Erro, identificador};
use crawrad_ir::Node;
use crawrad_note::note;

#[note("targets-egui-actions-acao-clique")]
pub fn acao_clique(node: &Node) -> Result<String, Erro> {
    if let Some(h) = node.events.get("on_click").filter(|h| !h.trim().is_empty()) {
        let fn_nome = identificador(h)?;
        Ok(format!(
            "            if clicked {{\n                self.{fn_nome}();\n            }}\n"
        ))
    } else {
        Ok("            let _ = clicked;\n".to_string())
    }
}

#[note("targets-egui-actions-acao-mudanca")]
pub fn acao_mudanca(node: &Node) -> Result<String, Erro> {
    if let Some(h) = node
        .events
        .get("on_change")
        .filter(|h| !h.trim().is_empty())
    {
        let fn_nome = identificador(h)?;
        Ok(format!(
            "            if changed {{\n                self.{fn_nome}();\n            }}\n"
        ))
    } else {
        Ok(String::new())
    }
}
