---
id: tabs-next-page
kind: doc
target_file: src/tabs.rs
line_hint: 39
author: 
created: 
title: "next_page"
tags: []
signature: "pub fn next_page (current : usize , eligible : & [bool]) -> Option < usize >"
---

Finds the next eligible page index after `current` without wrapping.

The next page that is shown and enabled after `current`,
or `None` if `current` is already at the last eligible page (no wrapping).

# Examples

```rust
use crawrad_runtime_makepad::tabs::next_page;

let eligible = [true, false, true];
assert_eq!(next_page(0, &eligible), Some(2));
assert_eq!(next_page(2, &eligible), None);
```
