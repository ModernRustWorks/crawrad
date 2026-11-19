---
id: history-panel-crawradhistorypanel-clicked
kind: doc
target_file: src/history_panel.rs
line_hint: 671
author: 
created: 
title: "CrawradHistoryPanel::clicked"
tags: []
signature: "pub fn clicked (& self , actions : & Actions) -> Option < HistoryPanelAction >"
---

Dispatches user clicks in the history panel.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn check_clicks(panel: &CrawradHistoryPanel, actions: &Actions) {
    let _action = panel.clicked(actions);
}
```
