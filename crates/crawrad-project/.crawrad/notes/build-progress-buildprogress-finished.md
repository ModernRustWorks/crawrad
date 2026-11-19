---
id: build-progress-buildprogress-finished
kind: doc
target_file: src/build_progress.rs
line_hint: 308
author: 
created: 
title: "BuildProgress::finished"
tags: []
signature: "pub fn finished (& self) -> Option < & Finish >"
---

The finished details, if completed.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.finished(), None);
```
