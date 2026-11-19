---
id: doc-note-doc-skeleton
kind: doc
target_file: src/doc_note.rs
line_hint: 1025
author: 
created: 
title: "doc_skeleton"
tags: []
signature: "pub fn doc_skeleton (source : & str , target : & DocTarget , crate_name : Option < & str >) -> String"
---

Generates a rustdoc skeleton for an item from its signature and facts.

# Examples

```rust
use crawrad_project::doc_note::{doc_skeleton, doc_target_at};

let src = "pub fn calculate() -> i32 { 42 }";
let target = doc_target_at(src, 0).unwrap();
let skeleton = doc_skeleton(src, &target, Some("my_crate"));
assert!(skeleton.contains("# Examples"));
```
