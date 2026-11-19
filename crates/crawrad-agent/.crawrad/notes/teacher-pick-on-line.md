---
id: teacher-pick-on-line
kind: doc
target_file: src/teacher.rs
line_hint: 88
author: 
created: 
title: "pick_on_line"
tags: []
signature: "pub fn pick_on_line < T > (items : & [T] , line0 : u32 , span : impl Fn (& T) -> (u32 , u32 , bool) ,) -> Option < & T >"
---

Selects the diagnostic to ground on covering a given 0-based line.

The diagnostic the teacher grounds on among those covering `line0`
(0-based): the first error, else the first of any severity. `span` gives
an item's first and last line and whether it is an error.

# Examples

```rust
use crawrad_agent::teacher::pick_on_line;

let items = vec![(0u32, 5u32, true), (2u32, 3u32, false)];
let picked = pick_on_line(&items, 2, |&(start, end, is_err)| (start, end, is_err));
assert_eq!(picked, Some(&(0u32, 5u32, true)));
```
