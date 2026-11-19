---
id: build-settings-buildstate-active-or-last-mode
kind: doc
target_file: src/build_settings.rs
line_hint: 365
author: 
created: 
title: "BuildState::active_or_last_mode"
tags: []
signature: "pub fn active_or_last_mode (& self) -> BuildMode"
---

Active running mode, or the last finished mode, or default mode.

# Examples

```rust
use crawrad_project::build_settings::{BuildMode, BuildState};

let state = BuildState::default();
assert_eq!(state.active_or_last_mode(), BuildMode::Build);
```
