---
id: adapter-store-recordstore-delete
kind: doc
target_file: src/adapter/store.rs
line_hint: 54
author: 
created: 
title: "RecordStore::delete"
tags: []
signature: "fn delete (& self) -> Result < () , String >"
---

Deletes the currently active record and repositions the cursor.

# Examples

```rust
use crawrad_binding::{DataRecord, MemoryStore, RecordStore};

let mut row = DataRecord::new();
row.insert("id".to_string(), "1".to_string());
let store = MemoryStore::new(&["id"], vec![row]);
assert!(store.delete().is_ok());
assert_eq!(store.len(), 0);
```

# Errors

Returns an error if there is no active record or if deletion fails.
