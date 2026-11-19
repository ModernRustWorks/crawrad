---
id: navigation-pending-ask
kind: doc
target_file: src/navigation.rs
line_hint: 77
author: 
created: 
title: "Pending::ask"
tags: []
signature: "pub fn ask (& mut self , file : PathBuf , version : i32) -> u64"
---

Asked about `file` at `version`: the ticket its answer must carry.

# Examples

```rust
use crawrad_host::navigation::Pending;
use std::path::PathBuf;

let mut pending = Pending::default();
let ticket = pending.ask(PathBuf::from("src/main.rs"), 1);
assert_eq!(pending.ticket(), Some(ticket));
```
