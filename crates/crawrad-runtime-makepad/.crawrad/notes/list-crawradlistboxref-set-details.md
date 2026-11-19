---
id: list-crawradlistboxref-set-details
kind: doc
target_file: src/list.rs
line_hint: 399
author: 
created: 
title: "CrawradListBoxRef::set_details"
tags: []
signature: "pub fn set_details (& self , cx : & mut Cx , details : & [String])"
---

Updates the trailing detail text for each row.

Shows `details` at the right end of the rows, one per item (#604);
only redraws when they changed.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn update(cx: &mut Cx, r: &CrawradListBoxRef) {
    r.set_details(cx, &["(1)".into()]);
}
```
