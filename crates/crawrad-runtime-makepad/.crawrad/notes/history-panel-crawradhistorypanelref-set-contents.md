---
id: history-panel-crawradhistorypanelref-set-contents
kind: doc
target_file: src/history_panel.rs
line_hint: 774
author: 
created: 
title: "CrawradHistoryPanelRef::set_contents"
tags: []
signature: "pub fn set_contents (& self , cx : & mut Cx , contents : & str)"
---

Sets the contents text for Contents mode.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_contents(cx, "");
}
```
