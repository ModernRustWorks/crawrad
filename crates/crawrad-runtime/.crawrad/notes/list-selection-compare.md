---
id: list-selection-compare
kind: doc
target_file: src/list_selection.rs
line_hint: 51
author: 
created: 
title: "compare"
tags: []
signature: "pub fn compare (a : & str , b : & str) -> Ordering"
---

Case-insensitive string comparison for list sorting.

The order `Sorted` keeps: alphabetical, ignoring case; equal texts keep
the order they came in.

# Examples

```rust
use std::cmp::Ordering;
use crawrad_runtime::list_selection::compare;

assert_eq!(compare("apple", "Banana"), Ordering::Less);
```
