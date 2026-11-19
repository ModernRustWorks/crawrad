---
id: welcome-list-height
kind: doc
target_file: src/welcome.rs
line_hint: 262
author: 
created: 
title: "list_height"
tags: []
signature: "pub fn list_height (rows : usize) -> f64"
---

Calculates the height of the recent projects list.

The Recent list's height: one 50 pt row each (the rule drawn over its top), at most eight.

# Examples

```rust
use crawrad_runtime_makepad::welcome::list_height;

assert_eq!(list_height(3), 150.0);
assert_eq!(list_height(10), 400.0);
```
