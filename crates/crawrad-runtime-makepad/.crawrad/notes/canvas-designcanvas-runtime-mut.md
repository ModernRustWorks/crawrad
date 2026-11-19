---
id: canvas-designcanvas-runtime-mut
kind: doc
target_file: src/canvas.rs
line_hint: 835
author: 
created: 
title: "DesignCanvas::runtime_mut"
tags: []
signature: "pub fn runtime_mut (& mut self) -> & mut MakepadRuntime"
---

O runtime, para mandar (`open`/`update`/`set_*`) e recolher (`drain_events`).

# Examples

```rust,no_run
// DesignCanvas is instantiated by the Makepad DSL runtime.
use crawrad_runtime_makepad::DesignCanvas;

fn check(canvas: &mut DesignCanvas) {
    let _rt = canvas.runtime_mut();
}
```
