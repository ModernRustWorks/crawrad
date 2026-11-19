---
id: in-collapsed-category
kind: doc
target_file: src/lib.rs
line_hint: 243
author: 
created: 
title: "in_collapsed_category"
tags: []
signature: "pub fn in_collapsed_category (doc : & Document , id : & str) -> bool"
---

Checks if an ancestor `CategoryPanel` is collapsed.

Whether a `CategoryPanel` above `id` (never `id` itself) is collapsed
(#500): its children go off the surface with it.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime::in_collapsed_category;

let doc = Document::new("form", "Form");
assert!(!in_collapsed_category(&doc, "form"));
```
