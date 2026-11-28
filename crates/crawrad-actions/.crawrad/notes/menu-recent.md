---
id: menu-recent
kind: doc
target_file: src/menu.rs
line_hint: 195
author: 
created: 
title: "RECENT"
tags: []
signature: "pub const RECENT : & str = \"Recent\""
---

The File menu's submenu of recently opened projects.

A classic RAD IDE puts the recent projects behind one row of File instead of
spilling them into the menu itself, and that is what this names.

# Examples

```rust
use crawrad_actions::menu;

// The header row and the submenu's contents agree on one name.
assert_eq!(menu::recent_header_row().submenu.as_deref(), Some(menu::RECENT));
```
