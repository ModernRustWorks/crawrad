---
id: build-progress-buildprogress-verb
kind: doc
target_file: src/build_progress.rs
line_hint: 283
author: 
created: 
title: "BuildProgress::verb"
tags: []
signature: "pub fn verb (& self) -> & str"
---

The action verb being executed.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.verb(), "Building");
```
