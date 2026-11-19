//! Makepad component implementation for `ToolBar`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct ToolBarCodegen;

impl MakepadComponentCodegen for ToolBarCodegen {
    fn class(&self) -> &'static str {
        "ToolBar"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ToolBar").expect("ToolBar desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct ToolBar {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ToolBar").expect("ToolBar desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("ToolBar {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ToolBar").expect("ToolBar desc");
        let fields = state::fields_of(desc);
        if fields.is_empty() {
            return String::new();
        }
        let reads = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("        let _ = ({reads});\n")
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_dsl(
        &self,
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        // its controls go IN it, where the designer laid them out:
        // a container's children are skipped by the form's own loop, and
        // until this the bar emitted none — a control on a ToolBar was
        // missing from the generated app
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               flow: Overlay,\n\
             \x20               show_bg: true,\n\
             \x20               draw_bg: {{ color: #2d3139 }}\n\
             {}\
             \x20           }}\n",
            super::emit_children(doc, reg, node, "    ")?
        ))
    }
}
