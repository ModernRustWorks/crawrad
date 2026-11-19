---
id: tabs-prev-page
kind: doc
target_file: src/tabs.rs
line_hint: 34
author: 
created: 
title: "prev_page"
tags: []
signature: "pub fn prev_page (current : usize , eligible : & [bool]) -> Option < usize >"
---

Finds the previous eligible page index before `current` without wrapping.

The previous page that is shown and enabled before `current`,
or `None` if `current` is already at the first eligible page (no wrapping).

# Examples

```rust
use crawrad_runtime_makepad::tabs::prev_page;

let eligible = [true, false, true];
assert_eq!(prev_page(2, &eligible), Some(0));
assert_eq!(prev_page(0, &eligible), None);
```
