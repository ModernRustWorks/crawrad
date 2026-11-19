---
id: build-progress-buildprogress-headline
kind: doc
target_file: src/build_progress.rs
line_hint: 155
author: 
created: 
title: "BuildProgress::headline"
tags: []
signature: "pub fn headline (& self) -> String"
---

Headline describing the current status of the build.

Running states distinguish whether a specific crate is actively being
compiled/checked. Finished states summarize the outcome.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.headline(), "Building demo…");
```
