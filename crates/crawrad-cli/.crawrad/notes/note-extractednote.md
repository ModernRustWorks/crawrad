---
id: note-extractednote
kind: doc
target_file: src/note.rs
line_hint: 203
author: 
created: 
title: "ExtractedNote"
tags: []
signature: "pub struct ExtractedNote"
---

A `///` block turned into a doc note.

# Examples

```rust
use crawrad_cli::note::ExtractedNote;

let note = ExtractedNote {
    id: "extracted_fn".to_string(),
    file: "src/lib.rs".to_string(),
    line: 20,
    item: "run".to_string(),
};
assert_eq!(note.item, "run");
```
