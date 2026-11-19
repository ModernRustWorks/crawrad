---
id: sidecar-notes-generate-note-id
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 179
author: 
created: 
title: "generate_note_id"
tags: []
signature: "pub fn generate_note_id (file_path : & str) -> String"
---

Generates a unique note identifier from the target file name.

# Examples

```rust
use crawrad_project::sidecar_notes::generate_note_id;

let id = generate_note_id("src/main.rs");
assert!(id.starts_with("main_"));
```
