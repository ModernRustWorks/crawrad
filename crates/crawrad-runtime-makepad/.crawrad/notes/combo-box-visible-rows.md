---
id: combo-box-visible-rows
kind: doc
target_file: src/combo_box.rs
line_hint: 137
author: 
created: 
title: "visible_rows"
tags: []
signature: "pub fn visible_rows (count : usize , len : usize) -> usize"
---

Returns the number of visible rows for the given drop-down count and length.

# Examples

```
use crawrad_runtime_makepad::combo_box::visible_rows;

assert_eq!(visible_rows(8, 20), 8);
assert_eq!(visible_rows(0, 20), 20);
assert_eq!(visible_rows(8, 3), 3);
```
