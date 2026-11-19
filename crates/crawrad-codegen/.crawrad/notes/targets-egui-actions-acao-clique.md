---
id: targets-egui-actions-acao-clique
kind: doc
target_file: src/targets/egui/actions.rs
line_hint: 9
author: 
created: 
title: "acao_clique"
tags: []
signature: "pub fn acao_clique (node : & Node) -> Result < String , Erro >"
---

Assembles the execution block of an action triggered by a click.

# Examples

```rust
use crawrad_codegen::targets::egui::actions::acao_clique;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
let code = acao_clique(&node).unwrap();
assert!(code.contains("let _ = clicked;"));
```

# Errors

Returns an error if the click handler identifier is invalid.
