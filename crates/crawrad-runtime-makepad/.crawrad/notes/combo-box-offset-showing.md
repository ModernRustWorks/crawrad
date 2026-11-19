---
id: combo-box-offset-showing
kind: doc
target_file: src/combo_box.rs
line_hint: 142
author: 
created: 
title: "offset_showing"
tags: []
signature: "pub fn offset_showing (row : usize , offset : usize , rows : usize , len : usize) -> usize"
---

Returns the scroll offset showing `row`, changing `offset` minimally.

# Examples

```
use crawrad_runtime_makepad::combo_box::offset_showing;

assert_eq!(offset_showing(2, 0, 5, 10), 0);
assert_eq!(offset_showing(5, 0, 5, 10), 1);
assert_eq!(offset_showing(3, 5, 5, 10), 3);
```
