---
id: history-panel-crawradhistorypanelref-set-note
kind: doc
target_file: src/history_panel.rs
line_hint: 781
author: 
created: 
title: "CrawradHistoryPanelRef::set_note"
tags: []
signature: "pub fn set_note (& self , cx : & mut Cx , note : & str)"
---

Sets the footer note for Information mode.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_note(cx, "Note");
}
```
