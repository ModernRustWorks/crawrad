---
id: menu-lsp-status-rows
kind: doc
target_file: src/menu.rs
line_hint: 573
author: 
created: 
title: "lsp_status_rows"
tags: []
signature: "pub fn lsp_status_rows (version : Option < String > , ctx : & Context) -> Vec < Row >"
---

Rows of the rust-analyzer status bar menu.

rust-analyzer's menu, as VS Code's: its version on top, then what can be
done to the server — catalogue actions, so an agent has the same doors.

# Examples

```rust
use crawrad_actions::menu::lsp_status_rows;
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
let rows = lsp_status_rows(None, &ctx);
assert!(!rows.is_empty());
```
