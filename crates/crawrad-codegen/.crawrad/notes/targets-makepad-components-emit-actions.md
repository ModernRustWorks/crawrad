---
id: targets-makepad-components-emit-actions
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 330
author: 
created: 
title: "emit_actions"
tags: []
signature: "pub fn emit_actions (doc : & Document , node : & Node , field : & str , emitted : & [& Node] ,) -> Result < String , Erro >"
---

Emits action handling logic for `handle_actions`.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_actions;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let acts = emit_actions(&doc, &node, "btn", &[]).unwrap();
assert!(acts.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
