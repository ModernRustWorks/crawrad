---
id: build-settings-buildstate-take-running
kind: doc
target_file: src/build_settings.rs
line_hint: 393
author: 
created: 
title: "BuildState::take_running"
tags: []
signature: "pub fn take_running (& mut self) -> (BuildMode , std :: time :: Duration)"
---

Takes the running state, returning the running mode and elapsed duration.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_settings::{BuildMode, BuildState};

let mut state = BuildState::default();
assert_eq!(state.take_running(), (BuildMode::Build, Duration::ZERO));
```
