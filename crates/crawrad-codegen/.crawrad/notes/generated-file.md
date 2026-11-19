---
id: generated-file
kind: doc
target_file: src/lib.rs
line_hint: 82
author: 
created: 
title: "Arquivo"
tags: []
signature: "pub struct Arquivo"
---

Um arquivo a escrever no projeto gerado.

# Examples

```rust
use std::path::PathBuf;
use crawrad_codegen::{Arquivo, Dono};

let arq = Arquivo {
    caminho: PathBuf::from("src/ui.rs"),
    conteudo: String::new(),
    dono: Dono::Gerador,
    adicoes: Vec::new(),
};
assert_eq!(arq.dono, Dono::Gerador);
```
