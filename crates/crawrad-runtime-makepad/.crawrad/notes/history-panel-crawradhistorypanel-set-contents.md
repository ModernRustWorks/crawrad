---
id: history-panel-crawradhistorypanel-set-contents
kind: doc
target_file: src/history_panel.rs
line_hint: 611
author: 
created: 
title: "CrawradHistoryPanel::set_contents"
tags: []
signature: "pub fn set_contents (& mut self , cx : & mut Cx , contents : & str)"
---

Sets the contents text for the Contents mode view.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::history_panel::CrawradHistoryPanel;

fn clear_contents(cx: &mut Cx, panel: &mut CrawradHistoryPanel) {
    panel.set_contents(cx, "");
}
```
