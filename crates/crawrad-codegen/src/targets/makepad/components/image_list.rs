//! Makepad component implementation for `ImageList`.
//!
//! Non-visual repository holding a collection of equal-sized graphic icons.

use super::codegen::MakepadComponentCodegen;
use crate::literal;
use crawrad_ir::{Document, Node, Value};

pub struct ImageListCodegen;

impl MakepadComponentCodegen for ImageListCodegen {
    fn class(&self) -> &'static str {
        "ImageList"
    }

    fn struct_definition(&self) -> String {
        struct_definition_image_list()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default_image_list(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        let _ = (self.form.{field}.image_width, self.form.{field}.image_height, &self.form.{field}.images);\n"
        )
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

pub fn struct_definition_image_list() -> String {
    "#[derive(Debug, Clone, Default, PartialEq, Eq)]\n\
     pub struct ImageItem {\n\
     \x20   pub name: String,\n\
     \x20   pub symbol: String,\n\
     }\n\n\
     #[derive(Debug, Clone, Default)]\n\
     pub struct ImageList {\n\
     \x20   pub image_width: i64,\n\
     \x20   pub image_height: i64,\n\
     \x20   pub images: Vec<ImageItem>,\n\
     }\n"
    .to_string()
}

pub fn extrair_icones(n: &Node) -> Vec<(String, String)> {
    match n.props.get("images_data") {
        Some(Value::List(itens)) => itens
            .iter()
            .filter_map(|item| {
                if let Value::Map(m) = item {
                    let nome = match m.get("name") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => String::new(),
                    };
                    let symbol = match m.get("symbol") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => String::new(),
                    };
                    Some((nome, symbol))
                } else {
                    None
                }
            })
            .collect(),
        _ => Vec::new(),
    }
}

pub fn emit_default_image_list(node: &Node) -> String {
    let width = match node.props.get("image_width") {
        Some(Value::Int(i)) => *i,
        _ => 16,
    };
    let height = match node.props.get("image_height") {
        Some(Value::Int(i)) => *i,
        _ => 16,
    };
    let icones = extrair_icones(node);
    let images_str = if icones.is_empty() {
        "Vec::new()".to_string()
    } else {
        let mut s = String::from("vec![");
        for (i, (name, symbol)) in icones.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&format!(
                "ImageItem {{ name: {}.to_string(), symbol: {}.to_string() }}",
                literal(name),
                literal(symbol)
            ));
        }
        s.push(']');
        s
    };
    format!("ImageList {{ image_width: {width}, image_height: {height}, images: {images_str} }}")
}
