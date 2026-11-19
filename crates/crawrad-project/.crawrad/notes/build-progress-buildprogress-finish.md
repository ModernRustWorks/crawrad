---
id: build-progress-buildprogress-finish
kind: doc
target_file: src/build_progress.rs
line_hint: 129
author: 
created: 
title: "BuildProgress::finish"
tags: []
signature: "pub fn finish (& mut self , f : Finish)"
---

Marks the build as finished with the given outcome.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_progress::{BuildProgress, Finish};

let mut progress = BuildProgress::new("Building", "demo", "cargo build");
progress.finish(Finish {
    success: true,
    cancelled: false,
    errors: 0,
    warnings: 0,
    elapsed: Duration::from_secs(1),
});
assert!(!progress.is_running());
```
