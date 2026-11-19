---
id: build-progress-buildprogress-clock
kind: doc
target_file: src/build_progress.rs
line_hint: 200
author: 
created: 
title: "BuildProgress::clock"
tags: []
signature: "pub fn clock (elapsed : Duration) -> String"
---

Formats an elapsed duration as MM:SS or H:MM:SS.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_progress::BuildProgress;

assert_eq!(BuildProgress::clock(Duration::from_secs(65)), "01:05");
```
