---
id: hostmodel-set-previous-rect
kind: doc
target_file: src/lib.rs
line_hint: 348
author: 
created: 
title: "HostModel::set_previous_rect"
tags: []
signature: "pub fn set_previous_rect (& mut self, rect : Option < crawrad_ir :: Rect >)"
---

Records the rectangle the front form had before this pass.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;
use crawrad_ir::{Document, Rect};

let mut host = HostModel::new();
host.open_form(Doc::new(Document::new("form_main", "Form")), None);
host.set_previous_rect(Some(Rect::new(0.0, 0.0, 640.0, 480.0)));
assert!(host.previous_rect().is_some());
```
