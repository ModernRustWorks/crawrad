---
id: scheme-preview-painted
kind: doc
target_file: src/scheme_preview.rs
line_hint: 196
author: 
created: 
title: "painted"
tags: []
signature: "pub fn painted (prefs : & crate :: workspace :: Preferences) -> Vec < PaintedLine >"
---

Resolves the preview lines with colors and decorations.

Resolves the preview lines with line number, number color, band, and colored spans.

# Examples

```rust
use crawrad_project::scheme_preview::painted;
use crawrad_project::workspace::Preferences;

let prefs = Preferences::default();
let list = painted(&prefs);
assert_eq!(list.len(), 6);
```
