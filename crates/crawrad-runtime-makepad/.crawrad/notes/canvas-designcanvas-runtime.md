---
id: canvas-designcanvas-runtime
kind: doc
target_file: src/canvas.rs
line_hint: 830
author: 
created: 
title: "DesignCanvas::runtime"
tags: []
signature: "pub fn runtime (& self) -> & MakepadRuntime"
---

The runtime this canvas renders — the shell talks to it through here.

# Examples

```rust,no_run
// DesignCanvas is instantiated by the Makepad DSL runtime.
use crawrad_runtime_makepad::DesignCanvas;

fn check(canvas: &DesignCanvas) {
    let _rt = canvas.runtime();
}
```
