---
id: designer-pointer-up
kind: doc
target_file: src/lib.rs
line_hint: 871
author: 
created: 
title: "Designer::pointer_up"
tags: []
signature: "pub fn pointer_up (& mut self , doc : & Document) -> Outcome"
---

Concludes the current pointer gesture.

Concludes the gesture. When there was a marquee, this is where selection happens —
**by intersection**, as in the classic IDE: touching already selects.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let mut d = Designer::default();
let doc = Document::new("form", "Form");
let outcome = d.pointer_up(&doc);
assert!(!outcome.dragging);
```
