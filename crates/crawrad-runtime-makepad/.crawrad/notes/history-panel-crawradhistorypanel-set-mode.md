---
id: history-panel-crawradhistorypanel-set-mode
kind: doc
target_file: src/history_panel.rs
line_hint: 549
author: 
created: 
title: "CrawradHistoryPanel::set_mode"
tags: []
signature: "pub fn set_mode (& mut self , cx : & mut Cx , mode : HistoryMode)"
---

Switches the display mode.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::{HistoryMode, CrawradHistoryPanel};

fn switch_mode(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_mode(cx, HistoryMode::Contents);
}
```
