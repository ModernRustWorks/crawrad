---
id: build-progress-buildprogress-current
kind: doc
target_file: src/build_progress.rs
line_hint: 293
author: 
created: 
title: "BuildProgress::current"
tags: []
signature: "pub fn current (& self) -> Option < & str >"
---

The current crate being compiled or checked, if any.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.current(), None);
```
