---
id: targets-makepad-components-ligacoes-do-terceiro
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 366
author: 
created: 
title: "ligacoes_do_terceiro"
tags: []
signature: "pub fn ligacoes_do_terceiro (node : & Node) -> Result < Vec < (String , String) > , Erro >"
---

Returns the node's events that the target knows how to deliver.

The events of this node that the target knows how to deliver — the list is target-neutral and
lives in [`crate::targets::compartilhado::EVENTOS_DO_DECLARADO`].

# Examples

```rust
use crawrad_codegen::targets::makepad::components::ligacoes_do_terceiro;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
let evs = ligacoes_do_terceiro(&node).unwrap();
assert!(evs.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
