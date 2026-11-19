---
id: menu-settings-rail-rows
kind: doc
target_file: src/menu.rs
line_hint: 536
author: 
created: 
title: "settings_rail_rows"
tags: []
signature: "pub fn settings_rail_rows (ctx : & Context) -> Vec < Row >"
---

The rows of the Settings/Manage popup menu of the Activity Rail gear icon.

# Examples

```rust
use crawrad_actions::menu::settings_rail_rows;
use crawrad_actions::{Args, Author, Context, EditorText, Project};
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
let rows = settings_rail_rows(&ctx);
assert!(!rows.is_empty());
```
