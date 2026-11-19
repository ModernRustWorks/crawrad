---
id: pull-state-pushed
kind: doc
target_file: src/pull.rs
line_hint: 56
author: 
created: 
title: "State::pushed"
tags: []
signature: "pub fn pushed (& mut self , key : & str , items : Vec < Value >)"
---

What the server pushed for `key`, replacing its last.

# Examples

```rust
use crawrad_lsp::pull::State;

let mut state = State::default();
state.pushed("lib.rs", Vec::new());
assert!(state.merged("lib.rs").is_empty());
```
