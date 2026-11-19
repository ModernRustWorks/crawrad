---
id: sidecar-notes-list-notes
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 231
author: 
created: 
title: "list_notes"
tags: []
signature: "pub fn list_notes (project_root : & Path) -> Vec < SidecarNote >"
---

Lists all notes present in the `.crawrad/notes/` directory.

# Examples

```rust
use std::path::Path;
use crawrad_project::sidecar_notes::list_notes;

let notes = list_notes(Path::new("non_existent_root_xyz"));
assert!(notes.is_empty());
```
