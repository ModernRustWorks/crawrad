---
id: diff-view-crawraddiffviewref-set-rows
kind: doc
target_file: src/diff_view.rs
line_hint: 472
author: 
created: 
title: "CrawradDiffViewRef::set_rows"
tags: []
signature: "pub fn set_rows (& self , cx : & mut Cx , rows : & [DiffViewRow])"
---

Sets the side-by-side diff rows.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::diff_view::CrawradDiffViewRef;

fn example(view: &CrawradDiffViewRef, cx: &mut Cx) {
    view.set_rows(cx, &[]);
}
```
