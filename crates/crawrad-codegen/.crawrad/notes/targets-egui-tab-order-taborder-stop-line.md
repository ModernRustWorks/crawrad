---
id: targets-egui-tab-order-taborder-stop-line
kind: doc
target_file: src/targets/egui/tab_order.rs
line_hint: 39
author: 
created: 
title: "TabOrder::stop_line"
tags: []
signature: "pub fn stop_line (& self , n : & Node , name : & str) -> String"
---

Emits the line recording a Tab stop for a drawn control.

The line that leaves `n`'s stop, right after `n` is drawn (`r` is its
rect there); empty for a control with none. A hidden or disabled
control leaves none, so Tab skips it.

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
let line = to.stop_line(btn, "btn");
assert!(line.contains("crawrad_tab.push"));
```
