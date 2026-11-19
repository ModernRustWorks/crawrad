---
id: build-progress-buildprogress-package
kind: doc
target_file: src/build_progress.rs
line_hint: 288
author: 
created: 
title: "BuildProgress::package"
tags: []
signature: "pub fn package (& self) -> & str"
---

The target package name.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.package(), "demo");
```
