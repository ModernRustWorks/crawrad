---
id: build-settings-buildstate-rail-label
kind: doc
target_file: src/build_settings.rs
line_hint: 320
author: 
created: 
title: "BuildState::rail_label"
tags: []
signature: "pub fn rail_label (& self , mode : BuildMode) -> String"
---

Returns the rail or toolbar label for the given command mode.

The rail/toolbar label of command `mode`: "◐ Building" while running (or normal text if timing opacity == 0).

# Examples

```rust
use crawrad_project::build_settings::{BuildMode, BuildState};

let state = BuildState::default();
assert_eq!(state.rail_label(BuildMode::Build), "🔨 Build");
```
