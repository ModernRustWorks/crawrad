---
id: document-ancestors
kind: doc
target_file: src/lib.rs
line_hint: 444
author: 
created: 
title: "Document::ancestors"
tags: []
signature: "pub fn ancestors (& self , id : & str) -> Vec < NodeId >"
---

The chain of ancestors, from immediate parent up to the root.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
doc.add_child("MainForm", Node::new("btn", "Button")).unwrap();
assert_eq!(doc.ancestors("btn"), vec!["MainForm".to_string()]);
```
