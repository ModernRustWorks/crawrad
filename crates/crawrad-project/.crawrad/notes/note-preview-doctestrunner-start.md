---
id: note-preview-doctestrunner-start
kind: doc
target_file: src/note_preview.rs
line_hint: 293
author: 
created: 
title: "DoctestRunner::start"
tags: []
signature: "pub fn start (& mut self , note : & mut crate :: note_dialog :: NoteDialogModel)"
---

Starts a doctest run in the background for the given note model.

Starts the note's doctest; does nothing when one is running or the
note has nothing to run.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::note_preview::DoctestRunner;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let mut runner = DoctestRunner::default();
runner.start(&mut model);
```
