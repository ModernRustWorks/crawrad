---
id: menu-insert-recent-header
kind: doc
target_file: src/menu.rs
line_hint: 224
author: 
created: 
title: "insert_recent_header"
tags: []
signature: "pub fn insert_recent_header (rows : & mut Vec < Row >)"
---

Puts the Recent row where it belongs in an already-built File menu.

Not at the end: the File group is one block in catalogue order, so pushing it
would park `Recent` after `Close`, below everything. It goes after
`Open Project...` and before `Remove from Recent`, so the three rows about the
project list read as one thought. A menu missing both of those gets the row at
the end rather than no row at all — out of place beats absent.

# Examples

```rust
use crawrad_actions::menu::{self, Row};

let mut rows = vec![
    Row { action: Some("project.open_folder".into()), ..Row::custom("Open Project...", None) },
    Row { action: Some("project.forget_recent".into()), ..Row::custom("Remove from Recent", None) },
];
menu::insert_recent_header(&mut rows);
// It lands between the two, not after them.
assert_eq!(rows[1].submenu.as_deref(), Some(menu::RECENT));
```
