---
id: build-settings-buildstate-status-line-now
kind: doc
target_file: src/build_settings.rs
line_hint: 374
author: 
created: 
title: "BuildState::status_line_now"
tags: []
signature: "pub fn status_line_now (& self) -> Option < String >"
---

Formatted status line of the currently running command.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert_eq!(state.status_line_now(), None);
```
