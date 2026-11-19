---
id: note-is-documented
kind: doc
target_file: src/note.rs
line_hint: 1929
author: 
created: 
title: "is_documented"
tags: []
signature: "fn is_documented (attrs : & [syn :: Attribute]) -> bool"
---

Checks whether attributes contain valid documentation.

Non-empty rustdoc, a doc note (the note is the documentation), or
`#[doc(hidden)]`, which the lint never asks to document.
