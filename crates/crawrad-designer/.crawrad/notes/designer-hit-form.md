---
id: designer-hit-form
kind: doc
target_file: src/lib.rs
line_hint: 477
author: 
created: 
title: "Designer::hit_form"
tags: []
signature: "pub fn hit_form (& self , doc : & Document , x : f32 , y : f32) -> bool"
---

Returns whether the point fell inside the form.

Did the point fall inside the form? In the classic IDE, clicking the form background
**selects the form** — the Object Inspector switches to showing Form1.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(!d.hit_form(&doc, 10.0, 10.0));
```
