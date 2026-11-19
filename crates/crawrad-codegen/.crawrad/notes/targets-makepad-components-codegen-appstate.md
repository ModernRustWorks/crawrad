---
id: targets-makepad-components-codegen-appstate
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 99
author: 
created: 
title: "AppState"
tags: []
signature: "pub struct AppState"
---

A component's part of the generated App.

A component's part of the generated `App` (see [`MakepadComponentCodegen::app_state`]).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::codegen::AppState;

let a = AppState::default();
assert!(a.field.is_empty());
```
