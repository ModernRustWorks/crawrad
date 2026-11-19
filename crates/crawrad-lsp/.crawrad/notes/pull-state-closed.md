---
id: pull-state-closed
kind: doc
target_file: src/pull.rs
line_hint: 68
author: 
created: 
title: "State::closed"
tags: []
signature: "pub fn closed (& mut self , key : & str)"
---

The file closed: nothing more is asked, and its pulled list goes.

# Examples

```rust
use crawrad_lsp::pull::State;

let mut state = State::default();
state.closed("lib.rs");
assert!(state.open().is_empty());
```
