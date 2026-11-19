---
id: note-preview-run-doctest
kind: doc
target_file: src/note_preview.rs
line_hint: 273
author: 
created: 
title: "run_doctest"
tags: []
signature: "pub fn run_doctest (run : & DoctestRun , dir : & std :: path :: Path) -> DoctestOutcome"
---

Runs cargo doctest for the given item and parses its outcome.

Runs `run` with cargo inside `dir` and reads the answer. Blocking: the IDE
calls it from a background task.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::note_preview::{run_doctest, DoctestRun};

let run = DoctestRun::new("pkg", None, "foo");
let _ = run_doctest(&run, Path::new("."));
```
