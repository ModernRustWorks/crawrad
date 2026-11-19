---
id: props-props-page-size
kind: doc
target_file: src/props.rs
line_hint: 1741
author: 
created: 
title: "Props::page_size"
tags: []
signature: "pub fn page_size (default : i64) -> PropDesc"
---

Number of steps when paging slider (`PageSize`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::page_size(10);
assert_eq!(p.key, "page_size");
```
