---
id: build-progress-buildprogress-status-line
kind: doc
target_file: src/build_progress.rs
line_hint: 245
author: 
created: 
title: "BuildProgress::status_line"
tags: []
signature: "pub fn status_line (& self , elapsed : Duration) -> String"
---

Concise one-line summary fit for a status bar.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.status_line(Duration::ZERO), "Building demo · 00:00");
```
