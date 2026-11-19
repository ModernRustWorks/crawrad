---
id: build-progress-finish
kind: doc
target_file: src/build_progress.rs
line_hint: 28
author: 
created: 
title: "Finish"
tags: []
signature: "pub struct Finish"
---

The outcome of the build process once completed.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_progress::Finish;

let fin = Finish {
    success: true,
    cancelled: false,
    errors: 0,
    warnings: 0,
    elapsed: Duration::from_secs(1),
};
assert!(fin.success);
```
