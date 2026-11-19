---
id: targets-egui-tab-order-plan
kind: doc
target_file: src/targets/egui/tab_order.rs
line_hint: 26
author: 
created: 
title: "plan"
tags: []
signature: "pub fn plan (reg : & crawrad_registry :: Registry , doc : & Document , drawn : & [& Node]) -> Option < TabOrder >"
---

The stops among the `drawn` controls.

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
let to = plan(&reg, &doc, &[btn]);
assert!(to.is_some());
```
