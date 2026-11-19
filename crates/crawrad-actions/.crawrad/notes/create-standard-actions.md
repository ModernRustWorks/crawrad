---
id: create-standard-actions
kind: doc
target_file: src/lib.rs
line_hint: 5621
author: 
created: 
title: "create_standard_actions"
tags: []
signature: "pub fn create_standard_actions (c : & Context , names : & [& str]) -> Effect"
---

Creates predefined actions inside the selected action list.

Creates one or more predefined actions inside the selected list, in a single
transaction — one Ctrl+Z undoes all of them at once.

It is the body of the `action.standard` action, and remains public because tests
call it directly. Outside callers use the catalogue, with
`Args::texts("classes", …)`.

# Examples

```rust
use crawrad_actions::{create_standard_actions, Args, Author, Context, EditorText, Project};
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
let effect = create_standard_actions(&ctx, &["edit.copy"]);
assert!(effect.is_empty());
```
