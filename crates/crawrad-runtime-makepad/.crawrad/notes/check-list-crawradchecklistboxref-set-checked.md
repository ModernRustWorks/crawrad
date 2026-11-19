---
id: check-list-crawradchecklistboxref-set-checked
kind: doc
target_file: src/check_list.rs
line_hint: 211
author: 
created: 
title: "CrawradCheckListBoxRef::set_checked"
tags: []
signature: "pub fn set_checked (& self , cx : & mut Cx , checked : & [bool])"
---

Which rows are checked, one per item (a missing one is not).

# Examples

```rust,no_run
// Makepad Cx and widget refs require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxRef;

fn example(cx: &mut Cx, list: &CrawradCheckListBoxRef) {
    list.set_checked(cx, &[true, false]);
}
```
