//! Slint component implementation for `OpenDialog`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::Node;

pub struct OpenDialogCodegen;

impl SlintComponentCodegen for OpenDialogCodegen {
    fn class(&self) -> &'static str {
        "OpenDialog"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::open_dialog();
        let mut fields = vec![
            "    pub file_name: String,".to_string(),
            "    pub files: Vec<String>,".to_string(),
        ];
        fields.extend(util::catalogue_fields(&desc, &["file_name", "files"]));
        let body = fields.join("\n");
        format!(
            "#[derive(Debug, Clone, Default)]\n\
             pub struct OpenDialog {{\n\
             {body}\n\
             }}\n\n\
             impl OpenDialog {{\n\
             \x20   pub fn execute(&mut self) -> bool {{\n\
             \x20       true\n\
             \x20   }}\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::open_dialog();
        let mut inits = vec![
            "file_name: String::new()".to_string(),
            "files: Vec::new()".to_string(),
        ];
        inits.extend(util::catalogue_inits(node, &desc, &["file_name", "files"]));
        let body = inits.join(", ");
        format!("OpenDialog {{ {body} }}")
    }
}
