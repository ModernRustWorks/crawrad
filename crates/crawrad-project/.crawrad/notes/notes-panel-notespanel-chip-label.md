---
id: notes-panel-notespanel-chip-label
kind: doc
target_file: src/notes_panel.rs
line_hint: 291
author: 
created: 
title: "NotesPanel::chip_label"
tags: []
signature: "pub fn chip_label (& self , kind : KindFilter) -> String"
---

Returns the label for a kind filter chip including its count.

# Examples

```rust
use crawrad_project::notes_panel::{KindFilter, NotesPanel};

let panel = NotesPanel::default();
assert_eq!(panel.chip_label(KindFilter::All), "All 0");
```
