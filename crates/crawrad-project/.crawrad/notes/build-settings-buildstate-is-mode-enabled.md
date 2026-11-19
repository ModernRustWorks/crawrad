---
id: build-settings-buildstate-is-mode-enabled
kind: doc
target_file: src/build_settings.rs
line_hint: 360
author: 
created: 
title: "BuildState::is_mode_enabled"
tags: []
signature: "pub fn is_mode_enabled (& self , mode : BuildMode) -> bool"
---

Returns whether command `mode` can currently be executed.

Whether command `mode` can be executed (enabled unless another command is running).

# Examples

```rust
use crawrad_project::build_settings::{BuildMode, BuildState};

let state = BuildState::default();
assert!(state.is_mode_enabled(BuildMode::Build));
```
