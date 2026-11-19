---
id: targets-egui-tab-order-taborder-after
kind: doc
target_file: src/targets/egui/tab_order.rs
line_hint: 64
author: 
created: 
title: "TabOrder::after"
tags: []
signature: "pub fn after (& self) -> & 'static str"
---

What closes them: the sequence for the next frame.

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
assert!(to.after().contains("crawrad_tab.after"));
```
