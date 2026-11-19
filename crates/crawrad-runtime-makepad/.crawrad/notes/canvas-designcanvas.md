---
id: canvas-designcanvas
kind: doc
target_file: src/canvas.rs
line_hint: 725
author: 
created: 
title: "DesignCanvas"
tags: []
signature: "pub struct DesignCanvas"
---

The CrawRAD design surface rendered by Makepad.

It **owns** the [`MakepadRuntime`]: the shell talks to the widget via the
[`crawrad_runtime::Runtime`] trait (`open`, `update`, `set_selection`, `set_marquee`,
`set_grid`, `set_guides`, `set_badges`) and collects gestures via
`drain_events()`. That is the entire contract — the widget knows nothing of
`crawrad-designer`.

# Examples

```rust,no_run
// DesignCanvas is a Makepad widget instantiated in the DSL.
use crawrad_runtime_makepad::DesignCanvas;

fn check_canvas(_canvas: &DesignCanvas) {}
```
