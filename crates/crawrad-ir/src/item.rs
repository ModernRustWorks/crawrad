//! Tree items: the `Items` of `TreeView`, and whatever comes after with the same
//! shape — menus, lists, columns with sub-columns.
//!
//! ## Why this exists
//!
//! `TreeView` was born with sample nodes **fixed in runtime code**:
//! `Node1`, `SubNode1`, `Node2` handwritten inside the Win32 `create_control`.
//! It worked for the eye and was a lie for everything else — the second
//! runtime would paint something else, codegen would have nothing to emit, and the
//! user would have no way to change it.
//!
//! Here items become **document data**, in the same form that `ImageList`
//! already used: `Value::List` of `Value::Map`. No new type in `Value` — the IR
//! already knew how to express collections all along; what was missing was using it.

use crate::Value;
use crawrad_note::note;
use std::collections::BTreeMap;

#[note("item-item")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub text: String,
    pub children: Vec<Item>,
    pub expanded: bool,
    pub image_index: i64,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            text: String::new(),
            children: Vec::new(),
            expanded: true,
            image_index: -1,
        }
    }
}

impl Item {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
            expanded: true,
            image_index: -1,
        }
    }

    pub fn with_children(text: impl Into<String>, children: Vec<Item>) -> Self {
        Self {
            text: text.into(),
            children,
            expanded: true,
            image_index: -1,
        }
    }

    #[note("item-item-from-value")]
    pub fn from_value(v: &Value) -> Option<Self> {
        let Value::Map(m) = v else { return None };
        let text = match m.get("text") {
            Some(Value::Str(s)) => s.clone(),
            _ => String::new(),
        };
        let children = match m.get("children") {
            Some(list) => Self::list_from(list),
            None => Vec::new(),
        };
        let expanded = match m.get("expanded") {
            Some(Value::Bool(b)) => *b,
            _ => true,
        };
        let image_index = match m.get("image_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        Some(Self {
            text,
            children,
            expanded,
            image_index,
        })
    }

    pub fn to_value(&self) -> Value {
        let mut m = BTreeMap::new();
        m.insert("text".to_string(), Value::Str(self.text.clone()));
        // Empty children do not go into the file: a leaf is a one-field map.
        if !self.children.is_empty() {
            m.insert("children".to_string(), Self::list_to(&self.children));
        }
        // An open item does not go into the file: expanded: false ONLY when false.
        if !self.expanded {
            m.insert("expanded".to_string(), Value::Bool(false));
        }
        if self.image_index >= 0 {
            m.insert("image_index".to_string(), Value::Int(self.image_index));
        }
        Value::Map(m)
    }

    #[note("item-item-list-from")]
    pub fn list_from(v: &Value) -> Vec<Self> {
        match v {
            Value::List(items) => items.iter().filter_map(Self::from_value).collect(),
            _ => Vec::new(),
        }
    }

    pub fn list_to(items: &[Self]) -> Value {
        Value::List(items.iter().map(Self::to_value).collect())
    }

    #[note("item-item-symbol")]
    pub fn symbol<'a>(&self, symbols: &'a [String]) -> Option<&'a str> {
        usize::try_from(self.image_index)
            .ok()
            .and_then(|i| symbols.get(i))
            .map(String::as_str)
    }

    #[note("item-item-linked-symbols")]
    pub fn linked_symbols(doc: &crate::Document, node: &crate::Node) -> Vec<String> {
        let name = match node.props.get("images") {
            Some(Value::Ref(s) | Value::Str(s)) if !s.trim().is_empty() => s.as_str(),
            _ => return Vec::new(),
        };
        let Some(list) = doc.node(name).or_else(|| doc.node_by_name(name)) else {
            return Vec::new();
        };
        match list.props.get("images_data") {
            Some(Value::List(icons)) if list.class == "ImageList" => icons
                .iter()
                .filter_map(|icon| match icon {
                    Value::Map(m) => match m.get("symbol") {
                        Some(Value::Str(s)) => Some(s.clone()),
                        _ => None,
                    },
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    #[note("item-item-total")]
    pub fn total(items: &[Self]) -> usize {
        items
            .iter()
            .map(|i| 1 + Self::total(&i.children))
            .sum::<usize>()
    }
}

#[cfg(test)]
#[path = "test/item/test.rs"]
mod tests;
