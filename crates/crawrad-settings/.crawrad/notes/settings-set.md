---
id: settings-set
kind: doc
target_file: src/lib.rs
line_hint: 128
author: 
created: 
title: "set"
tags: []
signature: "pub fn set (& mut self , key : & str , value : & str) -> Result < () , SettingsError >"
---

Stores a single key-value preference within an atomic transaction.

# Errors

Returns `SettingsError` when SQLite cannot be opened or written.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.set("editor-tab-size", "4").unwrap();
assert_eq!(store.get("editor-tab-size"), Some("4".to_string()));
```
