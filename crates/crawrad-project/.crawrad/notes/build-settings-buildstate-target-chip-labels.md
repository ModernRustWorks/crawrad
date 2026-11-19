---
id: build-settings-buildstate-target-chip-labels
kind: doc
target_file: src/build_settings.rs
line_hint: 456
author: 
created: 
title: "BuildState::target_chip_labels"
tags: []
signature: "pub fn target_chip_labels (& self) -> Vec < String >"
---

List of target chip labels for the active target selection.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert!(state.target_chip_labels().is_empty());
```
