---
id: document-walk
kind: doc
target_file: src/lib.rs
line_hint: 391
author: 
created: 
title: "Document::walk"
tags: []
signature: "pub fn walk (& self) -> Vec < & Node >"
---

Traverses the tree in depth-first order starting from the root, in z-order.

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let nodes = doc.walk();
assert_eq!(nodes.len(), 1);
assert_eq!(nodes[0].id, "MainForm");
```
