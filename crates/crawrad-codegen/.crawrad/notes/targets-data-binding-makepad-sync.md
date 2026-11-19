---
id: targets-data-binding-makepad-sync
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 464
author: 
created: 
title: "makepad_sync"
tags: []
signature: "pub fn makepad_sync (controls : & [BoundControl]) -> String"
---

Makepad `sync_ui` lines: opens datasets once and applies model -> widget sync.

# Examples

```rust
use crawrad_codegen::targets::data_binding::makepad_sync;

let code = makepad_sync(&[]);
assert!(code.is_empty());
```
