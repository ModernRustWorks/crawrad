---
id: document-node-by-name
kind: doc
target_file: src/lib.rs
line_hint: 346
author: 
created: 
title: "Document::node_by_name"
tags: []
signature: "pub fn node_by_name (& self , name : & str) -> Option < & Node >"
---

The node named this.

It is the search the UI performs — the user
knows names, not ids.

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert!(doc.node_by_name("MainForm").is_some());
```
