---
id: designer-hit-form-handle
kind: doc
target_file: src/lib.rs
line_hint: 485
author: 
created: 
title: "Designer::hit_form_handle"
tags: []
signature: "pub fn hit_form_handle (& self , doc : & Document , x : f32 , y : f32) -> Option < Handle >"
---

Returns the form handle under the point, if any.

Handles of the form itself: right, bottom, and corner — the three that
the classic IDE shows to resize the form.

**Does not query selection, on purpose.** In the classic IDE you point to the
border of the form and drag; nobody selects the window before changing its
size. Who decides turn is [`Designer::pointer_down`], and there
this comes AFTER the selected component's handle — both land on the
same point for any control touching the border.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.hit_form_handle(&doc, 10.0, 10.0).is_none());
```
