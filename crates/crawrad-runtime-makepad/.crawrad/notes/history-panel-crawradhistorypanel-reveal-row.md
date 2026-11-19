---
id: history-panel-crawradhistorypanel-reveal-row
kind: doc
target_file: src/history_panel.rs
line_hint: 603
author: 
created: 
title: "CrawradHistoryPanel::reveal_row"
tags: []
signature: "pub fn reveal_row (& mut self , cx : & mut Cx , row : usize)"
---

Scrolls the diff view to row.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn scroll(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.reveal_row(cx, 0);
}
```
