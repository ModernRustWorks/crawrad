---
id: layout-is-fixed
kind: doc
target_file: src/layout.rs
line_hint: 294
author: 
created: 
title: "is_fixed"
tags: []
signature: "pub fn is_fixed (doc : & Document , id : & str) -> bool"
---

Returns whether the component is fixed by `Align`.

Is the component fixed by `Align`?

# Examples

```rust
use crawrad_designer::layout::is_fixed;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(!is_fixed(&doc, "btn"));
```
