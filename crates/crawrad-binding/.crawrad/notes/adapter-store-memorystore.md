---
id: adapter-store-memorystore
kind: doc
target_file: src/adapter/store.rs
line_hint: 74
author: 
created: 
title: "MemoryStore"
tags: []
signature: "pub struct MemoryStore"
---

In-memory implementation of [`RecordStore`] over record maps.

In-memory implementation of [`RecordStore`] operating over stringified record maps.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
assert_eq!(store.len(), 0);
```
