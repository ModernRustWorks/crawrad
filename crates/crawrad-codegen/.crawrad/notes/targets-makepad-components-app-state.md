---
id: targets-makepad-components-app-state
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 144
author: 
created: 
title: "app_state"
tags: []
signature: "pub fn app_state (doc : & Document , node : & Node , field : & str ,) -> Result < Option < codegen :: AppState > , crate :: Erro >"
---

What `node`'s class keeps in the generated `App` (#437), if anything.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::app_state;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let state = app_state(&doc, &node, "btn").unwrap();
assert!(state.is_none());
```

# Errors

Returns an error if state resolution fails.
