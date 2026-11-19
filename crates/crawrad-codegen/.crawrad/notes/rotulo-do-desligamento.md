---
id: rotulo-do-desligamento
kind: doc
target_file: src/lib.rs
line_hint: 272
author: 
created: 
title: "ROTULO_DO_DESLIGAMENTO"
tags: []
signature: "pub const ROTULO_DO_DESLIGAMENTO : & str = \"Remove empty handlers\""
---

Label of the empty handler removal transaction for the IDE's Undo.

The transaction label that the shell applies with what [`vinculos_vencidos`]
returned, **in English** — it appears in the IDE's `Undo`, so it must describe
what happened, rather than what was requested.

# Examples

```rust
use crawrad_codegen::ROTULO_DO_DESLIGAMENTO;

assert_eq!(ROTULO_DO_DESLIGAMENTO, "Remove empty handlers");
```
