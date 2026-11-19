---
id: history-panel-crawradhistorypanelref-reveal-row
kind: doc
target_file: src/history_panel.rs
line_hint: 767
author: 
created: 
title: "CrawradHistoryPanelRef::reveal_row"
tags: []
signature: "pub fn reveal_row (& self , cx : & mut Cx , row : usize)"
---

Scrolls diff view to the row.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.reveal_row(cx, 0);
}
```
