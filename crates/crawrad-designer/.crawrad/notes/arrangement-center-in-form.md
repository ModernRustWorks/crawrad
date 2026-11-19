---
id: arrangement-center-in-form
kind: doc
target_file: src/arrangement.rs
line_hint: 206
author: 
created: 
title: "center_in_form"
tags: []
signature: "pub fn center_in_form (doc : & Document , sel : & [NodeId] , mode : AlignMode) -> Option < Transaction >"
---

Centers the selection in the form.

Centers the selection in the form, without changing the spacing between
components: moves the entire set.

# Examples

```rust
use crawrad_designer::arrangement::{center_in_form, AlignMode};
use crawrad_ir::{Document, Node, NodeId, Rect};

let mut doc = Document::new("form", "Form");
doc.node_mut("form").unwrap().set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
let mut n = Node::new("a", "Button");
n.set_rect(Rect::new(10.0, 10.0, 100.0, 20.0));
let _ = doc.add_child("form", n);

let sel = vec![NodeId::from("a")];
let tx = center_in_form(&doc, &sel, AlignMode::CenterHorizontal);
assert!(tx.is_some());
```
