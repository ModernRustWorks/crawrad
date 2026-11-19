---
id: build-settings-buildstate-run-mut
kind: doc
target_file: src/build_settings.rs
line_hint: 259
author: 
created: 
title: "BuildState::run_mut"
tags: []
signature: "pub fn run_mut (& mut self , m : BuildMode) -> Option < & mut BuildProgress >"
---

Returns a mutable reference to the last run of command `m`.

Returns a mutable reference to the progress tracker of the last run of command `m`.

# Examples

```rust
use crawrad_project::build_settings::{BuildMode, BuildState};

let mut state = BuildState::default();
assert!(state.run_mut(BuildMode::Build).is_none());
```
