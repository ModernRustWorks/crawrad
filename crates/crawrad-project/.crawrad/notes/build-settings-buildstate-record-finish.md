---
id: build-settings-buildstate-record-finish
kind: doc
target_file: src/build_settings.rs
line_hint: 284
author: 
created: 
title: "BuildState::record_finish"
tags: []
signature: "pub fn record_finish (& mut self , m : BuildMode , at : Instant)"
---

Records completion of a run for mode `m` at the given instant.

# Examples

```rust
use std::time::Instant;
use crawrad_project::build_settings::{BuildMode, BuildState};

let mut state = BuildState::default();
state.record_finish(BuildMode::Build, Instant::now());
assert!(state.finished_at.is_some());
```
