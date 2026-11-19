---
id: build-progress-buildprogress
kind: doc
target_file: src/build_progress.rs
line_hint: 43
author: 
created: 
title: "BuildProgress"
tags: []
signature: "pub struct BuildProgress"
---

What the Build window shows while cargo works, fed line by line.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.verb(), "Building");
```
