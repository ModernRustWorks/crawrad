---
id: menu-next-live-row
kind: doc
target_file: src/menu.rs
line_hint: 325
author: 
created: 
title: "next_live_row"
tags: []
signature: "pub fn next_live_row (rows : & [Row] , current : Option < usize > , down : bool) -> Option < usize >"
---

Calculates next live row index when navigating with arrow keys.

The row the arrow keys land on from `current` (#257): only rows that can
run (rules and muted rows skipped), wrapping at both ends. The bar menu and
its submenu walk the same way (#558).

# Examples

```rust
use crawrad_actions::menu::next_live_row;
use crawrad_actions::Row;

let rows = vec![Row::custom("Open", None)];
assert_eq!(next_live_row(&rows, None, true), Some(0));
```
