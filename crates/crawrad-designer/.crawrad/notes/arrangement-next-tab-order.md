---
id: arrangement-next-tab-order
kind: doc
target_file: src/arrangement.rs
line_hint: 381
author: 
created: 
title: "next_tab_order"
tags: []
signature: "pub fn next_tab_order (doc : & Document) -> i64"
---

Returns the next free number in the tab order sequence.

The next free number in the sequence — this is what the classic IDE gives to a
newly-created component, instead of leaving everyone with zero.

# Examples

```rust
use crawrad_designer::arrangement::next_tab_order;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert_eq!(next_tab_order(&doc), 0);
```
