---
id: targets-egui-actions-acao-mudanca
kind: doc
target_file: src/targets/egui/actions.rs
line_hint: 21
author: 
created: 
title: "acao_mudanca"
tags: []
signature: "pub fn acao_mudanca (node : & Node) -> Result < String , Erro >"
---

Assembles the execution block of an action triggered by a value change.

# Examples

```rust
use crawrad_codegen::targets::egui::actions::acao_mudanca;
use crawrad_ir::Node;

let node = Node::new("edit", "Edit");
let code = acao_mudanca(&node).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if the change handler identifier is invalid.
