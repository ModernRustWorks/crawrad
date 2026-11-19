---
id: document-remove-subtree
kind: doc
target_file: src/lib.rs
line_hint: 369
author: 
created: 
title: "Document::remove_subtree"
tags: []
signature: "pub fn remove_subtree (& mut self , id : & str) -> Vec < NodeId >"
---

Removes the node and all of its descendants. Returns the removed ids.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
doc.add_child("MainForm", Node::new("btn", "Button")).unwrap();
let removed = doc.remove_subtree("btn");
assert_eq!(removed, vec!["btn".to_string()]);
```
