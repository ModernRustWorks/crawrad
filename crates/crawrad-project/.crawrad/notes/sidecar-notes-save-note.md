---
id: sidecar-notes-save-note
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 202
author: 
created: 
title: "save_note"
tags: []
signature: "pub fn save_note (project_root : & Path , note : & SidecarNote) -> Result < PathBuf , String >"
---

Saves a SidecarNote to disk in the `.crawrad/notes/` directory.

# Errors

Returns an error message if the note ID is empty, if the directory cannot
be created, or if writing the note file fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::sidecar_notes::{save_note, SidecarNote};

// Avoid writing files during doctests.
let note = SidecarNote::new("n1".into(), "src/lib.rs", 1);
assert!(save_note(Path::new("."), &note).is_ok());
```
