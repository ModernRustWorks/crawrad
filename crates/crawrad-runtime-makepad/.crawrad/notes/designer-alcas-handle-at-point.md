---
id: designer-alcas-handle-at-point
kind: doc
target_file: src/designer/alcas.rs
line_hint: 33
author: 
created: 
title: "handle_at_point"
tags: []
signature: "pub fn handle_at_point (r : Rect , x : f32 , y : f32) -> Option < Axis >"
---

The handle axis under the point, in design units, or `None`.

Serves the CURSOR, and nothing else: what a click on the handle does is decided by
`crawrad-designer`. What this file cannot do is disagree with it about WHERE the
handle is — which is why geometry comes from `crawrad_runtime::handles` and the target is
`HANDLE_TARGET`, exactly like `Designer::hit_handle`
(`crawrad-designer/src/lib.rs:526-530`, with `handle_size = HANDLE_TARGET / 2`).

The vector order is a contract among three places — `Handle::ALL`,
`crawrad_runtime::handles`, and [`EIXO_DAS_ALCAS`] up here.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::designer::handle_at_point;

let r = Rect::new(10.0, 10.0, 100.0, 100.0);
let axis = handle_at_point(r, 10.0, 10.0);
assert!(axis.is_some());
```
