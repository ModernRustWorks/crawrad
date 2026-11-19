---
id: targets-egui-control-events-controlevents-before
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 214
author: 
created: 
title: "ControlEvents::before"
tags: []
signature: "pub fn before (& self) -> String"
---

What opens the components: the list their rects go into.

# Examples

```rust
use crawrad_codegen::targets::egui::control_events::plan;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut node = Node::new("btn", "Button");
node.events.insert("on_mouse_down".to_string(), "btn_down".to_string());
let ce = plan(&reg, &[&node], |_| Ok(String::new())).unwrap().unwrap();
let before = ce.before();
assert!(before.contains("crawrad_rects"));
```
