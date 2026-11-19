---
id: adicao
kind: doc
target_file: src/lib.rs
line_hint: 98
author: 
created: 
title: "Adicao"
tags: []
signature: "pub struct Adicao"
---

A snippet that the generator appends if it does not find the mark.

# Examples

```rust
use crawrad_codegen::Adicao;

let adicao = Adicao {
    nome_da_fn: "btn_click".to_string(),
    texto: "fn btn_click(&mut self) {}".to_string(),
    ancora: None,
};
assert_eq!(adicao.nome_da_fn, "btn_click");
```
