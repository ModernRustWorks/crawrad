---
id: recover-pending
kind: doc
target_file: src/recover.rs
line_hint: 23
author: 
created: 
title: "Pending"
tags: []
signature: "pub struct Pending"
---

One copy the IDE can offer back.

# Examples

```rust
use std::path::PathBuf;
use std::time::SystemTime;
use crawrad_project::recover::Pending;

let pending = Pending {
    file: PathBuf::from("src/lib.rs"),
    copy: SystemTime::UNIX_EPOCH,
    disk: None,
};
assert_eq!(pending.file, PathBuf::from("src/lib.rs"));
```
