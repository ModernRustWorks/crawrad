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

Err(text) when the OS refuses.

# Examples

```rust
use std::path::Path;
use crawrad_os::watch::ProjectWatcher;

// An empty folder of its own: a whole temp dir can hold folders the OS will not watch.
let dir = std::env::temp_dir().join(format!("crawrad-watch-doc-start-{}", std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let watcher = ProjectWatcher::start(&dir);
assert!(watcher.is_ok());
drop(watcher);
let _ = std::fs::remove_dir_all(&dir);
```

# Errors

Returns an error if the underlying OS watcher fails to initialize or watch `root`.
