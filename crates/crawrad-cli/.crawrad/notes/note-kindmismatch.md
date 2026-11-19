---
id: note-kindmismatch
kind: doc
target_file: src/note.rs
line_hint: 45
author: 
created: 
title: "KindMismatch"
tags: []
signature: "pub struct KindMismatch"
---

An anchor pointing to an internal or comment note.

# Examples

```rust
use crawrad_cli::note::KindMismatch;
use crawrad_note_format::NoteKind;

let mismatch = KindMismatch {
    id: "internal_note".to_string(),
    file: "src/lib.rs".to_string(),
    line: 25,
    kind: NoteKind::Internal,
};
assert_eq!(mismatch.kind, NoteKind::Internal);
```
