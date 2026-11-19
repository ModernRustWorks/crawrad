---
id: components-comum-makepad-regua-texto
kind: doc
target_file: src/components/comum.rs
line_hint: 109
author: 
created: 
title: "MAKEPAD_REGUA_TEXTO"
tags: []
signature: "pub const MAKEPAD_REGUA_TEXTO : [f32 ; 4] = rgb (0xC8 , 0xCE , 0xE4)"
---

The number written on the ruler. Mirrors RXP's `REGUA_TEXTO`.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_REGUA_TEXTO;

assert_eq!(MAKEPAD_REGUA_TEXTO[3], 1.0);
```
