---
id: scheme-preview-previewline
kind: doc
target_file: src/scheme_preview.rs
line_hint: 45
author: 
created: 
title: "PreviewLine"
tags: []
signature: "pub struct PreviewLine"
---

A line in the color scheme preview code sample.

# Examples

```rust
use crawrad_project::scheme_preview::{LineKind, PreviewLine};

let line = PreviewLine {
    number: 1,
    kind: LineKind::Normal,
    spans: Vec::new(),
};
assert_eq!(line.number, 1);
```
