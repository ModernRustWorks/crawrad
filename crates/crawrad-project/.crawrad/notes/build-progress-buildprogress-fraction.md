---
id: build-progress-buildprogress-fraction
kind: doc
target_file: src/build_progress.rs
line_hint: 139
author: 
created: 
title: "BuildProgress::fraction"
tags: []
signature: "pub fn fraction (& self) -> Option < f64 >"
---

Returns the fractional progress from 0.0 to 1.0, if known.

Returns the fractional progress from 0.0 to 1.0, or `None` if the total
number of crates is unknown. Completed successful builds always report `Some(1.0)`.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "demo", "cargo build");
assert_eq!(progress.fraction(), None);
```
