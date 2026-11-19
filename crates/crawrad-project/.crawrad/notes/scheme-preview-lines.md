---
id: scheme-preview-lines
kind: doc
target_file: src/scheme_preview.rs
line_hint: 56
author: 
created: 
title: "lines"
tags: []
signature: "pub fn lines () -> Vec < PreviewLine >"
---

Returns the six-line code sample lines for the color scheme preview.

# Examples

```rust
use crawrad_project::scheme_preview::lines;

let preview = lines();
assert_eq!(preview.len(), 6);
```
