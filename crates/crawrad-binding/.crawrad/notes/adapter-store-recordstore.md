---
id: adapter-store-recordstore
kind: doc
target_file: src/adapter/store.rs
line_hint: 11
author: 
created: 
title: "RecordStore"
tags: []
signature: "pub trait RecordStore : Send + Sync"
---

Storage backend driving records, cursor, and edit state for an adapter.

What a dataset adapter drives: the records, the cursor and the edit state live HERE.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store: Box<dyn RecordStore> = Box::new(MemoryStore::new(&["id"], Vec::new()));
assert_eq!(store.len(), 0);
```
