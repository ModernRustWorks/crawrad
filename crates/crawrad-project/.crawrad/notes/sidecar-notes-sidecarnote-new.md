---
id: sidecar-notes-sidecarnote-new
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 61
author: 
created: 
title: "SidecarNote::new"
tags: []
signature: "pub fn new (id : String , target_file : & str , line_hint : usize) -> Self"
---

Creates a new empty note associated with a file and line hint.

# Examples

```rust
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::new("note_1".into(), "src/main.rs", 10);
assert_eq!(note.id, "note_1");
```
