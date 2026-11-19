---
id: settings-push-recent-folder
kind: doc
target_file: src/lib.rs
line_hint: 220
author: 
created: 
title: "push_recent_folder"
tags: []
signature: "pub fn push_recent_folder (& mut self , folder : & Path) -> Result < () , SettingsError >"
---

Records a folder path, bumping its timestamp to the top of recents.

# Errors

Returns `SettingsError` when SQLite cannot be opened or written.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.push_recent_folder(Path::new("/workspace/project")).unwrap();
```
