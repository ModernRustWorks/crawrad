---
id: scheme-preview-previewrow
kind: doc
target_file: src/scheme_preview.rs
line_hint: 28
author: 
created: 
title: "PreviewRow"
tags: []
signature: "pub struct PreviewRow"
---

One row of the color scheme preview.

# Examples

```rust
use crawrad_runtime_makepad::scheme_preview::PreviewRow;

let row = PreviewRow::new("1", [0.5, 0.5, 0.5, 1.0], None, Vec::new());
assert_eq!(row.number, "1");
```
