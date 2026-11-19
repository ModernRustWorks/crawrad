---
id: history-panel-crawradhistorypanelref-clicked
kind: doc
target_file: src/history_panel.rs
line_hint: 794
author: 
created: 
title: "CrawradHistoryPanelRef::clicked"
tags: []
signature: "pub fn clicked (& self , actions : & Actions) -> Option < HistoryPanelAction >"
---

Dispatches panel actions.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn check(actions: &Actions, r: &CrawradHistoryPanelRef) {
    let _action = r.clicked(actions);
}
```
