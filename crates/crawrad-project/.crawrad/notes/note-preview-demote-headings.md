---
id: note-preview-demote-headings
kind: doc
target_file: src/note_preview.rs
line_hint: 21
author: 
created: 
title: "demote_headings"
tags: []
signature: "pub fn demote_headings (body : & str) -> String"
---

Demotes top-level section headings in markdown by three levels.

Section headings drop three levels (`# Examples` becomes `#### Examples`):
rustdoc shows them as section titles, and the Makepad markdown draws a
level-1 heading at 4.5 times the text. Lines inside a code fence stay.

# Examples

```rust
use crawrad_project::note_preview::demote_headings;

let demoted = demote_headings("# Examples");
assert!(demoted.starts_with("#### Examples"));
```
