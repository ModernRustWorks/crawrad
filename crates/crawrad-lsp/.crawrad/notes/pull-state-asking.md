---
id: pull-state-asking
kind: doc
target_file: src/pull.rs
line_hint: 39
author: 
created: 
title: "State::asking"
tags: []
signature: "pub fn asking (& mut self , key : & str , uri : & str , version : i32)"
---

`file` (its `uri`) at `version` is being asked about.

# Examples

```rust
use crawrad_lsp::pull::State;

let mut state = State::default();
state.asking("lib.rs", "file:///lib.rs", 1);
assert_eq!(state.open().len(), 1);
```
