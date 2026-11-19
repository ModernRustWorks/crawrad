---
id: targets-data-binding-makepad-writeback
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 469
author: 
created: 
title: "makepad_writeback"
tags: []
signature: "pub fn makepad_writeback (doc : & Document , node : & Node) -> Result < String , Erro >"
---

Generates Makepad action writeback code for a single control node.

Makepad action writeback for a single control node inside `handle_actions`,
after the action handler has stored the widget's new value in the form.

# Examples

```rust
use crawrad_codegen::targets::data_binding::makepad_writeback;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let code = makepad_writeback(&doc, &node).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if the node name or dataset references are invalid.
