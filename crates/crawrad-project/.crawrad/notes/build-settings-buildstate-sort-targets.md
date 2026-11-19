---
id: build-settings-buildstate-sort-targets
kind: doc
target_file: src/build_settings.rs
line_hint: 578
author: 
created: 
title: "BuildState::sort_targets"
tags: []
signature: "pub fn sort_targets (& mut self)"
---

Sorts selected targets in canonical order.

# Examples

```rust
use crawrad_project::build_settings::{BuildState, TargetPlatform};

let mut state = BuildState::default();
state.selected_targets = vec![TargetPlatform::Linux64, TargetPlatform::Win32];
state.sort_targets();
assert_eq!(state.selected_targets[0], TargetPlatform::Win32);
```
