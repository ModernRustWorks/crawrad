---
id: canvas-designcanvas-cursor-em
kind: doc
target_file: src/canvas.rs
line_hint: 845
author: 
created: 
title: "DesignCanvas::cursor_em"
tags: []
signature: "pub fn cursor_em (& self , x : f32 , y : f32) -> MouseCursor"
---

The cursor requested by the point (in design units).

The order mirrors that of `Designer::pointer_down`: selected component
handle first, form handle second. Other than that, the default
arrow — moving is what the designer decides, and it is not consulted here.

# Examples

```rust,no_run
// DesignCanvas is instantiated by the Makepad DSL runtime.
use crawrad_runtime_makepad::DesignCanvas;

fn check(canvas: &DesignCanvas) {
    let _cursor = canvas.cursor_em(10.0, 10.0);
}
```
