---
id: doc-note-suggested-sections
kind: doc
target_file: src/doc_note.rs
line_hint: 813
author: 
created: 
title: "suggested_sections"
tags: []
signature: "pub fn suggested_sections (source : & str , target : & DocTarget) -> Vec < Suggestion >"
---

What the signature (and, for fns, the body) asks the docs to carry.

# Examples

```rust
use crawrad_project::doc_note::{doc_target_at, suggested_sections};

let src = "pub fn check() -> Result<(), ()> { Ok(()) }";
let target = doc_target_at(src, 0).unwrap();
let suggestions = suggested_sections(src, &target);
assert!(!suggestions.is_empty());
```
