---
id: menu-explorer-context-rows
kind: doc
target_file: src/menu.rs
line_hint: 704
author: 
created: 
title: "explorer_context_rows"
tags: []
signature: "pub fn explorer_context_rows (is_dir : bool , path : & std :: path :: Path , role : crate :: WorkspaceRole , ctx : & Context ,) -> Vec < Row >"
---

The rows of the CONTEXT menu of an Explorer item (folder or file).

# Examples

```rust
use crawrad_actions::menu::explorer_context_rows;
use crawrad_actions::{Args, Author, Context, EditorText, Project, WorkspaceRole};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;
use std::path::Path;

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
let rows = explorer_context_rows(false, Path::new("src/main.rs"), WorkspaceRole::Root, &ctx);
assert!(!rows.is_empty());
```
