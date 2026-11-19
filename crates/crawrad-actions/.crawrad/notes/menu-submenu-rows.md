---
id: menu-submenu-rows
kind: doc
target_file: src/menu.rs
line_hint: 341
author: 
created: 
title: "submenu_rows"
tags: []
signature: "pub fn submenu_rows (menu : Menu , name : & str , ctx : & Context , scheme : crate :: KeymapScheme ,) -> Vec < Row >"
---

The rows of submenu `name` of `menu` (#558), in its own sections.

# Examples

```rust
use crawrad_actions::menu::{submenu_rows, Menu};
use crawrad_actions::{Args, Author, Context, EditorText, KeymapScheme, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;

let registry = Registry::builtin();
let clipboard = Clipboard::default();
let ctx = Context {
    doc: None,
    registry: &registry,
    clipboard: &clipboard,
    args: Args::none(),
    project: Project { has_binary: false, ..Project::default() },
    author: Author::User,
    editor_dirty: false,
    text: EditorText::default(),
};
let rows = submenu_rows(Menu::View, "Zoom", &ctx, KeymapScheme::Modern);
assert!(!rows.is_empty());
```
