---
id: build-progress-buildprogress-append-log-lines
kind: doc
target_file: src/build_progress.rs
line_hint: 223
author: 
created: 
title: "BuildProgress::append_log_lines"
tags: []
signature: "pub fn append_log_lines < S : Into < String > > (& mut self , lines : impl IntoIterator < Item = S >)"
---

Appends multiple lines to the log.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let mut progress = BuildProgress::new("Building", "demo", "cargo build");
progress.append_log_lines(["line 1", "line 2"]);
assert_eq!(progress.log().len(), 3);
```
