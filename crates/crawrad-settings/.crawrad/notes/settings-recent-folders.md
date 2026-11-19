---
id: settings-recent-folders
kind: doc
target_file: src/lib.rs
line_hint: 197
author: 
created: 
title: "recent_folders"
tags: []
signature: "pub fn recent_folders (& mut self , limit : usize) -> Vec < PathBuf >"
---

Returns recent folder paths ordered by most recently opened first.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.push_recent_folder(Path::new("/workspace/app")).unwrap();
let recents = store.recent_folders(10);
assert_eq!(recents.len(), 1);
```
