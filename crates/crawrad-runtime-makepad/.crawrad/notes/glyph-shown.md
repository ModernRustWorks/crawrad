---
id: glyph-shown
kind: doc
target_file: src/glyph.rs
line_hint: 16
author: 
created: 
title: "Shown"
tags: []
signature: "pub struct Shown"
---

Tracks the state and layout of a button glyph to avoid redundant redraws.

What one glyph shows now: the picture, and whether it is shown with how
many glyphs in it — so the sync, which runs on every pass, acts only on
a change.

# Examples

```rust
use crawrad_runtime_makepad::glyph::Shown;

let shown = Shown::default();
let _copy = shown;
```
