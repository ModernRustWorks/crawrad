---
id: scheme-preview-drawpreviewrect
kind: doc
target_file: src/scheme_preview.rs
line_hint: 58
author: 
created: 
title: "DrawPreviewRect"
tags: []
signature: "pub struct DrawPreviewRect"
---

Draw shader for filled background and row band rectangles.

# Examples

```rust,no_run
// Makepad shader structures require the Makepad application runtime.
use crawrad_runtime_makepad::scheme_preview::DrawPreviewRect;

fn check_shader(rect: &DrawPreviewRect) {
    let _c = rect.color;
}
```
