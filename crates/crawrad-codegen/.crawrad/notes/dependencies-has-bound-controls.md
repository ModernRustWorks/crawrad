---
id: dependencies-has-bound-controls
kind: doc
target_file: src/dependencies.rs
line_hint: 192
author: 
created: 
title: "has_bound_controls"
tags: []
signature: "pub fn has_bound_controls (doc : & crawrad_ir :: Document) -> bool"
---

Checks whether any control or action on the form is bound to a dataset.

Whether an `Edit` or a `CheckBox` on the form is bound to a `Dataset`, or an `Action` operates on one (#788).

# Examples

```rust
use crawrad_codegen::has_bound_controls;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert!(!has_bound_controls(&doc));
```
