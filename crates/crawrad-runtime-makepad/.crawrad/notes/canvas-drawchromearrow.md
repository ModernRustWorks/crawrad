---
id: canvas-drawchromearrow
kind: doc
target_file: src/canvas.rs
line_hint: 446
author: 
created: 
title: "DrawChromeArrow"
tags: []
signature: "pub struct DrawChromeArrow"
---

A container's chrome arrow, one layer above the box.

A container's chrome arrow, one layer above its box — the same
relationship that [`DrawSeta`] has with [`DrawCaixa`].

# Examples

```rust,no_run
// Makepad draw structures are instantiated and managed by the Makepad runtime.
use crawrad_runtime_makepad::canvas::DrawChromeArrow;

fn check_type(_draw: &DrawChromeArrow) {}
```
