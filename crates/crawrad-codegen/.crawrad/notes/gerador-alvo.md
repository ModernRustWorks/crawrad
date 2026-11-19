---
id: gerador-alvo
kind: doc
target_file: src/lib.rs
line_hint: 163
author: 
created: 
title: "Gerador::alvo"
tags: []
signature: "fn alvo (& self) -> & 'static str"
---

Returns the runtime target identifier.

Matches the property prefix in ADR 0002 and the runtime `target`:
`win32`, `egui`.

# Examples

```rust
use crawrad_codegen::{Gerador, GeradorRxp};

let gerador = GeradorRxp;
assert_eq!(gerador.alvo(), "egui");
```
