---
id: designer-visual
kind: doc
target_file: src/lib.rs
line_hint: 538
author: 
created: 
title: "Designer::visual"
tags: []
signature: "pub fn visual (& self , doc : & Document , id : & NodeId) -> bool"
---

Returns whether the component draws something on screen.

Does the component draw something on screen, or is it non-visual (ActionList,
ImageList)? Without a catalog, everything is visual — which is what the designer assumed
before non-visual components existed.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::{Document, NodeId};

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.visual(&doc, &NodeId::from("form")));
```
