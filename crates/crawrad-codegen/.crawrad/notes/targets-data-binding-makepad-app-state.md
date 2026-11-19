---
id: targets-data-binding-makepad-app-state
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 420
author: 
created: 
title: "makepad_app_state"
tags: []
signature: "pub fn makepad_app_state (controls : & [BoundControl] ,) -> Option < crate :: targets :: makepad :: components :: codegen :: AppState >"
---

Generates Makepad AppState contributions for bound controls.

Makepad `AppState` contributions: field definitions and imports for the
`App` struct.

# Examples

```rust
use crawrad_codegen::targets::data_binding::makepad_app_state;

let state = makepad_app_state(&[]);
assert!(state.is_none());
```
