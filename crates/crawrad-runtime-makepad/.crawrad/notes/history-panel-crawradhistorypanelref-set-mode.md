---
id: history-panel-crawradhistorypanelref-set-mode
kind: doc
target_file: src/history_panel.rs
line_hint: 747
author: 
created: 
title: "CrawradHistoryPanelRef::set_mode"
tags: []
signature: "pub fn set_mode (& self , cx : & mut Cx , mode : HistoryMode)"
---

Sets the panel mode.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::{HistoryMode, CrawradHistoryPanelRef};

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_mode(cx, HistoryMode::Contents);
}
```
