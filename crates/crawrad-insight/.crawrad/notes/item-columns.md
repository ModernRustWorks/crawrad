---
id: item-columns
kind: doc
target_file: src/item.rs
line_hint: 116
author: 
created: 
title: "Columns"
tags: []
signature: "pub struct Columns"
---

The four columns drawn for one row of the completion list.

# Examples

```rust
use crawrad_insight::item::Columns;

let col = Columns {
    icon: "*",
    label: "name".into(),
    origin: "".into(),
    signature: "".into(),
    deprecated: false,
};
assert_eq!(col.label, "name");
```
