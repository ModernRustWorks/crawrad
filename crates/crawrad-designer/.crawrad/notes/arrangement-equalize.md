---
id: arrangement-equalize
kind: doc
target_file: src/arrangement.rs
line_hint: 177
author: 
created: 
title: "equalize"
tags: []
signature: "pub fn equalize (doc : & Document , sel : & [NodeId] , mode : Equalize) -> Option < Transaction >"
---

Matches the size of the selection to that of the first selected component.

# Examples

```rust
use crawrad_designer::arrangement::{equalize, Equalize};
use crawrad_ir::{Document, Node, NodeId, Rect};

let mut doc = Document::new("form", "Form");
let mut n1 = Node::new("a", "Button");
n1.set_rect(Rect::new(10.0, 10.0, 100.0, 20.0));
let _ = doc.add_child("form", n1);
let mut n2 = Node::new("b", "Button");
n2.set_rect(Rect::new(50.0, 60.0, 60.0, 40.0));
let _ = doc.add_child("form", n2);

let sel = vec![NodeId::from("a"), NodeId::from("b")];
let tx = equalize(&doc, &sel, Equalize::Width);
assert!(tx.is_some());
```
