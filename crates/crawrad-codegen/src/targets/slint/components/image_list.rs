//! Slint component implementation for `ImageList`.

use super::codegen::SlintComponentCodegen;
use crate::literal;
use crawrad_ir::{Document, Node, Value};

pub struct ImageListCodegen;

impl SlintComponentCodegen for ImageListCodegen {
    fn class(&self) -> &'static str {
        "ImageList"
    }

    fn struct_definition(&self) -> String {
        "#[derive(Debug, Clone, Default, PartialEq, Eq)]\n\
         pub struct ImageItem {{\n\
         \x20   pub name: String,\n\
         \x20   pub symbol: String,\n\
         }}\n\n\
         #[derive(Debug, Clone, Default)]\n\
         pub struct ImageList {{\n\
         \x20   pub image_width: i64,\n\
         \x20   pub image_height: i64,\n\
         \x20   pub images: Vec<ImageItem>,\n\
         }}\n"
            .to_string()
    }

    fn emit_default(&self, node: &Node) -> String {
        let width = match node.props.get("image_width") {
            Some(Value::Int(i)) => *i,
            _ => 16,
        };
        let height = match node.props.get("image_height") {
            Some(Value::Int(i)) => *i,
            _ => 16,
        };
        let icons = extract_icons(node);
        let images_str = if icons.is_empty() {
            "Vec::new()".to_string()
        } else {
            let mut s = String::from("vec![");
            for (i, (name, symbol)) in icons.iter().enumerate() {
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
        format!(
            "ImageList {{ image_width: {width}, image_height: {height}, images: {images_str} }}"
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
        // image_list slint doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        // image_list slint stub
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

/// Extracts images from `images_data` prop.
pub fn extract_icons(node: &Node) -> Vec<(String, String)> {
    let Some(Value::List(entries)) = node.props.get("images_data") else {
        return Vec::new();
    };
    entries
        .iter()
        .filter_map(|entry| match entry {
            Value::Map(map) => {
                let key = match map.get("name") {
                    Some(Value::Str(val)) => val.clone(),
                    _ => String::new(),
                };
                let glyph = match map.get("symbol") {
                    Some(Value::Str(val)) => val.clone(),
                    _ => String::new(),
                };
                Some((key, glyph))
            }
            _ => None,
        })
        .collect()
}
