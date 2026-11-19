---
id: settings-get
kind: doc
target_file: src/lib.rs
line_hint: 111
author: 
created: 
title: "get"
tags: []
signature: "pub fn get (& mut self , key : & str) -> Option < String >"
---

Retrieves a single preference value by its key.

# Examples

```rust
use crawrad_settings::SettingsStore;
use std::path::Path;

let mut store = SettingsStore::open(Path::new(":memory:")).unwrap();
store.set("sample-key", "sample-val").unwrap();
assert_eq!(store.get("sample-key"), Some("sample-val".to_string()));
```
