---
id: designer-on-canvas
kind: doc
target_file: src/lib.rs
line_hint: 394
author: 
created: 
title: "Designer::on_canvas"
tags: []
signature: "pub fn on_canvas (& self , doc : & Document , id : & str) -> bool"
---

Returns whether the node appears in the design area.

Does the node appear in the design area?

The rule lives in the boundary (`crawrad_runtime::na_superficie`), not here,
because the adapter needs the SAME answer — when it existed on
both sides, two places had to change together and one always lagged behind.
What the designer contributes is its source of truth on what is
visual: the catalog.

Two things vanish here: what lives inside a non-visual (an
`Action` belongs to the `ActionList` editor) and what lives in a page
that is not active.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.on_canvas(&doc, "form"));
```
