---
id: targets-egui-control-events-plan
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 93
author: 
created: 
title: "plan"
tags: []
signature: "pub fn plan (reg : & crawrad_registry :: Registry , drawn : & [& Node] , body : impl Fn (& Node) -> Result < String , Erro > ,) -> Result < Option < ControlEvents > , Erro >"
---

Plans control event routing for drawn controls.

Which of the `drawn` controls the router serves. `body` emits a control's
own body, to tell which events its emitter delivers itself; it is asked
only for a control with a handler for some routed event.

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

# Errors

Returns an error if emitting a control body fails or an identifier is invalid.
