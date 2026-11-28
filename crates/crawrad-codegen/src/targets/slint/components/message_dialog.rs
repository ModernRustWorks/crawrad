//! Slint component implementation for `MessageDialog`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::Node;

pub struct MessageDialogCodegen;

impl SlintComponentCodegen for MessageDialogCodegen {
    fn class(&self) -> &'static str {
        "MessageDialog"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::message_dialog();
        let fields = util::catalogue_fields(&desc, &[]);
        let body = fields.join("\n");
        format!(
            "#[derive(Debug, Clone, Default)]\n\
             pub struct MessageDialog {{\n\
             {body}\n\
             }}\n\n\
             impl MessageDialog {{\n\
             \x20   pub fn execute(&mut self) -> bool {{\n\
             \x20       let choice = crawrad_runtime_slint::show_message_dialog(&self.message, \"\");\n\
             \x20       choice == crawrad_runtime_slint::DialogChoice::Ok || choice == crawrad_runtime_slint::DialogChoice::Yes\n\
             \x20   }}\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::message_dialog();
        let inits = util::catalogue_inits(node, &desc, &[]);
        let body = inits.join(", ");
        format!("MessageDialog {{ {body} }}")
    }
}
