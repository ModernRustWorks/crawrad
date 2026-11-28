//! Slint component implementation for `PopupMenu`.

use super::{codegen::SlintComponentCodegen, util};
use crawrad_ir::{Node, Value};

pub struct PopupMenuCodegen;

impl SlintComponentCodegen for PopupMenuCodegen {
    fn class(&self) -> &'static str {
        "PopupMenu"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::popup_menu::popup_menu();
        let mut fields = Vec::new();
        fields.push("    pub images: String,".to_string());
        fields.extend(util::catalogue_fields(&desc, &["images"]));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct PopupMenu {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::popup_menu::popup_menu();
        let images = match node.props.get("images") {
            Some(Value::Ref(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let mut inits = Vec::new();
        inits.push(format!("images: {}.to_string()", crate::literal(images)));
        inits.extend(util::catalogue_inits(node, &desc, &["images"]));
        let body = inits.join(", ");
        format!("PopupMenu {{ {body} }}")
    }
}
