---
id: doc-note-with-depth
kind: doc
target_file: src/doc_note.rs
line_hint: 1301
author: 
created: 
title: "with_depth"
tags: []
signature: "fn with_depth (s : & str) -> impl Iterator < Item = (usize , char , usize , usize) > + '_"
---

Iterates chars of a string tracking bracket nesting depth.

Every char of `s` with its byte index and the bracket depth before and
after it; `()`, `<>` and `[]` count, the `>` of `->` does not.
