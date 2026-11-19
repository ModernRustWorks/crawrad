---
id: notes-panel-alert-reason
kind: doc
target_file: src/notes_panel.rs
line_hint: 489
author: 
created: 
title: "alert_reason"
tags: []
signature: "pub fn alert_reason (note : & SidecarNote , source : Option < & str > , line : Option < usize > ,) -> Option < String >"
---

Returns why a documentation note is in alert, or `None` if it is not.

Why a documentation note is in alert, in a few words: its item's
signature changed, its body is out of the rustdoc pattern, or no source
anchors it (`source` and the 1-based `line` are its anchor). Internal
notes and comments are never in alert.

# Examples

```rust
use crawrad_project::notes_panel::alert_reason;
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::new("n1".into(), "file.rs", 1);
assert_eq!(alert_reason(&note, None, None), None);
```
