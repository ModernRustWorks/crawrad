---
id: note-scan
kind: doc
target_file: src/note.rs
line_hint: 250
author: 
created: 
title: "scan"
tags: []
signature: "pub fn scan (root : & Path) -> Result < NoteReport , String >"
---

Scans a crate root for notes and source anchors, building a report.

# Examples

```rust
use std::path::Path;
use crawrad_cli::note::scan;

let result = scan(Path::new("nonexistent_path"));
assert!(result.is_err());
```

# Errors

Returns an error if `Cargo.toml` is missing or source files cannot be read.
