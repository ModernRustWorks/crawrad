---
id: watch-projectwatcher-drain
kind: doc
target_file: src/watch.rs
line_hint: 93
author: 
created: 
title: "ProjectWatcher::drain"
tags: []
signature: "pub fn drain (& mut self) -> Vec < PathBuf >"
---

Returns every path changed since the last call.

Every path that changed since the last call, de-duplicated and sorted; paths under `target/`, `.git/` or `.crawrad/` (at any depth below root) are dropped.

# Examples

```rust
use std::path::Path;
use crawrad_os::watch::ProjectWatcher;

let dir = std::env::temp_dir();
if let Ok(mut watcher) = ProjectWatcher::start(&dir) {
    let changes = watcher.drain();
    assert!(changes.is_empty());
}
```
