---
id: targets-egui-tab-order-taborder-before
kind: doc
target_file: src/targets/egui/tab_order.rs
line_hint: 57
author: 
created: 
title: "TabOrder::before"
tags: []
signature: "pub fn before (& self) -> & 'static str"
---

Emits Tab order initialization code before components.

What opens the components: Tab moves the focus before any widget
sees it, and the list the stops go into.

# Examples

```rust
use crawrad_codegen::targets::egui::tab_order::plan;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut doc = Document::new("MainForm", "Form");
let node = reg.instantiate("Edit", "edit1", Rect::new(10.0, 10.0, 120.0, 24.0)).unwrap();
let id = node.id.clone();
doc.add_child("MainForm", node).unwrap();
let btn = doc.node(&id).unwrap();
let to = plan(&reg, &doc, &[btn]).unwrap();
assert!(to.before().contains("crawrad_tab"));
```
