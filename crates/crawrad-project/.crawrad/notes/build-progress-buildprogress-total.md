---
id: build-progress-buildprogress-total
kind: doc
target_file: src/build_progress.rs
line_hint: 303
author: 
created: 
title: "BuildProgress::total"
tags: []
signature: "pub fn total (& self) -> Option < usize >"
---

The total number of crates to compile, if known.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.total(), None);
```
