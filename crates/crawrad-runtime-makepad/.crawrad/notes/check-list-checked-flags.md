---
id: check-list-checked-flags
kind: doc
target_file: src/check_list.rs
line_hint: 258
author: 
created: 
title: "checked_flags"
tags: []
signature: "pub fn checked_flags (items : & [String] , checked : & [String]) -> Vec < bool >"
---

Computes a boolean mask for elements present in `checked`.

Computes a boolean mask for `items` indicating which elements are present in `checked`.

# Examples

```rust
use crawrad_runtime_makepad::check_list::checked_flags;

let items = vec!["A".to_string(), "B".to_string()];
let checked = vec!["A".to_string()];
assert_eq!(checked_flags(&items, &checked), vec![true, false]);
```
