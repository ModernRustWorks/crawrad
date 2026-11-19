---
id: recover-pending-2
kind: doc
target_file: src/recover.rs
line_hint: 142
author: 
created: 
title: "pending"
tags: []
signature: "pub fn pending (root : & Path) -> Vec < Pending >"
---

The copies newer than their file on disk (or whose file is gone), in
path order.

# Examples

```rust
use std::path::Path;
use crawrad_project::recover::pending;

let list = pending(Path::new("non_existent_dir_xyz"));
assert!(list.is_empty());
```
