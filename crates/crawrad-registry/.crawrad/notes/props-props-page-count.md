---
id: props-props-page-count
kind: doc
target_file: src/props.rs
line_hint: 273
author: 
created: 
title: "Props::page_count"
tags: []
signature: "pub fn page_count (default : i64) -> PropDesc"
---

How many pages a `Pagination` spans (`PageCount`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::page_count(10);
assert_eq!(p.key, "page_count");
```
