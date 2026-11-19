---
id: designer-hit-handle
kind: doc
target_file: src/lib.rs
line_hint: 514
author: 
created: 
title: "Designer::hit_handle"
tags: []
signature: "pub fn hit_handle (& self , doc : & Document , sel : & [NodeId] , x : f32 , y : f32) -> Option < Handle >"
---

Handle under the point, if there is a selection.

**The form does not pass through here.** Its handles are THREE — E, S and SE
—, and whoever knows them is [`Designer::hit_form_handle`]. While the
selected form fell into this path, it gained EIGHT, although
the adapter draws three: the five invisible ones hit, and a click near
the top-left corner started a resize that the screen did not
announce anywhere.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.hit_handle(&doc, &[], 10.0, 10.0).is_none());
```
