---
id: item-columns-layout
kind: doc
target_file: src/item.rs
line_hint: 138
author: 
created: 
title: "Columns::layout"
tags: []
signature: "pub fn layout (& self , row_width : f64 , mut width : impl FnMut (& str) -> f64) -> RowLayout"
---

Lays the row out in `row_width`, `width` measuring a text in its font.

# Examples

```rust
use crawrad_insight::item::Columns;

let col = Columns {
    icon: "*",
    label: "name".into(),
    origin: "".into(),
    signature: "()".into(),
    deprecated: false,
};
let layout = col.layout(200.0, |_| 10.0);
assert_eq!(layout.strike, (0.0, 0.0));
```
