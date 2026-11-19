---
id: layout-apply-anchors
kind: doc
target_file: src/layout.rs
line_hint: 366
author: 
created: 
title: "apply_anchors"
tags: []
signature: "pub fn apply_anchors (doc : & Document , dw : f32 , dh : f32) -> Option < Transaction >"
---

Readjusts components after the form has changed size.

Those with `Align` are handled by [`apply`]; others follow anchors.

# Examples

```rust
use crawrad_designer::layout::apply_anchors;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(apply_anchors(&doc, 0.0, 0.0).is_none());
```
