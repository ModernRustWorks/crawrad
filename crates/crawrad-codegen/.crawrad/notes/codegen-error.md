---
id: codegen-error
kind: doc
target_file: src/lib.rs
line_hint: 787
author: 
created: 
title: "Erro"
tags: []
signature: "pub enum Erro"
---

What the generator rejects.

# Examples

```rust
use crawrad_codegen::Erro;

let erro = Erro::SemRaiz;
assert_eq!(erro.to_string(), "the document has no root");
```

**Read by the user**: the Explorer writes `cannot plan: <message>` in the
files column (`crawrad-ide/src/explorer.rs:640`) and the status bar writes
`generate: <message>` (`main.rs:2289` and `:2346`). Hence in ENGLISH —
enforced by `crawrad-doc/src/portao_ingles.rs`.
