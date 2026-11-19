---
id: arrangement-align
kind: doc
target_file: src/arrangement.rs
line_hint: 68
author: 
created: 
title: "align"
tags: []
signature: "pub fn align (doc : & Document , sel : & [NodeId] , mode : AlignMode , reference : Reference ,) -> Option < Transaction >"
---

Aligns the selection.

Aligns the selection. Requires two or more components (or the form
as reference).

# Examples

```rust
use crawrad_designer::arrangement::{align, AlignMode, Reference};
use crawrad_ir::{Document, Node, NodeId, Rect};

let mut doc = Document::new("form", "Form");
let mut n1 = Node::new("a", "Button");
n1.set_rect(Rect::new(10.0, 10.0, 100.0, 20.0));
let _ = doc.add_child("form", n1);
let mut n2 = Node::new("b", "Button");
n2.set_rect(Rect::new(50.0, 60.0, 60.0, 40.0));
let _ = doc.add_child("form", n2);

let sel = vec![NodeId::from("a"), NodeId::from("b")];
let tx = align(&doc, &sel, AlignMode::Left, Reference::First);
assert!(tx.is_some());
```
