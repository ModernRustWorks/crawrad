---
id: build-settings-buildstate-finish-failed-run
kind: doc
target_file: src/build_settings.rs
line_hint: 401
author: 
created: 
title: "BuildState::finish_failed_run"
tags: []
signature: "pub fn finish_failed_run (& mut self , mode : BuildMode , elapsed : std :: time :: Duration , err : & str)"
---

Marks a run as failed with 1 error, recording the error in the log.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_settings::{BuildMode, BuildState};

let mut state = BuildState::default();
state.finish_failed_run(BuildMode::Build, Duration::ZERO, "failed");
```
