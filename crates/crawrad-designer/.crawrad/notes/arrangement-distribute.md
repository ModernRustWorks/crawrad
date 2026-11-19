---
id: arrangement-distribute
kind: doc
target_file: src/arrangement.rs
line_hint: 114
author: 
created: 
title: "distribute"
tags: []
signature: "pub fn distribute (doc : & Document , sel : & [NodeId] , mode : Distribute) -> Option < Transaction >"
---

Spaces the selection equally between the first and the last.

Spaces the selection equally between the first and the last, keeping the endpoints
in place. Requires three or more.

# Examples

```rust
use crawrad_designer::arrangement::{distribute, Distribute};
use crawrad_ir::{Document, Node, NodeId, Rect};

let mut doc = Document::new("form", "Form");
for (id, x) in [("a", 10.0), ("b", 30.0), ("c", 100.0)] {
    let mut n = Node::new(id, "Button");
    n.set_rect(Rect::new(x, 10.0, 20.0, 20.0));
    let _ = doc.add_child("form", n);
}

let sel = vec![NodeId::from("a"), NodeId::from("b"), NodeId::from("c")];
let tx = distribute(&doc, &sel, Distribute::Horizontal);
assert!(tx.is_some());
```
