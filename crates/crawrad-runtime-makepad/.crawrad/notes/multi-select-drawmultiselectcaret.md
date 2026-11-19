---
id: multi-select-drawmultiselectcaret
kind: doc
target_file: src/multi_select.rs
line_hint: 93
author: 
created: 
title: "DrawMultiSelectCaret"
tags: []
signature: "pub struct DrawMultiSelectCaret"
---

The shader of the field's "▾": a filled triangle in `color`.

# Examples

```rust,no_run
// Makepad shader structures require the Makepad application runtime.
use crawrad_runtime_makepad::multi_select::DrawMultiSelectCaret;

fn check_caret(caret: &DrawMultiSelectCaret) {
    let _c = caret.color;
}
```
