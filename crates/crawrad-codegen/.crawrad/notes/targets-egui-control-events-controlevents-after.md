---
id: targets-egui-control-events-controlevents-after
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 250
author: 
created: 
title: "ControlEvents::after"
tags: []
signature: "pub fn after (& self) -> (String , bool , bool , bool)"
---

The event dispatch code emitted after components.

The dispatch after the components, and which conversions it calls:
`(code, button, modifiers, key)`.

# Examples

```rust
use crawrad_codegen::targets::egui::control_events::plan;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut node = Node::new("btn", "Button");
node.events.insert("on_mouse_down".to_string(), "btn_down".to_string());
let ce = plan(&reg, &[&node], |_| Ok(String::new())).unwrap().unwrap();
let (after, _, _, _) = ce.after();
assert!(!after.is_empty());
```
