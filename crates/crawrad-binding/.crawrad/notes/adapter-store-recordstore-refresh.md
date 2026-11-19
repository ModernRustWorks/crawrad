---
id: adapter-store-recordstore-refresh
kind: doc
target_file: src/adapter/store.rs
line_hint: 57
author: 
created: 
title: "RecordStore::refresh"
tags: []
signature: "fn refresh (& self) -> Result < () , String >"
---

Reloads records from the underlying storage source.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
assert!(store.refresh().is_ok());
```

# Errors

Returns an error if reloading from the storage source fails.
