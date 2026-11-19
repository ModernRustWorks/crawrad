---
id: targets-egui-control-events-controlevents-items
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 241
author: 
created: 
title: "ControlEvents::items"
tags: []
signature: "pub fn items (& self) -> String"
---

The router helper items for the top of the designer file.

The router's own items, for the top of the designer file; the size
watch only where someone listens to it (unused, it is dead code).

# Examples

```rust
use crawrad_codegen::targets::egui::control_events::plan;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut node = Node::new("btn", "Button");
node.events.insert("on_mouse_down".to_string(), "btn_down".to_string());
let ce = plan(&reg, &[&node], |_| Ok(String::new())).unwrap().unwrap();
let items = ce.items();
assert!(items.contains("CrawradControlRouter"));
```
