---
id: build-settings-buildstate-needs-tick
kind: doc
target_file: src/build_settings.rs
line_hint: 289
author: 
created: 
title: "BuildState::needs_tick"
tags: []
signature: "pub fn needs_tick (& self) -> bool"
---

Returns whether any build animation requires clock ticks.

Whether any build animation (run, corner card linger, or page turn) requires clock ticks.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert!(!state.needs_tick());
```
