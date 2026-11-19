---
id: dependencies-caminho-local-crate
kind: doc
target_file: src/dependencies.rs
line_hint: 79
author: 
created: 
title: "caminho_local_crate"
tags: []
signature: "pub fn caminho_local_crate (nome : & str) -> PathBuf"
---

Localiza o caminho de um crate local do workspace.

Localiza o caminho de um crate local do workspace (`crawrad-runtime`, `crawrad-binding`).

# Examples

```rust
use crawrad_codegen::dependencies::caminho_local_crate;

let path = caminho_local_crate("crawrad-runtime");
assert!(path.ends_with("crawrad-runtime"));
```
