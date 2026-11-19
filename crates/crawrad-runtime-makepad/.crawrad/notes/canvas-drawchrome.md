---
id: canvas-drawchrome
kind: doc
target_file: src/canvas.rs
line_hint: 430
author: 
created: 
title: "DrawChrome"
tags: []
signature: "pub struct DrawChrome"
---

A container's chrome box drawn above the children.

A container's CHROME box — track, button, and thumb of the bar it
places over the children.

Same shape as [`DrawCaixa`]; what changes is the LAYER, and that is the only reason
it exists. See the note in the DSL.

# Examples

```rust,no_run
// Makepad draw structures are instantiated and managed by the Makepad runtime.
use crawrad_runtime_makepad::canvas::DrawChrome;

fn check_type(_draw: &DrawChrome) {}
```
