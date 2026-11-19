---
id: adapter-store-recordstore-columns
kind: doc
target_file: src/adapter/store.rs
line_hint: 13
author: 
created: 
title: "RecordStore::columns"
tags: []
signature: "fn columns (& self) -> Vec < String >"
---

Column names exposed by this store.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id", "name"], Vec::new());
assert_eq!(store.columns(), vec!["id".to_string(), "name".to_string()]);
```
