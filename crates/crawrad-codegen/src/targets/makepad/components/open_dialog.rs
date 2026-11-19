//! Makepad component implementation for `OpenDialog`.

use super::codegen::MakepadComponentCodegen;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct OpenDialogCodegen;

impl MakepadComponentCodegen for OpenDialogCodegen {
    fn class(&self) -> &'static str {
        "OpenDialog"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("OpenDialog").expect("OpenDialog desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"#[derive(Debug, Clone, Default)]
pub struct OpenDialog {{
{body}
}}

impl OpenDialog {{
    /// Opens the system file dialog; `true` when a file was chosen
    /// (`file_name`, and every pick in `files`).
    pub fn execute(&mut self) -> bool {{
        let req = crawrad_runtime::file_dialog::FileDialogRequest {{
            title: self.title.clone(),
            filter: self.filter.clone(),
            initial_dir: self.initial_dir.clone(),
            file_name: self.file_name.clone(),
            default_ext: self.default_ext.clone(),
            multi: false,
        }};
        let Some(paths) = crawrad_runtime::file_dialog::pick_open(&req) else {{
            return false;
        }};
        self.files = paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();
        self.file_name = self.files.first().cloned().unwrap_or_default();
        true
    }}
}}
"#
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("OpenDialog").expect("OpenDialog desc");
        let mut inits: Vec<String> = state::inits_of(node, desc)
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect();
        inits.push("files: Vec::new()".to_string());
        format!("OpenDialog {{ {} }}", inits.join(", "))
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
