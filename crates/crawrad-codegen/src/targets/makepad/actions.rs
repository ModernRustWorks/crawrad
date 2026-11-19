//! Makepad action dispatch code generation (`handle_actions`).

use super::components;
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-actions-generate-actions-dispatch")]
pub fn generate_actions_dispatch(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<String, Erro> {
    let mut code = String::new();

    for n in emitted {
        let field = identificador(n.name())?;
        let block = components::emit_actions_com(reg, doc, n, &field, emitted)?;
        // A disabled control fires no event — classic RAD `Enabled`. On
        // Makepad a dimmed CheckBox, RadioButton or TextInput still reports
        // its changes, so they stop here.
        let effective = reg.effective_class(&n.class);
        if block.trim().is_empty() || !components::has_enabled_field(effective) {
            code.push_str(&block);
            continue;
        }
        code.push_str(&guarded(&format!("self.form.{field}.enabled"), &block));
    }

    Ok(code)
}

#[note("targets-makepad-actions-guarded")]
pub fn guarded(guard: &str, block: &str) -> String {
    if let Some(cond) = single_if(block) {
        let body = &block[block.find('\n').map_or(0, |i| i + 1)..];
        let cond = if cond.contains("||") && !cond.starts_with("let ") {
            format!("({cond})")
        } else {
            cond.to_string()
        };
        return format!("        if {guard} && {cond} {{\n{body}");
    }
    let inner: String = block.lines().map(|l| format!("    {l}\n")).collect();
    format!("        if {guard} {{\n{inner}        }}\n")
}

#[note("targets-makepad-actions-single-if")]
fn single_if(block: &str) -> Option<&str> {
    let lines: Vec<&str> = block.lines().collect();
    let (first, last) = (lines.first()?, lines.last()?);
    let cond = first.strip_prefix("        if ")?.strip_suffix(" {")?;
    if *last != "        }" || lines.len() < 2 {
        return None;
    }
    let at_level = |l: &&str| l.starts_with("        ") && !l.starts_with("         ");
    if lines[1..lines.len() - 1].iter().any(at_level) {
        return None;
    }
    Some(cond)
}

pub fn gerar_despacho_acoes(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitidos: &[&Node],
) -> Result<String, Erro> {
    generate_actions_dispatch(reg, doc, emitidos)
}

#[cfg(test)]
#[path = "../../test/makepad_guarded/test.rs"]
mod tests;
