---
id: adapter-store-memorystore-new
kind: doc
target_file: src/adapter/store.rs
line_hint: 80
author: 
created: 
title: "MemoryStore::new"
tags: []
signature: "pub fn new (schema_fields : & [& str] , records : Vec < DataRecord >) -> Self"
---

Creates a new in-memory store with schema and records.

Creates a new in-memory store initialized with schema columns and initial records.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["name"], Vec::new());
assert_eq!(store.columns(), vec!["name".to_string()]);
```
