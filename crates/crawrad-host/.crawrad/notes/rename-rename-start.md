---
id: rename-rename-start
kind: doc
target_file: src/rename.rs
line_hint: 155
author: 
created: 
title: "Rename::start"
tags: []
signature: "pub fn start (& mut self , file : PathBuf , caret : (usize , usize) , version : i32 , old : & str) -> u64"
---

The caret's name is asked about; the dialog opens on the old name.

# Examples

```rust
use crawrad_host::rename::Rename;
use std::path::PathBuf;

let mut rename = Rename::default();
let ticket = rename.start(PathBuf::from("src/main.rs"), (0, 0), 1, "old_name");
assert!(rename.is_open());
```
