//! Slint component implementation for `SaveDialog`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::Node;

pub struct SaveDialogCodegen;

impl SlintComponentCodegen for SaveDialogCodegen {
    fn class(&self) -> &'static str {
        "SaveDialog"
    }

    fn struct_definition(&self) -> String {
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

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::save_dialog();
        let mut inits = vec!["file_name: String::new()".to_string()];
        inits.extend(util::catalogue_inits(node, &desc, &["file_name"]));
        let body = inits.join(", ");
        format!("SaveDialog {{ {body} }}")
    }
}
