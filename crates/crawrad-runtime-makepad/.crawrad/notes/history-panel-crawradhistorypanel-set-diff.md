---
id: history-panel-crawradhistorypanel-set-diff
kind: doc
target_file: src/history_panel.rs
line_hint: 581
author: 
created: 
title: "CrawradHistoryPanel::set_diff"
tags: []
signature: "pub fn set_diff (& mut self , cx : & mut Cx , diff : & [DiffViewRow] , blocks : usize , current_block : Option < usize > ,)"
---

Updates the diff rows and change blocks navigation bar.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn reset_diff(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_diff(cx, &[], 0, None);
}
```
