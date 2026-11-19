---
id: adapter-store-recordstore-state
kind: doc
target_file: src/adapter/store.rs
line_hint: 16
author: 
created: 
title: "RecordStore::state"
tags: []
signature: "fn state (& self) -> DatasetState"
---

Current operational state of the dataset.

# Examples

```rust
use crawrad_binding::{DatasetState, MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
assert_eq!(store.state(), DatasetState::Inactive);
```
