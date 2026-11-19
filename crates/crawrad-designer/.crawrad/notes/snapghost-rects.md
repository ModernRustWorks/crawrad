---
id: snapghost-rects
kind: doc
target_file: src/lib.rs
line_hint: 182
author: 
created: 
title: "SnapGhost::rects"
tags: []
signature: "pub fn rects (& self , doc : & Document) -> Option < (Rect , Rect) >"
---

Returns the landing rect and pointer rect in form coordinates.

Where the anchor lands and where the pointer would have put it, in
form coordinates, once the move is applied to `doc`.

# Examples

```rust
use crawrad_designer::SnapGhost;
use crawrad_ir::{Document, NodeId};

let ghost = SnapGhost { node: NodeId::from("btn"), dx: 2.0, dy: 3.0 };
let doc = Document::new("form", "Form");
assert!(ghost.rects(&doc).is_none());
```
