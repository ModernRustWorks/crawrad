---
id: pull-state
kind: doc
target_file: src/pull.rs
line_hint: 29
author: 
created: 
title: "State"
tags: []
signature: "pub struct State"
---

Each file's two lists, by [`file_key`], and the open files to ask again.

# Examples

```rust
use crawrad_lsp::pull::State;

let state = State::default();
assert!(state.open().is_empty());
```
