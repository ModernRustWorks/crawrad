---
id: doc-note-doctarget
kind: doc
target_file: src/doc_note.rs
line_hint: 25
author: 
created: 
title: "DocTarget"
tags: []
signature: "pub struct DocTarget"
---

The target Rust item identified by text scanning at a given cursor position.

# Examples

```rust
use crawrad_project::doc_note::{DocTarget, ItemKind};

let target = DocTarget {
    kind: ItemKind::Fn,
    name: "run".to_string(),
    owner: None,
    signature: "pub fn run()".to_string(),
    item_line: 0,
    insert_line: 0,
    indent: String::new(),
    doc_lines: 0..0,
    existing_docs: String::new(),
    existing_note: None,
    body: None,
    is_unsafe: false,
    returns_result: false,
};
assert_eq!(target.name, "run");
```
