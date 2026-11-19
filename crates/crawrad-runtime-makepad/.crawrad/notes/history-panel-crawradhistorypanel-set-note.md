---
id: history-panel-crawradhistorypanel-set-note
kind: doc
target_file: src/history_panel.rs
line_hint: 618
author: 
created: 
title: "CrawradHistoryPanel::set_note"
tags: []
signature: "pub fn set_note (& mut self , cx : & mut Cx , note : & str)"
---

Sets the footer note for Information mode.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn update_note(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_note(cx, "Note");
}
```
