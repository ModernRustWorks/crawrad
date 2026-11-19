---
id: notes-panel-notespanel-footer
kind: doc
target_file: src/notes_panel.rs
line_hint: 281
author: 
created: 
title: "NotesPanel::footer"
tags: []
signature: "pub fn footer (& self) -> String"
---

Formats the panel footer text.

Formats the panel footer text (e.g. `".crawrad/notes · 9 files"` or `".crawrad/notes · 1 file"`).

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert_eq!(panel.footer(), ".crawrad/notes \u{00B7} 0 files");
```
