---
id: build-progress-buildprogress-append-log-line
kind: doc
target_file: src/build_progress.rs
line_hint: 218
author: 
created: 
title: "BuildProgress::append_log_line"
tags: []
signature: "pub fn append_log_line (& mut self , line : impl Into < String >)"
---

Appends a single line to the log.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let mut progress = BuildProgress::new("Building", "demo", "cargo build");
progress.append_log_line("hello");
assert_eq!(progress.log().len(), 2);
```
