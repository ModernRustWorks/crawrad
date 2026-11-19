---
id: targets-shared-ligacoes-do-declarado
kind: doc
target_file: src/targets/shared.rs
line_hint: 72
author: 
created: 
title: "ligacoes_do_declarado"
tags: []
signature: "pub fn ligacoes_do_declarado (node : & crawrad_ir :: Node) -> Result < Vec < (String , String) > , crate :: Erro >"
---

This node's events the targets can deliver, each with its handler.

**The events of THIS node that the targets know how to deliver**, with the handler of each one.

The order is that of [`EVENTOS_DO_DECLARADO`], and not that of the node's event map:
two generations of the same form must turn out byte-for-byte identical.

An event bound in the Inspector that the list does not know is left out — and that is
declared debt, measured by the gate, not silence.

# Examples

```rust
use crawrad_codegen::targets::shared::ligacoes_do_declarado;
use crawrad_ir::Node;

let node = Node::new("gauge1", "Gauge");
let links = ligacoes_do_declarado(&node).unwrap();
assert!(links.is_empty());
```

# Errors

Returns an error if any handler name is not a valid Rust identifier.
