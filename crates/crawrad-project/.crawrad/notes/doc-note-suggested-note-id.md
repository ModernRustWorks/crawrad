---
id: doc-note-suggested-note-id
kind: doc
target_file: src/doc_note.rs
line_hint: 849
author: 
created: 
title: "suggested_note_id"
tags: []
signature: "pub fn suggested_note_id (target : & DocTarget) -> String"
---

Suggested note id: owner and name in kebab case, or just name in kebab case.

# Examples

```rust
use crawrad_project::doc_note::{doc_target_at, suggested_note_id};

let target = doc_target_at("pub fn do_something() {}", 0).unwrap();
assert_eq!(suggested_note_id(&target), "do-something");
```
