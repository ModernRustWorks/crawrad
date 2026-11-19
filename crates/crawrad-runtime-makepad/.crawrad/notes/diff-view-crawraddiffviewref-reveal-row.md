---
id: diff-view-crawraddiffviewref-reveal-row
kind: doc
target_file: src/diff_view.rs
line_hint: 479
author: 
created: 
title: "CrawradDiffViewRef::reveal_row"
tags: []
signature: "pub fn reveal_row (& self , cx : & mut Cx , row : usize)"
---

Scrolls so that the specified row is at the top.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::diff_view::CrawradDiffViewRef;

fn example(view: &CrawradDiffViewRef, cx: &mut Cx) {
    view.reveal_row(cx, 0);
}
```
