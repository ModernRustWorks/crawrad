---
id: document-parent-of
kind: doc
target_file: src/lib.rs
line_hint: 433
author: 
created: 
title: "Document::parent_of"
tags: []
signature: "pub fn parent_of (& self , id : & str) -> Option < & NodeId >"
---

The parent of a node, or `None` for the root (and for nonexistent ids).

The tree is top-down — the node stores children, not the parent —,
so this is a linear scan. In a form this is dozens of nodes, not
thousands; storing the parent on the node would create a second source of truth that
`validate` would have to reconcile, and there have already been files with a node under two parents.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
doc.add_child("MainForm", Node::new("btn", "Button")).unwrap();
assert_eq!(doc.parent_of("btn"), Some(&"MainForm".to_string()));
assert_eq!(doc.parent_of("MainForm"), None);
```
