---
id: build-settings-buildstate-command-view-info
kind: doc
target_file: src/build_settings.rs
line_hint: 428
author: 
created: 
title: "BuildState::command_view_info"
tags: []
signature: "pub fn command_view_info (& self , mode : BuildMode) -> (String , String , String , Option < f64 >)"
---

Computes view information for the given command mode.

Computes headline, counter, clock string, and progress fraction for command `mode`.

# Examples

```rust
use crawrad_project::build_settings::{BuildMode, BuildState};

let state = BuildState::default();
let (headline, _, _, _) = state.command_view_info(BuildMode::Build);
assert_eq!(headline, "Build not run yet");
```
