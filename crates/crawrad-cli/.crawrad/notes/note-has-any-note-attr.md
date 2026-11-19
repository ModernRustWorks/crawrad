---
id: note-has-any-note-attr
kind: doc
target_file: src/note.rs
line_hint: 1591
author: 
created: 
title: "has_any_note_attr"
tags: []
signature: "fn has_any_note_attr (lines : & [String]) -> bool"
---

Checks whether source lines contain any `#[note]` attribute.

Whether the source still carries a `#[note]`; one that does not parse
keeps its `use`, the safe side.
