---
id: targets-egui-layout-linked-action-exec
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 211
author: 
created: 
title: "linked_action_exec"
tags: []
signature: "pub fn linked_action_exec (doc : & Document , act : & Node) -> Result < String , Erro >"
---

Returns the execution code for a linked action click.

Returns the execution code for a linked action click: user handler if present,
else dataset operation if it is a dataset action (#788).

# Examples

```rust
use crawrad_codegen::targets::egui::layout::linked_action_exec;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let act = Node::new("act", "Action");
let code = linked_action_exec(&doc, &act).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if an identifier name is invalid or dataset resolution fails.
