---
id: targets-egui-declarado-pintura
kind: doc
target_file: src/targets/egui/declarado.rs
line_hint: 44
author: 
created: 
title: "Pintura"
tags: []
signature: "pub struct Pintura"
---

Result of emitting drawing code for a declared component.

What a node's drawing needs: the code, and the shared sections that
it calls.

# Examples

```rust
use crawrad_codegen::targets::egui::declarado::Pintura;

let p = Pintura::default();
assert!(p.corpo.is_empty());
```
