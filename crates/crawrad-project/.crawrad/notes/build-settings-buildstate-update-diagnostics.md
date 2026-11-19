---
id: build-settings-buildstate-update-diagnostics
kind: doc
target_file: src/build_settings.rs
line_hint: 416
author: 
created: 
title: "BuildState::update_diagnostics"
tags: []
signature: "pub fn update_diagnostics (& mut self , diags : & [crate :: diagnostics_model :: Diagnostic])"
---

Updates error and warning counts from parsed diagnostics.

# Examples

```rust
use crawrad_project::build_settings::BuildState;

let mut state = BuildState::default();
state.update_diagnostics(&[]);
assert_eq!(state.errors_count, 0);
```
