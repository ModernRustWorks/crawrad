---
id: note-preview-note-at-definition
kind: doc
target_file: src/note_preview.rs
line_hint: 40
author: 
created: 
title: "note_at_definition"
tags: []
signature: "pub fn note_at_definition (source : & str , line : usize) -> Option < String >"
---

Finds the note ID on the item defined at line of source.

The note on the item defined at `line` of `source` — its
`#[note("<id>")]` — for the rich hover card (#659).

# Examples

```rust
use crawrad_project::note_preview::note_at_definition;

assert_eq!(note_at_definition("fn test() {}", 0), None);
```
