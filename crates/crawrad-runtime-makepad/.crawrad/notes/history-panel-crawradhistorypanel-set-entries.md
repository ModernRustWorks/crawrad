---
id: history-panel-crawradhistorypanel-set-entries
kind: doc
target_file: src/history_panel.rs
line_hint: 534
author: 
created: 
title: "CrawradHistoryPanel::set_entries"
tags: []
signature: "pub fn set_entries (& mut self , cx : & mut Cx , entries : & [HistoryEntryView])"
---

Updates the history entry list.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn clear_entries(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_entries(cx, &[]);
}
```
