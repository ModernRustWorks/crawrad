---
id: tab-order-tabstop
kind: doc
target_file: src/tab_order.rs
line_hint: 14
author: 
created: 
title: "TabStop"
tags: []
signature: "pub type TabStop = (bool , bool , WidgetRef)"
---

Tab stop tuple `(can_focus, keeps_tab, widget)` in the Tab order sequence.

One control in the Tab sequence: whether it can take the focus now
(visible and enabled), whether it keeps Tab for itself, and its widget.

# Examples

```rust,no_run
// Makepad WidgetRef requires the Makepad application runtime.
use makepad_widgets::WidgetRef;
use crawrad_runtime_makepad::tab_order::TabStop;

fn check_stop(w: WidgetRef) {
    let _stop: TabStop = (true, false, w);
}
```
