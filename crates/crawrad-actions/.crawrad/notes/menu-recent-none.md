---
id: menu-recent-none
kind: doc
target_file: src/menu.rs
line_hint: 199
author: 
created: 
title: "RECENT_NONE"
tags: []
signature: "pub const RECENT_NONE : & str = \"no recent projects yet\""
---

What the Recent submenu says when the list is empty.

A submenu that opens onto blankness is a box with no height — the same defect
`NOTHING_HERE` exists for, one level down. On a first run there is nothing to
reopen, and the screen has to say so rather than look broken.

# Examples

```rust
use crawrad_actions::menu;

let rows = menu::recent_rows(&[]);
assert_eq!(rows[0].label, menu::RECENT_NONE);
assert!(!rows[0].enabled);
```
