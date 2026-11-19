---
id: build-settings-buildstate-start-run
kind: doc
target_file: src/build_settings.rs
line_hint: 267
author: 
created: 
title: "BuildState::start_run"
tags: []
signature: "pub fn start_run (& mut self , m : BuildMode , p : BuildProgress)"
---

Records a new run for command `m`, replacing any previous run of that command.

# Examples

```rust
use crawrad_project::build_progress::BuildProgress;
use crawrad_project::build_settings::{BuildMode, BuildState};

let mut state = BuildState::default();
let progress = BuildProgress::new("Building", "demo", "cargo build");
state.start_run(BuildMode::Build, progress);
assert!(state.run(BuildMode::Build).is_some());
```
