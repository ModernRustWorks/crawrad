---
id: tabs-look
kind: doc
target_file: src/tabs.rs
line_hint: 17
author: 
created: 
title: "look"
tags: []
signature: "pub fn look (cx : & mut Cx , mark : & WidgetRef , ring : & WidgetRef , open : bool , focused : bool)"
---

Updates tab active mark and focus ring visibility based on tab state.

Shows the `mark` under a tab while it is the `open` one, and the `ring`
round it while it also holds the key focus (#139). Acts only on a change:
the sync runs on every pass.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::tabs::look;

fn update_tab(cx: &mut Cx, mark: &WidgetRef, ring: &WidgetRef) {
    look(cx, mark, ring, true, false);
}
```
