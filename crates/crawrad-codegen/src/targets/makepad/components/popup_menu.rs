//! Makepad component implementation for `PopupMenu`.

use super::codegen::MakepadComponentCodegen;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct PopupMenuCodegen;

impl MakepadComponentCodegen for PopupMenuCodegen {
    fn class(&self) -> &'static str {
        "PopupMenu"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("PopupMenu").expect("PopupMenu desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct PopupMenu {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("PopupMenu").expect("PopupMenu desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("PopupMenu {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("PopupMenu").expect("PopupMenu desc");
        let fields = state::fields_of(desc);
        if fields.is_empty() {
            return String::new();
        }
        let reads = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        // one field is no tuple (`unused_parens`)
        if fields.len() == 1 {
            format!("        let _ = {reads};\n")
        } else {
            format!("        let _ = ({reads});\n")
        }
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
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }
}
