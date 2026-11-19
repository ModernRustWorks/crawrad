---
id: props-props-page-index
kind: doc
target_file: src/props.rs
line_hint: 287
author: 
created: 
title: "Props::page_index"
tags: []
signature: "pub fn page_index (default : i64) -> PropDesc"
---

The active page index of a `Pagination` (`PageIndex`, #500).

The active page of a `Pagination` (0-based, shown as 1-based; `PageIndex`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::page_index(0);
assert_eq!(p.key, "page_index");
```
