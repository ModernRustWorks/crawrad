---
id: targets-egui-components-compartilhado-com
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 94
author: 
created: 
title: "Compartilhado::com"
tags: []
signature: "pub fn com (self , outro : Self) -> Self"
---

The union of two — the form gathers what all components require.

# Examples

```rust
use crawrad_codegen::targets::egui::components::Compartilhado;

let a = Compartilhado { tracejado: true, marca: false, fracao: false };
let b = Compartilhado { tracejado: false, marca: true, fracao: false };
let uniao = a.com(b);
assert!(uniao.tracejado && uniao.marca);
```
