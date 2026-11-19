---
id: disk-watch-diskreaction
kind: doc
target_file: src/disk_watch.rs
line_hint: 41
author: 
created: 
title: "DiskReaction"
tags: []
signature: "pub struct DiskReaction"
---

What the shell must do for one changed open file.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::disk_watch::{DiskAction, DiskReaction};

let reaction = DiskReaction {
    path: PathBuf::from("src/main.rs"),
    name: "main.rs".into(),
    action: DiskAction::Same,
    active: true,
};
assert_eq!(reaction.action, DiskAction::Same);
```
