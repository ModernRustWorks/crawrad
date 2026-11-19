---
id: sidecar-notes-load-note
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 220
author: 
created: 
title: "load_note"
tags: []
signature: "pub fn load_note (project_root : & Path , id : & str) -> Result < SidecarNote , String >"
---

Loads a SidecarNote by ID.

# Errors

Returns an error message if the note file cannot be found, read, or decoded.

# Examples

```rust
use std::path::Path;
use crawrad_project::sidecar_notes::load_note;

let res = load_note(Path::new("non_existent_root_xyz"), "n1");
assert!(res.is_err());
```
