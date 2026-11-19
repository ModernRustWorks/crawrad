---
id: history-panel-crawradhistorypanelref-set-entries
kind: doc
target_file: src/history_panel.rs
line_hint: 733
author: 
created: 
title: "CrawradHistoryPanelRef::set_entries"
tags: []
signature: "pub fn set_entries (& self , cx : & mut Cx , entries : & [HistoryEntryView])"
---

Sets the history entries.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_entries(cx, &[]);
}
```
