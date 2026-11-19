---
id: history-panel-crawradhistorypanel-set-selected
kind: doc
target_file: src/history_panel.rs
line_hint: 542
author: 
created: 
title: "CrawradHistoryPanel::set_selected"
tags: []
signature: "pub fn set_selected (& mut self , cx : & mut Cx , selected : Option < usize >)"
---

Sets the currently selected copy index.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn select_first(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_selected(cx, Some(0));
}
```
