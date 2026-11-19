---
id: pull-state-pulled
kind: doc
target_file: src/pull.rs
line_hint: 45
author: 
created: 
title: "State::pulled"
tags: []
signature: "pub fn pulled (& mut self , key : & str , version : i32 , items : Vec < Value >) -> bool"
---

Records a pulled diagnostic report at `version`, checking if newer.

A `full` answer asked at `version`; `false` when a newer one is
already kept (answers can cross on the wire).

# Examples

```rust
use crawrad_lsp::pull::State;

let mut state = State::default();
assert!(state.pulled("lib.rs", 1, Vec::new()));
```
