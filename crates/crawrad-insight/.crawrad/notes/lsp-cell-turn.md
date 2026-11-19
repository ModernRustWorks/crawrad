---
id: lsp-cell-turn
kind: doc
target_file: src/lsp_cell.rs
line_hint: 76
author: 
created: 
title: "turn"
tags: []
signature: "pub fn turn (seconds : f64) -> f64"
---

One revolution every 0.9 s: the arc's turn (0..1) at `seconds`.

# Examples

```rust
use crawrad_insight::lsp_cell::turn;

assert_eq!(turn(0.0), 0.0);
```
