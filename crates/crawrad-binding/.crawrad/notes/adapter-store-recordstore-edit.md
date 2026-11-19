---
id: adapter-store-recordstore-edit
kind: doc
target_file: src/adapter/store.rs
line_hint: 39
author: 
created: 
title: "RecordStore::edit"
tags: []
signature: "fn edit (& self) -> Result < () , String >"
---

Enters `Edit` state, capturing a buffer for potential rollback.

# Examples

```rust
use crawrad_binding::{DataRecord, DatasetState, MemoryStore, RecordStore};

let mut row = DataRecord::new();
row.insert("id".to_string(), "1".to_string());
let store = MemoryStore::new(&["id"], vec![row]);
assert!(store.edit().is_ok());
assert_eq!(store.state(), DatasetState::Edit);
```

# Errors

Returns an error if the store cannot transition to edit mode.
