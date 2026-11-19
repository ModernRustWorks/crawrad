---
id: doc-note-note-mark
kind: doc
target_file: src/doc_note.rs
line_hint: 834
author: 
created: 
title: "note_mark"
tags: []
signature: "pub fn note_mark (root : & Path , id : & str , source : & str , line : usize) -> (String , bool , bool)"
---

Returns the preview text, alert status, and hidden status for a note.

What the editor shows for the anchor of note `id` on `line`: the preview
(title, then body), whether the icon carries the alert dot, and whether
the note is hidden (a resolved comment). A note that cannot be read
previews empty, is not in alert, and is not hidden.

# Examples

```rust
use std::path::Path;
use crawrad_project::doc_note::note_mark;

let (text, alert, hidden) = note_mark(Path::new("."), "nonexistent", "", 0);
assert!(text.is_empty());
assert!(!alert);
assert!(!hidden);
```
