---
id: diff-view-crawraddiffview-reveal-row
kind: doc
target_file: src/diff_view.rs
line_hint: 460
author: 
created: 
title: "CrawradDiffView::reveal_row"
tags: []
signature: "pub fn reveal_row (& mut self , cx : & mut Cx , row : usize)"
---

Scrolls the diff view so that `row` is at the top.

# Examples

```rust,no_run
// Makepad Cx and Widget require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::diff_view::CrawradDiffView;

fn example(view: &mut CrawradDiffView, cx: &mut Cx) {
    view.reveal_row(cx, 0);
}
```
