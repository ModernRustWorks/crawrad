---
id: gate-allows
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Gate::allows"
tags: []
signature: "pub fn allows (& self , c : & Context) -> bool"
---

Tells whether the action may run in this context.

# Examples

```rust
use crawrad_actions::{Args, Author, Context, Project, by_id};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;

let (r, a) = (Registry::builtin(), Clipboard::default());
let c = Context {
    doc: None,
    registry: &r,
    clipboard: &a,
    args: Args::none(),
    project: Project::default(),
    author: Author::User,
    editor_dirty: false,
    text: Default::default(),
};
assert!(by_id("project.new").unwrap().enabled.allows(&c));
assert!(!by_id("file.save").unwrap().enabled.allows(&c));
```
