---
id: diff-view-crawraddiffview-set-rows
kind: doc
target_file: src/diff_view.rs
line_hint: 445
author: 
created: 
title: "CrawradDiffView::set_rows"
tags: []
signature: "pub fn set_rows (& mut self , cx : & mut Cx , rows : & [DiffViewRow])"
---

Updates the rows displayed in the side-by-side diff.

# Examples

```rust,no_run
// Makepad Cx and Widget require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::diff_view::CrawradDiffView;

fn example(view: &mut CrawradDiffView, cx: &mut Cx) {
    view.set_rows(cx, &[]);
}
```
