---
id: history-panel-crawradhistorypanelref-set-selected
kind: doc
target_file: src/history_panel.rs
line_hint: 740
author: 
created: 
title: "CrawradHistoryPanelRef::set_selected"
tags: []
signature: "pub fn set_selected (& self , cx : & mut Cx , selected : Option < usize >)"
---

Sets the selected copy index.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_selected(cx, Some(0));
}
```
