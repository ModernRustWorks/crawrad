---
id: doc-note-apply-note-attribute
kind: doc
target_file: src/doc_note.rs
line_hint: 896
author: 
created: 
title: "apply_note_attribute"
tags: []
signature: "pub fn apply_note_attribute (source : & str , target : & DocTarget , id : & str) -> String"
---

Applies the note attribute to source code and strips existing doc lines.

The source after Save: the item's `///` lines removed (their text moved into the note)
and `#[crawrad_note::note("<id>")]` inserted at `insert_line` with `indent`.
If `existing_note` is Some, the source is returned unchanged.

# Examples

```rust
use crawrad_project::doc_note::{apply_note_attribute, doc_target_at};

let src = "pub fn demo() {}";
let target = doc_target_at(src, 0).unwrap();
let updated = apply_note_attribute(src, &target, "demo-note");
assert!(updated.contains("#[crawrad_note::note(\"demo-note\")]"));
```
