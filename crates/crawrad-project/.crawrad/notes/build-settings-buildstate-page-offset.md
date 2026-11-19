---
id: build-settings-buildstate-page-offset
kind: doc
target_file: src/build_settings.rs
line_hint: 340
author: 
created: 
title: "BuildState::page_offset"
tags: []
signature: "pub fn page_offset (& self) -> f64"
---

Horizontal offset in px of the incoming page during animation.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert_eq!(state.page_offset(), 0.0);
```
