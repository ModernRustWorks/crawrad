---
id: menu-rows-with-scheme
kind: doc
target_file: src/menu.rs
line_hint: 183
author: 
created: 
title: "rows_with_scheme"
tags: []
signature: "pub fn rows_with_scheme (menu : Menu , ctx : & Context , scheme : crate :: KeymapScheme) -> Vec < Row >"
---

The rows of a menu, configured for the specified keymap scheme.

# Examples

```rust
use crawrad_actions::menu::{rows_with_scheme, Menu};
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
let rows = rows_with_scheme(Menu::File, &ctx, KeymapScheme::Modern);
assert!(!rows.is_empty());
```
