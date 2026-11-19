---
id: document-in-paint-order
kind: doc
target_file: src/lib.rs
line_hint: 485
author: 
created: 
title: "Document::in_paint_order"
tags: []
signature: "pub fn in_paint_order (& self) -> Vec < NodeId >"
---

All nodes in the order they are painted.

All nodes in the order they are painted: parent before children, and
siblings in document order — which is the Z-order **within the parent**.

The root is left out: it is the surface, not a child.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
doc.add_child("MainForm", Node::new("btn", "Button")).unwrap();
assert_eq!(doc.in_paint_order(), vec!["btn".to_string()]);
```
