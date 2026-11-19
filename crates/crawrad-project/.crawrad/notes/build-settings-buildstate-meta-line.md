---
id: build-settings-buildstate-meta-line
kind: doc
target_file: src/build_settings.rs
line_hint: 464
author: 
created: 
title: "BuildState::meta_line"
tags: []
signature: "pub fn meta_line (& self) -> String"
---

Concise metadata string: profile · platform(s) or host · environment variables.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let state = BuildState::default();
assert!(!state.meta_line().is_empty());
```
