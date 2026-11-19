---
id: history-panel-crawradhistorypanelref-set-title
kind: doc
target_file: src/history_panel.rs
line_hint: 726
author: 
created: 
title: "CrawradHistoryPanelRef::set_title"
tags: []
signature: "pub fn set_title (& self , cx : & mut Cx , title : & str)"
---

Sets the panel header title.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_title(cx, "Title");
}
```
