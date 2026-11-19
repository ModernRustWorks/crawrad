---
id: canvas-cursor-for
kind: doc
target_file: src/canvas.rs
line_hint: 692
author: 
created: 
title: "cursor_for"
tags: []
signature: "pub fn cursor_for (selecao : Option < RectIr > , formulario : RectIr , x : f32 , y : f32) -> MouseCursor"
---

The cursor requested by a point (in design units).

The order mirrors that of `Designer::pointer_down` (`crawrad-designer/src/lib.rs:556`):
selected component handle first, form handle second. If both
fall on the same point — and they do, for every control touching the edge —
the component's wins, just like there.

# Examples

```rust
use makepad_widgets::MouseCursor;
use crawrad_ir::Rect as RectIr;
use crawrad_runtime_makepad::canvas::cursor_for;

let cursor = cursor_for(None, RectIr::new(0.0, 0.0, 640.0, 480.0), 10.0, 10.0);
assert_eq!(cursor, MouseCursor::Default);
```
