---
id: scheme-preview-paintedline
kind: doc
target_file: src/scheme_preview.rs
line_hint: 183
author: 
created: 
title: "PaintedLine"
tags: []
signature: "pub struct PaintedLine"
---

One preview line resolved to colours: what the widget draws, nothing to look up.

# Examples

```rust
use crawrad_project::scheme_preview::{GUTTER, PaintedLine};

let line = PaintedLine {
    number: "1".into(),
    number_color: GUTTER,
    band: None,
    spans: Vec::new(),
};
assert_eq!(line.number, "1");
```
