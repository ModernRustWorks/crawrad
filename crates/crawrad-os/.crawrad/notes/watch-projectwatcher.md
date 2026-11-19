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

let dir = std::env::temp_dir();
if let Ok(mut watcher) = ProjectWatcher::start(&dir) {
    assert_eq!(watcher.root(), dir.as_path());
    let _ = watcher.drain();
}
```
