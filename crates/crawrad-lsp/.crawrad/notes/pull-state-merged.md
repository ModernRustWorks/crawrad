---
id: pull-state-merged
kind: doc
target_file: src/pull.rs
line_hint: 61
author: 
created: 
title: "State::merged"
tags: []
signature: "pub fn merged (& self , key : & str) -> Vec < Value >"
---

Pushed, then pulled.

# Examples

```rust
use crawrad_lsp::pull::State;

let state = State::default();
assert!(state.merged("lib.rs").is_empty());
```
