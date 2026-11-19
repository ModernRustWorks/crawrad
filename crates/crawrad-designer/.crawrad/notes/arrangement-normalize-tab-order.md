---
id: arrangement-normalize-tab-order
kind: doc
target_file: src/arrangement.rs
line_hint: 391
author: 
created: 
title: "normalize_tab_order"
tags: []
signature: "pub fn normalize_tab_order (doc : & Document) -> Option < Transaction >"
---

Renumbers the tab sequence from 0 to n-1 in current order.

Renumbers the tab sequence from 0 to n-1 in current order, eliminating gaps and ties
that appear after deleting or pasting components.

# Examples

```rust
use crawrad_designer::arrangement::normalize_tab_order;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(normalize_tab_order(&doc).is_none());
```
