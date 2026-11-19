---
id: targets-shared-trecho-da-geometria
kind: doc
target_file: src/targets/shared.rs
line_hint: 38
author: 
created: 
title: "trecho_da_geometria"
tags: []
signature: "pub fn trecho_da_geometria (nome : & str) -> String"
---

The `nome` section of [`GEOMETRIA`].

# Examples

```rust
use crawrad_codegen::targets::shared::trecho_da_geometria;

let s = trecho_da_geometria("fraction");
assert!(s.contains("fraction"));
```
