---
id: build-progress-buildprogress-counter
kind: doc
target_file: src/build_progress.rs
line_hint: 179
author: 
created: 
title: "BuildProgress::counter"
tags: []
signature: "pub fn counter (& self) -> Option < String >"
---

Returns a formatted progress counter or diagnostic summary.

Progress counter string, e.g. `"38 / 214 crates"` when running with a known
total, or a diagnostic summary when finished.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.counter(), None);
```
