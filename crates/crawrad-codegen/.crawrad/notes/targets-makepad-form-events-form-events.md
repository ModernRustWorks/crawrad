---
id: targets-makepad-form-events-form-events
kind: doc
target_file: src/targets/makepad/form_events.rs
line_hint: 64
author: 
created: 
title: "form_events"
tags: []
signature: "pub fn form_events (root : & Node) -> Result < FormEvents , Erro >"
---

Everything the form's wired events need in the Makepad designer.

# Examples

```rust
use crawrad_codegen::targets::makepad::form_events::form_events;
use crawrad_ir::Node;

let root = Node::new("form1", "Form");
let events = form_events(&root).unwrap();
assert!(events.empty());
```

# Errors

Returns an error if any wired event handler name is not a valid identifier.
