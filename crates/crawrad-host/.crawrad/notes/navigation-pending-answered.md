---
id: navigation-pending-answered
kind: doc
target_file: src/navigation.rs
line_hint: 88
author: 
created: 
title: "Pending::answered"
tags: []
signature: "pub fn answered (& mut self , ticket : u64) -> Option < (PathBuf , i32) >"
---

The answer for `ticket` arrived: the file and version asked, once.

# Examples

```rust
use crawrad_host::navigation::Pending;
use std::path::PathBuf;

let mut pending = Pending::default();
let path = PathBuf::from("src/main.rs");
let ticket = pending.ask(path.clone(), 1);
assert_eq!(pending.answered(ticket), Some((path, 1)));
```
