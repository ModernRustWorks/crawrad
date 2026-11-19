---
id: build-progress-buildprogress-log
kind: doc
target_file: src/build_progress.rs
line_hint: 213
author: 
created: 
title: "BuildProgress::log"
tags: []
signature: "pub fn log (& self) -> & [String]"
---

Returns the accumulated log lines.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.log().len(), 1);
```
