---
id: dependencies-has-bound-grid
kind: doc
target_file: src/dependencies.rs
line_hint: 184
author: 
created: 
title: "has_bound_grid"
tags: []
signature: "pub fn has_bound_grid (doc : & crawrad_ir :: Document) -> bool"
---

Whether the form has a `Grid` bound to a `Connection` (#442).

# Examples

```rust
use crawrad_codegen::has_bound_grid;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert!(!has_bound_grid(&doc));
```
