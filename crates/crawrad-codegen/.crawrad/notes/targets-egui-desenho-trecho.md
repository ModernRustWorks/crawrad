---
id: targets-egui-desenho-trecho
kind: doc
target_file: src/targets/egui/desenho.rs
line_hint: 7
author: 
created: 
title: "trecho"
tags: []
signature: "pub fn trecho (nome : & str) -> String"
---

The `nome` section of [`DESENHO_COMUM`], ready to be emitted.

# Examples

```rust
use crawrad_codegen::targets::egui::desenho::trecho;

let code = trecho("abas");
assert!(!code.is_empty());
```
