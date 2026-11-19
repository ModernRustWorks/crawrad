---
id: document-subtree
kind: doc
target_file: src/lib.rs
line_hint: 406
author: 
created: 
title: "Document::subtree"
tags: []
signature: "pub fn subtree (& self , id : & str) -> Vec < NodeId >"
---

The node and all descendants, from parent to children.

Checks what the format cannot prevent on its own:
existing root, existing children, no cycles, no orphaned nodes, and the
**property contract** of ADR 0002.
The node and all descendants, from parent to children. Deletion traverses
this list from back to front: the removal command only accepts leaves.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
doc.add_child("MainForm", Node::new("btn", "Button")).unwrap();
assert_eq!(doc.subtree("MainForm"), vec!["MainForm".to_string(), "btn".to_string()]);
```
