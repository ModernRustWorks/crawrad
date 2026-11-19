---
id: build-progress-buildprogress-is-running
kind: doc
target_file: src/build_progress.rs
line_hint: 134
author: 
created: 
title: "BuildProgress::is_running"
tags: []
signature: "pub fn is_running (& self) -> bool"
---

Returns `true` while the build is still actively running.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert!(progress.is_running());
```
