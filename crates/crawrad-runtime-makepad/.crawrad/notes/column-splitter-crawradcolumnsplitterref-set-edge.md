---
id: column-splitter-crawradcolumnsplitterref-set-edge
kind: doc
target_file: src/column_splitter.rs
line_hint: 96
author: 
created: 
title: "CrawradColumnSplitterRef::set_edge"
tags: []
signature: "pub fn set_edge (& self , width : f64 , left : f64)"
---

Sets the column width and the strip's left edge offset from its parent.

The column is `width` wide and the strip's left edge is `left` from
its parent's — called before the strip draws, so it lands this frame.

# Examples

```rust,no_run
// WidgetRef requires the Makepad application runtime.
use crawrad_runtime_makepad::column_splitter::CrawradColumnSplitterRef;

fn example(splitter: &CrawradColumnSplitterRef) {
    splitter.set_edge(120.0, 100.0);
}
```
