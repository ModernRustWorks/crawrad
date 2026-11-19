---
id: column-splitter-crawradcolumnsplitterref-dragged
kind: doc
target_file: src/column_splitter.rs
line_hint: 104
author: 
created: 
title: "CrawradColumnSplitterRef::dragged"
tags: []
signature: "pub fn dragged (& self , actions : & Actions) -> Option < f64 >"
---

The column width a drag asks for, if the finger moved.

# Examples

```rust,no_run
// Actions require the Makepad event loop.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::column_splitter::CrawradColumnSplitterRef;

fn example(actions: &Actions, splitter: &CrawradColumnSplitterRef) {
    let _ = splitter.dragged(actions);
}
```
