---
id: layout-anchoring-adjust
kind: doc
target_file: src/layout.rs
line_hint: 326
author: 
created: 
title: "Anchoring::adjust"
tags: []
signature: "fn adjust (self , r : Rect , dw : f32 , dh : f32) -> Rect"
---

How the rectangle reacts when the form changes size.

The calculation is that of the box ([`anchored`]); what this function adds is the
house `Rect` at both ends. **Without floor**, and the box doc explains
why a floor here would be destructive.

# Examples

```rust
use crawrad_designer::layout::Anchoring;
use crawrad_component::Anchors;
use crawrad_ir::Rect;

let r = Rect::new(10.0, 10.0, 100.0, 50.0);
let adjusted = Anchors::default().adjust(r, 20.0, 10.0);
assert_eq!(adjusted.w, 100.0);
```
