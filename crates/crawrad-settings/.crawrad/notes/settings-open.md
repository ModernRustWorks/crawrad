---
id: settings-open
kind: doc
target_file: src/lib.rs
line_hint: 53
author: 
created: 
title: "open"
tags: []
signature: "pub fn open (file : & Path) -> Result < Self , SettingsError >"
---

Opens or creates a SQLite settings store at the given database file path.

# Errors

Returns `SettingsError` when SQLite cannot be opened or written.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
assert!(store.get("unknown-key").is_none());
```
