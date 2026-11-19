---
id: arrangement-move-tab-order
kind: doc
target_file: src/arrangement.rs
line_hint: 408
author: 
created: 
title: "move_tab_order"
tags: []
signature: "pub fn move_tab_order (doc : & Document , id : & NodeId , up : bool) -> Option < Transaction >"
---

Moves a component in the tab sequence, swapping with its neighbor.

# Examples

```rust
use crawrad_designer::arrangement::move_tab_order;
use crawrad_ir::{Document, NodeId};

let doc = Document::new("form", "Form");
assert!(move_tab_order(&doc, &NodeId::from("a"), true).is_none());
```
