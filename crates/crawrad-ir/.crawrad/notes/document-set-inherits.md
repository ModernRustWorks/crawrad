---
id: document-set-inherits
kind: doc
target_file: src/lib.rs
line_hint: 203
author: 
created: 
title: "Document::set_inherits"
tags: []
signature: "pub fn set_inherits (& mut self , base : Option < impl Into < String > >)"
---

Sets or clears the visual inheritance ancestor form.

# Examples

```rust
use crawrad_ir::Document;

let mut doc = Document::new("DerivedForm", "Form");
doc.set_inherits(Some("BaseForm"));
assert_eq!(doc.inherits(), Some("BaseForm"));
```
