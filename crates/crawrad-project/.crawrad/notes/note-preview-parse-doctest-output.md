---
id: note-preview-parse-doctest-output
kind: doc
target_file: src/note_preview.rs
line_hint: 208
author: 
created: 
title: "parse_doctest_output"
tags: []
signature: "pub fn parse_doctest_output (output : & str , run : & DoctestRun) -> DoctestOutcome"
---

Parses cargo doctest output for tests matching the target path.

Reads cargo's output (stdout and stderr together) of `run`, counting only
the doctests of `run.path`.

# Examples

```rust
use crawrad_project::note_preview::{parse_doctest_output, DoctestOutcome, DoctestRun};

let run = DoctestRun::new("pkg", None, "foo");
let outcome = parse_doctest_output("test result: ok. 0 passed", &run);
assert_eq!(outcome, DoctestOutcome::NoTest);
```
