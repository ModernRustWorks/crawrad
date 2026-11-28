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
// This example used to assert the first drain was EMPTY. It raced the operating
// system and failed at random on macOS, where FSEvents replays the folder's own
// creation a moment earlier on its own schedule -- it broke CI on 28/11/2026 for
// a change in another crate entirely. A drain is a reader, not a promise about
// when the OS speaks, so the example reads it; `watcher_detects_changes_and_
// filters_target` is where the behaviour is asserted, with a deadline.
for changed in watcher.drain() {
    println!("{}", changed.display());
}
drop(watcher);
let _ = std::fs::remove_dir_all(&dir);
```
