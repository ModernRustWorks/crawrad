---
id: pull-state-open
kind: doc
target_file: src/pull.rs
line_hint: 74
author: 
created: 
title: "State::open"
tags: []
signature: "pub fn open (& self) -> Vec < (String , String , i32) >"
---

Every open file: its uri and the version last asked.

# Examples

```rust
use crawrad_lsp::pull::State;

let state = State::default();
assert!(state.open().is_empty());
```
