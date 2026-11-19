---
id: designer-hit
kind: doc
target_file: src/lib.rs
line_hint: 370
author: 
created: 
title: "Designer::hit"
tags: []
signature: "pub fn hit (& self , doc : & Document , x : f32 , y : f32) -> Option < NodeId >"
---

Returns the component under the point, respecting Z-order.

The component under the point, respecting Z-order (top one wins).
Does not consider the form — for that [`Designer::hit_form`] exists.

Traverses the TREE, not only the root's children: with containers, clicking
inside a panel must grab the button, not the panel. It was once
wrong, and the symptom is cruel — the component appears on screen and cannot
be selected.

`in_paint_order` returns parent before children and siblings in Z-order,
so **the last one containing the point is the top one** — and the child, which was
drawn after the parent, wins over it. It is the same drawing rule, read
in reverse.

**A non-visual wins over everything.** Its badge is drawn in front of
every control (the owner's rule, see the canvas brush), so the click
reads it first: dropped on top of a panel, the badge was on screen and
the click grabbed the panel under it.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.hit(&doc, 50.0, 50.0).is_none());
```
