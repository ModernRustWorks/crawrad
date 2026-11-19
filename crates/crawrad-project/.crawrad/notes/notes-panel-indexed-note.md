---
id: notes-panel-indexed-note
kind: doc
target_file: src/notes_panel.rs
line_hint: 566
author: 
created: 
title: "indexed_note"
tags: []
signature: "fn indexed_note (note : SidecarNote , anchor : Option < (& str , & str , usize) >) -> IndexedNote"
---

Constructs a note as listed in the panel.

A note as the panel lists it; `anchor` is its file, that file's text and
the 1-based line. A documentation note with no title shows its item.
