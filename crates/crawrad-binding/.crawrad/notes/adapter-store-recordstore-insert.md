---
id: adapter-store-recordstore-insert
kind: doc
target_file: src/adapter/store.rs
line_hint: 42
author: 
created: 
title: "RecordStore::insert"
tags: []
signature: "fn insert (& self) -> Result < () , String >"
---

Enters `Insert` state, preparing an empty buffer for a new record.

# Examples

```rust
use crawrad_binding::{DatasetState, MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
assert!(store.insert().is_ok());
assert_eq!(store.state(), DatasetState::Insert);
```

# Errors

Returns an error if the store cannot transition to insert mode.
