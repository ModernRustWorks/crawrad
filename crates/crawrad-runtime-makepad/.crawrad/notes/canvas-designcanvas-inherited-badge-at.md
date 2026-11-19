---
id: canvas-designcanvas-inherited-badge-at
kind: doc
target_file: src/canvas.rs
line_hint: 1
author:
created:
title: "DesignCanvas::inherited_badge_at"
tags: []
signature: "pub fn inherited_badge_at (& self , at : DVec2) -> Option < NodeId >"
---

The inherited component whose corner badge is under a window point (#851).

The IDE asks it as the pointer moves, to show "Inherited from <base>" over
the badge. The topmost badge wins, as the topmost component wins a click.

# Examples

```rust,no_run
// DesignCanvas is instantiated by the Makepad DSL runtime.
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::DesignCanvas;

fn check(canvas: &DesignCanvas) {
    let _under = canvas.inherited_badge_at(dvec2(10.0, 10.0));
}
```
