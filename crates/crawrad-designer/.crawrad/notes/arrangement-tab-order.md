---
id: arrangement-tab-order
kind: doc
target_file: src/arrangement.rs
line_hint: 355
author: 
created: 
title: "tab_order"
tags: []
signature: "pub fn tab_order (doc : & Document) -> Vec < (NodeId , i64) >"
---

The current tab order, in the sequence in which Tab will traverse.

# Examples

```rust
use crawrad_designer::arrangement::tab_order;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(tab_order(&doc).is_empty());
```
