---
id: components-comum-makepad-grid
kind: doc
target_file: src/components/comum.rs
line_hint: 43
author: 
created: 
title: "MAKEPAD_GRID"
tags: []
signature: "pub const MAKEPAD_GRID : [f32 ; 4] = rgb (0x7A , 0x7F , 0x9A)"
---

The grid dots. Mirrors RXP's `PONTO_GRADE`.

RXP measures 1.9:1 against its paper and that suffices on light paper, where the
dot is DARKER than the background and the eye reads it as drawing board texture.
On dark paper the dot is lighter than the background, and at 2.1:1 (which was the
previous `#585b70`) it simply vanishes — the owner saw it and said *"the grid
almost vanishes"*. From here comes 3.8:1, which brings the drawing board back.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_GRID;

assert_eq!(MAKEPAD_GRID[3], 1.0);
```
