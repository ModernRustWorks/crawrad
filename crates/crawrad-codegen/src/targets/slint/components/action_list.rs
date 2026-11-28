//! Slint component implementation for `ActionList` and `Action`.

use super::codegen::SlintComponentCodegen;
use crate::{literal, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ActionListCodegen;

impl SlintComponentCodegen for ActionListCodegen {
    fn class(&self) -> &'static str {
        "ActionList"
    }

    fn struct_definition(&self) -> String {
        "#[derive(Debug, Clone, Default)]\npub struct ActionList {\n    pub enabled: bool,\n    pub images: String,\n}\n".to_string()
    }

    fn emit_default(&self, node: &Node) -> String {
        let en = layout::is_enabled(node);
        let images = match node.props.get("images") {
            Some(Value::Ref(s)) => literal(s),
            Some(Value::Str(s)) => literal(s),
            _ => "\"\"".to_string(),
        };
        format!("ActionList {{ enabled: {en}, images: {images}.to_string() }}")
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
        // action_list slint doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        // action_list slint stub
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn slint_callbacks(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }
}

pub struct ActionCodegen;

impl SlintComponentCodegen for ActionCodegen {
    fn class(&self) -> &'static str {
        "Action"
    }

    fn struct_definition(&self) -> String {
        "#[derive(Debug, Clone, Default)]\npub struct Action {\n    pub visible: bool,\n    pub enabled: bool,\n    pub text: String,\n    pub hint: String,\n    pub shortcut: Shortcut,\n    pub category: String,\n    pub image_index: i64,\n    pub checked: bool,\n    pub dataset: String,\n    pub dataset_operation: String,\n}\n".to_string()
    }

    fn emit_default(&self, node: &Node) -> String {
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let text = match node.props.get("text") {
            Some(Value::Str(s)) => literal(s),
            _ => "\"\"".to_string(),
        };
        let hint = match node.props.get("hint") {
            Some(Value::Str(s)) => literal(s),
            _ => "\"\"".to_string(),
        };
        let lit_shortcut = match node.props.get("shortcut") {
            Some(v) => crate::targets::compartilhado::literal_da_peca(v)
                .unwrap_or_else(|| "Shortcut::NONE".to_string()),
            None => "Shortcut::NONE".to_string(),
        };
        let category = match node.props.get("category") {
            Some(Value::Str(s)) => literal(s),
            _ => "\"\"".to_string(),
        };
        let image_index = match node.props.get("image_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        let checked = matches!(node.props.get("checked"), Some(Value::Bool(true)));
        let dataset = match node.props.get("dataset") {
            Some(Value::Ref(id)) => format!("{}.to_string()", literal(id)),
            _ => "String::new()".to_string(),
        };
        let dataset_operation = match node.props.get("dataset_operation") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => format!("{}.to_string()", literal(s)),
            _ => "\"None\".to_string()".to_string(),
        };

        format!(
            "Action {{ visible: {vis}, enabled: {en}, text: {text}.to_string(), hint: {hint}.to_string(), shortcut: {lit_shortcut}, category: {category}.to_string(), image_index: {image_index}, checked: {checked}, dataset: {dataset}, dataset_operation: {dataset_operation} }}"
        )
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
        // action_list slint doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        // action_list slint stub
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn slint_callbacks(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }
}
