---
id: targets-makepad-components-emit-actions-com
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 344
author: 
created: 
title: "emit_actions_com"
tags: []
signature: "pub fn emit_actions_com (reg : & crawrad_registry :: Registry , doc : & Document , node : & Node , field : & str , emitted : & [& Node] ,) -> Result < String , Erro >"
---

Emits the action logic for `handle_actions` using the catalog.

The same, with the CATALOG — and this is the one the form calls.

**A third-party component's event binds through here**, and it is the first to
come from [`super::binding::BINDINGS`] instead of a hand-written `emit_actions`.
A declared component has no hand-written function to write: it only has
the table.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_actions_com;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let acts = emit_actions_com(&reg, &doc, &node, "btn", &[]).unwrap();
assert!(acts.is_empty());
```

# Errors

Returns an error if an identifier name is invalid or action emission fails.

# Panics

Panics if declared binding lookup fails unexpectedly for an active event.
