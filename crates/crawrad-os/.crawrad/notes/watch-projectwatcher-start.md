---
id: watch-projectwatcher-start
kind: doc
target_file: src/watch.rs
line_hint: 54
author: 
created: 
title: "ProjectWatcher::start"
tags: []
signature: "pub fn start (root : & Path) -> Result < Self , String >"
---

Watches `root` recursively.

Watches `root` recursively. Err(text) when the OS refuses.

# Examples

```rust
use std::path::Path;
use crawrad_os::watch::ProjectWatcher;

let dir = std::env::temp_dir();
let watcher = ProjectWatcher::start(&dir);
assert!(watcher.is_ok());
```

# Errors

Returns an error if the underlying OS watcher fails to initialize or watch `root`.
