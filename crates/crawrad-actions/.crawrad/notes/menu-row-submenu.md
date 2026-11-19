---
id: menu-row-submenu
kind: doc
target_file: src/menu.rs
line_hint: 160
author: 
created: 
title: "Row::submenu"
tags: []
signature: "pub fn submenu (name : & 'static str) -> Self"
---

The row that opens submenu `name` (#558): it runs nothing itself.

# Examples

```rust
use crawrad_actions::Row;

let row = Row::submenu("Zoom");
assert_eq!(row.submenu.as_deref(), Some("Zoom"));
```
