---
id: document-parent-origin
kind: doc
target_file: src/lib.rs
line_hint: 462
author: 
created: 
title: "Document::parent_origin"
tags: []
signature: "pub fn parent_origin (& self , id : & str) -> (f32 , f32)"
---

How much to add to a node's `rect` to reach the FORM
coordinate.

A component's `rect` is **relative to the parent**, as in the classic RAD IDE
(ADR 0012).
For a direct child of the form this is zero, which is why the rest of the
system worked until now without knowing that the distinction existed.

The root is not included in the sum: its origin is the corner of the form, and whoever
puts the form on the screen is the adapter.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
doc.add_child("MainForm", Node::new("btn", "Button")).unwrap();
assert_eq!(doc.parent_origin("btn"), (0.0, 0.0));
```
