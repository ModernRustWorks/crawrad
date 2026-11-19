---
id: build-settings-buildstate-card-now
kind: doc
target_file: src/build_settings.rs
line_hint: 307
author: 
created: 
title: "BuildState::card_now"
tags: []
signature: "pub fn card_now (& self) -> Option < crate :: build_busy :: Card >"
---

Returns the corner card model for the active or lingering run.

Returns the corner card model for the currently running or lingering finished run.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert!(state.card_now().is_none());
```
