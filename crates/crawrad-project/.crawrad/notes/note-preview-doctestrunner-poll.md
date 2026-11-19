---
id: note-preview-doctestrunner-poll
kind: doc
target_file: src/note_preview.rs
line_hint: 307
author: 
created: 
title: "DoctestRunner::poll"
tags: []
signature: "pub fn poll (& mut self , note : Option < & mut crate :: note_dialog :: NoteDialogModel >) -> bool"
---

Hands the answers that arrived to the note; `true` when one did.

# Examples

```rust
use crawrad_project::note_preview::DoctestRunner;

let mut runner = DoctestRunner::default();
assert!(!runner.poll(None));
```
