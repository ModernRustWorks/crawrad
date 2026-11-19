---
id: ide-app-view-state-ideapp-select-node
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 58
author: 
created: 
title: "IdeApp::select_node"
tags: []
signature: "pub fn select_node (& mut self , cx : & mut Cx , id : & str)"
---

Selects a component and updates the canvas, the tree and the status bar.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.select_node(cx, "btn_ok");
assert!(app.status.is_none());
```

## Inside `select_node`: `canvas.redraw(cx);`

AND THE CANVAS, EXPLICITLY — the `self.ui.redraw` above DOES NOT reach
down to it, for the same reason as `bombear` and `reabrir_no_canvas`.

## Inside `select_node`: `self.status = None;`

For the same reason as `bombear`: choosing another node concludes the subject
of the previous message, and the bar returns to displaying the open file.
