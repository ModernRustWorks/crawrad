---
id: build-progress-buildprogress-done
kind: doc
target_file: src/build_progress.rs
line_hint: 298
author: 
created: 
title: "BuildProgress::done"
tags: []
signature: "pub fn done (& self) -> usize"
---

The number of completed crates so far.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.done(), 0);
```
