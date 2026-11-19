---
id: build-settings-buildstate-refuse-busy
kind: doc
target_file: src/build_settings.rs
line_hint: 387
author: 
created: 
title: "BuildState::refuse_busy"
tags: []
signature: "pub fn refuse_busy (& mut self) -> String"
---

Refuses a command because a build is currently running.

Refuses a command because a build is running: returns the bar's text
and holds it there for [`REFUSAL_HOLD`].

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let mut state = BuildState::default();
assert!(state.refuse_busy().contains("already building"));
```
