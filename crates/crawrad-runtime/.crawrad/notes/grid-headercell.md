---
id: grid-headercell
kind: doc
target_file: src/grid.rs
line_hint: 621
author: 
created: 
title: "HeaderCell"
tags: []
signature: "pub struct HeaderCell"
---

Formatted header column cell.

# Examples

```rust
use crawrad_runtime::grid::{Color, ColumnAlign, Font, HeaderCell};

let cell = HeaderCell {
    title: "Name".into(),
    arrow: "",
    align: ColumnAlign::Left,
    title_color: Color::transparent(),
    title_hint: String::new(),
    title_font: Font::default(),
    title_word_wrap: false,
    auto_size: false,
    resizable: true,
    movable: true,
};
assert_eq!(cell.title, "Name");
```
