---
id: note-dialog-noteview-section-tone
kind: doc
target_file: src/note_dialog.rs
line_hint: 322
author: 
created: 
title: "NoteView::section_tone"
tags: []
signature: "pub fn section_tone (& self , section : & Section) -> Tone"
---

A section button is warm while the item still misses that section.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::{NoteDialogModel, Tone};
use crawrad_project::doc_note::Section;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert_eq!(view.section_tone(&Section::Examples), Tone::Plain);
```
