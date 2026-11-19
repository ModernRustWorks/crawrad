---
id: note-extract-notes
kind: doc
target_file: src/note.rs
line_hint: 451
author: 
created: 
title: "extract_notes"
tags: []
signature: "pub fn extract_notes (root : & Path , write : bool , all : bool) -> Result < ExtractReport , String >"
---

Extracts `///` doc comments into `.crawrad/notes` doc notes.

Moves the `///` docs of the crate at `root` into `.crawrad/notes` doc notes, leaving `#[note("id")]`.

# Examples

```rust
use std::path::Path;
use crawrad_cli::note::extract_notes;

let res = extract_notes(Path::new("nonexistent_path"), false, false);
assert!(res.is_err());
```

# Errors

Returns an error if `Cargo.toml` is missing or source files cannot be read.
