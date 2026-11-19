---
id: note-preview-has-example
kind: doc
target_file: src/note_preview.rs
line_hint: 75
author: 
created: 
title: "has_example"
tags: []
signature: "pub fn has_example (body : & str) -> bool"
---

Checks whether markdown body contains a runnable doctest block.

Whether the body has a fenced code block, which rustdoc runs as a doctest
unless it is marked as another language or `ignore`.

# Examples

```rust
use crawrad_project::note_preview::has_example;

assert!(has_example("```rust\nassert!(true);\n```"));
assert!(!has_example("no code"));
```
