---
id: recover-mirror-sync
kind: doc
target_file: src/recover.rs
line_hint: 64
author: 
created: 
title: "Mirror::sync"
tags: []
signature: "pub fn sync < 'a > (& self , edits : impl IntoIterator < Item = (& 'a Path , & 'a str) > ,) -> std :: io :: Result < usize >"
---

One pass: the copies follow the unsaved edits, the waiting ones stay.

# Errors

Returns an I/O error if creating directories or writing recovery copies fails.

# Examples

```rust
use std::path::Path;
use crawrad_project::recover::Mirror;

let mirror = Mirror::default();
let res = mirror.sync([]);
assert_eq!(res.unwrap(), 0);
```
