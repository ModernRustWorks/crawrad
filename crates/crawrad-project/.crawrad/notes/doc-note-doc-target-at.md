---
id: doc-note-doc-target-at
kind: doc
target_file: src/doc_note.rs
line_hint: 49
author: 
created: 
title: "doc_target_at"
tags: []
signature: "pub fn doc_target_at (source : & str , cursor_line : usize) -> Option < DocTarget >"
---

Identifies the documented Rust item at the given cursor line, if any.

# Examples

```rust
use crawrad_project::doc_note::doc_target_at;

let target = doc_target_at("pub fn hello() {}", 0);
assert!(target.is_some());
```
