---
id: document-new-inherited
kind: doc
target_file: src/lib.rs
line_hint: 187
author: 
created: 
title: "Document::new_inherited"
tags: []
signature: "pub fn new_inherited (root_id : impl Into < NodeId > , root_class : impl Into < String > , base_form : impl Into < String > ,) -> Self"
---

New document inherited from another ancestor form (ADR 0042).

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new_inherited("DerivedForm", "Form", "BaseForm");
assert_eq!(doc.inherits(), Some("BaseForm"));
```
