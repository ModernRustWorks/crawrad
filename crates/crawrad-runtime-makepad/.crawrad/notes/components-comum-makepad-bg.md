---
id: components-comum-makepad-bg
kind: doc
target_file: src/components/comum.rs
line_hint: 37
author: 
created: 
title: "MAKEPAD_BG"
tags: []
signature: "pub const MAKEPAD_BG : [f32 ; 4] = rgb (0x1D , 0x1F , 0x20)"
---

The surface behind the form paper.

The surface behind the form paper. Mirrors RXP's `SUPERFICIE` and unifies with CRAWRAD_PAINEL.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_BG;

assert_eq!(MAKEPAD_BG[3], 1.0);
```
