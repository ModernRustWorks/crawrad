---
id: designer-static-guides
kind: doc
target_file: src/lib.rs
line_hint: 1288
author: 
created: 
title: "Designer::static_guides"
tags: []
signature: "pub fn static_guides (& self , doc : & Document , sel : & [NodeId]) -> Vec < Guide >"
---

Alignments that **already exist** between the selected item and neighbors.

This is what the classic IDE shows when holding Ctrl: without dragging anything, the
strokes reveal who the component is aligned with. The line here
crosses both — in the gap, three coincident relations turn into an
unreadable clump.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.static_guides(&doc, &[]).is_empty());
```
