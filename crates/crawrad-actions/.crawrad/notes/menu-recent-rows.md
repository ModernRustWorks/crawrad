---
id: menu-recent-rows
kind: doc
target_file: src/menu.rs
line_hint: 224
author: 
created: 
title: "recent_rows"
tags: []
signature: "pub fn recent_rows (items : & [(String, std :: path :: PathBuf)]) -> Vec < Row >"
---

The rows inside the Recent submenu, newest first.

Each runs `project.open_folder` with its own folder as the `path` argument — the
same command the Welcome page's rows run, and the same one `Open Project...` runs
with no argument at all. The caller passes the labels it already built for the
Welcome page, so the menu and that page cannot drift into two different names for
one project.

# Examples

```rust
use crawrad_actions::menu;
use std::path::PathBuf;

let items = vec![("crawrad — E:/DeveloperRust".to_string(), PathBuf::from("E:/DeveloperRust/CrawRAD"))];
let rows = menu::recent_rows(&items);
assert_eq!(rows.len(), 1);
assert_eq!(rows[0].action.as_deref(), Some("project.open_folder"));
// The folder travels in the row's own arguments, so the command needs no picker.
assert!(rows[0].args.as_text("path").is_some());

// An empty list draws one disabled notice, never nothing at all.
let vazio = menu::recent_rows(&[]);
assert_eq!(vazio.len(), 1);
assert!(!vazio[0].enabled);
```
