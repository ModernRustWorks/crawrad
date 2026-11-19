---
id: components-comum-makepad-paper
kind: doc
target_file: src/components/comum.rs
line_hint: 40
author: 
created: 
title: "MAKEPAD_PAPER"
tags: []
signature: "pub const MAKEPAD_PAPER : [f32 ; 4] = rgb (0x25 , 0x25 , 0x38)"
---

The form paper.

The form paper. Mirrors RXP's `PAPEL` — and is the only color whose
VALUE diverges on purpose (light there, dark here); everything else in this
palette is derived from this paper.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_PAPER;

assert_eq!(MAKEPAD_PAPER[3], 1.0);
```
