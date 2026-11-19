---
id: check-list-crawradchecklistboxref-toggled
kind: doc
target_file: src/check_list.rs
line_hint: 242
author: 
created: 
title: "CrawradCheckListBoxRef::toggled"
tags: []
signature: "pub fn toggled (& self , actions : & Actions) -> Option < (usize , bool) >"
---

The row the user toggled in `actions`, and its new state.

# Examples

```rust,no_run
// Actions require the Makepad event loop.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxRef;

fn example(list: &CrawradCheckListBoxRef, actions: &Actions) {
    let _toggled = list.toggled(actions);
}
```
