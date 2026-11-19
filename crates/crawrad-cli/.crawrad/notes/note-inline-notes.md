---
id: note-inline-notes
kind: doc
target_file: src/note.rs
line_hint: 348
author: 
created: 
title: "inline_notes"
tags: []
signature: "pub fn inline_notes (root : & Path , write : bool) -> Result < InlineReport , String >"
---

Inlines doc notes into Rust source files as doc comments.

Inlines doc notes back into Rust source files as `///` doc comments and removes their anchors.

When `write` is true, rewritten files are written to disk and inlined note `.md` files are deleted.
When `write` is false (`--check`), nothing is written to disk.

# Examples

```rust
use std::path::Path;
use crawrad_cli::note::inline_notes;

let res = inline_notes(Path::new("nonexistent_path"), false);
assert!(res.is_err());
```

# Errors

Returns an error if `Cargo.toml` is missing or files cannot be read or written.
