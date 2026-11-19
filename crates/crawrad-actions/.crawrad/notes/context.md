---
id: context
kind: doc
target_file: src/lib.rs
line_hint: 83
author: 
created: 
title: "Context"
tags: []
signature: "pub struct Context < 'a >"
---

What the action needs to know to decide whether it can act, and to act.

# Examples

```rust
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
assert!(ctx.doc.is_none());
```
