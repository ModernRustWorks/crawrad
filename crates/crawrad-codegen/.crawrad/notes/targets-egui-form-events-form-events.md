---
id: targets-egui-form-events-form-events
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 75
author: 
created: 
title: "form_events"
tags: []
signature: "pub fn form_events (root : & Node) -> Result < FormEvents , Erro >"
---

Extracts form event routing pieces ready for code generation.

Everything the form's wired events need, ready to be dropped into the
designer file.

# Examples

```rust
use crawrad_codegen::targets::egui::form_events::form_events;
use crawrad_ir::Node;

let root = Node::new("MainForm", "Form");
let fe = form_events(&root).unwrap();
assert!(fe.empty());
```

# Errors

Returns an error if an identifier name is invalid.

# Panics

Panics if internal focus state invariants are violated.
