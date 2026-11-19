---
id: adapter-store-recordstore-move-to
kind: doc
target_file: src/adapter/store.rs
line_hint: 30
author: 
created: 
title: "RecordStore::move_to"
tags: []
signature: "fn move_to (& self , index : usize) -> Result < () , String >"
---

Moves the active cursor to the specified record index.

# Examples

```rust
use crawrad_binding::{DataRecord, MemoryStore, RecordStore};

let mut row = DataRecord::new();
row.insert("id".to_string(), "1".to_string());
let store = MemoryStore::new(&["id"], vec![row]);
assert!(store.move_to(0).is_ok());
```

# Errors

Returns an error if the index is out of bounds or if navigation is refused during editing.
