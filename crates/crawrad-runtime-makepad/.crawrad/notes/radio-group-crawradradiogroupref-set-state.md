---
id: radio-group-crawradradiogroupref-set-state
kind: doc
target_file: src/radio_group.rs
line_hint: 263
author: 
created: 
title: "CrawradRadioGroupRef::set_state"
tags: []
signature: "pub fn set_state (& self , cx : & mut Cx , state : GroupState)"
---

Updates the group state, redrawing only when it changed.

What the form holds now; redraws only when it changed (the sync runs
after every event).

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::radio_group::{GroupState, CrawradRadioGroupRef};

fn update(cx: &mut Cx, r: &CrawradRadioGroupRef) {
    r.set_state(cx, GroupState::default());
}
```
