---
id: build-settings-buildstate-ensure-default-targets
kind: doc
target_file: src/build_settings.rs
line_hint: 588
author: 
created: 
title: "BuildState::ensure_default_targets"
tags: []
signature: "pub fn ensure_default_targets (& mut self)"
---

Ensures default targets are selected if empty or legacy dual target.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let mut state = BuildState::default();
state.ensure_default_targets();
assert!(!state.selected_targets.is_empty());
```
