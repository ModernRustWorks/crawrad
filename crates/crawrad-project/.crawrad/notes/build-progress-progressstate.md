---
id: build-progress-progressstate
kind: doc
target_file: src/build_progress.rs
line_hint: 15
author: 
created: 
title: "ProgressState"
tags: []
signature: "pub enum ProgressState"
---

High-level lifecycle state of the build process.

# Examples

```rust
use crawrad_project::build_progress::ProgressState;

assert_eq!(ProgressState::Running, ProgressState::Running);
```
