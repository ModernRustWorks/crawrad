---
id: adapter-store-recordstore-cancel
kind: doc
target_file: src/adapter/store.rs
line_hint: 51
author: 
created: 
title: "RecordStore::cancel"
tags: []
signature: "fn cancel (& self) -> Result < () , String >"
---

Cancels buffered modifications and returns to `Browse` (or `Inactive`).

# Examples

```rust
use crawrad_binding::{DatasetState, MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
store.insert().unwrap();
assert!(store.cancel().is_ok());
assert_eq!(store.state(), DatasetState::Inactive);
```

# Errors

Returns an error if canceling the active changes fails.
