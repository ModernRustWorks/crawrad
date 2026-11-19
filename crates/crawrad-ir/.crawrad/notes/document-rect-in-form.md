---
id: document-rect-in-form
kind: doc
target_file: src/lib.rs
line_hint: 478
author: 
created: 
title: "Document::rect_in_form"
tags: []
signature: "pub fn rect_in_form (& self , id : & str) -> Option < Rect >"
---

The node's rectangle in form coordinates.

# Examples

```rust
use crawrad_ir::{Document, Node, Rect};

let mut doc = Document::new("MainForm", "Form");
let mut btn = Node::new("btn", "Button");
btn.set_rect(Rect::new(10.0, 20.0, 100.0, 30.0));
doc.add_child("MainForm", btn).unwrap();
assert_eq!(doc.rect_in_form("btn"), Some(Rect::new(10.0, 20.0, 100.0, 30.0)));
```
