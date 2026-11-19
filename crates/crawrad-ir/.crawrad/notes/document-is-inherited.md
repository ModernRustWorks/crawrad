---
id: document-is-inherited
kind: doc
target_file: src/lib.rs
line_hint: 208
author: 
created: 
title: "Document::is_inherited"
tags: []
signature: "pub fn is_inherited (& self , node_id : & str , base : & Document) -> bool"
---

Reports whether a given node originated in the ancestor form `base`.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut base = Document::new("BaseForm", "Form");
base.add_child("BaseForm", Node::new("btn", "Button")).unwrap();
let derived = Document::new_inherited("DerivedForm", "Form", "BaseForm");
assert!(derived.is_inherited("btn", &base));
```
