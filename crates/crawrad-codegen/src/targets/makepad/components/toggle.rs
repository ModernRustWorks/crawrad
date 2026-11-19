//! Makepad component implementation for `Toggle`: makepad's own
//! `<Toggle>`, a `CheckBox` drawn as a switch.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct ToggleCodegen;

impl MakepadComponentCodegen for ToggleCodegen {
    fn class(&self) -> &'static str {
        "Toggle"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Toggle").expect("Toggle desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Toggle {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Toggle").expect("Toggle desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Toggle {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        // the switch follows `checked`, the caption `text`\n\
             \x20       if let Some(mut inner) = self.ui.check_box(id!({field})).borrow_mut() {{\n\
             \x20           let is_on = inner.animator_in_state(cx, id!(active.on));\n\
             \x20           if is_on != self.form.{field}.checked {{\n\
             \x20               let state = if self.form.{field}.checked {{ id!(active.on) }} else {{ id!(active.off) }};\n\
             \x20               inner.animator_play(cx, state);\n\
             \x20           }}\n\
             \x20       }}\n\
             \x20       if self.ui.check_box(id!({field})).text() != self.form.{field}.text {{\n\
             \x20           self.ui.check_box(id!({field})).set_text(&self.form.{field}.text);\n\
             \x20       }}\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut calls = String::new();
        // the click first, then the change, as the CheckBox orders them
        for event in ["on_click", "on_change"] {
            if let Some(h) = layout::handler(node, event) {
                calls.push_str(&format!(
                    "            self.form.{}();\n",
                    identificador(&h)?
                ));
            }
        }
        Ok(format!(
            "        if let Some(_val) = self.ui.check_box(id!({field})).changed(actions) {{\n\
             \x20           self.form.{field}.checked = _val;\n\
             {calls}\
             \x20       }}\n"
        ))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        let caption = layout::text(node)
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        Ok(format!(
            "            {name} = <Toggle> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               text: \"{caption}\"\n\
             \x20           }}\n"
        ))
    }
}
