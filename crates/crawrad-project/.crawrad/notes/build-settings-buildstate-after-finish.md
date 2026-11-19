---
id: build-settings-buildstate-after-finish
kind: doc
target_file: src/build_settings.rs
line_hint: 347
author: 
created: 
title: "BuildState::after_finish"
tags: []
signature: "pub fn after_finish (& mut self , mode : BuildMode , now : Instant) -> bool"
---

Handles completion decisions and returns whether the window must open.

Handles completion decisions: records finish timestamp, and switches to Errors tab if run failed while closed.
Returns true if the Build window must be opened.

# Examples

```rust
use std::time::Instant;
use crawrad_project::build_settings::{BuildMode, BuildState};

let mut state = BuildState::default();
assert!(!state.after_finish(BuildMode::Build, Instant::now()));
```
