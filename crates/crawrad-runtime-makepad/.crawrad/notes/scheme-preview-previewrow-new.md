---
id: scheme-preview-previewrow-new
kind: doc
target_file: src/scheme_preview.rs
line_hint: 42
author: 
created: 
title: "PreviewRow::new"
tags: []
signature: "pub fn new (number : impl Into < String > , number_color : [f32 ; 4] , band : Option < [f32 ; 4] > , spans : Vec < (String , [f32 ; 4]) > ,) -> Self"
---

Creates a new preview row.

# Examples

```rust
use crawrad_runtime_makepad::scheme_preview::PreviewRow;

let row = PreviewRow::new("1", [0.5, 0.5, 0.5, 1.0], None, Vec::new());
assert_eq!(row.number, "1");
```
