---
id: history-panel-crawradhistorypanelref-drawn-controls
kind: doc
target_file: src/history_panel.rs
line_hint: 788
author: 
created: 
title: "CrawradHistoryPanelRef::drawn_controls"
tags: []
signature: "pub fn drawn_controls (& self , cx : & Cx) -> Vec < (String , Rect) >"
---

Returns drawn controls for MCP.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn check(cx: &Cx, r: &CrawradHistoryPanelRef) {
    let _rects = r.drawn_controls(cx);
}
```
