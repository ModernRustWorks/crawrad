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

// An empty folder of its own: a whole temp dir can hold folders the OS will not watch.
let dir = std::env::temp_dir().join(format!("crawrad-watch-doc-drain-{}", std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let mut watcher = ProjectWatcher::start(&dir).unwrap();
assert!(watcher.drain().is_empty());
drop(watcher);
let _ = std::fs::remove_dir_all(&dir);
```
