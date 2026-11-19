---
id: targets-egui-control-events-controlevents
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 64
author: 
created: 
title: "ControlEvents"
tags: []
signature: "pub struct ControlEvents"
---

Control event routing configuration for an egui form.

What the form needs for its controls' events; built by [`plan`], `None`
when no control has an event the router delivers.

# Examples

```rust
use crawrad_codegen::targets::egui::control_events::plan;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let node = Node::new("btn", "Button");
let ce = plan(&reg, &[&node], |_| Ok(String::new())).unwrap();
assert!(ce.is_none());
```
