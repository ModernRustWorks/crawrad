---
id: note-skippednote
kind: doc
target_file: src/note.rs
line_hint: 181
author: 
created: 
title: "SkippedNote"
tags: []
signature: "pub struct SkippedNote"
---

A note anchor skipped during inlining.

# Examples

```rust
use crawrad_cli::note::SkippedNote;

let skipped = SkippedNote {
    id: "internal_note".to_string(),
    file: "src/lib.rs".to_string(),
    line: 10,
    reason: "note is internal".to_string(),
};
assert_eq!(skipped.id, "internal_note");
```
