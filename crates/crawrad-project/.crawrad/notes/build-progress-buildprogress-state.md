---
id: build-progress-buildprogress-state
kind: doc
target_file: src/build_progress.rs
line_hint: 267
author: 
created: 
title: "BuildProgress::state"
tags: []
signature: "pub fn state (& self) -> ProgressState"
---

Returns the high-level progress state.

# Examples

```rust
use crawrad_project::build_progress::{BuildProgress, ProgressState};

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.state(), ProgressState::Running);
```
