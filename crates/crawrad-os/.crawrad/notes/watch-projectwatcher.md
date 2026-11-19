---
id: watch-projectwatcher
kind: doc
target_file: src/watch.rs
line_hint: 46
author: 
created: 
title: "ProjectWatcher"
tags: []
signature: "pub struct ProjectWatcher"
---

Watches project roots for filesystem changes.

One watcher for a project root. Changes arrive as paths; every batch wakes the UI (`crate::tasks::wake`).

# Examples

```rust
use std::path::Path;
use crawrad_os::watch::ProjectWatcher;

// An empty folder of its own: a whole temp dir can hold folders the OS will not watch.
let dir = std::env::temp_dir().join(format!("crawrad-watch-doc-root-{}", std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let mut watcher = ProjectWatcher::start(&dir).unwrap();
assert_eq!(watcher.root(), dir.as_path());
let _ = watcher.drain();
drop(watcher);
let _ = std::fs::remove_dir_all(&dir);
```
