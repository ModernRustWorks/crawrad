---
id: targets-egui-control-events-controlevents-field
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 226
author: 
created: 
title: "ControlEvents::field"
tags: []
signature: "pub fn field (& self) -> (& 'static str , & 'static str)"
---

The field the form keeps the router in, and its initialiser.

# Examples

```rust
use crawrad_codegen::targets::egui::control_events::plan;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut node = Node::new("btn", "Button");
node.events.insert("on_mouse_down".to_string(), "btn_down".to_string());
let ce = plan(&reg, &[&node], |_| Ok(String::new())).unwrap().unwrap();
let (field, init) = ce.field();
assert!(field.contains("crawrad_router"));
```
