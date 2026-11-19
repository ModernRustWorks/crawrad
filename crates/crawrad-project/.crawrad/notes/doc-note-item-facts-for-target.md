---
id: doc-note-item-facts-for-target
kind: doc
target_file: src/doc_note.rs
line_hint: 756
author: 
created: 
title: "item_facts_for_target"
tags: []
signature: "pub fn item_facts_for_target (source : & str , target : & DocTarget) -> ItemFacts"
---

Extracts `ItemFacts` for a `DocTarget` by inspecting its signature and body.

# Examples

```rust
use crawrad_project::doc_note::{doc_target_at, item_facts_for_target};

let src = "pub fn add(a: i32, b: i32) -> i32 { a + b }";
let target = doc_target_at(src, 0).unwrap();
let facts = item_facts_for_target(src, &target);
assert!(facts.is_public);
```
