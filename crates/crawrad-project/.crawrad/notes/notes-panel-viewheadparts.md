---
id: notes-panel-viewheadparts
kind: doc
target_file: src/notes_panel.rs
line_hint: 111
author: 
created: 
title: "ViewHeadParts"
tags: []
signature: "pub type ViewHeadParts = ([String ; 4] , usize , Vec < String > , usize , String , bool , String)"
---

The parts of the panel head as painted by the view.

The panel's head as the view paints it: the four chip labels (All, Documentation, Notes, Comments),
the active chip index, the file choices ("All files" first, then `files()`), the selected choice index,
the alert label, whether "In alert" is on, and the footer.

# Examples

```rust
use crawrad_project::notes_panel::ViewHeadParts;

let head: ViewHeadParts = (
    ["All 0".into(), "Doc 0".into(), "Notes 0".into(), "Comments 0".into()],
    0,
    vec!["All files".into()],
    0,
    "In alert 0".into(),
    false,
    ".crawrad/notes \u{00B7} 0 files".into(),
);
assert_eq!(head.1, 0);
```
