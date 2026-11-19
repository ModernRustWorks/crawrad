---
id: build-settings-buildstate
kind: doc
target_file: src/build_settings.rs
line_hint: 191
author: 
created: 
title: "BuildState"
tags: []
signature: "pub struct BuildState"
---

The whole state of the Build Manager (ADR 0043, PR 7).

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert!(!state.dialog);
```
