---
id: build-settings-buildstate-run
kind: doc
target_file: src/build_settings.rs
line_hint: 251
author: 
created: 
title: "BuildState::run"
tags: []
signature: "pub fn run (& self , m : BuildMode) -> Option < & BuildProgress >"
---

Returns the progress tracker of the last run of command `m`.

# Examples

```rust
use crawrad_project::build_settings::{BuildMode, BuildState};

let state = BuildState::default();
assert!(state.run(BuildMode::Build).is_none());
```
