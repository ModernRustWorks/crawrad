---
id: settings-all
kind: doc
target_file: src/lib.rs
line_hint: 174
author: 
created: 
title: "all"
tags: []
signature: "pub fn all (& mut self) -> Vec < (String , String) >"
---

Returns all key-value preferences ordered alphabetically by key.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.set("key", "val").unwrap();
let all = store.all();
assert_eq!(all, vec![("key".to_string(), "val".to_string())]);
```
