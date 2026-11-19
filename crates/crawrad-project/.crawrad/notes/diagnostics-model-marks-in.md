---
id: diagnostics-model-marks-in
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 81
author: 
created: 
title: "marks_in"
tags: []
signature: "pub fn marks_in (diags : & [Diagnostic] , name : & str) -> Vec < (usize , usize , bool) >"
---

Extracts editor diagnostic marks for the given file name.

The build's diagnostics of the file named `name` as the editor marks
them without a language server: (0-based line, 0-based column, error?).
The build gives a place, not a range; the mark is 8 bytes from it.

# Examples

```rust
use crawrad_project::diagnostics_model::marks_in;

let marks = marks_in(&[], "main.rs");
assert!(marks.is_empty());
```
