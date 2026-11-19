---
id: settings-forget-recent-folder
kind: doc
target_file: src/lib.rs
line_hint: 242
author: 
created: 
title: "forget_recent_folder"
tags: []
signature: "pub fn forget_recent_folder (& mut self , folder : & Path) -> Result < () , SettingsError >"
---

Removes a folder path from the recent folders list in SQLite.

# Errors

Returns `SettingsError` when SQLite cannot be opened or written.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.push_recent_folder(Path::new("/workspace/project")).unwrap();
store.forget_recent_folder(Path::new("/workspace/project")).unwrap();
assert!(store.recent_folders(10).is_empty());
```
