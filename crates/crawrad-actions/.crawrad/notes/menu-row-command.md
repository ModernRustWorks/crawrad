---
id: menu-row-command
kind: doc
target_file: src/menu.rs
line_hint: 130
author: 
created: 
title: "Row::command"
tags: []
signature: "pub fn command (a : & crate :: Action , ctx : & Context) -> Self"
---

An ordinary row: one catalogue command, no tick, no rule.

# Examples

```rust
use crawrad_actions::{by_id, Args, Author, Context, EditorText, Project, Row};
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
let action = by_id("file.save").unwrap();
let row = Row::command(&action, &ctx);
assert_eq!(row.label, "Save");
```
