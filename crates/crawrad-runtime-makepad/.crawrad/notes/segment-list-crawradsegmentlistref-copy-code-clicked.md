---
id: segment-list-crawradsegmentlistref-copy-code-clicked
kind: doc
target_file: src/segment_list.rs
line_hint: 199
author: 
created: 
title: "CrawradSegmentListRef::copy_code_clicked"
tags: []
signature: "pub fn copy_code_clicked (& self , actions : & Actions) -> Option < String >"
---

Checks if any code child's copy button was clicked in `actions`.

Check if any code child's `btn_copy_code` button was clicked in `actions`,
returning the code text associated with that child.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::segment_list::CrawradSegmentListRef;

fn check_click(list: &CrawradSegmentListRef, actions: &Actions) {
    let _code = list.copy_code_clicked(actions);
}
```
