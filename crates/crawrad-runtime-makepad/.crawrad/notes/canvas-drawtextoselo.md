---
id: canvas-drawtextoselo
kind: doc
target_file: src/canvas.rs
line_hint: 458
author: 
created: 
title: "DrawTextoSelo"
tags: []
signature: "pub struct DrawTextoSelo"
---

The badge text of a non-visual component.

The badge text — initials and name. It derives from `DrawText` only to get
its own draw call, above that of component text.

# Examples

```rust,no_run
// Makepad draw structures are instantiated and managed by the Makepad runtime.
use crawrad_runtime_makepad::canvas::DrawTextoSelo;

fn check_type(_draw: &DrawTextoSelo) {}
```
