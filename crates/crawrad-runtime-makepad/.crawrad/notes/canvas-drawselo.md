---
id: canvas-drawselo
kind: doc
target_file: src/canvas.rs
line_hint: 414
author: 
created: 
title: "DrawSelo"
tags: []
signature: "pub struct DrawSelo"
---

The badge box of a non-visual component.

The badge box of the non-visual component. It is the same drawing as [`DrawCaixa`] — what
changes is the LAYER, and that is the only reason it exists.

# Examples

```rust,no_run
// Makepad draw structures are instantiated and managed by the Makepad runtime.
use crawrad_runtime_makepad::canvas::DrawSelo;

fn check_type(_draw: &DrawSelo) {}
```
