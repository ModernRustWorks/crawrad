---
id: history-panel-crawradhistorypanelref-set-diff
kind: doc
target_file: src/history_panel.rs
line_hint: 754
author: 
created: 
title: "CrawradHistoryPanelRef::set_diff"
tags: []
signature: "pub fn set_diff (& self , cx : & mut Cx , diff : & [DiffViewRow] , blocks : usize , current_block : Option < usize > ,)"
---

Sets the diff view rows and changes info.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanelRef;

fn update(cx: &mut Cx, r: &CrawradHistoryPanelRef) {
    r.set_diff(cx, &[], 0, None);
}
```
