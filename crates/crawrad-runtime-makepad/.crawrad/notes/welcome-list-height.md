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

The Recent list's height: one row each (the rule drawn over its top), at most eight.

The row height is `RECENT_ROW_HEIGHT`, the same height the live_design declares
on `WelcomeRecentRow`; `recent_row_guard` fails the build if the two drift
apart. It was raised from 50 to 62 because the selection highlight was
touching the project name above and its path below.

# Examples

```rust
use crawrad_runtime_makepad::welcome::list_height;

use crawrad_runtime_makepad::welcome::RECENT_ROW_HEIGHT;

assert_eq!(list_height(3), 3.0 * RECENT_ROW_HEIGHT);
assert_eq!(list_height(10), 8.0 * RECENT_ROW_HEIGHT);
```
