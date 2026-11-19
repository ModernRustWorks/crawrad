//! Makepad component implementation for `SaveDialog`.

use super::codegen::MakepadComponentCodegen;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct SaveDialogCodegen;

impl MakepadComponentCodegen for SaveDialogCodegen {
    fn class(&self) -> &'static str {
        "SaveDialog"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SaveDialog").expect("SaveDialog desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"#[derive(Debug, Clone, Default)]
pub struct SaveDialog {{
{body}
}}

impl SaveDialog {{
    /// Opens the system save dialog; `true` when a name was chosen
    /// (`file_name`, with `default_ext` added when it had none).
    pub fn execute(&mut self) -> bool {{
        let req = crawrad_runtime::file_dialog::FileDialogRequest {{
            title: self.title.clone(),
            filter: self.filter.clone(),
            initial_dir: self.initial_dir.clone(),
            file_name: self.file_name.clone(),
            default_ext: self.default_ext.clone(),
            multi: false,
        }};
        let Some(path) = crawrad_runtime::file_dialog::pick_save(&req) else {{
            return false;
        }};
        self.file_name = path.to_string_lossy().into_owned();
        true
    }}
}}
"#
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SaveDialog").expect("SaveDialog desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("SaveDialog {{ {body} }}")
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
