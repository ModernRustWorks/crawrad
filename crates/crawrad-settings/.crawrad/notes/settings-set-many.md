---
id: settings-set-many
kind: doc
target_file: src/lib.rs
line_hint: 150
author: 
created: 
title: "set_many"
tags: []
signature: "pub fn set_many (& mut self , pairs : & [(& str , & str)]) -> Result < () , SettingsError >"
---

Stores multiple key-value pairs within a single atomic transaction.

# Errors

Returns `SettingsError` when SQLite cannot be opened or written.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.set_many(&[("key1", "val1"), ("key2", "val2")]).unwrap();
assert_eq!(store.get("key1"), Some("val1".to_string()));
```
