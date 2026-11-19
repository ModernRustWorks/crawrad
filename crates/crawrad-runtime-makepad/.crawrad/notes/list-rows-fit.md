---
id: list-rows-fit
kind: doc
target_file: src/list.rs
line_hint: 296
author: 
created: 
title: "rows_fit"
tags: []
signature: "pub fn rows_fit (count : usize , row_h : f64 , list_h : f64) -> bool"
---

Whether `count` rows of `row_h` fit in a list `list_h` tall (#604).

# Examples

```rust
use crawrad_runtime_makepad::list::rows_fit;

assert!(rows_fit(5, 20.0, 100.0));
assert!(!rows_fit(10, 20.0, 100.0));
```
