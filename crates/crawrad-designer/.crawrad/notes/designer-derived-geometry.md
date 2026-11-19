---
id: designer-derived-geometry
kind: doc
target_file: src/lib.rs
line_hint: 363
author: 
created: 
title: "Designer::derived_geometry"
tags: []
signature: "pub fn derived_geometry (& self , doc : & Document , id : & str) -> bool"
---

Returns whether node geometry is imposed by its container.

Is the geometry of this node imposed by the container, rather than chosen?

`TabSheet` is the case: it IS the client area of `PageControl`. Whoever
answers is the **catalog** — the `geometria_derivada` flag —, not a
list of classes written here.

Without this the interface lied in three ways: the page gained the eight
handles, dragging it generated a movement that normalization rewound in
the next turn (two undo entries per gesture, zero result), and
`Align` stayed editable fighting with normalization for the same
rectangle, never converging.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(!d.derived_geometry(&doc, "form"));
```
