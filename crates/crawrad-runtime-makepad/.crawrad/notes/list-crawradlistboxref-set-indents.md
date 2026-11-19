---
id: list-crawradlistboxref-set-indents
kind: doc
target_file: src/list.rs
line_hint: 409
author: 
created: 
title: "CrawradListBoxRef::set_indents"
tags: []
signature: "pub fn set_indents (& self , cx : & mut Cx , indents : & [f64])"
---

Sets left padding indents for each row, restyling on change.

Sets each row's left padding indent (#448); only redraws and restyles
when the indents changed.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn update(cx: &mut Cx, r: &CrawradListBoxRef) {
    r.set_indents(cx, &[16.0]);
}
```
