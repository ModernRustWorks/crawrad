---
id: menu-recent-header-row
kind: doc
target_file: src/menu.rs
line_hint: 206
author: 
created: 
title: "recent_header_row"
tags: []
signature: "pub fn recent_header_row () -> Row"
---

The row in File that opens the Recent submenu.

It runs nothing itself: a row with no action and a submenu name is how the chrome
knows to open one.

# Examples

```rust
use crawrad_actions::menu;

let row = menu::recent_header_row();
assert_eq!(row.submenu.as_deref(), Some(menu::RECENT));
// It opens a submenu; it does not run a command.
assert!(row.action.is_none());
```
