---
id: canvas-designcanvas-transformacao
kind: doc
target_file: src/canvas.rs
line_hint: 840
author: 
created: 
title: "DesignCanvas::transformacao"
tags: []
signature: "pub fn transformacao (& self) -> Transformacao"
---

The canvas transformation — **the only one** in the adapter (ADR 0019).

# Examples

```rust,no_run
// DesignCanvas is instantiated by the Makepad DSL runtime.
use crawrad_runtime_makepad::DesignCanvas;

fn check(canvas: &DesignCanvas) {
    let _t = canvas.transformacao();
}
```
