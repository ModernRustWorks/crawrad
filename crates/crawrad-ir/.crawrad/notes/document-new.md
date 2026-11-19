---
id: document-new
kind: doc
target_file: src/lib.rs
line_hint: 172
author: 
created: 
title: "Document::new"
tags: []
signature: "pub fn new (root_id : impl Into < NodeId > , root_class : impl Into < String >) -> Self"
---

New document with a root node of the specified class.

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert_eq!(doc.root, "MainForm");
assert!(doc.node("MainForm").is_some());
```
