//! Slint component implementation for `MainMenu` and `MenuItem`.

use super::{codegen::SlintComponentCodegen, util};
use crate::identificador;
use crawrad_ir::{Document, Node, Value};

pub struct MainMenuCodegen;

impl SlintComponentCodegen for MainMenuCodegen {
    fn class(&self) -> &'static str {
        "MainMenu"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::main_menu::main_menu();
        let mut fields = Vec::new();
        fields.push("    pub images: String,".to_string());
        fields.extend(util::catalogue_fields(&desc, &["images"]));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct MainMenu {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::main_menu::main_menu();
        let images = match node.props.get("images") {
            Some(Value::Ref(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let mut inits = Vec::new();
        inits.push(format!("images: {}.to_string()", crate::literal(images)));
        inits.extend(util::catalogue_inits(node, &desc, &["images"]));
        let body = inits.join(", ");
        format!("MainMenu {{ {body} }}")
    }
}

pub struct MenuItemCodegen;

impl SlintComponentCodegen for MenuItemCodegen {
    fn class(&self) -> &'static str {
        "MenuItem"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::main_menu::menu_item();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_TEXT.to_string());
        fields.push(util::FIELD_CHECKED.to_string());
        fields.push("    pub shortcut: Shortcut,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible", "enabled", "text", "caption", "checked", "shortcut", "action",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct MenuItem {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::main_menu::menu_item();
        let sc = match node.props.get("shortcut") {
            Some(v) => util::value_to_rust_literal(v),
            None => "Shortcut::NONE".to_string(),
        };
        let mut inits = util::common_control_inits(node);
        inits.push(util::init_text(node));
        inits.push(util::init_checked(node));
        inits.push(format!("shortcut: {sc}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible", "enabled", "text", "caption", "checked", "shortcut", "action",
            ],
        ));
        let body = inits.join(", ");
        format!("MenuItem {{ {body} }}")
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut calls = String::new();
        if let Some(h) = crate::targets::slint::layout::event_handler(node, "on_click") {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
        if calls.is_empty() {
            return Ok(String::new());
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_clicked(move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             {calls}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        if crate::targets::slint::layout::event_handler(node, "on_click").is_some() {
            vec![format!("callback {name}_clicked();")]
        } else {
            Vec::new()
        }
    }
}
