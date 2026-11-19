---
id: build-progress-buildprogress-log-text
kind: doc
target_file: src/build_progress.rs
line_hint: 235
author: 
created: 
title: "BuildProgress::log_text"
tags: []
signature: "pub fn log_text (& self) -> String"
---

Returns the full console output text joined with newlines.

Returns the full console output text joined with newlines, with a trailing newline.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.log_text(), "> cargo build\n");
```
