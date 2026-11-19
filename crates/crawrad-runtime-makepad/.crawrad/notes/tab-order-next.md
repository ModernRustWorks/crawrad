---
id: tab-order-next
kind: doc
target_file: src/tab_order.rs
line_hint: 38
author: 
created: 
title: "next"
tags: []
signature: "pub fn next (at : Option < usize > , active : & [bool] , forward : bool) -> Option < usize >"
---

Finds the index of the next active Tab stop in forward or backward order.

The stop after `at` (before it, backwards) among the `active` ones,
wrapping round; with no focus, the first (backwards, the last). `None`
when no stop is active.

# Examples

```rust
use crawrad_runtime_makepad::tab_order::next;

let active = [true, false, true];
assert_eq!(next(Some(0), &active, true), Some(2));
assert_eq!(next(Some(2), &active, true), Some(0));
assert_eq!(next(Some(2), &active, false), Some(0));
```
