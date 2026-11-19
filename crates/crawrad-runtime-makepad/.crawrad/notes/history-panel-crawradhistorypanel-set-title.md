---
id: history-panel-crawradhistorypanel-set-title
kind: doc
target_file: src/history_panel.rs
line_hint: 529
author: 
created: 
title: "CrawradHistoryPanel::set_title"
tags: []
signature: "pub fn set_title (& mut self , cx : & mut Cx , title : & str)"
---

Sets the header title label.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn update_title(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_title(cx, "History");
}
```
