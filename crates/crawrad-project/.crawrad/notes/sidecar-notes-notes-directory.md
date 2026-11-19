---
id: sidecar-notes-notes-directory
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 197
author: 
created: 
title: "notes_directory"
tags: []
signature: "pub fn notes_directory (project_root : & Path) -> PathBuf"
---

Returns the sidecar notes storage directory.

Returns the sidecar notes storage directory (`.crawrad/notes/` under the workspace root).

# Examples

```rust
use std::path::Path;
use crawrad_project::sidecar_notes::notes_directory;

assert_eq!(notes_directory(Path::new("root")), Path::new("root/.crawrad/notes"));
```
