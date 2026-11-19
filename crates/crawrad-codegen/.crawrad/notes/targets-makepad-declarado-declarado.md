---
id: targets-makepad-declarado-declarado
kind: doc
target_file: src/targets/makepad/declarado.rs
line_hint: 42
author: 
created: 
title: "Declarado"
tags: []
signature: "pub struct Declarado"
---

The drawing of a node, in the two forms that the Makepad target needs.

# Examples

```rust
use crawrad_codegen::targets::makepad::declarado::Declarado;

let dec = Declarado::default();
assert!(dec.dsl.is_empty());
```
