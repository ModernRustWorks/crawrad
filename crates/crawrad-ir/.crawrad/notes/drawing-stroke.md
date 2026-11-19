---
id: drawing-stroke
kind: doc
target_file: src/drawing.rs
line_hint: 179
author: 
created: 
title: "Stroke"
tags: []
signature: "pub enum Stroke"
---

**WHAT is painted.** `Brush` turned into data.

Five, and not twelve: they are what a declared component needs today, and each
one that enters later enters because a component requested it. A vocabulary that
is born large is a list.

# Examples

```rust
use crawrad_ir::drawing::{Ink, Stroke};

let stroke = Stroke::Frame { color: Ink::Border, thickness: 1.0 };
assert!(matches!(stroke, Stroke::Frame { .. }));
```
