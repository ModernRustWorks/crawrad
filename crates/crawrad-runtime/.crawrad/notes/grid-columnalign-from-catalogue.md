---
id: grid-columnalign-from-catalogue
kind: doc
target_file: src/grid.rs
line_hint: 19
author: 
created: 
title: "ColumnAlign::from_catalogue"
tags: []
signature: "pub fn from_catalogue (s : & str) -> Self"
---

The catalogue's `alignment` value as a column alignment.

Anything unknown is left.

# Examples

```rust
use crawrad_runtime::grid::ColumnAlign;

assert_eq!(ColumnAlign::from_catalogue("taRightJustify"), ColumnAlign::Right);
assert_eq!(ColumnAlign::from_catalogue("taCenter"), ColumnAlign::Center);
assert_eq!(ColumnAlign::from_catalogue(""), ColumnAlign::Left);
```
