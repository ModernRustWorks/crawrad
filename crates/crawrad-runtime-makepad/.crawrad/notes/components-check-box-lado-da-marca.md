---
id: components-check-box-lado-da-marca
kind: doc
target_file: src/components/check_box.rs
line_hint: 14
author: 
created: 
title: "LADO_DA_MARCA"
tags: []
signature: "pub const LADO_DA_MARCA : f32 = 14.0"
---

O lado da marca, em unidades de design.

**Espelha o RXP**, que usa `t.medida(14.0)` (`crawrad-runtime-egui/src/lib.rs:2646`)
— 7 de meio-lado a partir do centro vertical.

# Examples

```rust
use crawrad_runtime_makepad::components::check_box::LADO_DA_MARCA;

assert_eq!(LADO_DA_MARCA, 14.0);
```
