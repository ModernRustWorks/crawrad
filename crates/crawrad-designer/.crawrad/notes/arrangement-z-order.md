---
id: arrangement-z-order
kind: doc
target_file: src/arrangement.rs
line_hint: 263
author: 
created: 
title: "z_order"
tags: []
signature: "pub fn z_order (doc : & Document , sel : & [NodeId] , mode : ZOrder) -> Option < Transaction >"
---

Moves the selection in Z-order.

Moves the selection in Z-order. The last sibling draws on top, as in the
classic IDE.

The order is **per parent** (ADR 0012),
and this was read directly in the classic component library source: `SetZOrderPosition` operates on
`FParent.FControls`, never on a global list. A child of a panel brought
forward rises among its SIBLINGS — it never goes in front of an uncle. What
can go in front of an uncle is the panel itself.

When the selection is spread across multiple parents, each parent receives its own
reordering in the SAME transaction: a single Ctrl+Z undoes everything.

# Examples

```rust
use crawrad_designer::arrangement::{z_order, ZOrder};
use crawrad_ir::{Document, Node, NodeId};

let mut doc = Document::new("form", "Form");
let _ = doc.add_child("form", Node::new("a", "Button"));
let _ = doc.add_child("form", Node::new("b", "Button"));

let sel = vec![NodeId::from("a")];
let tx = z_order(&doc, &sel, ZOrder::Front);
assert!(tx.is_some());
```
