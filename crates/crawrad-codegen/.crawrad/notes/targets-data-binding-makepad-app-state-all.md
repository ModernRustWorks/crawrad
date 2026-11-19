---
id: targets-data-binding-makepad-app-state-all
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 386
author: 
created: 
title: "makepad_app_state_all"
tags: []
signature: "pub fn makepad_app_state_all (controls : & [BoundControl] , actions : & [DatasetAction] ,) -> Option < crate :: targets :: makepad :: components :: codegen :: AppState >"
---

Generates Makepad AppState contributions for all bound datasets and actions.

Makepad `AppState` contributions: field definitions and imports for the
`App` struct, covering datasets used by controls or actions.

# Examples

```rust
use crawrad_codegen::targets::data_binding::makepad_app_state_all;

let state = makepad_app_state_all(&[], &[]);
assert!(state.is_none());
```
