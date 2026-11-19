---
id: dependencies-dependencia
kind: doc
target_file: src/dependencies.rs
line_hint: 17
author: 
created: 
title: "Dependencia"
tags: []
signature: "pub enum Dependencia"
---

Representation of a crate dependency for `cargo add`.

# Examples

```rust
use crawrad_codegen::Dependencia;

let dep = Dependencia::registro("makepad-widgets", "1.0");
assert_eq!(dep.nome(), "makepad-widgets");
```
