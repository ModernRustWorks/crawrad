---
id: need-met
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Need::met"
tags: []
signature: "pub fn met (self , c : & Context) -> bool"
---

Tells whether the need is met in this context.

Reads the same facts the actions' `enabled` read: the project (`open`,
`has_forms`, `has_binary`, `debug`), the document and its selection, the
text editor and the clipboards.

# Examples

```rust
use crawrad_actions::{Args, Author, Context, Need, Project};
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
assert!(Need::Project.met(&c));
assert!(!Need::Form.met(&c));
```
