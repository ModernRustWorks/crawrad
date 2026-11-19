---
id: note-preview-doctestoutcome-label
kind: doc
target_file: src/note_preview.rs
line_hint: 180
author: 
created: 
title: "DoctestOutcome::label"
tags: []
signature: "pub fn label (& self) -> String"
---

The line the dialog shows beside the example.

# Examples

```rust
use crawrad_project::note_preview::DoctestOutcome;

let outcome = DoctestOutcome::NoTest;
assert!(outcome.label().contains("No doctest ran"));
```
