---
id: desligamento
kind: doc
target_file: src/lib.rs
line_hint: 197
author: 
created: 
title: "Desligamento"
tags: []
signature: "pub struct Desligamento"
---

Describes an empty handler to remove and its associated bindings.

An empty handler that **will** be removed, and the bindings that need to be removed before
it — the output of the [`vinculos_vencidos`] pre-pass.

# Examples

```rust
use crawrad_codegen::Desligamento;

let desligamento = Desligamento {
    handler: "btn_click".to_string(),
    vinculos: vec![],
};
assert_eq!(desligamento.handler, "btn_click");
```
