---
id: radio-group-crawradradiogroupref-picked
kind: doc
target_file: src/radio_group.rs
line_hint: 273
author: 
created: 
title: "CrawradRadioGroupRef::picked"
tags: []
signature: "pub fn picked (& self , actions : & Actions) -> Option < usize >"
---

The item the user clicked in `actions`, if any.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::radio_group::CrawradRadioGroupRef;

fn check(r: &CrawradRadioGroupRef, actions: &Actions) {
    let _picked = r.picked(actions);
}
```
