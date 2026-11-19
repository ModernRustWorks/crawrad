---
id: note-preview-doctestoutcome
kind: doc
target_file: src/note_preview.rs
line_hint: 162
author: 
created: 
title: "DoctestOutcome"
tags: []
signature: "pub enum DoctestOutcome"
---

What a doctest run answered.

# Examples

```rust
use crawrad_project::note_preview::DoctestOutcome;

let outcome = DoctestOutcome::NoTest;
assert!(!outcome.passed());
```
