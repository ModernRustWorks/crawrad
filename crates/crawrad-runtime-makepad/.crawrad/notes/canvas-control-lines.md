---
id: canvas-control-lines
kind: doc
target_file: src/canvas.rs
line_hint: 511
author: 
created: 
title: "control_lines"
tags: []
signature: "pub fn control_lines (c : & MakepadCtl) -> Vec < ContentLine >"
---

WHERE a control's line content comes from.

The answer belongs to the **component**, not the canvas: `Memo` reads `text`, `ListBox`
reads the collection, and `TreeView` flattens the tree, each in its own file. This
function is just the gateway — and it exists because the question is measurable without a window.

# Examples

```rust,no_run
// MakepadCtl instances are managed by MakepadRuntime.
use crawrad_runtime_makepad::canvas::control_lines;
use crawrad_runtime_makepad::MakepadCtl;

fn example(c: &MakepadCtl) {
    let _lines = control_lines(c);
}
```
