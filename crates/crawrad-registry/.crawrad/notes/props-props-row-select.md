---
id: props-props-row-select
kind: doc
target_file: src/props.rs
line_hint: 1902
author: 
created: 
title: "Props::row_select"
tags: []
signature: "pub fn row_select (default : bool) -> PropDesc"
---

Whether full rows are highlighted on selection (`RowSelect`, #500).

Whether full rows are highlighted on selection in a `ListView` (`RowSelect`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::row_select(true);
assert_eq!(p.key, "row_select");
```
