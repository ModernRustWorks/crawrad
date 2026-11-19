---
id: generator-trait
kind: doc
target_file: src/lib.rs
line_hint: 161
author: 
created: 
title: "Gerador"
tags: []
signature: "pub trait Gerador"
---

What every generation target knows how to do.

# Examples

```rust
use crawrad_codegen::{Gerador, GeradorRxp};

let gerador = GeradorRxp;
assert_eq!(gerador.alvo(), "egui");
```
