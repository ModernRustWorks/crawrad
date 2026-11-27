//! Slint component implementation for `SaveDialog`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::{Document, Node};

pub struct SaveDialogCodegen;

impl SlintComponentCodegen for SaveDialogCodegen {
    fn class(&self) -> &'static str {
        "SaveDialog"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
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

    fn emit_slint(
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

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::save_dialog();
    let mut fields = vec!["    pub file_name: String,".to_string()];
    fields.extend(util::catalogue_fields(&desc, &["file_name"]));
    let body = fields.join("\n");
    format!(
        "#[derive(Debug, Clone, Default)]\n\
         pub struct SaveDialog {{\n\
         {body}\n\
         }}\n\n\
         impl SaveDialog {{\n\
         \x20   pub fn execute(&mut self) -> bool {{\n\
         \x20       true\n\
         \x20   }}\n\
         }}\n"
    )
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::save_dialog();
    let mut inits = vec!["file_name: String::new()".to_string()];
    inits.extend(util::catalogue_inits(node, &desc, &["file_name"]));
    let body = inits.join(", ");
    format!("SaveDialog {{ {body} }}")
}
