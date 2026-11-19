---
id: adapter-store-recordstore-record
kind: doc
target_file: src/adapter/store.rs
line_hint: 33
author: 
created: 
title: "RecordStore::record"
tags: []
signature: "fn record (& self , index : usize) -> Option < DataRecord >"
---

Retrieves a record snapshot formatted as string key-value pairs.

Retrieves a record snapshot formatted as string key-value pairs for visual grids.

# Examples

```rust
use crawrad_binding::{DataRecord, MemoryStore, RecordStore};

let mut row = DataRecord::new();
row.insert("id".to_string(), "1".to_string());
let store = MemoryStore::new(&["id"], vec![row]);
assert!(store.record(0).is_some());
```
