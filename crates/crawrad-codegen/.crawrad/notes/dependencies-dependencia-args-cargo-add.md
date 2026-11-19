---
id: dependencies-dependencia-args-cargo-add
kind: doc
target_file: src/dependencies.rs
line_hint: 63
author: 
created: 
title: "Dependencia::args_cargo_add"
tags: []
signature: "pub fn args_cargo_add (& self) -> Vec < String >"
---

Retorna os argumentos para passar para `cargo add`.

# Examples

```rust
use crawrad_codegen::Dependencia;

let dep = Dependencia::registro("eframe", "0.36");
assert_eq!(dep.args_cargo_add(), vec!["eframe@0.36"]);
```
