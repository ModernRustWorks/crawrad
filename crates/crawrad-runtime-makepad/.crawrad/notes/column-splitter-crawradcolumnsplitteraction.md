---
id: column-splitter-crawradcolumnsplitteraction
kind: doc
target_file: src/column_splitter.rs
line_hint: 29
author: 
created: 
title: "CrawradColumnSplitterAction"
tags: []
signature: "pub enum CrawradColumnSplitterAction"
---

What a drag of the strip asks its owner.

# Examples

```rust
use crawrad_runtime_makepad::column_splitter::CrawradColumnSplitterAction;

let action = CrawradColumnSplitterAction::Dragged(120.0);
match action {
    CrawradColumnSplitterAction::Dragged(w) => assert_eq!(w, 120.0),
    _ => unreachable!(),
}
```
