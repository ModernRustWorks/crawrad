---
id: explorer-rows-rows-note-note
kind: doc
target_file: src/explorer_rows/rows/note.rs
line_hint: 11
author: 
created: 
title: "Note"
tags: []
signature: "pub struct Note"
---

One sentence, with or without the warning colour.

# Examples

```rust
use crawrad_project::explorer_rows::rows::note::Note;
use crawrad_project::explorer_rows::row::ExplorerRow;

let n = Note::new("No forms yet", false, 0);
assert_eq!(n.text(), "No forms yet");
```
