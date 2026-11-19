---
id: build-progress-buildprogress-clear-log
kind: doc
target_file: src/build_progress.rs
line_hint: 230
author: 
created: 
title: "BuildProgress::clear_log"
tags: []
signature: "pub fn clear_log (& mut self)"
---

Clears the accumulated log lines.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let mut progress = BuildProgress::new("Building", "demo", "cargo build");
progress.clear_log();
assert!(progress.log().is_empty());
```
