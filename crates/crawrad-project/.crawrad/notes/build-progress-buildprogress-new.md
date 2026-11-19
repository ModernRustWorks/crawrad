---
id: build-progress-buildprogress-new
kind: doc
target_file: src/build_progress.rs
line_hint: 63
author: 
created: 
title: "BuildProgress::new"
tags: []
signature: "pub fn new (verb : & str , package : & str , command_line : & str) -> Self"
---

Creates a new progress tracker initialized with the invoked command line.

The command line is recorded as the first line of the output console
prefixed with `"> "` so that users can inspect what was executed.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.package(), "demo");
```
