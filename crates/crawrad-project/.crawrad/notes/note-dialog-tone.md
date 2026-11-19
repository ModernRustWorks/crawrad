---
id: note-dialog-tone
kind: doc
target_file: src/note_dialog.rs
line_hint: 35
author: 
created: 
title: "Tone"
tags: []
signature: "pub enum Tone"
---

The colour role of a line of the dialog; the shell maps it to its theme.

# Examples

```rust
use crawrad_project::note_dialog::Tone;

assert_eq!(Tone::Plain.rgba(), 0xE7E7EAFF);
```
