---
id: targets-egui-components-main-menu-items-of
kind: doc
target_file: src/targets/egui/components/main_menu.rs
line_hint: 101
author: 
created: 
title: "items_of"
tags: []
signature: "pub fn items_of < 'a > (doc : & 'a Document , owner : & Node) -> Vec < & 'a Node >"
---

Returns the `MenuItem` children of `owner`, in order.

The `MenuItem` children of `owner`, in their order.

# Examples

```rust
use crawrad_codegen::targets::egui::components::main_menu::items_of;
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
let mut menu = Node::new("menu", "MainMenu");
let item = Node::new("item1", "MenuItem");
doc.nodes.insert("item1".to_string(), item);
menu.children.push("item1".to_string());
doc.nodes.insert("menu".to_string(), menu);
let owner = doc.node("menu").unwrap();
let items = items_of(&doc, owner);
assert_eq!(items.len(), 1);
```
