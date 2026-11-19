---
id: designer-parent-for-drop
kind: doc
target_file: src/lib.rs
line_hint: 1316
author: 
created: 
title: "Designer::parent_for_drop"
tags: []
signature: "pub fn parent_for_drop (& self , doc : & Document , x : f32 , y : f32) -> (NodeId , f32 , f32)"
---

Finds the container and relative coordinates to drop a new component.

Where to drop a new component coming from the palette, already snapped to grid.
Which container receives a component dropped at point `(x, y)`.

Returns the **parent id** and the position already **relative to it**. The point is
that of the component's corner, and input coordinate is form-relative.

Who can receive children is not a question for the user: it is
`ComponentDesc::area_para_filhos`, analogue of `csAcceptsControls` in the
classic component library — and the question is of the AREA, not the tree: `ActionList` accepts
children and has no surface, and while the two were a single flag,
dropping a component over its icon made the component vanish. And
the **innermost** wins — with panel inside panel, the inner one.
Nesting ties are broken by Z-order, the one on top.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::{Document, NodeId};

let d = Designer::default();
let doc = Document::new("form", "Form");
let (parent, x, y) = d.parent_for_drop(&doc, 20.0, 20.0);
assert_eq!(parent, NodeId::from("form"));
```
