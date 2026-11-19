---
id: menu-editor-more-rows
kind: doc
target_file: src/menu.rs
line_hint: 591
author: 
created: 
title: "editor_more_rows"
tags: []
signature: "pub fn editor_more_rows (ctx : & Context) -> Vec < Row >"
---

Rows of the More Actions popup menu in the editor tabs corner.

The rows of the More Actions (...) popup menu in the right corner of the editor tabs (modern-editor style).

# Examples

```rust
use crawrad_actions::menu::editor_more_rows;
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
let rows = editor_more_rows(&ctx);
assert!(!rows.is_empty());
```
