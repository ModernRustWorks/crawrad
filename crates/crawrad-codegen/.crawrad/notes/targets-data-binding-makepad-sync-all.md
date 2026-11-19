---
id: targets-data-binding-makepad-sync-all
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 444
author: 
created: 
title: "makepad_sync_all"
tags: []
signature: "pub fn makepad_sync_all (controls : & [BoundControl] , actions : & [DatasetAction]) -> String"
---

Generates Makepad sync_ui code for datasets, actions, and controls.

Makepad `sync_ui` lines: opens datasets once, sets action enabled, and
applies model -> widget sync.

# Examples

```rust
use crawrad_codegen::targets::data_binding::makepad_sync_all;

let code = makepad_sync_all(&[], &[]);
assert!(code.is_empty());
```
