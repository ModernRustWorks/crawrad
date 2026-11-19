---
id: layout-apply
kind: doc
target_file: src/layout.rs
line_hint: 171
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (doc : & Document) -> Option < Transaction >"
---

Recalculates positions of all components with `Align`.

Recalculates the position of every component that has `Align`, and returns the
transaction with what changed. `None` when everything is already in place.

# Examples

```rust
use crawrad_designer::layout::apply;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(apply(&doc).is_none());
```
