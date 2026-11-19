---
id: history-panel-crawradhistorypanel-drawn-controls
kind: doc
target_file: src/history_panel.rs
line_hint: 623
author: 
created: 
title: "CrawradHistoryPanel::drawn_controls"
tags: []
signature: "pub fn drawn_controls (& self , cx : & Cx) -> Vec < (String , Rect) >"
---

Returns button and copy row rects for MCP tools.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn get_controls(cx: &Cx, panel: &CrawradHistoryPanel) {
    let _rects = panel.drawn_controls(cx);
}
```
