---
id: designer-alcas-handle-at-point-in-form
kind: doc
target_file: src/designer/alcas.rs
line_hint: 42
author: 
created: 
title: "handle_at_point_in_form"
tags: []
signature: "pub fn handle_at_point_in_form (r : Rect , x : f32 , y : f32) -> Option < Axis >"
---

The same, for the form — which has THREE handles, not eight.

Mirrors `Designer::hit_form_handle` (`crawrad-designer/src/lib.rs:494-500`),
which only looks at `E`, `S`, and `SE`. While the form got all eight, five
invisible handles caught the click.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::designer::handle_at_point_in_form;

let r = Rect::new(0.0, 0.0, 200.0, 200.0);
let axis = handle_at_point_in_form(r, 200.0, 200.0);
assert!(axis.is_some());
```
