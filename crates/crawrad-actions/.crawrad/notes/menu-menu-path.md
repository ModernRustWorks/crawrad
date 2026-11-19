---
id: menu-menu-path
kind: doc
target_file: src/menu.rs
line_hint: 99
author: 
created: 
title: "menu_path"
tags: []
signature: "pub fn menu_path (a : & crate :: Action) -> String"
---

Returns the path of an action in the menu hierarchy.

Path of an action in the menu hierarchy (Issue #558):
`"View › Zoom › Zoom in"`, `"Edit › Undo"`.

# Examples

```rust
use crawrad_actions::by_id;
use crawrad_actions::menu_path;

let action = by_id("file.save").unwrap();
assert_eq!(menu_path(&action), "File › Save");
```
